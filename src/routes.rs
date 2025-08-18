use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;
use crate::handlers::graphql::{graphql_handler, graphql_playground};
use crate::handlers::http::{
    create_document_handler, get_document_handler, health_check_handler, search_documents_handler,
};
// use crate::handlers::csv_import::{
//     upload_and_import_csv, get_import_executions, get_import_execution,
//     download_csv_template, validate_csv, get_import_progress,
// };
// use crate::handlers::batch::{
//     execute_batch_manually, get_batch_executions, get_batch_execution,
//     get_batch_statistics, get_running_batches, get_batch_types,
//     get_batch_schedules, cancel_batch,
// };

/// APIルーターの設定
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // ヘルスチェックエンドポイント
        .route("/health", get(health_check_handler))
        // Document API
        .route("/api/documents", post(create_document_handler))
        .route("/api/documents/{id}", get(get_document_handler))
        .route("/api/documents", get(search_documents_handler))
        // CSV Import API (temporarily disabled)
        // .route("/api/admin/csv/import", post(upload_and_import_csv))
        // .route("/api/admin/csv/validate", post(validate_csv))
        // .route("/api/admin/csv/template", get(download_csv_template))
        // .route("/api/admin/csv/executions", get(get_import_executions))
        // .route("/api/admin/csv/executions/{import_id}", get(get_import_execution))
        // .route("/api/admin/csv/progress/{import_id}", get(get_import_progress))
        // Batch Processing API (temporarily disabled)
        // .route("/api/admin/batch/execute", post(execute_batch_manually))
        // .route("/api/admin/batch/executions", get(get_batch_executions))
        // .route("/api/admin/batch/executions/{execution_id}", get(get_batch_execution))
        // .route("/api/admin/batch/statistics", get(get_batch_statistics))
        // .route("/api/admin/batch/running", get(get_running_batches))
        // .route("/api/admin/batch/types", get(get_batch_types))
        // .route("/api/admin/batch/schedules", get(get_batch_schedules))
        // .route("/api/admin/batch/cancel/{execution_id}", post(cancel_batch))
        // GraphQL エンドポイント（Playground付き）
        .route("/graphql", get(graphql_playground).post(graphql_handler))
}
