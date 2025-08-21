use crate::error::SearchError;
use crate::models::{
    AdvancedEmployeeSearchInput, EmployeeAutocompleteInput, EmployeeSearchResult,
    AutocompleteResult, EmployeeSearchField, EmployeeSortField, SortOrder, PaginationInput
};
use crate::services::{AdvancedSearchService, UserPermissions};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub fn advanced_search_routes() -> Router<Arc<AdvancedSearchService>> {
    Router::new()
        .route("/search/employees/advanced", post(advanced_employee_search))
        .route("/search/employees/autocomplete", get(employee_autocomplete))
        .route("/search/employees/:employee_id/businesses", get(employee_business_history))
}

// ハンドラー関数

/// 高度社員検索
async fn advanced_employee_search(
    State(service): State<Arc<AdvancedSearchService>>,
    Json(filters): Json<AdvancedEmployeeSearchInput>,
) -> Result<Json<EmployeeSearchResult>, (StatusCode, String)> {
    let user_permissions = get_user_permissions(); // 仮実装
    
    match service.search_employees_advanced(filters, &user_permissions).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((
                StatusCode::from(crate::error::AppError::Search(err)),
                error_msg
            ))
        }
    }
}

/// 社員オートコンプリート
async fn employee_autocomplete(
    Query(params): Query<HashMap<String, String>>,
    State(service): State<Arc<AdvancedSearchService>>,
) -> Result<Json<AutocompleteResult>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    // クエリパラメータから入力を構築
    let input = EmployeeAutocompleteInput {
        query: params.get("query").cloned().unwrap_or_default(),
        field: params.get("field")
            .map(|s| EmployeeSearchField::from(s.clone()))
            .unwrap_or(EmployeeSearchField::Name),
        limit: params.get("limit").and_then(|s| s.parse().ok()),
        include_inactive: params.get("include_inactive").and_then(|s| s.parse().ok()),
    };
    
    match service.employee_autocomplete(input, &user_permissions).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((
                StatusCode::from(crate::error::AppError::Search(err)),
                error_msg
            ))
        }
    }
}

/// 社員の業務従事履歴取得
async fn employee_business_history(
    Path(employee_id): Path<i32>,
    State(service): State<Arc<AdvancedSearchService>>,
) -> Result<Json<Vec<crate::models::BusinessMember>>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.get_employee_business_history(employee_id, &user_permissions).await {
        Ok(history) => Ok(Json(history)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((
                StatusCode::from(crate::error::AppError::Search(err)),
                error_msg
            ))
        }
    }
}

// レスポンス型とリクエスト型（必要に応じて）

#[derive(Debug, Serialize, Deserialize)]
pub struct AdvancedSearchRequest {
    pub filters: AdvancedEmployeeSearchInput,
}

#[derive(Debug, Serialize)]
pub struct SearchStatsResponse {
    pub total_employees: i64,
    pub active_employees: i64,
    pub departments: i64,
    pub recent_searches: i64,
}

// ヘルパー関数

/// 現在のユーザー権限を取得する（仮実装）
fn get_user_permissions() -> UserPermissions {
    // TODO: JWT トークンまたはセッションから実際の権限を取得
    UserPermissions {
        employee_id: 1,
        can_create_businesses: true,
        can_manage_business_members: true,
        can_view_all_businesses: true,
        accessible_departments: vec![1, 2, 3],
    }
}

/// クエリパラメータを検索フィルターに変換
pub fn query_params_to_search_filters(params: &HashMap<String, String>) -> AdvancedEmployeeSearchInput {
    AdvancedEmployeeSearchInput {
        name: params.get("name").cloned(),
        employee_number: params.get("employee_number").cloned(),
        email: params.get("email").cloned(),
        department_id: params.get("department_id").and_then(|s| s.parse().ok()),
        // current_position removed - not available in Employee model
        has_business_experience: params.get("has_business_experience").cloned(),
        // joining_date fields removed - not available in Employee model
        is_active: params.get("is_active").and_then(|s| s.parse().ok()),
        skill_keywords: params.get("skill_keywords")
            .map(|s| s.split(',').map(|item| item.trim().to_string()).collect()),
        sort_by: params.get("sort_by")
            .map(|s| EmployeeSortField::from(s.clone())),
        sort_order: params.get("sort_order")
            .map(|s| SortOrder::from(s.clone())),
        pagination: PaginationInput {
            limit: params.get("limit").and_then(|s| s.parse().ok()).unwrap_or(20),
            offset: params.get("offset").and_then(|s| s.parse().ok()).unwrap_or(0),
        },
    }
}

/// 検索結果のフォーマット（統計情報付き）
pub fn format_search_results_with_stats(
    result: EmployeeSearchResult,
) -> serde_json::Value {
    serde_json::json!({
        "employees": result.employees,
        "total_count": result.total_count,
        "has_next_page": result.has_next_page,
        "aggregations": result.aggregations,
        "search_meta": {
            "result_count": result.employees.len(),
            "search_time_ms": 0, // TODO: 実際の検索時間を計測
            "cache_hit": false,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_query_params_to_search_filters() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "田中".to_string());
        params.insert("limit".to_string(), "50".to_string());
        params.insert("is_active".to_string(), "true".to_string());
        
        let filters = query_params_to_search_filters(&params);
        
        assert_eq!(filters.name, Some("田中".to_string()));
        assert_eq!(filters.pagination.limit, 50);
        assert_eq!(filters.is_active, Some(true));
    }
}