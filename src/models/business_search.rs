use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::models::{Business, PaginationInput, BusinessStatus, BusinessRole};

// 高度業務検索用の入力型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedBusinessSearchInput {
    pub business_number: Option<String>,
    pub name: Option<String>,
    pub customer_name: Option<String>,
    pub description: Option<String>,
    pub member_employee_id: Option<i32>,
    pub member_role: Option<BusinessRole>,
    pub status: Option<BusinessStatus>,
    pub start_date_from: Option<NaiveDate>,
    pub start_date_to: Option<NaiveDate>,
    pub end_date_from: Option<NaiveDate>,
    pub end_date_to: Option<NaiveDate>,
    pub has_documents: Option<bool>,
    pub created_by: Option<i32>,
    pub sort_by: Option<BusinessSortField>,
    pub sort_order: Option<SortOrder>,
    pub pagination: PaginationInput,
}

// 業務オートコンプリート用の入力型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessAutocompleteInput {
    pub query: String,
    pub field: BusinessSearchField,
    pub limit: Option<i32>,
    pub include_completed: Option<bool>,
}

impl Default for BusinessAutocompleteInput {
    fn default() -> Self {
        Self {
            query: String::new(),
            field: BusinessSearchField::Name,
            limit: Some(10),
            include_completed: Some(false),
        }
    }
}

// 業務検索フィールド列挙型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BusinessSearchField {
    BusinessNumber,
    Name,
    CustomerName,
    Description,
}

// 業務ソートフィールド列挙型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BusinessSortField {
    BusinessNumber,
    Name,
    CustomerName,
    StartDate,
    EndDate,
    CreatedAt,
}

// ソート順列挙型はadvanced_searchから使用
pub use crate::models::advanced_search::SortOrder;

// 業務検索結果型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSearchResult {
    pub businesses: Vec<Business>,
    pub total_count: i64,
    pub has_next_page: bool,
    pub aggregations: BusinessSearchAggregations,
}

// 業務検索集計情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSearchAggregations {
    pub status_counts: Vec<BusinessStatusCount>,
    pub customer_counts: Vec<CustomerCount>,
    pub member_counts: Vec<MemberCount>,
    pub year_counts: Vec<YearCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessStatusCount {
    pub status: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCount {
    pub customer_name: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCount {
    pub employee_id: i32,
    pub employee_name: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearCount {
    pub year: i32,
    pub count: i32,
}

// 社員業務一覧（逆引き検索結果）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeBusinesses {
    pub employee_id: i32,
    pub employee_name: String,
    pub businesses: Vec<BusinessWithRole>,
    pub total_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessWithRole {
    pub business: Business,
    pub role: BusinessRole,
    pub participation_level: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
}

// 文字列からenumへの変換実装
impl From<String> for BusinessSearchField {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "business_number" => BusinessSearchField::BusinessNumber,
            "name" => BusinessSearchField::Name,
            "customer_name" => BusinessSearchField::CustomerName,
            "description" => BusinessSearchField::Description,
            _ => BusinessSearchField::Name,
        }
    }
}

impl From<BusinessSearchField> for String {
    fn from(field: BusinessSearchField) -> Self {
        match field {
            BusinessSearchField::BusinessNumber => "business_number".to_string(),
            BusinessSearchField::Name => "name".to_string(),
            BusinessSearchField::CustomerName => "customer_name".to_string(),
            BusinessSearchField::Description => "description".to_string(),
        }
    }
}

impl From<String> for BusinessSortField {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "business_number" => BusinessSortField::BusinessNumber,
            "name" => BusinessSortField::Name,
            "customer_name" => BusinessSortField::CustomerName,
            "start_date" => BusinessSortField::StartDate,
            "end_date" => BusinessSortField::EndDate,
            "created_at" => BusinessSortField::CreatedAt,
            _ => BusinessSortField::Name,
        }
    }
}

impl From<BusinessSortField> for String {
    fn from(field: BusinessSortField) -> Self {
        match field {
            BusinessSortField::BusinessNumber => "business_number".to_string(),
            BusinessSortField::Name => "name".to_string(),
            BusinessSortField::CustomerName => "customer_name".to_string(),
            BusinessSortField::StartDate => "start_date".to_string(),
            BusinessSortField::EndDate => "end_date".to_string(),
            BusinessSortField::CreatedAt => "created_at".to_string(),
        }
    }
}


// デフォルト実装
impl Default for AdvancedBusinessSearchInput {
    fn default() -> Self {
        Self {
            business_number: None,
            name: None,
            customer_name: None,
            description: None,
            member_employee_id: None,
            member_role: None,
            status: None,
            start_date_from: None,
            start_date_to: None,
            end_date_from: None,
            end_date_to: None,
            has_documents: None,
            created_by: None,
            sort_by: Some(BusinessSortField::CreatedAt),
            sort_order: Some(SortOrder::Desc),
            pagination: PaginationInput {
                limit: 20,
                offset: 0,
            },
        }
    }
}