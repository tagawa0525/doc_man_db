use crate::error::BusinessError;
use crate::models::{
    Business, BusinessMember, BusinessSearchFilters, CreateBusinessMemberRequest,
    CreateBusinessRequest, ExternalContact, UpdateBusinessMemberRequest, UpdateBusinessRequest,
};
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

#[async_trait]
pub trait BusinessRepository: Send + Sync {
    async fn create(&self, business: CreateBusinessRequest) -> Result<Business, BusinessError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Business>, BusinessError>;
    async fn get_by_business_number(
        &self,
        business_number: &str,
    ) -> Result<Option<Business>, BusinessError>;
    async fn search(
        &self,
        filters: BusinessSearchFilters,
    ) -> Result<(Vec<Business>, i64), BusinessError>;
    async fn update(
        &self,
        id: i32,
        business: UpdateBusinessRequest,
    ) -> Result<Business, BusinessError>;
    async fn delete(&self, id: i32) -> Result<(), BusinessError>;

    // 業務従事者関連
    async fn get_members(&self, business_id: i32) -> Result<Vec<BusinessMember>, BusinessError>;
    async fn get_member_by_id(
        &self,
        member_id: i32,
    ) -> Result<Option<BusinessMember>, BusinessError>;
    async fn add_member(
        &self,
        member: CreateBusinessMemberRequest,
    ) -> Result<BusinessMember, BusinessError>;
    async fn update_member(
        &self,
        member_id: i32,
        member: UpdateBusinessMemberRequest,
    ) -> Result<BusinessMember, BusinessError>;
    async fn remove_member(&self, member_id: i32) -> Result<(), BusinessError>;
    async fn get_member_history(
        &self,
        employee_id: i32,
    ) -> Result<Vec<BusinessMember>, BusinessError>;

    // 外部連絡先関連
    async fn get_external_contacts(
        &self,
        business_id: i32,
    ) -> Result<Vec<ExternalContact>, BusinessError>;
    async fn add_external_contact(
        &self,
        business_id: i32,
        external_contact_id: i32,
    ) -> Result<(), BusinessError>;
    async fn remove_external_contact(
        &self,
        business_id: i32,
        external_contact_id: i32,
    ) -> Result<(), BusinessError>;

    // 業務番号生成
    async fn generate_business_number(&self) -> Result<String, BusinessError>;
}

pub struct SqliteBusinessRepository {
    pool: SqlitePool,
}

impl SqliteBusinessRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BusinessRepository for SqliteBusinessRepository {
    async fn create(&self, request: CreateBusinessRequest) -> Result<Business, BusinessError> {
        let mut tx = self.pool.begin().await?;

        // 業務番号生成（未指定の場合）
        let business_number = if let Some(number) = request.business_number {
            // 重複チェック
            if self.get_by_business_number(&number).await?.is_some() {
                return Err(BusinessError::BusinessNumberExists(number));
            }
            number
        } else {
            self.generate_business_number().await?
        };

        let business_id = sqlx::query(
            r#"
            INSERT INTO businesses (
                business_number, name, description, customer_name, 
                start_date, end_date, status, created_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&business_number)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.customer_name)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.status.unwrap_or_else(|| "active".to_string()))
        .bind(request.created_by)
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();

        tx.commit().await?;

        self.get_by_id(business_id as i32)
            .await?
            .ok_or(BusinessError::NotFound)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Business>, BusinessError> {
        let row = sqlx::query(
            r#"
            SELECT b.*, e.name as creator_name
            FROM businesses b
            JOIN employees e ON b.created_by = e.id
            WHERE b.id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Business {
                id: row.get("id"),
                business_number: row.get("business_number"),
                name: row.get("name"),
                description: row.get("description"),
                customer_name: row.get("customer_name"),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                status: row.get::<String, _>("status").into(),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_by_business_number(
        &self,
        business_number: &str,
    ) -> Result<Option<Business>, BusinessError> {
        let row = sqlx::query(
            r#"
            SELECT b.*, e.name as creator_name
            FROM businesses b
            JOIN employees e ON b.created_by = e.id
            WHERE b.business_number = ?
            "#,
        )
        .bind(business_number)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Business {
                id: row.get("id"),
                business_number: row.get("business_number"),
                name: row.get("name"),
                description: row.get("description"),
                customer_name: row.get("customer_name"),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                status: row.get::<String, _>("status").into(),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn search(
        &self,
        filters: BusinessSearchFilters,
    ) -> Result<(Vec<Business>, i64), BusinessError> {
        let mut conditions = Vec::new();
        let mut bind_values = Vec::new();

        if let Some(business_number) = &filters.business_number {
            conditions.push("b.business_number LIKE ?");
            bind_values.push(format!("%{business_number}%"));
        }

        if let Some(name) = &filters.name {
            conditions.push("b.name LIKE ?");
            bind_values.push(format!("%{name}%"));
        }

        if let Some(customer_name) = &filters.customer_name {
            conditions.push("b.customer_name LIKE ?");
            bind_values.push(format!("%{customer_name}%"));
        }

        if let Some(status) = &filters.status {
            conditions.push("b.status = ?");
            bind_values.push(status.clone());
        }

        if let Some(member_employee_id) = filters.member_employee_id {
            conditions.push(
                "EXISTS (
                    SELECT 1 FROM business_members bm 
                    WHERE bm.business_id = b.id 
                    AND bm.employee_id = ?
                    AND (bm.end_date IS NULL OR bm.end_date >= DATE('now'))
                )",
            );
            bind_values.push(member_employee_id.to_string());
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // 総件数取得
        let count_query = format!("SELECT COUNT(*) as count FROM businesses b {where_clause}");

        let mut count_query_builder = sqlx::query(&count_query);
        for value in &bind_values {
            count_query_builder = count_query_builder.bind(value);
        }

        let total: i64 = count_query_builder
            .fetch_one(&self.pool)
            .await?
            .get("count");

        // データ取得
        let data_query = format!(
            r#"
            SELECT b.*, e.name as creator_name
            FROM businesses b
            JOIN employees e ON b.created_by = e.id
            {where_clause}
            ORDER BY b.created_at DESC
            LIMIT ? OFFSET ?
            "#
        );

        let mut data_query_builder = sqlx::query(&data_query);
        for value in &bind_values {
            data_query_builder = data_query_builder.bind(value);
        }
        data_query_builder = data_query_builder
            .bind(filters.pagination.limit)
            .bind(filters.pagination.offset);

        let rows = data_query_builder.fetch_all(&self.pool).await?;

        let businesses = rows
            .into_iter()
            .map(|row| Business {
                id: row.get("id"),
                business_number: row.get("business_number"),
                name: row.get("name"),
                description: row.get("description"),
                customer_name: row.get("customer_name"),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                status: row.get::<String, _>("status").into(),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok((businesses, total))
    }

    async fn update(
        &self,
        id: i32,
        request: UpdateBusinessRequest,
    ) -> Result<Business, BusinessError> {
        let mut set_clauses = Vec::new();
        let mut bind_values = Vec::new();

        if let Some(name) = request.name {
            set_clauses.push("name = ?");
            bind_values.push(name);
        }

        if let Some(description) = request.description {
            set_clauses.push("description = ?");
            bind_values.push(description);
        }

        if let Some(customer_name) = request.customer_name {
            set_clauses.push("customer_name = ?");
            bind_values.push(customer_name);
        }

        if let Some(start_date) = request.start_date {
            set_clauses.push("start_date = ?");
            bind_values.push(start_date.to_string());
        }

        if let Some(end_date) = request.end_date {
            set_clauses.push("end_date = ?");
            bind_values.push(end_date.to_string());
        }

        if let Some(status) = request.status {
            set_clauses.push("status = ?");
            bind_values.push(status);
        }

        if set_clauses.is_empty() {
            return self.get_by_id(id).await?.ok_or(BusinessError::NotFound);
        }

        set_clauses.push("updated_at = CURRENT_TIMESTAMP");

        let update_query = format!(
            "UPDATE businesses SET {} WHERE id = ?",
            set_clauses.join(", ")
        );

        let mut query_builder = sqlx::query(&update_query);
        for value in bind_values {
            query_builder = query_builder.bind(value);
        }
        query_builder = query_builder.bind(id);

        let result = query_builder.execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return Err(BusinessError::NotFound);
        }

        self.get_by_id(id).await?.ok_or(BusinessError::NotFound)
    }

    async fn delete(&self, id: i32) -> Result<(), BusinessError> {
        let result = sqlx::query("DELETE FROM businesses WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(BusinessError::NotFound);
        }

        Ok(())
    }

    async fn get_members(&self, business_id: i32) -> Result<Vec<BusinessMember>, BusinessError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                bm.*,
                b.business_number,
                b.name as business_name,
                e.name as employee_name,
                e.employee_number,
                e.email as employee_email
            FROM business_members bm
            JOIN businesses b ON bm.business_id = b.id
            JOIN employees e ON bm.employee_id = e.id
            WHERE bm.business_id = ?
            ORDER BY bm.start_date DESC
            "#,
        )
        .bind(business_id)
        .fetch_all(&self.pool)
        .await?;

        let members = rows
            .into_iter()
            .map(|row| BusinessMember {
                id: row.get("id"),
                business_id: row.get("business_id"),
                employee_id: row.get("employee_id"),
                role: row.get::<String, _>("role").into(),
                participation_level: row.get::<String, _>("participation_level").into(),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                notes: row.get("notes"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(members)
    }

    async fn get_member_by_id(
        &self,
        member_id: i32,
    ) -> Result<Option<BusinessMember>, BusinessError> {
        let row = sqlx::query(
            r#"
            SELECT 
                bm.*,
                b.business_number,
                b.name as business_name,
                e.name as employee_name
            FROM business_members bm
            JOIN businesses b ON bm.business_id = b.id
            JOIN employees e ON bm.employee_id = e.id
            WHERE bm.id = ?
            "#,
        )
        .bind(member_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(BusinessMember {
                id: row.get("id"),
                business_id: row.get("business_id"),
                employee_id: row.get("employee_id"),
                role: row.get::<String, _>("role").into(),
                participation_level: row.get::<String, _>("participation_level").into(),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                notes: row.get("notes"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn add_member(
        &self,
        request: CreateBusinessMemberRequest,
    ) -> Result<BusinessMember, BusinessError> {
        // 重複チェック
        let existing = sqlx::query(
            r#"
            SELECT id FROM business_members
            WHERE business_id = ? AND employee_id = ? 
            AND (end_date IS NULL OR end_date >= ?)
            "#,
        )
        .bind(request.business_id)
        .bind(request.employee_id)
        .bind(request.start_date)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            return Err(BusinessError::MemberAlreadyExists);
        }

        let member_id = sqlx::query(
            r#"
            INSERT INTO business_members (
                business_id, employee_id, role, participation_level,
                start_date, end_date, notes, created_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(request.business_id)
        .bind(request.employee_id)
        .bind(&request.role)
        .bind(&request.participation_level)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(&request.notes)
        .bind(request.created_by)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.get_member_by_id(member_id as i32)
            .await?
            .ok_or(BusinessError::NotFound)
    }

    async fn update_member(
        &self,
        member_id: i32,
        request: UpdateBusinessMemberRequest,
    ) -> Result<BusinessMember, BusinessError> {
        let mut set_clauses = Vec::new();
        let mut bind_values = Vec::new();

        if let Some(role) = request.role {
            set_clauses.push("role = ?");
            bind_values.push(role);
        }

        if let Some(participation_level) = request.participation_level {
            set_clauses.push("participation_level = ?");
            bind_values.push(participation_level);
        }

        if let Some(start_date) = request.start_date {
            set_clauses.push("start_date = ?");
            bind_values.push(start_date.to_string());
        }

        if let Some(end_date) = request.end_date {
            set_clauses.push("end_date = ?");
            bind_values.push(end_date.to_string());
        }

        if let Some(notes) = request.notes {
            set_clauses.push("notes = ?");
            bind_values.push(notes);
        }

        if set_clauses.is_empty() {
            return self
                .get_member_by_id(member_id)
                .await?
                .ok_or(BusinessError::NotFound);
        }

        set_clauses.push("updated_at = CURRENT_TIMESTAMP");

        let update_query = format!(
            "UPDATE business_members SET {} WHERE id = ?",
            set_clauses.join(", ")
        );

        let mut query_builder = sqlx::query(&update_query);
        for value in bind_values {
            query_builder = query_builder.bind(value);
        }
        query_builder = query_builder.bind(member_id);

        let result = query_builder.execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return Err(BusinessError::NotFound);
        }

        self.get_member_by_id(member_id)
            .await?
            .ok_or(BusinessError::NotFound)
    }

    async fn remove_member(&self, member_id: i32) -> Result<(), BusinessError> {
        let result = sqlx::query("DELETE FROM business_members WHERE id = ?")
            .bind(member_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(BusinessError::NotFound);
        }

        Ok(())
    }

    async fn get_member_history(
        &self,
        employee_id: i32,
    ) -> Result<Vec<BusinessMember>, BusinessError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                bm.*,
                b.business_number,
                b.name as business_name,
                e.name as employee_name
            FROM business_members bm
            JOIN businesses b ON bm.business_id = b.id
            JOIN employees e ON bm.employee_id = e.id
            WHERE bm.employee_id = ?
            ORDER BY bm.start_date DESC
            "#,
        )
        .bind(employee_id)
        .fetch_all(&self.pool)
        .await?;

        let members = rows
            .into_iter()
            .map(|row| BusinessMember {
                id: row.get("id"),
                business_id: row.get("business_id"),
                employee_id: row.get("employee_id"),
                role: row.get::<String, _>("role").into(),
                participation_level: row.get::<String, _>("participation_level").into(),
                start_date: row.get("start_date"),
                end_date: row.get("end_date"),
                notes: row.get("notes"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(members)
    }

    async fn get_external_contacts(
        &self,
        business_id: i32,
    ) -> Result<Vec<ExternalContact>, BusinessError> {
        let rows = sqlx::query(
            r#"
            SELECT ec.*
            FROM external_contacts ec
            JOIN business_external_contacts bec ON ec.id = bec.external_contact_id
            WHERE bec.business_id = ? AND ec.is_active = 1
            ORDER BY ec.name
            "#,
        )
        .bind(business_id)
        .fetch_all(&self.pool)
        .await?;

        let contacts = rows
            .into_iter()
            .map(|row| ExternalContact {
                id: row.get("id"),
                name: row.get("name"),
                company_name: row.get("company_name"),
                email: row.get("email"),
                phone: row.get("phone"),
                address: row.get("address"),
                contact_type: row.get::<String, _>("contact_type").into(),
                is_active: row.get("is_active"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(contacts)
    }

    async fn add_external_contact(
        &self,
        business_id: i32,
        external_contact_id: i32,
    ) -> Result<(), BusinessError> {
        sqlx::query(
            r#"
            INSERT INTO business_external_contacts (business_id, external_contact_id)
            VALUES (?, ?)
            "#,
        )
        .bind(business_id)
        .bind(external_contact_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_external_contact(
        &self,
        business_id: i32,
        external_contact_id: i32,
    ) -> Result<(), BusinessError> {
        sqlx::query(
            r#"
            DELETE FROM business_external_contacts 
            WHERE business_id = ? AND external_contact_id = ?
            "#,
        )
        .bind(business_id)
        .bind(external_contact_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn generate_business_number(&self) -> Result<String, BusinessError> {
        let current_year = chrono::Utc::now().format("%y").to_string();

        // 今年度の最大番号を取得
        let max_number = sqlx::query(
            r#"
            SELECT business_number 
            FROM businesses 
            WHERE business_number LIKE ?
            ORDER BY business_number DESC 
            LIMIT 1
            "#,
        )
        .bind(format!("BUS-{current_year}%"))
        .fetch_optional(&self.pool)
        .await?;

        let next_number = if let Some(row) = max_number {
            let current_number: String = row.get("business_number");
            let number_part: i32 = current_number
                .split('-')
                .nth(1)
                .and_then(|s| s[2..].parse().ok())
                .unwrap_or(0);
            number_part + 1
        } else {
            1
        };

        Ok(format!("BUS-{current_year}{next_number:03}"))
    }
}
