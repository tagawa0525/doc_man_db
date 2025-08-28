use crate::models::{
    AdvancedBusinessSearchInput, AutocompleteResult, BusinessAutocompleteInput,
    BusinessSearchField, BusinessSearchResult, BusinessSortField, EmployeeBusinesses,
    PaginationInput, SortOrder,
};
use crate::services::{
    BusinessSearchService, BusinessSearchSuggestions, BusinessStatistics, UserPermissions,
};
use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

pub fn business_search_routes() -> Router<Arc<BusinessSearchService>> {
    Router::new()
        .route(
            "/search/businesses/advanced",
            post(advanced_business_search),
        )
        .route(
            "/search/businesses/autocomplete",
            get(business_autocomplete),
        )
        .route(
            "/search/businesses/employees/:employee_id",
            get(employee_businesses),
        )
        .route("/search/businesses/statistics", get(business_statistics))
        .route("/search/businesses/suggestions", get(search_suggestions))
}

// ハンドラー関数

/// 高度業務検索
async fn advanced_business_search(
    State(service): State<Arc<BusinessSearchService>>,
    Json(filters): Json<AdvancedBusinessSearchInput>,
) -> Result<Json<BusinessSearchResult>, (StatusCode, String)> {
    let user_permissions = get_user_permissions(); // 仮実装

    match service
        .search_businesses_advanced(filters, &user_permissions)
        .await
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((
                StatusCode::from(crate::error::AppError::Search(err)),
                error_msg,
            ))
        }
    }
}

/// 業務オートコンプリート
async fn business_autocomplete(
    Query(params): Query<HashMap<String, String>>,
    State(service): State<Arc<BusinessSearchService>>,
) -> Result<Json<AutocompleteResult>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();

    // クエリパラメータから入力を構築
    let input = BusinessAutocompleteInput {
        query: params.get("query").cloned().unwrap_or_default(),
        field: params
            .get("field")
            .map(|s| BusinessSearchField::from(s.clone()))
            .unwrap_or(BusinessSearchField::Name),
        limit: params.get("limit").and_then(|s| s.parse().ok()),
        include_completed: params.get("include_completed").and_then(|s| s.parse().ok()),
    };

    match service
        .business_autocomplete(input, &user_permissions)
        .await
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((
                StatusCode::from(crate::error::AppError::Search(err)),
                error_msg,
            ))
        }
    }
}

/// 社員の業務一覧取得（逆引き検索）
async fn employee_businesses(
    Path(employee_id): Path<i32>,
    Query(params): Query<HashMap<String, String>>,
    State(service): State<Arc<BusinessSearchService>>,
) -> Result<Json<EmployeeBusinesses>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    let include_completed = params
        .get("include_completed")
        .and_then(|s| s.parse().ok())
        .unwrap_or(false);

    match service
        .get_employee_businesses(employee_id, include_completed, &user_permissions)
        .await
    {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((
                StatusCode::from(crate::error::AppError::Search(err)),
                error_msg,
            ))
        }
    }
}

/// 業務統計情報取得
async fn business_statistics(
    State(service): State<Arc<BusinessSearchService>>,
) -> Result<Json<BusinessStatisticsResponse>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();

    match service.get_business_statistics(&user_permissions).await {
        Ok(stats) => Ok(Json(BusinessStatisticsResponse::from(stats))),
        Err(err) => {
            let error_msg = err.to_string();
            Err((
                StatusCode::from(crate::error::AppError::Search(err)),
                error_msg,
            ))
        }
    }
}

/// 検索候補取得
async fn search_suggestions(
    State(service): State<Arc<BusinessSearchService>>,
) -> Result<Json<BusinessSearchSuggestions>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();

    match service.get_search_suggestions(&user_permissions).await {
        Ok(suggestions) => Ok(Json(suggestions)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((
                StatusCode::from(crate::error::AppError::Search(err)),
                error_msg,
            ))
        }
    }
}

// レスポンス型定義

#[derive(Debug, Serialize)]
pub struct BusinessStatisticsResponse {
    pub total_businesses: i32,
    pub active_businesses: i32,
    pub completed_businesses: i32,
    pub my_businesses: i32,
    pub completed_this_month: i32,
    pub completion_rate: f64,
}

impl From<BusinessStatistics> for BusinessStatisticsResponse {
    fn from(stats: BusinessStatistics) -> Self {
        let completion_rate = if stats.total_businesses > 0 {
            stats.completed_businesses as f64 / stats.total_businesses as f64
        } else {
            0.0
        };

        Self {
            total_businesses: stats.total_businesses,
            active_businesses: stats.active_businesses,
            completed_businesses: stats.completed_businesses,
            my_businesses: stats.my_businesses,
            completed_this_month: stats.completed_this_month,
            completion_rate,
        }
    }
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

/// クエリパラメータを業務検索フィルターに変換
pub fn query_params_to_business_search_filters(
    params: &HashMap<String, String>,
) -> AdvancedBusinessSearchInput {
    AdvancedBusinessSearchInput {
        business_number: params.get("business_number").cloned(),
        name: params.get("name").cloned(),
        customer_name: params.get("customer_name").cloned(),
        description: params.get("description").cloned(),
        member_employee_id: params
            .get("member_employee_id")
            .and_then(|s| s.parse().ok()),
        member_role: params
            .get("member_role")
            .map(|s| crate::models::BusinessRole::from(s.clone())),
        status: params
            .get("status")
            .map(|s| crate::models::BusinessStatus::from(s.clone())),
        start_date_from: params.get("start_date_from").and_then(|s| s.parse().ok()),
        start_date_to: params.get("start_date_to").and_then(|s| s.parse().ok()),
        end_date_from: params.get("end_date_from").and_then(|s| s.parse().ok()),
        end_date_to: params.get("end_date_to").and_then(|s| s.parse().ok()),
        has_documents: params.get("has_documents").and_then(|s| s.parse().ok()),
        created_by: params.get("created_by").and_then(|s| s.parse().ok()),
        sort_by: params
            .get("sort_by")
            .map(|s| BusinessSortField::from(s.clone())),
        sort_order: params.get("sort_order").map(|s| SortOrder::from(s.clone())),
        pagination: PaginationInput {
            limit: params
                .get("limit")
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
            offset: params
                .get("offset")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
        },
    }
}

/// 検索結果のフォーマット（メタデータ付き）
pub fn format_business_search_results(result: BusinessSearchResult) -> serde_json::Value {
    serde_json::json!({
        "businesses": result.businesses,
        "total_count": result.total_count,
        "has_next_page": result.has_next_page,
        "aggregations": result.aggregations,
        "search_meta": {
            "result_count": result.businesses.len(),
            "search_time_ms": 0, // TODO: 実際の検索時間を計測
            "facets_available": true,
        }
    })
}
