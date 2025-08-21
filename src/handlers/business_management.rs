use crate::error::BusinessError;
use crate::models::{
    CreateBusinessMemberRequest, CreateBusinessRequest, UpdateBusinessMemberRequest,
    UpdateBusinessRequest, BusinessSearchFilters, PaginationInput,
};
use crate::services::{BusinessService, UserPermissions, BusinessResponse, BusinessMemberResponse, DeleteResponse};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub fn business_routes() -> Router<Arc<BusinessService>> {
    Router::new()
        .route("/businesses", get(search_businesses).post(create_business))
        .route("/businesses/:id", get(get_business).put(update_business).delete(delete_business))
        .route("/businesses/:id/members", get(get_business_members).post(add_business_member))
        .route("/businesses/:id/members/:member_id", put(update_business_member).delete(remove_business_member))
        .route("/businesses/:id/circulation-candidates", get(get_circulation_candidates))
        .route("/employees/:employee_id/businesses", get(get_employee_business_history))
        .route("/businesses/generate-number", post(generate_business_number))
}

// ハンドラー関数

/// 業務を作成する
async fn create_business(
    State(service): State<Arc<BusinessService>>,
    Json(request): Json<CreateBusinessRequest>,
) -> Result<Json<BusinessResponse>, (StatusCode, String)> {
    // TODO: 認証からユーザー権限を取得
    let user_permissions = get_user_permissions(); // 仮実装
    
    match service.create_business(request, &user_permissions).await {
        Ok(business) => Ok(Json(BusinessResponse::from(business))),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務を取得する
async fn get_business(
    Path(id): Path<i32>,
    State(service): State<Arc<BusinessService>>,
) -> Result<Json<Option<crate::models::Business>>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.get_business(id, &user_permissions).await {
        Ok(business) => Ok(Json(business)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務を検索する
async fn search_businesses(
    Query(params): Query<HashMap<String, String>>,
    State(service): State<Arc<BusinessService>>,
) -> Result<Json<BusinessSearchResponse>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    // クエリパラメータから検索フィルターを構築
    let filters = BusinessSearchFilters {
        business_number: params.get("business_number").cloned(),
        name: params.get("name").cloned(),
        customer_name: params.get("customer_name").cloned(),
        status: params.get("status").cloned(),
        member_employee_id: params.get("member_employee_id").and_then(|s| s.parse().ok()),
        date_range: None, // TODO: 日付範囲の解析実装
        pagination: PaginationInput {
            limit: params.get("limit").and_then(|s| s.parse().ok()).unwrap_or(20),
            offset: params.get("offset").and_then(|s| s.parse().ok()).unwrap_or(0),
        },
    };
    
    match service.search_businesses(filters, &user_permissions).await {
        Ok((businesses, total)) => {
            let businesses_len = businesses.len();
            Ok(Json(BusinessSearchResponse {
                businesses,
                total_count: total,
                has_next_page: businesses_len as i64 == 20 && total > (businesses_len as i64),
            }))
        },
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務を更新する
async fn update_business(
    Path(id): Path<i32>,
    State(service): State<Arc<BusinessService>>,
    Json(request): Json<UpdateBusinessRequest>,
) -> Result<Json<BusinessResponse>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.update_business(id, request, &user_permissions).await {
        Ok(business) => Ok(Json(BusinessResponse::from(business))),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務を削除する
async fn delete_business(
    Path(id): Path<i32>,
    State(service): State<Arc<BusinessService>>,
) -> Result<Json<DeleteResponse>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.delete_business(id, &user_permissions).await {
        Ok(_) => Ok(Json(DeleteResponse {
            success: true,
            message: Some("業務が正常に削除されました".to_string()),
        })),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務従事者一覧を取得する
async fn get_business_members(
    Path(id): Path<i32>,
    State(service): State<Arc<BusinessService>>,
) -> Result<Json<Vec<crate::models::BusinessMember>>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.get_business_members(id, &user_permissions).await {
        Ok(members) => Ok(Json(members)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務従事者を追加する
async fn add_business_member(
    Path(id): Path<i32>,
    State(service): State<Arc<BusinessService>>,
    Json(mut request): Json<CreateBusinessMemberRequest>,
) -> Result<Json<BusinessMemberResponse>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    request.business_id = id; // パスから業務IDを設定
    
    match service.add_business_member(request, &user_permissions).await {
        Ok(member) => Ok(Json(BusinessMemberResponse::from(member))),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務従事者を更新する
async fn update_business_member(
    Path((_, member_id)): Path<(i32, i32)>,
    State(service): State<Arc<BusinessService>>,
    Json(request): Json<UpdateBusinessMemberRequest>,
) -> Result<Json<BusinessMemberResponse>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.update_business_member(member_id, request, &user_permissions).await {
        Ok(member) => Ok(Json(BusinessMemberResponse::from(member))),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務従事者を削除する
async fn remove_business_member(
    Path((_, member_id)): Path<(i32, i32)>,
    State(service): State<Arc<BusinessService>>,
) -> Result<Json<DeleteResponse>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.remove_business_member(member_id, &user_permissions).await {
        Ok(_) => Ok(Json(DeleteResponse {
            success: true,
            message: Some("業務従事者が正常に削除されました".to_string()),
        })),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 社員の業務従事履歴を取得する
async fn get_employee_business_history(
    Path(employee_id): Path<i32>,
    State(service): State<Arc<BusinessService>>,
) -> Result<Json<Vec<crate::models::BusinessMember>>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.get_employee_business_history(employee_id, &user_permissions).await {
        Ok(history) => Ok(Json(history)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 回覧候補を取得する
async fn get_circulation_candidates(
    Path(id): Path<i32>,
    State(service): State<Arc<BusinessService>>,
) -> Result<Json<crate::models::CirculationCandidates>, (StatusCode, String)> {
    let user_permissions = get_user_permissions();
    
    match service.suggest_circulation_candidates(id, &user_permissions).await {
        Ok(candidates) => Ok(Json(candidates)),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

/// 業務番号を生成する
async fn generate_business_number(
    State(service): State<Arc<BusinessService>>,
) -> Result<Json<BusinessNumberResponse>, (StatusCode, String)> {
    match service.generate_business_number().await {
        Ok(number) => Ok(Json(BusinessNumberResponse { business_number: number })),
        Err(err) => {
            let error_msg = err.to_string();
            Err((StatusCode::from(crate::error::AppError::Business(err)), error_msg))
        },
    }
}

// レスポンス型定義

#[derive(Debug, Serialize)]
pub struct BusinessSearchResponse {
    pub businesses: Vec<crate::models::Business>,
    pub total_count: i64,
    pub has_next_page: bool,
}

#[derive(Debug, Serialize)]
pub struct BusinessNumberResponse {
    pub business_number: String,
}

// 型定義はservicesモジュールから再利用

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