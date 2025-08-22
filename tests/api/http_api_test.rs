use axum::http::StatusCode;
use doc_man_db::models::CreatedDocumentWithNumber;
use reqwest::Client;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// テスト用のアプリケーションサーバー起動ヘルパー
async fn spawn_app() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // TODO: 実際のアプリケーションインスタンスを作成
    let app = doc_man_db::create_app().await;

    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // サーバーが起動するまで少し待機
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    (addr, server_handle)
}

#[tokio::test]
async fn test_health_check_endpoint() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    // When: ヘルスチェックエンドポイントにリクエスト
    let response = client
        .get(format!("http://{addr}/health"))
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返される
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "healthy");
}

#[tokio::test]
async fn test_create_document_with_number_api() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    let request_body = json!({
        "title": "API経由のテスト文書",
        "document_type_code": "A",
        "department_code": "T",
        "created_by": 1,
        "created_date": "2025-08-17"
    });

    // When: 文書作成APIにリクエスト
    let response = client
        .post(format!("http://{addr}/api/documents"))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 201 Createdが返される
    assert_eq!(response.status(), StatusCode::CREATED);

    let created_document: CreatedDocumentWithNumber = response.json().await.unwrap();
    assert_eq!(created_document.document.title, "API経由のテスト文書");
    assert!(created_document.document_number.starts_with("T-25"));
    assert_eq!(created_document.generated_number.rule_id, 1);
}

#[tokio::test]
async fn test_get_document_by_id_api() {
    // Given: テストサーバーを起動し、文書を作成
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    // まず文書を作成
    let request_body = json!({
        "title": "取得テスト用文書",
        "document_type_code": "B",
        "department_code": "T",
        "created_by": 1,
        "created_date": "2025-08-17"
    });

    let create_response = client
        .post(format!("http://{addr}/api/documents"))
        .json(&request_body)
        .send()
        .await
        .unwrap();

    // Debug: ステータスをチェック
    println!("Create response status: {}", create_response.status());
    if !create_response.status().is_success() {
        let error_text = create_response.text().await.unwrap();
        panic!("Document creation failed: {error_text}");
    }

    let created: CreatedDocumentWithNumber = create_response.json().await.unwrap();
    let document_id = created.document.id;

    // When: 文書取得APIにリクエスト
    let response = client
        .get(format!("http://{addr}/api/documents/{document_id}"))
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返される
    assert_eq!(response.status(), StatusCode::OK);

    let document: doc_man_db::models::Document = response.json().await.unwrap();
    assert_eq!(document.id, document_id);
    assert_eq!(document.title, "取得テスト用文書");
}

#[tokio::test]
async fn test_search_documents_api() {
    // Given: テストサーバーを起動し、複数の文書を作成
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    // 複数文書を作成（ユニークな番号を保証するためランダムサフィックス使用）
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let unique_id = COUNTER.fetch_add(1000, Ordering::SeqCst)
        + std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

    for i in 1..=3 {
        let request_body = json!({
            "title": format!("検索テスト文書{}_{}", i, unique_id + i as u64),
            "document_type_code": "A",
            "department_code": "T",
            "created_by": 1,
            "created_date": "2025-08-17"
        });

        let create_response = client
            .post(format!("http://{addr}/api/documents"))
            .json(&request_body)
            .send()
            .await
            .unwrap();

        if !create_response.status().is_success() {
            let error_body = create_response.text().await.unwrap();
            panic!("Document creation {} failed: {}", i, error_body);
        }
    }

    // When: 文書検索APIにリクエスト
    let response = client
        .get(format!("http://{addr}/api/documents?title=検索テスト"))
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返される
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    let documents = body["documents"].as_array().unwrap();
    let total = body["total"].as_u64().unwrap();

    assert!(documents.len() >= 3);
    assert!(total >= 3);
}

#[tokio::test]
async fn test_create_document_validation_error() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    let invalid_request = json!({
        "title": "", // 空のタイトル（バリデーションエラー）
        "document_type_code": "A",
        "department_code": "T",
        "created_by": 1,
        "created_date": "2025-08-17"
    });

    // When: 無効なリクエストで文書作成APIにリクエスト
    let response = client
        .post(format!("http://{addr}/api/documents"))
        .json(&invalid_request)
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 400 Bad Requestが返される
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let error_body: serde_json::Value = response.json().await.unwrap();
    assert!(error_body["error"].as_str().unwrap().contains("Title"));
}

#[tokio::test]
async fn test_get_nonexistent_document() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    // When: 存在しない文書IDで取得APIにリクエスト
    let response = client
        .get(format!("http://{addr}/api/documents/999999"))
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 404 Not Foundが返される
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_cors_headers() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    // When: CORSプリフライトリクエスト
    let response = client
        .request(
            reqwest::Method::OPTIONS,
            format!("http://{addr}/api/documents"),
        )
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "POST")
        .header("Access-Control-Request-Headers", "Content-Type")
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 適切なCORSヘッダーが返される
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .headers()
            .contains_key("access-control-allow-origin")
    );
    assert!(
        response
            .headers()
            .contains_key("access-control-allow-methods")
    );
}
