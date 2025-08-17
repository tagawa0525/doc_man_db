use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;
use crate::handlers::graphql::{graphql_handler, graphql_playground};
use crate::handlers::http::{
    create_document_handler, get_document_handler, health_check_handler, search_documents_handler,
};
use crate::handlers::csv_import::{
    upload_and_import_csv, get_import_executions, get_import_execution,
    download_csv_template, validate_csv, get_import_progress,
};

/// APIルーターの設定
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // ヘルスチェックエンドポイント
        .route("/health", get(health_check_handler))
        // Document API
        .route("/api/documents", post(create_document_handler))
        .route("/api/documents/{id}", get(get_document_handler))
        .route("/api/documents", get(search_documents_handler))
        // CSV Import API
        .route("/api/admin/csv/import", post(upload_and_import_csv))
        .route("/api/admin/csv/validate", post(validate_csv))
        .route("/api/admin/csv/template", get(download_csv_template))
        .route("/api/admin/csv/executions", get(get_import_executions))
        .route("/api/admin/csv/executions/{import_id}", get(get_import_execution))
        .route("/api/admin/csv/progress/{import_id}", get(get_import_progress))
        // GraphQL エンドポイント（Playground付き）
        .route("/graphql", get(graphql_playground).post(graphql_handler))
}
