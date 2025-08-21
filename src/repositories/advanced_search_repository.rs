use crate::error::SearchError;
use crate::models::{
    AdvancedEmployeeSearchInput, AutocompleteResult, AutocompleteSuggestion, DepartmentCount,
    Employee, EmployeeAutocompleteInput, EmployeeSearchAggregations, EmployeeSearchField,
    EmployeeSearchResult, EmployeeSortField, PositionCount, SortOrder, StatusCount,
};
use crate::services::UserPermissions;
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

#[async_trait]
pub trait AdvancedSearchRepository: Send + Sync {
    async fn search_employees_advanced(
        &self,
        filters: AdvancedEmployeeSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<EmployeeSearchResult, SearchError>;

    async fn employee_autocomplete(
        &self,
        input: EmployeeAutocompleteInput,
        user_permissions: &UserPermissions,
    ) -> Result<AutocompleteResult, SearchError>;

    async fn get_employee_search_aggregations(
        &self,
        filters: &AdvancedEmployeeSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<EmployeeSearchAggregations, SearchError>;
}

pub struct SqliteAdvancedSearchRepository {
    pool: SqlitePool,
}

impl SqliteAdvancedSearchRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AdvancedSearchRepository for SqliteAdvancedSearchRepository {
    async fn search_employees_advanced(
        &self,
        filters: AdvancedEmployeeSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<EmployeeSearchResult, SearchError> {
        let mut conditions = Vec::<String>::new();
        let mut joins = Vec::new();
        let mut bind_values = Vec::new();

        // 基本的な検索条件
        if let Some(name) = &filters.name {
            conditions.push("e.name LIKE ?".to_string());
            bind_values.push(format!("%{name}%"));
        }

        if let Some(employee_number) = &filters.employee_number {
            conditions.push("e.employee_number LIKE ?".to_string());
            bind_values.push(format!("%{employee_number}%"));
        }

        if let Some(email) = &filters.email {
            conditions.push("e.email LIKE ?".to_string());
            bind_values.push(format!("%{email}%"));
        }

        if let Some(department_id) = filters.department_id {
            joins.push(
                "LEFT JOIN department_assignments da ON e.id = da.employee_id 
                 AND (da.end_date IS NULL OR da.end_date >= DATE('now'))",
            );
            conditions.push("da.department_id = ?".to_string());
            bind_values.push(department_id.to_string());
        }

        // Position field not available in current Employee model
        // Removed current_position filter

        // 業務従事経験フィルター
        if let Some(business_number) = &filters.has_business_experience {
            joins.push(
                "LEFT JOIN business_members bm ON e.id = bm.employee_id
                 LEFT JOIN businesses b ON bm.business_id = b.id",
            );
            conditions.push("b.business_number LIKE ?".to_string());
            bind_values.push(format!("%{business_number}%"));
        }

        // Joining date fields not available in current Employee model
        // Removed joining_date filters

        // アクティブ状態
        if let Some(is_active) = filters.is_active {
            conditions.push("e.is_active = ?".to_string());
            bind_values.push(is_active.to_string());
        }

        // 権限による部署フィルター
        if !user_permissions.accessible_departments.is_empty() {
            let dept_placeholders = user_permissions
                .accessible_departments
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(",");

            if !joins.iter().any(|j| j.contains("department_assignments")) {
                joins.push(
                    "LEFT JOIN department_assignments da ON e.id = da.employee_id 
                     AND (da.end_date IS NULL OR da.end_date >= DATE('now'))",
                );
            }

            conditions.push(format!("da.department_id IN ({dept_placeholders})"));
            for dept_id in &user_permissions.accessible_departments {
                bind_values.push(dept_id.to_string());
            }
        }

        let joins_clause = joins.join(" ");
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // ソート順設定
        let order_by = match filters.sort_by.as_ref().unwrap_or(&EmployeeSortField::Name) {
            EmployeeSortField::Name => "e.name",
            EmployeeSortField::EmployeeNumber => "e.employee_number",
            EmployeeSortField::CreatedAt => "e.created_at",
            EmployeeSortField::Department => "d.name",
            EmployeeSortField::LastUpdated => "e.updated_at",
        };

        let sort_order = match filters.sort_order.as_ref().unwrap_or(&SortOrder::Asc) {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };

        // メインクエリ実行
        let query = format!(
            r#"
            SELECT DISTINCT e.*, d.name as department_name
            FROM employees e
            LEFT JOIN department_assignments da2 ON e.id = da2.employee_id 
                AND (da2.end_date IS NULL OR da2.end_date >= DATE('now'))
            LEFT JOIN departments d ON da2.department_id = d.id
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

        let employees = rows
            .into_iter()
            .map(|row| Employee {
                id: row.get("id"),
                employee_number: row.get("employee_number"),
                name: row.get("name"),
                email: row.get("email"),
                ad_username: row.get("ad_username"),
                department_id: row.get("department_id"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect::<Vec<_>>();

        // 総件数取得
        let count_query = format!(
            "SELECT COUNT(DISTINCT e.id) as count FROM employees e {joins_clause} {where_clause}"
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
        let aggregations = if filters.pagination.offset == 0 {
            Some(
                self.get_employee_search_aggregations(&filters, user_permissions)
                    .await?,
            )
        } else {
            None
        };

        let employees_count = employees.len();
        Ok(EmployeeSearchResult {
            employees,
            total_count: total,
            has_next_page: employees_count as i64 == filters.pagination.limit as i64
                && total > (filters.pagination.offset + employees_count as i32) as i64,
            aggregations,
        })
    }

    async fn employee_autocomplete(
        &self,
        input: EmployeeAutocompleteInput,
        _user_permissions: &UserPermissions,
    ) -> Result<AutocompleteResult, SearchError> {
        let mut suggestions = Vec::new();
        let limit = input.limit.unwrap_or(10);

        match input.field {
            EmployeeSearchField::Name => {
                let query_text = r#"
                    SELECT DISTINCT e.name, e.employee_id, d.name as department_name
                    FROM employees e
                    LEFT JOIN department_assignments da ON e.id = da.employee_id 
                        AND (da.end_date IS NULL OR da.end_date >= DATE('now'))
                    LEFT JOIN departments d ON da.department_id = d.id
                    WHERE e.name LIKE ? 
                    AND (? OR e.is_active = 1)
                    ORDER BY e.name
                    LIMIT ?
                "#;

                let rows = sqlx::query(query_text)
                    .bind(format!("%{query}%", query = input.query))
                    .bind(input.include_inactive.unwrap_or(false))
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?;

                for row in rows {
                    let name: String = row.get("name");
                    let employee_id: String = row.get("employee_id");
                    let department_name: Option<String> = row.get("department_name");

                    suggestions.push(AutocompleteSuggestion {
                        value: name.clone(),
                        label: format!("{name} ({employee_id})"),
                        category: department_name,
                        metadata: Some(serde_json::json!({
                            "employeeId": employee_id
                        })),
                    });
                }
            }

            EmployeeSearchField::EmployeeNumber => {
                let query_text = r#"
                    SELECT DISTINCT e.employee_id, e.name, d.name as department_name
                    FROM employees e
                    LEFT JOIN department_assignments da ON e.id = da.employee_id 
                        AND (da.end_date IS NULL OR da.end_date >= DATE('now'))
                    LEFT JOIN departments d ON da.department_id = d.id
                    WHERE e.employee_id LIKE ?
                    AND (? OR e.is_active = 1)
                    ORDER BY e.employee_id
                    LIMIT ?
                "#;

                let rows = sqlx::query(query_text)
                    .bind(format!("%{query}%", query = input.query))
                    .bind(input.include_inactive.unwrap_or(false))
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?;

                for row in rows {
                    let employee_id: String = row.get("employee_id");
                    let name: String = row.get("name");
                    let department_name: Option<String> = row.get("department_name");

                    suggestions.push(AutocompleteSuggestion {
                        value: employee_id.clone(),
                        label: format!("{employee_id} - {name}"),
                        category: department_name,
                        metadata: Some(serde_json::json!({
                            "name": name
                        })),
                    });
                }
            }

            EmployeeSearchField::Department => {
                let query_text = r#"
                    SELECT DISTINCT d.name, d.code, COUNT(da.employee_id) as employee_count
                    FROM departments d
                    LEFT JOIN department_assignments da ON d.id = da.department_id 
                        AND (da.end_date IS NULL OR da.end_date >= DATE('now'))
                    WHERE d.name LIKE ?
                    GROUP BY d.id, d.name, d.code
                    ORDER BY d.name
                    LIMIT ?
                "#;

                let rows = sqlx::query(query_text)
                    .bind(format!("%{query}%", query = input.query))
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?;

                for row in rows {
                    let name: String = row.get("name");
                    let code: Option<String> = row.get("code");
                    let employee_count: i32 = row.get("employee_count");

                    suggestions.push(AutocompleteSuggestion {
                        value: name.clone(),
                        label: format!("{name} ({employee_count} 名)"),
                        category: Some("部署".to_string()),
                        metadata: Some(serde_json::json!({
                            "code": code,
                            "employeeCount": employee_count
                        })),
                    });
                }
            }

            EmployeeSearchField::Email => {
                let query_text = r#"
                    SELECT DISTINCT e.email, e.name, e.employee_id
                    FROM employees e
                    WHERE e.email LIKE ?
                    AND (? OR e.is_active = 1)
                    ORDER BY e.email
                    LIMIT ?
                "#;

                let rows = sqlx::query(query_text)
                    .bind(format!("%{query}%", query = input.query))
                    .bind(input.include_inactive.unwrap_or(false))
                    .bind(limit)
                    .fetch_all(&self.pool)
                    .await?;

                for row in rows {
                    let email: String = row.get("email");
                    let name: String = row.get("name");
                    let employee_id: String = row.get("employee_id");

                    suggestions.push(AutocompleteSuggestion {
                        value: email.clone(),
                        label: format!("{email} ({name})"),
                        category: Some("メールアドレス".to_string()),
                        metadata: Some(serde_json::json!({
                            "name": name,
                            "employeeId": employee_id
                        })),
                    });
                }
            }
        }

        Ok(AutocompleteResult { suggestions })
    }

    async fn get_employee_search_aggregations(
        &self,
        _filters: &AdvancedEmployeeSearchInput,
        _user_permissions: &UserPermissions,
    ) -> Result<EmployeeSearchAggregations, SearchError> {
        // 部署別集計
        let dept_query = r#"
            SELECT d.id, d.name, COUNT(da.employee_id) as count
            FROM departments d
            LEFT JOIN department_assignments da ON d.id = da.department_id
                AND (da.end_date IS NULL OR da.end_date >= DATE('now'))
            LEFT JOIN employees e ON da.employee_id = e.id AND e.is_active = 1
            GROUP BY d.id, d.name
            ORDER BY count DESC
        "#;

        let dept_rows = sqlx::query(dept_query).fetch_all(&self.pool).await?;
        let department_counts = dept_rows
            .into_iter()
            .map(|row| DepartmentCount {
                department_id: row.get("id"),
                department_name: row.get("name"),
                count: row.get("count"),
            })
            .collect();

        // 役職別集計
        let position_query = r#"
            SELECT position, COUNT(*) as count
            FROM employees
            WHERE is_active = 1 AND position IS NOT NULL
            GROUP BY position
            ORDER BY count DESC
        "#;

        let position_rows = sqlx::query(position_query).fetch_all(&self.pool).await?;
        let position_counts = position_rows
            .into_iter()
            .map(|row| PositionCount {
                position: row.get("position"),
                count: row.get("count"),
            })
            .collect();

        // ステータス別集計
        let status_query = r#"
            SELECT 
                SUM(CASE WHEN is_active = 1 THEN 1 ELSE 0 END) as active,
                SUM(CASE WHEN is_active = 0 THEN 1 ELSE 0 END) as inactive
            FROM employees
        "#;

        let status_row = sqlx::query(status_query).fetch_one(&self.pool).await?;
        let status_counts = StatusCount {
            active: status_row.get("active"),
            inactive: status_row.get("inactive"),
        };

        Ok(EmployeeSearchAggregations {
            department_counts,
            position_counts,
            status_counts,
        })
    }
}
