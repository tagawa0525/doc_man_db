use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Department model for organizational management
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Department {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub parent_id: Option<i32>,
    pub level: i32,
    pub manager_id: Option<i32>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub budget: Option<f64>,
    pub is_active: bool,
    pub created_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Department with manager name (joined data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentWithManager {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub parent_id: Option<i32>,
    pub parent_name: Option<String>,
    pub level: i32,
    pub manager_id: Option<i32>,
    pub manager_name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub budget: Option<f64>,
    pub is_active: bool,
    pub employee_count: i64,
    pub created_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Department search filters
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DepartmentSearchFilters {
    pub name: Option<String>,
    pub code: Option<String>,
    pub is_active: Option<bool>,
    pub manager_name: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

/// Department creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDepartmentRequest {
    pub code: String,
    pub name: String,
    pub parent_id: Option<i32>,
    pub manager_id: Option<i32>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub budget: Option<f64>,
    pub created_date: Option<NaiveDate>,
}

/// Department update request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDepartmentRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<i32>,
    pub manager_id: Option<i32>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub budget: Option<f64>,
    pub is_active: Option<bool>,
}
