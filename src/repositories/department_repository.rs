use crate::models::{
    CreateDepartmentRequest, Department, DepartmentSearchFilters, DepartmentWithManager,
    UpdateDepartmentRequest,
};
use sqlx::{Pool, Row, Sqlite};

#[derive(Clone)]
pub struct DepartmentRepository {
    pool: Pool<Sqlite>,
}

impl DepartmentRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn new_with_file_db(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_all_departments(&self) -> Result<Vec<DepartmentWithManager>, sqlx::Error> {
        let query = r#"
            SELECT 
                d.id,
                d.code,
                d.name,
                d.parent_id,
                parent_dept.name as parent_name,
                d.level,
                d.manager_id,
                manager.name as manager_name,
                d.description,
                d.location,
                d.phone_number,
                d.email,
                d.budget,
                d.is_active,
                COUNT(e.id) as employee_count,
                d.created_date,
                d.created_at,
                d.updated_at
            FROM departments d
            LEFT JOIN departments parent_dept ON d.parent_id = parent_dept.id
            LEFT JOIN employees manager ON d.manager_id = manager.id
            LEFT JOIN employees e ON d.code = e.department AND e.is_active = 1
            GROUP BY d.id, d.code, d.name, d.parent_id, parent_dept.name, d.level, 
                     d.manager_id, manager.name, d.description, d.location, d.phone_number, 
                     d.email, d.budget, d.is_active, d.created_date, d.created_at, d.updated_at
            ORDER BY d.level, d.code
        "#;

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let mut departments = Vec::new();
        for row in rows {
            departments.push(DepartmentWithManager {
                id: row.get("id"),
                code: row.get("code"),
                name: row.get("name"),
                parent_id: row.get("parent_id"),
                parent_name: row.get("parent_name"),
                level: row.get("level"),
                manager_id: row.get("manager_id"),
                manager_name: row.get("manager_name"),
                description: row.get("description"),
                location: row.get("location"),
                phone_number: row.get("phone_number"),
                email: row.get("email"),
                budget: row.get("budget"),
                is_active: row.get::<i32, _>("is_active") == 1,
                employee_count: row.get("employee_count"),
                created_date: row.get("created_date"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(departments)
    }

    pub async fn get_department_by_id(
        &self,
        id: i32,
    ) -> Result<Option<DepartmentWithManager>, sqlx::Error> {
        let query = r#"
            SELECT 
                d.id,
                d.code,
                d.name,
                d.parent_id,
                parent_dept.name as parent_name,
                d.level,
                d.manager_id,
                manager.name as manager_name,
                d.description,
                d.location,
                d.phone_number,
                d.email,
                d.budget,
                d.is_active,
                COUNT(e.id) as employee_count,
                d.created_date,
                d.created_at,
                d.updated_at
            FROM departments d
            LEFT JOIN departments parent_dept ON d.parent_id = parent_dept.id
            LEFT JOIN employees manager ON d.manager_id = manager.id
            LEFT JOIN employees e ON d.code = e.department AND e.is_active = 1
            WHERE d.id = ?
            GROUP BY d.id, d.code, d.name, d.parent_id, parent_dept.name, d.level, 
                     d.manager_id, manager.name, d.description, d.location, d.phone_number, 
                     d.email, d.budget, d.is_active, d.created_date, d.created_at, d.updated_at
        "#;

        let row = sqlx::query(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| DepartmentWithManager {
            id: row.get("id"),
            code: row.get("code"),
            name: row.get("name"),
            parent_id: row.get("parent_id"),
            parent_name: row.get("parent_name"),
            level: row.get("level"),
            manager_id: row.get("manager_id"),
            manager_name: row.get("manager_name"),
            description: row.get("description"),
            location: row.get("location"),
            phone_number: row.get("phone_number"),
            email: row.get("email"),
            budget: row.get("budget"),
            is_active: row.get::<i32, _>("is_active") == 1,
            employee_count: row.get("employee_count"),
            created_date: row.get("created_date"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
    }

    pub async fn search_departments(
        &self,
        filters: &DepartmentSearchFilters,
    ) -> Result<Vec<DepartmentWithManager>, sqlx::Error> {
        // For now, simplify by using get_all_departments and applying filters in Rust
        // TODO: Implement proper SQL-level filtering for better performance
        let mut departments = self.get_all_departments().await?;

        // Apply filters
        if let Some(name_filter) = &filters.name {
            departments.retain(|d| d.name.to_lowercase().contains(&name_filter.to_lowercase()));
        }

        if let Some(code_filter) = &filters.code {
            departments.retain(|d| d.code.to_lowercase().contains(&code_filter.to_lowercase()));
        }

        if let Some(is_active_filter) = filters.is_active {
            departments.retain(|d| d.is_active == is_active_filter);
        }

        if let Some(manager_name_filter) = &filters.manager_name {
            departments.retain(|d| {
                if let Some(manager_name) = &d.manager_name {
                    manager_name
                        .to_lowercase()
                        .contains(&manager_name_filter.to_lowercase())
                } else {
                    false
                }
            });
        }

        // Apply pagination
        let start = filters.offset as usize;
        let end = (start + filters.limit as usize).min(departments.len());

        if start >= departments.len() {
            return Ok(vec![]);
        }

        Ok(departments[start..end].to_vec())
    }

    pub async fn create_department(
        &self,
        request: &CreateDepartmentRequest,
    ) -> Result<Department, sqlx::Error> {
        let now = chrono::Utc::now().naive_utc();

        let result = sqlx::query(r#"
            INSERT INTO departments (code, name, parent_id, manager_id, description, location, 
                                   phone_number, email, budget, created_date, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(&request.code)
        .bind(&request.name)
        .bind(request.parent_id)
        .bind(request.manager_id)
        .bind(&request.description)
        .bind(&request.location)
        .bind(&request.phone_number)
        .bind(&request.email)
        .bind(request.budget)
        .bind(request.created_date)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let department_id = result.last_insert_rowid() as i32;

        // Return the created department
        let department = sqlx::query_as::<_, Department>(
            r#"
            SELECT * FROM departments WHERE id = ?
        "#,
        )
        .bind(department_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(department)
    }

    pub async fn update_department(
        &self,
        id: i32,
        request: &UpdateDepartmentRequest,
    ) -> Result<Option<Department>, sqlx::Error> {
        // For now, implement a simplified version that handles each field individually
        // TODO: Improve with dynamic query building

        let now = chrono::Utc::now().naive_utc();

        // Execute individual update statements for each provided field
        if let Some(code) = &request.code {
            sqlx::query("UPDATE departments SET code = ?, updated_at = ? WHERE id = ?")
                .bind(code)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(name) = &request.name {
            sqlx::query("UPDATE departments SET name = ?, updated_at = ? WHERE id = ?")
                .bind(name)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(parent_id) = request.parent_id {
            sqlx::query("UPDATE departments SET parent_id = ?, updated_at = ? WHERE id = ?")
                .bind(parent_id)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(manager_id) = request.manager_id {
            sqlx::query("UPDATE departments SET manager_id = ?, updated_at = ? WHERE id = ?")
                .bind(manager_id)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(description) = &request.description {
            sqlx::query("UPDATE departments SET description = ?, updated_at = ? WHERE id = ?")
                .bind(description)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(location) = &request.location {
            sqlx::query("UPDATE departments SET location = ?, updated_at = ? WHERE id = ?")
                .bind(location)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(phone_number) = &request.phone_number {
            sqlx::query("UPDATE departments SET phone_number = ?, updated_at = ? WHERE id = ?")
                .bind(phone_number)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(email) = &request.email {
            sqlx::query("UPDATE departments SET email = ?, updated_at = ? WHERE id = ?")
                .bind(email)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(budget) = request.budget {
            sqlx::query("UPDATE departments SET budget = ?, updated_at = ? WHERE id = ?")
                .bind(budget)
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(is_active) = request.is_active {
            sqlx::query("UPDATE departments SET is_active = ?, updated_at = ? WHERE id = ?")
                .bind(if is_active { 1 } else { 0 })
                .bind(now)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        self.get_department_by_id_simple(id).await
    }

    pub async fn delete_department(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result =
            sqlx::query("UPDATE departments SET is_active = 0, updated_at = ? WHERE id = ?")
                .bind(chrono::Utc::now().naive_utc())
                .bind(id)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_department_by_id_simple(
        &self,
        id: i32,
    ) -> Result<Option<Department>, sqlx::Error> {
        let department = sqlx::query_as::<_, Department>("SELECT * FROM departments WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(department)
    }
}
