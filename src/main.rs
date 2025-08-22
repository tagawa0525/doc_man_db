use doc_man_db::create_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ロギングを初期化
    tracing_subscriber::fmt::init();

    // アプリケーションを作成
    let app = create_app().await;

    // サーバーを起動
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    println!("🚀 Document Management System starting on http://127.0.0.1:8080");
    println!("📚 GraphQL Playground: http://127.0.0.1:8080/graphql");
    println!("🏥 Health Check: http://127.0.0.1:8080/health");

    axum::serve(listener, app).await?;

    Ok(())
}
