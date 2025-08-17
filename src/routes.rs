use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::http::{
    create_document_handler, get_document_handler, health_check_handler, search_documents_handler,
};
use crate::handlers::graphql::{graphql_handler, graphql_playground};
use crate::AppState;

/// APIルーターの設定
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // ヘルスチェックエンドポイント
        .route("/health", get(health_check_handler))
        // Document API
        .route("/api/documents", post(create_document_handler))
        .route("/api/documents/{id}", get(get_document_handler))
        .route("/api/documents", get(search_documents_handler))
        // GraphQL エンドポイント（Playground付き）
        .route("/graphql", get(graphql_playground).post(graphql_handler))
}