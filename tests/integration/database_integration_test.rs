use chrono::NaiveDate;
use doc_man_db::models::CreateDocumentRequest;
use doc_man_db::repositories::{DocumentRepository, SqliteDocumentRepository};
use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};
use tempfile::TempDir;

// テスト用データベースセットアップヘルパー
async fn setup_test_database() -> (SqlitePool, TempDir) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    let database_url = format!("sqlite://{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // マイグレーション実行
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    (pool, temp_dir)
}

#[tokio::test]
async fn test_database_connection() {
    // Given: テスト用データベース
    let (pool, _temp_dir) = setup_test_database().await;

    // When: 簡単なクエリ実行
    let result = sqlx::query("SELECT 1 as test_value").fetch_one(&pool).await;

    // Then: 成功する
    assert!(result.is_ok());
    let row = result.unwrap();
    let test_value: i32 = row.get("test_value");
    assert_eq!(test_value, 1);
}

#[tokio::test]
async fn test_documents_table_exists() {
    // Given: マイグレーション適用済みデータベース
    let (pool, _temp_dir) = setup_test_database().await;

    // When: documents テーブルから選択
    let result = sqlx::query("SELECT COUNT(*) as count FROM documents")
        .fetch_one(&pool)
        .await;

    // Then: テーブルが存在する（エラーにならない）
    assert!(result.is_ok());
    let row = result.unwrap();
    let count: i32 = row.get("count");
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_manual_document_insert() {
    // Given: テスト用データベース
    let (pool, _temp_dir) = setup_test_database().await;

    // When: 手動でdocumentを挿入
    let result = sqlx::query(
        r#"
        INSERT INTO documents (number, title, document_type_id, created_by, created_date)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind("TEST-000001")
    .bind("テスト文書")
    .bind(1)
    .bind(1)
    .bind("2024-12-15")
    .execute(&pool)
    .await;

    // Then: 挿入が成功する
    assert!(result.is_ok());
    let rows_affected = result.unwrap().rows_affected();
    assert_eq!(rows_affected, 1);

    // And: 挿入されたデータを取得できる
    let row = sqlx::query(
        "SELECT id, number, title, document_type_id, created_by, created_date, created_at, updated_at FROM documents WHERE title = ?"
    )
    .bind("テスト文書")
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch inserted document");

    let number: String = row.get("number");
    let title: String = row.get("title");
    let document_type_id: i32 = row.get("document_type_id");
    let created_by: i32 = row.get("created_by");
    let created_date: String = row.get("created_date");

    assert_eq!(number, "TEST-000001");
    assert_eq!(title, "テスト文書");
    assert_eq!(document_type_id, 1);
    assert_eq!(created_by, 1);
    assert_eq!(created_date, "2024-12-15");
}

// Repository実装テスト
#[tokio::test]
async fn test_repository_create_document() {
    // Given: リポジトリとテストデータベース
    let (pool, _temp_dir) = setup_test_database().await;
    let repo = SqliteDocumentRepository::new(pool);

    let request = CreateDocumentRequest {
        number: None,
        title: "Repository テスト文書".to_string(),
        document_type_id: 1,
        business_number: None,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
    };

    // When: リポジトリ経由で文書作成
    let result = repo.create(request).await;

    // Then: 成功し、IDが付与される
    match result {
        Ok(document) => {
            assert!(document.id > 0);
            assert_eq!(document.title, "Repository テスト文書");
            assert_eq!(document.document_type_id, 1);
            assert_eq!(document.created_by, 1);
        }
        Err(e) => {
            panic!("Repository create failed: {e:?}");
        }
    }
}
