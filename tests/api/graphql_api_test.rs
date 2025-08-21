use axum::http::StatusCode;
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
async fn test_graphql_introspection() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    let introspection_query = json!({
        "query": r#"
            query IntrospectionQuery {
                __schema {
                    types {
                        name
                        kind
                    }
                }
            }
        "#
    });

    // When: GraphQLイントロスペクションクエリを実行
    let response = client
        .post(format!("http://{addr}/graphql"))
        .header("Content-Type", "application/json")
        .json(&introspection_query)
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返され、スキーマ情報が取得できる
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["data"]["__schema"]["types"].is_array());

    // DocumentとCreateDocumentInputタイプが存在することを確認
    let types = body["data"]["__schema"]["types"].as_array().unwrap();
    let type_names: Vec<&str> = types.iter().filter_map(|t| t["name"].as_str()).collect();

    assert!(type_names.contains(&"Document"));
    assert!(type_names.contains(&"CreateDocumentInput"));
}

#[tokio::test]
async fn test_graphql_create_document_mutation() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    let mutation = json!({
        "query": r#"
            mutation CreateDocument($input: CreateDocumentInput!) {
                createDocument(input: $input) {
                    document {
                        id
                        title
                        documentTypeId
                        createdBy
                        createdDate
                    }
                    documentNumber
                    generatedNumber {
                        ruleId
                        sequenceNumber
                        templateUsed
                    }
                }
            }
        "#,
        "variables": {
            "input": {
                "title": "GraphQL経由のテスト文書",
                "documentTypeCode": "A",
                "departmentCode": "T",
                "createdBy": 1,
                "createdDate": "2025-08-17"
            }
        }
    });

    // When: GraphQL文書作成ミューテーションを実行
    let response = client
        .post(format!("http://{addr}/graphql"))
        .header("Content-Type", "application/json")
        .json(&mutation)
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返され、文書が作成される
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["errors"].is_null());

    let created_document = &body["data"]["createDocument"];
    assert_eq!(
        created_document["document"]["title"],
        "GraphQL経由のテスト文書"
    );
    assert!(
        created_document["documentNumber"]
            .as_str()
            .unwrap()
            .starts_with("T-25")
    );
    assert_eq!(created_document["generatedNumber"]["ruleId"], 1);
}

#[tokio::test]
async fn test_graphql_query_document_by_id() {
    // Given: テストサーバーを起動し、文書を作成
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    // まず文書を作成
    let create_mutation = json!({
        "query": r#"
            mutation CreateDocument($input: CreateDocumentInput!) {
                createDocument(input: $input) {
                    document {
                        id
                    }
                }
            }
        "#,
        "variables": {
            "input": {
                "title": "GraphQL取得テスト文書",
                "documentTypeCode": "B",
                "departmentCode": "T",
                "createdBy": 1,
                "createdDate": "2025-08-17"
            }
        }
    });

    let create_response = client
        .post(format!("http://{addr}/graphql"))
        .json(&create_mutation)
        .send()
        .await
        .unwrap();

    let create_body: serde_json::Value = create_response.json().await.unwrap();
    let document_id = create_body["data"]["createDocument"]["document"]["id"]
        .as_i64()
        .unwrap();

    let query = json!({
        "query": r#"
            query GetDocument($id: Int!) {
                document(id: $id) {
                    id
                    title
                    documentTypeId
                    createdBy
                    createdDate
                    createdAt
                    updatedAt
                }
            }
        "#,
        "variables": {
            "id": document_id
        }
    });

    // When: GraphQL文書取得クエリを実行
    let response = client
        .post(format!("http://{addr}/graphql"))
        .header("Content-Type", "application/json")
        .json(&query)
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返され、文書が取得される
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["errors"].is_null());

    let document = &body["data"]["document"];
    assert_eq!(document["id"], document_id);
    assert_eq!(document["title"], "GraphQL取得テスト文書");
}

#[tokio::test]
async fn test_graphql_search_documents() {
    // Given: テストサーバーを起動し、複数の文書を作成
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    // 複数文書を作成
    for i in 1..=3 {
        let create_mutation = json!({
            "query": r#"
                mutation CreateDocument($input: CreateDocumentInput!) {
                    createDocument(input: $input) {
                        document {
                            id
                        }
                    }
                }
            "#,
            "variables": {
                "input": {
                    "title": format!("GraphQL検索テスト文書{}", i),
                    "documentTypeCode": "A",
                    "departmentCode": "T",
                    "createdBy": 1,
                    "createdDate": "2025-08-17"
                }
            }
        });

        client
            .post(format!("http://{addr}/graphql"))
            .json(&create_mutation)
            .send()
            .await
            .unwrap();
    }

    let search_query = json!({
        "query": r#"
            query SearchDocuments($filters: DocumentSearchFilters!) {
                searchDocuments(filters: $filters) {
                    documents {
                        id
                        title
                        documentTypeId
                    }
                    total
                }
            }
        "#,
        "variables": {
            "filters": {
                "title": "GraphQL検索テスト",
                "limit": 10,
                "offset": 0
            }
        }
    });

    // When: GraphQL文書検索クエリを実行
    let response = client
        .post(format!("http://{addr}/graphql"))
        .header("Content-Type", "application/json")
        .json(&search_query)
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返され、検索結果が取得される
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["errors"].is_null());

    let search_result = &body["data"]["searchDocuments"];
    let documents = search_result["documents"].as_array().unwrap();
    let total = search_result["total"].as_u64().unwrap();

    assert!(documents.len() >= 3);
    assert!(total >= 3);
}

#[tokio::test]
async fn test_graphql_validation_error() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    let invalid_mutation = json!({
        "query": r#"
            mutation CreateDocument($input: CreateDocumentInput!) {
                createDocument(input: $input) {
                    document {
                        id
                        title
                    }
                }
            }
        "#,
        "variables": {
            "input": {
                "title": "", // 空のタイトル（バリデーションエラー）
                "documentTypeCode": "A",
                "departmentCode": "T",
                "createdBy": 1,
                "createdDate": "2025-08-17"
            }
        }
    });

    // When: 無効なデータでGraphQLミューテーションを実行
    let response = client
        .post(format!("http://{addr}/graphql"))
        .header("Content-Type", "application/json")
        .json(&invalid_mutation)
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返されるが、GraphQLエラーが含まれる
    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["errors"].is_array());
    assert!(!body["errors"].as_array().unwrap().is_empty());

    let error_message = body["errors"][0]["message"].as_str().unwrap();
    assert!(error_message.contains("Title"));
}

#[tokio::test]
async fn test_graphql_playground_endpoint() {
    // Given: テストサーバーを起動
    let (addr, _server_handle) = spawn_app().await;
    let client = Client::new();

    // When: GraphQL Playgroundエンドポイントにアクセス
    let response = client
        .get(format!("http://{addr}/graphql"))
        .header("Accept", "text/html")
        .send()
        .await
        .expect("Failed to execute request");

    // Then: 200 OKが返され、HTMLが返される
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap();
    assert!(content_type.to_str().unwrap().contains("text/html"));
}
