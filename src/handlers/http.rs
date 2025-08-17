use axum::{extract, Json};
use serde_json;
use std::collections::HashMap;

use crate::{models, AppState};

/// ヘルスチェックエンドポイント
pub async fn health_check_handler(
    extract::State(state): extract::State<AppState>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match state.health_handler.health_check().await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "healthy"}))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// 文書作成エンドポイント
pub async fn create_document_handler(
    extract::State(state): extract::State<AppState>,
    Json(request): Json<models::CreateDocumentWithNumberRequest>,
) -> Result<Json<models::CreatedDocumentWithNumber>, axum::http::StatusCode> {
    match state.document_handlers.create_document(request).await {
        Ok(created) => Ok(Json(created)),
        Err(_) => Err(axum::http::StatusCode::BAD_REQUEST),
    }
}

/// 文書取得エンドポイント
pub async fn get_document_handler(
    extract::State(state): extract::State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<Json<models::Document>, axum::http::StatusCode> {
    match state.document_handlers.get_document(id).await {
        Ok(document) => Ok(Json(document)),
        Err(_) => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

/// 文書検索エンドポイント
pub async fn search_documents_handler(
    extract::State(state): extract::State<AppState>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let filters = models::DocumentSearchFilters {
        title: params.get("title").cloned(),
        document_type_id: params.get("document_type_id").and_then(|s| s.parse().ok()),
        created_by: params.get("created_by").and_then(|s| s.parse().ok()),
        created_date_from: params
            .get("created_date_from")
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
        created_date_to: params
            .get("created_date_to")
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()),
        limit: params
            .get("limit")
            .and_then(|s| s.parse().ok())
            .unwrap_or(10),
        offset: params
            .get("offset")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
    };

    match state.document_handlers.search_documents(filters).await {
        Ok((documents, total)) => Ok(Json(serde_json::json!({
            "documents": documents,
            "total": total
        }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}