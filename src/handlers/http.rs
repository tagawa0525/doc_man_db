use axum::{Json, extract};
use serde_json;
use std::collections::HashMap;

use crate::{AppState, models};

/// ヘルスチェックエンドポイント
pub async fn health_check_handler(
    extract::State(state): extract::State<AppState>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    match state.health_handler.health_check().await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "healthy"}))),
        Err(err) => {
            let error_body = serde_json::json!({
                "error": err.to_string()
            });
            Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_body),
            ))
        }
    }
}

/// 文書作成エンドポイント
pub async fn create_document_handler(
    extract::State(state): extract::State<AppState>,
    Json(request): Json<models::CreateDocumentWithNumberRequest>,
) -> Result<
    (
        axum::http::StatusCode,
        Json<models::CreatedDocumentWithNumber>,
    ),
    (axum::http::StatusCode, Json<serde_json::Value>),
> {
    match state.document_handlers.create_document(request).await {
        Ok(created) => Ok((axum::http::StatusCode::CREATED, Json(created))),
        Err(err) => {
            let error_message = err.to_string();
            let status = axum::http::StatusCode::from(err);
            let error_body = serde_json::json!({
                "error": error_message
            });
            Err((status, Json(error_body)))
        }
    }
}

/// 文書取得エンドポイント
pub async fn get_document_handler(
    extract::State(state): extract::State<AppState>,
    extract::Path(id): extract::Path<i32>,
) -> Result<Json<models::Document>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    match state.document_handlers.get_document(id).await {
        Ok(document) => Ok(Json(document)),
        Err(err) => {
            let error_message = err.to_string();
            let status = axum::http::StatusCode::from(err);
            let error_body = serde_json::json!({
                "error": error_message
            });
            Err((status, Json(error_body)))
        }
    }
}

/// 文書検索エンドポイント
pub async fn search_documents_handler(
    extract::State(state): extract::State<AppState>,
    extract::Query(params): extract::Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
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
        Err(err) => {
            let error_message = err.to_string();
            let status = axum::http::StatusCode::from(err);
            let error_body = serde_json::json!({
                "error": error_message
            });
            Err((status, Json(error_body)))
        }
    }
}
