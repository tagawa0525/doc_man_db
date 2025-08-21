use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// 検索履歴
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SearchHistory {
    pub id: i32,
    pub employee_id: i32,
    pub search_type: SearchType,
    pub search_query: String, // JSON形式
    pub result_count: Option<i32>,
    pub execution_time_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// お気に入り検索
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FavoriteSearch {
    pub id: i32,
    pub employee_id: i32,
    pub name: String,
    pub search_type: SearchType,
    pub search_query: String, // JSON形式
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 検索候補
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SearchSuggestion {
    pub id: i32,
    pub search_type: SearchType,
    pub field_name: String,
    pub suggestion: String,
    pub frequency: i32,
    pub last_used: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// 検索タイプ列挙型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SearchType {
    Employee,
    Business,
    Document,
}

// 入力型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSearchHistoryRequest {
    pub employee_id: i32,
    pub search_type: SearchType,
    pub search_query: String,
    pub result_count: Option<i32>,
    pub execution_time_ms: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFavoriteSearchRequest {
    pub employee_id: i32,
    pub name: String,
    pub search_type: SearchType,
    pub search_query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFavoriteSearchRequest {
    pub name: Option<String>,
    pub search_query: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSearchSuggestionRequest {
    pub search_type: SearchType,
    pub field_name: String,
    pub suggestion: String,
}

// フィルター・検索用の型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryFilters {
    pub employee_id: Option<i32>,
    pub search_type: Option<SearchType>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteSearchFilters {
    pub employee_id: Option<i32>,
    pub search_type: Option<SearchType>,
    pub is_active: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionFilters {
    pub search_type: Option<SearchType>,
    pub field_name: Option<String>,
    pub query: Option<String>,
    pub limit: Option<i32>,
}

// レスポンス型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryResponse {
    pub history: Vec<SearchHistory>,
    pub total_count: i64,
    pub has_next_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteSearchResponse {
    pub favorites: Vec<FavoriteSearch>,
    pub total_count: i64,
    pub has_next_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionResponse {
    pub suggestions: Vec<SearchSuggestion>,
    pub total_count: i64,
}

// 検索統計
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchStatistics {
    pub total_searches: i32,
    pub searches_this_month: i32,
    pub favorite_searches: i32,
    pub most_used_search_type: SearchType,
    pub average_execution_time_ms: f64,
}

// 文字列からenumへの変換
impl From<String> for SearchType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "employee" => SearchType::Employee,
            "business" => SearchType::Business,
            "document" => SearchType::Document,
            _ => SearchType::Employee,
        }
    }
}

impl From<SearchType> for String {
    fn from(search_type: SearchType) -> Self {
        match search_type {
            SearchType::Employee => "employee".to_string(),
            SearchType::Business => "business".to_string(),
            SearchType::Document => "document".to_string(),
        }
    }
}

// SQLXからの変換
impl SearchHistory {
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(SearchHistory {
            id: row.try_get("id")?,
            employee_id: row.try_get("employee_id")?,
            search_type: SearchType::from(row.try_get::<String, _>("search_type")?),
            search_query: row.try_get("search_query")?,
            result_count: row.try_get("result_count")?,
            execution_time_ms: row.try_get("execution_time_ms")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

impl FavoriteSearch {
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(FavoriteSearch {
            id: row.try_get("id")?,
            employee_id: row.try_get("employee_id")?,
            name: row.try_get("name")?,
            search_type: SearchType::from(row.try_get::<String, _>("search_type")?),
            search_query: row.try_get("search_query")?,
            is_active: row.try_get("is_active")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl SearchSuggestion {
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(SearchSuggestion {
            id: row.try_get("id")?,
            search_type: SearchType::from(row.try_get::<String, _>("search_type")?),
            field_name: row.try_get("field_name")?,
            suggestion: row.try_get("suggestion")?,
            frequency: row.try_get("frequency")?,
            last_used: row.try_get("last_used")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

// デフォルト実装
impl Default for SearchHistoryFilters {
    fn default() -> Self {
        Self {
            employee_id: None,
            search_type: None,
            date_from: None,
            date_to: None,
            limit: Some(20),
            offset: Some(0),
        }
    }
}

impl Default for FavoriteSearchFilters {
    fn default() -> Self {
        Self {
            employee_id: None,
            search_type: None,
            is_active: Some(true),
            limit: Some(50),
            offset: Some(0),
        }
    }
}

impl Default for SearchSuggestionFilters {
    fn default() -> Self {
        Self {
            search_type: None,
            field_name: None,
            query: None,
            limit: Some(10),
        }
    }
}
