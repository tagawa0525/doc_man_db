use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::models::{Employee, PaginationInput};

// 高度社員検索用の入力型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedEmployeeSearchInput {
    pub name: Option<String>,
    pub employee_id: Option<String>,
    pub email: Option<String>,
    pub department_id: Option<i32>,
    pub current_position: Option<String>,
    pub has_business_experience: Option<String>, // 業務番号
    pub joining_date_from: Option<NaiveDate>,
    pub joining_date_to: Option<NaiveDate>,
    pub is_active: Option<bool>,
    pub skill_keywords: Option<Vec<String>>,
    pub sort_by: Option<EmployeeSortField>,
    pub sort_order: Option<SortOrder>,
    pub pagination: PaginationInput,
}

// オートコンプリート用の入力型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeAutocompleteInput {
    pub query: String,
    pub field: EmployeeSearchField,
    pub limit: Option<i32>,
    pub include_inactive: Option<bool>,
}

impl Default for EmployeeAutocompleteInput {
    fn default() -> Self {
        Self {
            query: String::new(),
            field: EmployeeSearchField::Name,
            limit: Some(10),
            include_inactive: Some(false),
        }
    }
}

// 検索フィールド列挙型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmployeeSearchField {
    Name,
    EmployeeId,
    Email,
    Department,
}

// ソートフィールド列挙型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmployeeSortField {
    Name,
    EmployeeId,
    JoiningDate,
    Department,
    LastUpdated,
}

// ソート順列挙型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

// 検索結果型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeSearchResult {
    pub employees: Vec<Employee>,
    pub total_count: i64,
    pub has_next_page: bool,
    pub aggregations: Option<EmployeeSearchAggregations>,
}

// 検索集計情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeSearchAggregations {
    pub department_counts: Vec<DepartmentCount>,
    pub position_counts: Vec<PositionCount>,
    pub status_counts: StatusCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentCount {
    pub department_id: i32,
    pub department_name: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionCount {
    pub position: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCount {
    pub active: i32,
    pub inactive: i32,
}

// オートコンプリート結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteResult {
    pub suggestions: Vec<AutocompleteSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteSuggestion {
    pub value: String,
    pub label: String,
    pub category: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// 文字列からenumへの変換実装
impl From<String> for EmployeeSearchField {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "name" => EmployeeSearchField::Name,
            "employee_id" => EmployeeSearchField::EmployeeId,
            "email" => EmployeeSearchField::Email,
            "department" => EmployeeSearchField::Department,
            _ => EmployeeSearchField::Name,
        }
    }
}

impl From<EmployeeSearchField> for String {
    fn from(field: EmployeeSearchField) -> Self {
        match field {
            EmployeeSearchField::Name => "name".to_string(),
            EmployeeSearchField::EmployeeId => "employee_id".to_string(),
            EmployeeSearchField::Email => "email".to_string(),
            EmployeeSearchField::Department => "department".to_string(),
        }
    }
}

impl From<String> for EmployeeSortField {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "name" => EmployeeSortField::Name,
            "employee_id" => EmployeeSortField::EmployeeId,
            "joining_date" => EmployeeSortField::JoiningDate,
            "department" => EmployeeSortField::Department,
            "last_updated" => EmployeeSortField::LastUpdated,
            _ => EmployeeSortField::Name,
        }
    }
}

impl From<EmployeeSortField> for String {
    fn from(field: EmployeeSortField) -> Self {
        match field {
            EmployeeSortField::Name => "name".to_string(),
            EmployeeSortField::EmployeeId => "employee_id".to_string(),
            EmployeeSortField::JoiningDate => "joining_date".to_string(),
            EmployeeSortField::Department => "department".to_string(),
            EmployeeSortField::LastUpdated => "last_updated".to_string(),
        }
    }
}

impl From<String> for SortOrder {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "asc" => SortOrder::Asc,
            "desc" => SortOrder::Desc,
            _ => SortOrder::Asc,
        }
    }
}

impl From<SortOrder> for String {
    fn from(order: SortOrder) -> Self {
        match order {
            SortOrder::Asc => "asc".to_string(),
            SortOrder::Desc => "desc".to_string(),
        }
    }
}