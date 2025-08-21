use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::circulation::*;
use crate::models::employee::UserPermissions;
use crate::services::circulation_service::CirculationService;

#[derive(Debug, Deserialize)]
pub struct GetPendingCirculationsQuery {
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
        }
    }
}

pub async fn get_workflows(
    State(circulation_service): State<Arc<CirculationService>>,
) -> Result<Json<ApiResponse<Vec<CirculationWorkflow>>>, StatusCode> {
    match circulation_service.get_workflows().await {
        Ok(workflows) => Ok(Json(ApiResponse::success(workflows, "ワークフローを取得しました"))),
        Err(e) => {
            tracing::error!("Failed to get workflows: {}", e);
            Ok(Json(ApiResponse::error(&format!("ワークフローの取得に失敗しました: {}", e))))
        }
    }
}

pub async fn create_circulation(
    State(circulation_service): State<Arc<CirculationService>>,
    Json(input): Json<CreateCirculationInput>,
) -> Result<Json<ApiResponse<DocumentCirculation>>, StatusCode> {
    // In a real implementation, extract user permissions from JWT token or session
    let user_permissions = UserPermissions {
        user_id: 1, // TODO: Get from auth context
        is_admin: false,
        department_id: Some(1),
        business_id: Some(1),
    };

    match circulation_service.create_circulation(input, &user_permissions).await {
        Ok(circulation) => Ok(Json(ApiResponse::success(circulation, "回覧を開始しました"))),
        Err(e) => {
            tracing::error!("Failed to create circulation: {}", e);
            Ok(Json(ApiResponse::error(&format!("回覧の開始に失敗しました: {}", e))))
        }
    }
}

pub async fn complete_step(
    State(circulation_service): State<Arc<CirculationService>>,
    Json(input): Json<CompleteStepInput>,
) -> Result<Json<ApiResponse<CirculationStep>>, StatusCode> {
    // In a real implementation, extract user permissions from JWT token or session
    let user_permissions = UserPermissions {
        user_id: 1, // TODO: Get from auth context
        is_admin: false,
        department_id: Some(1),
        business_id: Some(1),
    };

    match circulation_service.complete_step(input, &user_permissions).await {
        Ok(step) => Ok(Json(ApiResponse::success(step, "ステップを完了しました"))),
        Err(e) => {
            tracing::error!("Failed to complete step: {}", e);
            Ok(Json(ApiResponse::error(&format!("ステップの完了に失敗しました: {}", e))))
        }
    }
}

pub async fn get_pending_circulations(
    State(circulation_service): State<Arc<CirculationService>>,
    Query(query): Query<GetPendingCirculationsQuery>,
) -> Result<Json<ApiResponse<Vec<CirculationStep>>>, StatusCode> {
    let user_id = query.user_id.unwrap_or(1); // TODO: Get from auth context

    match circulation_service.get_pending_circulations_for_user(user_id).await {
        Ok(steps) => Ok(Json(ApiResponse::success(steps, "承認待ち回覧を取得しました"))),
        Err(e) => {
            tracing::error!("Failed to get pending circulations: {}", e);
            Ok(Json(ApiResponse::error(&format!("承認待ち回覧の取得に失敗しました: {}", e))))
        }
    }
}

pub async fn get_document_circulations(
    State(circulation_service): State<Arc<CirculationService>>,
    Path(document_id): Path<i32>,
) -> Result<Json<ApiResponse<Vec<DocumentCirculation>>>, StatusCode> {
    match circulation_service.get_document_circulations(document_id).await {
        Ok(circulations) => Ok(Json(ApiResponse::success(circulations, "文書の回覧履歴を取得しました"))),
        Err(e) => {
            tracing::error!("Failed to get document circulations: {}", e);
            Ok(Json(ApiResponse::error(&format!("文書の回覧履歴の取得に失敗しました: {}", e))))
        }
    }
}

pub async fn get_circulation_details(
    State(circulation_service): State<Arc<CirculationService>>,
    Path(circulation_id): Path<i32>,
) -> Result<Json<ApiResponse<CirculationWithDetails>>, StatusCode> {
    match circulation_service.get_circulation_with_details(circulation_id).await {
        Ok(Some(details)) => Ok(Json(ApiResponse::success(details, "回覧詳細を取得しました"))),
        Ok(None) => Ok(Json(ApiResponse::error("指定された回覧が見つかりません"))),
        Err(e) => {
            tracing::error!("Failed to get circulation details: {}", e);
            Ok(Json(ApiResponse::error(&format!("回覧詳細の取得に失敗しました: {}", e))))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CancelCirculationRequest {
    pub reason: Option<String>,
}

pub async fn cancel_circulation(
    State(circulation_service): State<Arc<CirculationService>>,
    Path(circulation_id): Path<i32>,
    Json(request): Json<CancelCirculationRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // In a real implementation, extract user permissions from JWT token or session
    let user_permissions = UserPermissions {
        user_id: 1, // TODO: Get from auth context
        is_admin: false,
        department_id: Some(1),
        business_id: Some(1),
    };

    match circulation_service.cancel_circulation(circulation_id, request.reason, &user_permissions).await {
        Ok(_) => Ok(Json(ApiResponse::success((), "回覧をキャンセルしました"))),
        Err(e) => {
            tracing::error!("Failed to cancel circulation: {}", e);
            Ok(Json(ApiResponse::error(&format!("回覧のキャンセルに失敗しました: {}", e))))
        }
    }
}