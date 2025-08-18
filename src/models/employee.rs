use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 社員情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: i32,
    pub employee_number: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub ad_username: Option<String>,
    pub department_id: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 社員作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub employee_number: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub ad_username: Option<String>,
    pub department_id: Option<i32>,
}

/// 社員更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateEmployeeRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub ad_username: Option<String>,
    pub department_id: Option<i32>,
    pub is_active: Option<bool>,
}

/// 社員検索条件
#[derive(Debug, Deserialize)]
pub struct EmployeeSearchQuery {
    pub name: Option<String>,
    pub employee_number: Option<String>,
    pub department_id: Option<i32>,
    pub is_active: Option<bool>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Default for EmployeeSearchQuery {
    fn default() -> Self {
        Self {
            name: None,
            employee_number: None,
            department_id: None,
            is_active: Some(true), // デフォルトでアクティブなユーザーのみ
            limit: Some(50),
            offset: Some(0),
        }
    }
}
