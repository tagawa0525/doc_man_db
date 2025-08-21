use axum::extract::{Path, State};
use doc_man_db::app::AppState;
use doc_man_db::handlers::csv_import::{
    download_csv_template, get_import_execution, get_import_executions, get_import_progress,
};
use doc_man_db::handlers::{DocumentHandlers, HealthHandler};
use doc_man_db::repositories::{SqliteDocumentNumberRuleRepository, SqliteDocumentRepository};
use doc_man_db::services::DocumentService;
use uuid::Uuid;

// テスト用のAppState作成ヘルパー
async fn create_test_app_state() -> AppState {
    let doc_repo = SqliteDocumentRepository::new_in_memory()
        .await
        .expect("Failed to create document repository");
    let rule_repo = SqliteDocumentNumberRuleRepository::new_in_memory()
        .await
        .expect("Failed to create rule repository");

    let document_service = DocumentService::new(doc_repo, rule_repo);
    let document_handlers = DocumentHandlers::new(document_service);
    let health_handler = HealthHandler::new();

    AppState {
        document_handlers,
        health_handler,
    }
}

#[tokio::test]
async fn test_download_csv_template() {
    // When: CSV テンプレート ダウンロード
    let result = download_csv_template().await;

    // Then: 成功する
    assert!(result.is_ok());
    let (headers, content) = result.unwrap();

    // ヘッダーチェック
    assert!(headers.contains_key(axum::http::header::CONTENT_TYPE));
    assert!(headers.contains_key(axum::http::header::CONTENT_DISPOSITION));

    // コンテンツチェック
    assert!(content.contains("title,document_type_code"));
    assert!(content.contains("サンプル文書1"));
    assert!(content.contains("tech"));
    assert!(content.contains("山田太郎"));
}

#[tokio::test]
async fn test_get_import_executions() {
    // Given: テスト用AppState
    let app_state = create_test_app_state().await;

    // When: インポート実行履歴取得
    let result = get_import_executions(State(app_state)).await;

    // Then: 成功する
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.0["executions"].is_array());

    let executions = response.0["executions"].as_array().unwrap();
    assert_eq!(executions.len(), 1);
    assert!(executions[0]["import_id"].is_string());
    assert_eq!(executions[0]["file_name"], "documents_2024.csv");
    assert_eq!(executions[0]["total_records"], 100);
    assert_eq!(executions[0]["successful_imports"], 95);
    assert_eq!(executions[0]["failed_imports"], 5);
    assert_eq!(executions[0]["status"], "completed");
}

#[tokio::test]
async fn test_get_import_execution_success() {
    // Given: テスト用AppStateと有効なUUID
    let app_state = create_test_app_state().await;
    let import_id = Uuid::new_v4();

    // When: 特定のインポート実行詳細取得
    let result = get_import_execution(State(app_state), Path(import_id)).await;

    // Then: 成功する
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0["import_id"], import_id.to_string());
    assert_eq!(response.0["file_name"], "documents_2024.csv");
    assert_eq!(response.0["total_records"], 100);
    assert_eq!(response.0["successful_imports"], 95);
    assert_eq!(response.0["failed_imports"], 5);
    assert_eq!(response.0["status"], "completed");
    assert!(response.0["errors"].is_array());
}

#[tokio::test]
async fn test_get_import_progress() {
    // Given: テスト用AppStateと有効なUUID
    let app_state = create_test_app_state().await;
    let import_id = Uuid::new_v4();

    // When: インポート進捗取得
    let result = get_import_progress(State(app_state), Path(import_id)).await;

    // Then: 成功する
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0["import_id"], import_id.to_string());
    assert_eq!(response.0["status"], "completed");
    assert_eq!(response.0["progress"], 100);
    assert_eq!(response.0["current_record"], 0);
    assert_eq!(response.0["total_records"], 0);
    assert_eq!(response.0["message"], "Import completed");
}

#[tokio::test]
async fn test_csv_template_content_format() {
    // When: CSV テンプレート取得
    let result = download_csv_template().await;
    assert!(result.is_ok());
    let (_, content) = result.unwrap();

    // Then: 正しいCSV形式である
    let lines: Vec<&str> = content.trim().split('\n').collect();
    assert_eq!(lines.len(), 3); // ヘッダー + 2データ行

    // ヘッダー行の確認
    let header = lines[0];
    let expected_headers = vec![
        "title",
        "document_type_code",
        "creator_name",
        "created_date",
        "business_number",
        "department_code",
        "internal_external",
        "importance_class",
        "personal_info",
        "notes",
    ];
    for expected_header in expected_headers {
        assert!(header.contains(expected_header));
    }

    // データ行の確認
    assert!(lines[1].contains("サンプル文書1"));
    assert!(lines[1].contains("tech"));
    assert!(lines[2].contains("サンプル文書2"));
    assert!(lines[2].contains("plan"));
}

#[tokio::test]
async fn test_import_execution_response_structure() {
    // Given: テスト用AppState
    let app_state = create_test_app_state().await;

    // When: インポート実行履歴取得
    let result = get_import_executions(State(app_state)).await;
    assert!(result.is_ok());
    let response = result.unwrap();

    // Then: レスポンス構造が正しい
    let executions = &response.0["executions"];
    assert!(executions.is_array());

    let execution = &executions[0];

    // 必要なフィールドがすべて存在する
    let required_fields = vec![
        "import_id",
        "file_name",
        "total_records",
        "successful_imports",
        "failed_imports",
        "start_time",
        "end_time",
        "status",
    ];

    for field in required_fields {
        assert!(
            execution[field].is_string() || execution[field].is_number(),
            "Field '{field}' should exist and have correct type"
        );
    }
}

#[tokio::test]
async fn test_import_progress_response_structure() {
    // Given: テスト用AppStateと有効なUUID
    let app_state = create_test_app_state().await;
    let import_id = Uuid::new_v4();

    // When: インポート進捗取得
    let result = get_import_progress(State(app_state), Path(import_id)).await;
    assert!(result.is_ok());
    let response = result.unwrap();

    // Then: レスポンス構造が正しい
    let required_fields = vec![
        ("import_id", "string"),
        ("status", "string"),
        ("progress", "number"),
        ("current_record", "number"),
        ("total_records", "number"),
        ("message", "string"),
    ];

    for (field, expected_type) in required_fields {
        match expected_type {
            "string" => assert!(
                response.0[field].is_string(),
                "Field '{field}' should be a string"
            ),
            "number" => assert!(
                response.0[field].is_number(),
                "Field '{field}' should be a number"
            ),
            _ => panic!("Unexpected type: {expected_type}"),
        }
    }
}
