use axum::{Router, extract::DefaultBodyLimit};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{DocumentHandlers, HealthHandler};
use crate::repositories::{
    DepartmentRepository, SqliteDocumentNumberRuleRepository, SqliteDocumentRepository,
};
use crate::routes::create_routes;
use crate::services::DocumentService;

/// アプリケーション状態
#[derive(Clone)]
pub struct AppState {
    pub document_handlers: DocumentHandlers,
    pub health_handler: HealthHandler,
    pub department_repository: DepartmentRepository,
}

/// アプリケーションのメインエントリーポイント
/// テストと本番環境の両方で使用される
pub async fn create_app() -> Router {
    // リポジトリの初期化（テスト用にSQLiteを使用）
    let doc_repo = SqliteDocumentRepository::new_in_memory()
        .await
        .expect("Failed to create document repository");
    let rule_repo = SqliteDocumentNumberRuleRepository::new_in_memory()
        .await
        .expect("Failed to create rule repository");
    let dept_repo = DepartmentRepository::new_with_file_db("sqlite://./data/dev.db")
        .await
        .expect("Failed to create department repository");

    // サービス層の初期化
    let document_service = DocumentService::new(doc_repo, rule_repo);

    // ハンドラーの初期化
    let document_handlers = DocumentHandlers::new(document_service);
    let health_handler = HealthHandler::new();

    // アプリケーション状態
    let state = AppState {
        document_handlers,
        health_handler,
        department_repository: dept_repo,
    };

    // ルーターとミドルウェアの構築
    create_routes()
        .layer(
            ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                )
                .layer(DefaultBodyLimit::max(1024 * 1024)), // 1MB制限
        )
        .with_state(state)
}
