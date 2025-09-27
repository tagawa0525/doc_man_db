use std::net::SocketAddr;
use tokio::net::TcpListener;

/// テスト用のアプリケーションサーバー起動ヘルパー
pub async fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // 各テスト用に一意の共有インメモリデータベース名を生成
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_name = format!("test_db_{}_{}", std::process::id(), timestamp);
    let test_db_url = format!("sqlite:file:{}?mode=memory&cache=shared", db_name);
    let test_db_path = std::path::PathBuf::from(format!(":memory:{}", db_name));

    println!("API Test DB URL: {}", test_db_url);
    println!("API Test DB Path: {:?}", test_db_path);

    // データベース接続プールを作成（同一プロセス内で共有）
    let pool = sqlx::SqlitePool::connect(&test_db_url).await.unwrap();

    // マイグレーションを実行
    let migrator = sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
        .await
        .unwrap();
    migrator.run(&pool).await.unwrap();

    // シードシステムを使用してテストデータを投入
    use doc_man_db::seeds::{Environment, Seeder};
    let seeder = Seeder::new(pool.clone());
    seeder
        .seed_all(&Environment::Test, false, false)
        .await
        .unwrap();

    let app = doc_man_db::create_app_with_db_url(&test_db_url)
        .await
        .unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // サーバーが起動するまで少し待機
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    addr
}
