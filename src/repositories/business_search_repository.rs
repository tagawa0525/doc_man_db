use crate::error::SearchError;
use crate::models::{
    AdvancedBusinessSearchInput, AutocompleteResult, AutocompleteSuggestion, Business,
    BusinessAutocompleteInput, BusinessRole, BusinessSearchAggregations, BusinessSearchField,
    BusinessSearchResult, BusinessSortField, BusinessStatusCount, BusinessWithRole, CustomerCount,
    EmployeeBusinesses, MemberCount, SortOrder, YearCount,
};
use crate::services::UserPermissions;
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

#[async_trait]
pub trait BusinessSearchRepository: Send + Sync {
    async fn search_businesses_advanced(
        &self,
        filters: AdvancedBusinessSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessSearchResult, SearchError>;

    async fn business_autocomplete(
        &self,
        input: BusinessAutocompleteInput,
        user_permissions: &UserPermissions,
    ) -> Result<AutocompleteResult, SearchError>;

    async fn get_employee_businesses(
        &self,
        employee_id: i32,
        include_completed: bool,
        user_permissions: &UserPermissions,
    ) -> Result<EmployeeBusinesses, SearchError>;

    async fn get_business_search_aggregations(
        &self,
        filters: &AdvancedBusinessSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessSearchAggregations, SearchError>;
}

pub struct SqliteBusinessSearchRepository {
    pool: SqlitePool,
}

impl SqliteBusinessSearchRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BusinessSearchRepository for SqliteBusinessSearchRepository {
    async fn search_businesses_advanced(
        &self,
        filters: AdvancedBusinessSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessSearchResult, SearchError> {
        let mut conditions = Vec::new();
        let mut joins = Vec::new();
        let mut bind_values = Vec::new();

        // 基本的な検索条件
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

        if let Some(description) = &filters.description {
            conditions.push("b.description LIKE ?");
            bind_values.push(format!("%{description}%"));
        }

        if let Some(status) = &filters.status {
            conditions.push("b.status = ?");
            bind_values.push(String::from(status.clone()));
        }

        // 従事者フィルター
        if let Some(member_employee_id) = filters.member_employee_id {
            joins.push("LEFT JOIN business_members bm ON b.id = bm.business_id");
            conditions.push("bm.employee_id = ?");
            bind_values.push(member_employee_id.to_string());

            if let Some(member_role) = &filters.member_role {
                conditions.push("bm.role = ?");
                bind_values.push(String::from(member_role.clone()));
            }
        }

        // 日付範囲フィルター
        if let Some(start_date_from) = &filters.start_date_from {
            conditions.push("b.start_date >= ?");
            bind_values.push(start_date_from.to_string());
        }

        if let Some(start_date_to) = &filters.start_date_to {
            conditions.push("b.start_date <= ?");
            bind_values.push(start_date_to.to_string());
        }

        if let Some(end_date_from) = &filters.end_date_from {
            conditions.push("b.end_date >= ?");
            bind_values.push(end_date_from.to_string());
        }

        if let Some(end_date_to) = &filters.end_date_to {
            conditions.push("b.end_date <= ?");
            bind_values.push(end_date_to.to_string());
        }

        // ドキュメント有無フィルター
        if let Some(has_documents) = filters.has_documents {
            if has_documents {
                joins.push("LEFT JOIN documents d ON d.business_id = b.id");
                conditions.push("d.id IS NOT NULL");
            }
        }

        // 作成者フィルター
        if let Some(created_by) = filters.created_by {
            conditions.push("b.created_by = ?");
            bind_values.push(created_by.to_string());
        }

        // 権限による制限
        if !user_permissions.can_view_all_businesses {
            joins.push("LEFT JOIN business_members bm_perm ON b.id = bm_perm.business_id");
            conditions.push("bm_perm.employee_id = ?");
            bind_values.push(user_permissions.employee_id.to_string());
        }

        let joins_clause = joins.join(" ");
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // ソート順設定
        let order_by = match filters
            .sort_by
            .as_ref()
            .unwrap_or(&BusinessSortField::CreatedAt)
        {
            BusinessSortField::BusinessNumber => "b.business_number",
            BusinessSortField::Name => "b.name",
            BusinessSortField::CustomerName => "b.customer_name",
            BusinessSortField::StartDate => "b.start_date",
            BusinessSortField::EndDate => "b.end_date",
            BusinessSortField::CreatedAt => "b.created_at",
        };

        let sort_order = match filters.sort_order.as_ref().unwrap_or(&SortOrder::Desc) {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };

        // メインクエリ実行
        let query = format!(
            r#"
            SELECT DISTINCT b.*, e.name as creator_name
            FROM businesses b
            LEFT JOIN employees e ON b.created_by = e.id
            {joins_clause}
            {where_clause}
            ORDER BY {order_by} {sort_order}
            LIMIT ? OFFSET ?
            "#
        );

        let mut query_builder = sqlx::query(&query);
        for value in &bind_values {
            query_builder = query_builder.bind(value);
        }
        query_builder = query_builder
            .bind(filters.pagination.limit)
            .bind(filters.pagination.offset);

        let rows = query_builder.fetch_all(&self.pool).await?;

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
            .collect::<Vec<_>>();

        // 総件数取得
        let count_query = format!(
            "SELECT COUNT(DISTINCT b.id) as count FROM businesses b {joins_clause} {where_clause}"
        );

        let mut count_query_builder = sqlx::query(&count_query);
        for value in &bind_values {
            count_query_builder = count_query_builder.bind(value);
        }

        let total: i64 = count_query_builder
            .fetch_one(&self.pool)
            .await?
            .get("count");

        // 集計情報取得
        let aggregations = self
            .get_business_search_aggregations(&filters, user_permissions)
            .await?;

        let businesses_count = businesses.len();
        Ok(BusinessSearchResult {
            businesses,
            total_count: total,
            has_next_page: businesses_count as i64 == filters.pagination.limit as i64
                && total > (filters.pagination.offset + businesses_count as i32) as i64,
            aggregations,
        })
    }

    async fn business_autocomplete(
        &self,
        input: BusinessAutocompleteInput,
        _user_permissions: &UserPermissions,
    ) -> Result<AutocompleteResult, SearchError> {
        let mut suggestions = Vec::new();
        let limit = input.limit.unwrap_or(10);

        match input.field {
            BusinessSearchField::BusinessNumber => {
                let query_text = r#"
                    SELECT DISTINCT b.business_number, b.name, b.customer_name
                    FROM businesses b
                    WHERE b.business_number LIKE ?
                    AND (? OR b.status != 'completed')
                    ORDER BY b.business_number
                    LIMIT ?
                "#;

                let rows = sqlx::query(query_text)
                    .bind(format!("%{}%", input.query))
                    .bind(input.include_completed.unwrap_or(false))
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?;

                for row in rows {
                    let business_number: String = row.get("business_number");
                    let name: String = row.get("name");
                    let customer_name: Option<String> = row.get("customer_name");

                    suggestions.push(AutocompleteSuggestion {
                        value: business_number.clone(),
                        label: format!("{business_number} - {name}"),
                        category: customer_name,
                        metadata: Some(serde_json::json!({
                            "name": name
                        })),
                    });
                }
            }

            BusinessSearchField::Name => {
                let query_text = r#"
                    SELECT DISTINCT b.name, b.business_number, b.customer_name
                    FROM businesses b
                    WHERE b.name LIKE ?
                    AND (? OR b.status != 'completed')
                    ORDER BY b.name
                    LIMIT ?
                "#;

                let rows = sqlx::query(query_text)
                    .bind(format!("%{}%", input.query))
                    .bind(input.include_completed.unwrap_or(false))
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?;

                for row in rows {
                    let name: String = row.get("name");
                    let business_number: String = row.get("business_number");
                    let customer_name: Option<String> = row.get("customer_name");

                    suggestions.push(AutocompleteSuggestion {
                        value: name.clone(),
                        label: format!("{name} ({business_number})"),
                        category: customer_name,
                        metadata: Some(serde_json::json!({
                            "businessNumber": business_number
                        })),
                    });
                }
            }

            BusinessSearchField::CustomerName => {
                let query_text = r#"
                    SELECT DISTINCT b.customer_name, COUNT(*) as business_count
                    FROM businesses b
                    WHERE b.customer_name LIKE ?
                    AND b.customer_name IS NOT NULL
                    AND (? OR b.status != 'completed')
                    GROUP BY b.customer_name
                    ORDER BY business_count DESC
                    LIMIT ?
                "#;

                let rows = sqlx::query(query_text)
                    .bind(format!("%{}%", input.query))
                    .bind(input.include_completed.unwrap_or(false))
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?;

                for row in rows {
                    let customer_name: String = row.get("customer_name");
                    let business_count: i32 = row.get("business_count");

                    suggestions.push(AutocompleteSuggestion {
                        value: customer_name.clone(),
                        label: format!("{customer_name} ({business_count} 件)"),
                        category: Some("顧客".to_string()),
                        metadata: Some(serde_json::json!({
                            "businessCount": business_count
                        })),
                    });
                }
            }

            BusinessSearchField::Description => {
                let query_text = r#"
                    SELECT DISTINCT b.name, b.business_number, b.description
                    FROM businesses b
                    WHERE b.description LIKE ?
                    AND b.description IS NOT NULL
                    AND (? OR b.status != 'completed')
                    ORDER BY b.name
                    LIMIT ?
                "#;

                let rows = sqlx::query(query_text)
                    .bind(format!("%{}%", input.query))
                    .bind(input.include_completed.unwrap_or(false))
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?;

                for row in rows {
                    let name: String = row.get("name");
                    let business_number: String = row.get("business_number");
                    let description: String = row.get("description");

                    suggestions.push(AutocompleteSuggestion {
                        value: description.clone(),
                        label: format!("{name} ({business_number})"),
                        category: Some("説明".to_string()),
                        metadata: Some(serde_json::json!({
                            "name": name,
                            "businessNumber": business_number
                        })),
                    });
                }
            }
        }

        Ok(AutocompleteResult { suggestions })
    }

    async fn get_employee_businesses(
        &self,
        employee_id: i32,
        include_completed: bool,
        _user_permissions: &UserPermissions,
    ) -> Result<EmployeeBusinesses, SearchError> {
        let status_condition = if include_completed {
            String::new()
        } else {
            "AND b.status != 'completed'".to_string()
        };

        let query = format!(
            r#"
            SELECT 
                b.*, bm.role, bm.participation_level, 
                bm.start_date as member_start_date, bm.end_date as member_end_date,
                e.name as employee_name
            FROM businesses b
            JOIN business_members bm ON b.id = bm.business_id
            JOIN employees e ON bm.employee_id = e.id
            WHERE bm.employee_id = ?
            {status_condition}
            ORDER BY bm.start_date DESC
        "#
        );

        let rows = sqlx::query(&query)
            .bind(employee_id)
            .fetch_all(&self.pool)
            .await?;

        let employee_name = if !rows.is_empty() {
            rows[0].get("employee_name")
        } else {
            "Unknown".to_string()
        };

        let businesses = rows
            .into_iter()
            .map(|row| {
                let business = Business {
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
                };

                let member_end_date: Option<chrono::NaiveDate> = row.get("member_end_date");
                let is_active =
                    member_end_date.is_none_or(|end| end >= chrono::Utc::now().date_naive());

                BusinessWithRole {
                    business,
                    role: BusinessRole::from(row.get::<String, _>("role")),
                    participation_level: row.get("participation_level"),
                    start_date: row.get("member_start_date"),
                    end_date: member_end_date,
                    is_active,
                }
            })
            .collect::<Vec<_>>();

        Ok(EmployeeBusinesses {
            employee_id,
            employee_name,
            total_count: businesses.len() as i32,
            businesses,
        })
    }

    async fn get_business_search_aggregations(
        &self,
        _filters: &AdvancedBusinessSearchInput,
        _user_permissions: &UserPermissions,
    ) -> Result<BusinessSearchAggregations, SearchError> {
        // ステータス別集計
        let status_query = r#"
            SELECT status, COUNT(*) as count
            FROM businesses
            GROUP BY status
            ORDER BY count DESC
        "#;

        let status_rows = sqlx::query(status_query).fetch_all(&self.pool).await?;
        let status_counts = status_rows
            .into_iter()
            .map(|row| BusinessStatusCount {
                status: row.get("status"),
                count: row.get("count"),
            })
            .collect();

        // 顧客別集計
        let customer_query = r#"
            SELECT customer_name, COUNT(*) as count
            FROM businesses
            WHERE customer_name IS NOT NULL
            GROUP BY customer_name
            ORDER BY count DESC
            LIMIT 10
        "#;

        let customer_rows = sqlx::query(customer_query).fetch_all(&self.pool).await?;
        let customer_counts = customer_rows
            .into_iter()
            .map(|row| CustomerCount {
                customer_name: row.get("customer_name"),
                count: row.get("count"),
            })
            .collect();

        // 従事者別集計（上位10名）
        let member_query = r#"
            SELECT bm.employee_id, e.name as employee_name, COUNT(*) as count
            FROM business_members bm
            JOIN employees e ON bm.employee_id = e.id
            GROUP BY bm.employee_id, e.name
            ORDER BY count DESC
            LIMIT 10
        "#;

        let member_rows = sqlx::query(member_query).fetch_all(&self.pool).await?;
        let member_counts = member_rows
            .into_iter()
            .map(|row| MemberCount {
                employee_id: row.get("employee_id"),
                employee_name: row.get("employee_name"),
                count: row.get("count"),
            })
            .collect();

        // 年度別集計
        let year_query = r#"
            SELECT 
                strftime('%Y', start_date) as year,
                COUNT(*) as count
            FROM businesses
            WHERE start_date IS NOT NULL
            GROUP BY strftime('%Y', start_date)
            ORDER BY year DESC
        "#;

        let year_rows = sqlx::query(year_query).fetch_all(&self.pool).await?;
        let year_counts = year_rows
            .into_iter()
            .map(|row| YearCount {
                year: row.get::<String, _>("year").parse().unwrap_or(0),
                count: row.get("count"),
            })
            .collect();

        Ok(BusinessSearchAggregations {
            status_counts,
            customer_counts,
            member_counts,
            year_counts,
        })
    }
}
