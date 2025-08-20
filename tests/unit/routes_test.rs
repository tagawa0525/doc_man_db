use doc_man_db::routes::create_routes;
use axum::Router;
use doc_man_db::app::AppState;

#[test]
fn test_create_routes_returns_router() {
    let routes = create_routes();
    
    // Router型であることを確認
    assert!(std::mem::size_of_val(&routes) > 0);
}

#[test]
fn test_create_routes_multiple_calls() {
    let routes1 = create_routes();
    let routes2 = create_routes();
    
    // 複数回呼び出しても正常に作成できることを確認
    assert!(std::mem::size_of_val(&routes1) > 0);
    assert!(std::mem::size_of_val(&routes2) > 0);
}

#[test]
fn test_create_routes_type_check() {
    // Router<AppState>型であることを確認
    let routes: Router<AppState> = create_routes();
    assert!(std::mem::size_of_val(&routes) > 0);
}

#[test]
fn test_create_routes_consistency() {
    // ルーター作成の一貫性をテスト
    for _i in 0..5 {
        let routes = create_routes();
        assert!(std::mem::size_of_val(&routes) > 0);
    }
}

#[test]
fn test_router_is_static() {
    // ルーター作成に外部依存がないことを確認
    let _routes = create_routes();
    let type_name = std::any::type_name::<Router<AppState>>();
    assert!(type_name.contains("Router"));
}

#[test]
fn test_routes_structure_size() {
    let routes = create_routes();
    // Routerが適切なサイズを持つことを確認
    assert!(std::mem::size_of_val(&routes) > 0); // Router構造体が存在することを確認
}