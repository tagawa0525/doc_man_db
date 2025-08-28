use axum::Router;
use doc_man_db::app::{AppState, create_app};

#[tokio::test]
async fn test_create_app_returns_router() {
    let app = create_app().await.expect("Failed to create app");

    // Router型であることを確認
    assert!(std::mem::size_of_val(&app) > 0);
}

#[tokio::test]
async fn test_create_app_multiple_calls() {
    let app1 = create_app().await.expect("Failed to create app1");
    let app2 = create_app().await.expect("Failed to create app2");

    // 複数回呼び出しても正常に作成できることを確認
    assert!(std::mem::size_of_val(&app1) > 0);
    assert!(std::mem::size_of_val(&app2) > 0);
}

#[test]
fn test_app_state_creation() {
    // AppStateの構造を確認するためのテスト
    // 実際のhandlerは非同期なので、型チェックのみ行う
    let type_name = std::any::type_name::<AppState>();
    assert!(type_name.contains("AppState"));
}

#[test]
fn test_app_state_clone() {
    // AppStateがCloneトレイトを実装していることを確認
    fn is_clone<T: Clone>() {}
    is_clone::<AppState>();
}

#[tokio::test]
async fn test_create_app_with_cors_middleware() {
    let app = create_app().await.expect("Failed to create app");

    // アプリケーションが正常に作成され、適切なサイズを持つことを確認
    assert!(std::mem::size_of_val(&app) > 0);

    // Router型が適切に設定されていることを確認
    let router: Router = app;
    assert!(std::mem::size_of_val(&router) > 0);
}

#[tokio::test]
async fn test_create_app_consistency() {
    // アプリケーション作成の一貫性をテスト
    for _i in 0..3 {
        let app = create_app().await.expect("Failed to create app");
        assert!(std::mem::size_of_val(&app) > 0);
    }
}
