# Phase 6: テスト・品質保証・運用準備 (Week 12)

## フェーズ概要

- **期間**: Week 12 (1週間)
- **目標**: システムの品質保証とリリース準備
- **成果物**: テスト完了・品質保証、運用環境構築、本番リリース

## タスク一覧

### TASK-038: 単体テスト完成

- **説明**: 全モジュールの単体テスト
- **優先度**: High
- **見積工数**: 16h
- **状態**: 未着手
- **依存関係**: TASK-037

#### 実装内容(TASK-038)

1. Repository テストカバレッジ 90%以上
2. Service レイヤーテスト完成
3. ビジネスロジックテスト
4. エラーケーステスト

#### Repository テスト例

```rust
// tests/repositories/document_repository_test.rs
use sqlx::SqlitePool;
use crate::repositories::DocumentRepository;
use crate::models::{Document, CreateDocumentRequest};
use chrono::{NaiveDate, Utc};

#[sqlx::test]
async fn test_create_document_success(pool: SqlitePool) {
    // テストデータ準備
    setup_test_data(&pool).await;
    
    let repo = DocumentRepository::new(pool);
    let request = CreateDocumentRequest {
        number: Some("TEST-001".to_string()),
        title: "テスト文書".to_string(),
        document_type_id: 1,
        business_number: Some("JOB-2024-001".to_string()),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        internal_external: Some("internal".to_string()),
        importance_class: Some("class2".to_string()),
        personal_info: Some("none".to_string()),
        notes: Some("テスト用文書".to_string()),
    };

    // テスト実行
    let result = repo.create(request).await;

    // 検証
    assert!(result.is_ok());
    let document = result.unwrap();
    assert_eq!(document.title, "テスト文書");
    assert_eq!(document.number, "TEST-001");
    assert!(document.id > 0);
}

#[sqlx::test]
async fn test_search_documents_with_filters(pool: SqlitePool) {
    // テストデータ準備
    setup_test_documents(&pool).await;
    
    let repo = DocumentRepository::new(pool);
    let filters = DocumentSearchFilters {
        title: Some("会議".to_string()),
        created_date_from: Some(NaiveDate::from_ymd_opt(2024, 12, 1).unwrap()),
        created_date_to: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
        pagination: Pagination { offset: 0, limit: 10 },
        ..Default::default()
    };

    // テスト実行
    let result = repo.search(filters).await;

    // 検証
    assert!(result.is_ok());
    let (documents, total) = result.unwrap();
    assert_eq!(documents.len(), 2);
    assert_eq!(total, 2);
    assert!(documents.iter().all(|d| d.title.contains("会議")));
}

#[sqlx::test]
async fn test_update_document_not_found(pool: SqlitePool) {
    let repo = DocumentRepository::new(pool);
    let update = UpdateDocumentRequest {
        title: Some("更新タイトル".to_string()),
        ..Default::default()
    };

    // 存在しないIDで更新試行
    let result = repo.update(999, update).await;

    // エラーが返されることを確認
    assert!(result.is_err());
    assert_matches!(result.unwrap_err(), DocumentError::NotFound { .. });
}

async fn setup_test_documents(pool: &SqlitePool) {
    sqlx::query!(
        r#"
        INSERT INTO documents (
            number, title, document_type_id, created_by, created_date, 
            internal_external, importance_class, personal_info, is_active
        ) VALUES 
        ('TEST-001', '月次会議議事録', 1, 1, '2024-12-15', 'internal', 'class2', 'none', 1),
        ('TEST-002', '定例会議資料', 1, 1, '2024-12-14', 'internal', 'class2', 'none', 1),
        ('TEST-003', '提案書', 2, 2, '2024-12-13', 'external', 'class1', 'present', 1)
        "#
    )
    .execute(pool)
    .await
    .unwrap();
}
```

#### Service テスト例

```rust
// tests/services/document_service_test.rs
use mockall::predicate::*;
use crate::services::DocumentService;
use crate::repositories::MockDocumentRepository;
use crate::services::MockNumberGenerationService;

#[tokio::test]
async fn test_create_document_with_number_generation() {
    // モック設定
    let mut mock_repo = MockDocumentRepository::new();
    let mut mock_number_service = MockNumberGenerationService::new();
    
    mock_number_service
        .expect_generate_number()
        .with(eq("T"), eq("A"), always())
        .times(1)
        .returning(|_, _, _| Ok("CTA-2508001".to_string()));
    
    mock_repo
        .expect_create()
        .with(always())
        .times(1)
        .returning(|req| {
            Ok(Document {
                id: 1,
                number: req.number.unwrap(),
                title: req.title,
                document_type_id: req.document_type_id,
                created_by: req.created_by,
                created_date: req.created_date,
                internal_external: req.internal_external,
                importance_class: req.importance_class,
                personal_info: req.personal_info,
                notes: req.notes,
                network_path: Some("\\\\server01\\docs\\2024\\技術部\\報告書".to_string()),
                is_active: true,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            })
        });
    
    let service = DocumentService::new(
        Box::new(mock_repo), 
        Box::new(mock_number_service),
        Box::new(MockPathGenerationService::new())
    );
    
    let request = CreateDocumentRequest {
        number: None, // 自動生成
        title: "テスト文書".to_string(),
        document_type_id: 1,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        ..Default::default()
    };
    
    let user_permissions = UserPermissions::default();
    
    // テスト実行
    let result = service.create_document(request, &user_permissions).await;
    
    // 検証
    assert!(result.is_ok());
    let document = result.unwrap();
    assert_eq!(document.number, "CTA-2508001");
    assert_eq!(document.title, "テスト文書");
}

#[tokio::test]
async fn test_search_with_permission_filter() {
    let mut mock_repo = MockDocumentRepository::new();
    
    // 権限に基づくフィルタリング
    mock_repo
        .expect_search_with_permissions()
        .with(always(), eq(vec![1, 2])) // アクセス可能部署
        .times(1)
        .returning(|_, _| Ok((vec![/* 権限のある文書のみ */], 5)));
    
    let service = DocumentService::new(
        Box::new(mock_repo),
        Box::new(MockNumberGenerationService::new()),
        Box::new(MockPathGenerationService::new())
    );
    
    let user_permissions = UserPermissions {
        accessible_departments: vec![1, 2],
        can_view_confidential: false,
        ..Default::default()
    };
    
    // テスト実行
    let result = service.search_documents(
        DocumentSearchFilters::default(),
        &user_permissions
    ).await;
    
    // 検証
    assert!(result.is_ok());
    // 権限フィルタリングが適用されていることを確認
}
```

#### テスト実行設定

```bash
# テストカバレッジ測定
cargo install cargo-tarpaulin

# カバレッジレポート生成
cargo tarpaulin --out Html --output-dir coverage

# 単体テスト実行
cargo test --lib

# 特定モジュールのテスト
cargo test repositories::document_repository
```

#### Property-based Testing

```rust
// tests/property/document_properties.rs
use proptest::prelude::*;
use crate::services::NumberGenerationService;
use crate::models::CreateDocumentRequest;

// 文書番号生成のプロパティテスト
proptest! {
    #[test]
    fn test_document_number_generation_properties(
        dept_code in "[A-Z]{1,3}",
        year in 2020u32..2030u32,
        sequence in 1u32..99999u32
    ) {
        let service = NumberGenerationService::new();
        let result = service.format_number(&dept_code, year, sequence);
        
        // プロパティ検証
        prop_assert!(result.len() <= 20, "Document number too long: {}", result);
        prop_assert!(result.contains(&dept_code), "Missing department code: {}", result);
        prop_assert!(result.contains(&year.to_string()[2..]), "Missing year: {}", result);
        prop_assert!(!result.contains(" "), "Contains spaces: {}", result);
        prop_assert!(result.chars().all(|c| c.is_alphanumeric() || c == '-'), "Invalid characters: {}", result);
    }
    
    #[test] 
    fn test_search_input_validation(
        title in "[\\w\\s\u3042-\u3093\u30a2-\u30f3\u4e00-\u9fff]{0,100}",
        limit in 1u32..1000u32,
        offset in 0u32..100000u32
    ) {
        let search_input = DocumentSearchInput {
            title: if title.trim().is_empty() { None } else { Some(title) },
            pagination: Pagination { limit, offset },
            ..Default::default()
        };
        
        let validation_result = validate_search_input(&search_input);
        
        // 入力検証プロパティ
        prop_assert!(validation_result.is_ok(), "Valid input should pass validation");
        prop_assert!(search_input.pagination.limit > 0, "Limit must be positive");
        prop_assert!(search_input.pagination.limit <= 1000, "Limit must not exceed 1000");
    }
    
    #[test]
    fn test_network_path_generation_properties(
        document_number in "[A-Z]+-[0-9]{8}",
        year in 2020u32..2030u32,
        dept_name in "[\\w]{1,20}"
    ) {
        let path_service = PathGenerationService::new();
        let result = path_service.generate_path(&document_number, &dept_name, year);
        
        prop_assert!(result.is_ok(), "Path generation should succeed");
        
        if let Ok(path) = result {
            prop_assert!(path.starts_with("\\\\"), "Path should be UNC path: {}", path);
            prop_assert!(path.contains(&year.to_string()), "Path should contain year: {}", path);
            prop_assert!(path.contains(&dept_name), "Path should contain department: {}", path);
            prop_assert!(!path.contains("//"), "Path should not have double slashes: {}", path);
        }
    }
}

// セキュリティプロパティテスト
proptest! {
    #[test]
    fn test_sql_injection_resistance(
        malicious_input in ".*['\";<>\\\\]",
        table_name in "(users|documents|employees)",
        operation in "(SELECT|INSERT|UPDATE|DELETE)"
    ) {
        let repository = DocumentRepositoryImpl::new(test_pool());
        
        // 悪意ある入力での検索テスト
        let search_filters = DocumentSearchFilters {
            title: Some(malicious_input.clone()),
            business_number: Some(malicious_input),
            ..Default::default()
        };
        
        let result = repository.search_with_permissions(
            search_filters, 
            &UserPermissions::default()
        ).await;
        
        // SQLインジェクションが成功しないことを確認
        prop_assert!(
            result.is_ok() || matches!(result, Err(DocumentError::Validation(_))),
            "Malicious input should be handled safely"
        );
    }
}
```

#### 成果物(TASK-038)

- **90%以上のテストカバレッジ**
- **全Repository・Serviceテスト**
- **エラーケーステスト**
- **モックテスト**
- **🚀 Property-based Testing**
- **🔒 セキュリティプロパティテスト**

---

### TASK-039: 統合テスト

- **説明**: エンドツーエンドテスト
- **優先度**: High
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-038

#### 実装内容(TASK-039)

1. API統合テスト
2. データベーストランザクションテスト
3. 外部システム連携テスト
4. エラーレスポンステスト

#### API統合テスト

```rust
// tests/integration/api_integration_test.rs
use axum_test::TestServer;
use sqlx::SqlitePool;
use serde_json::json;

#[sqlx::test]
async fn test_document_crud_workflow(pool: SqlitePool) {
    // テストサーバー起動
    let app = create_test_app(pool).await;
    let server = TestServer::new(app).unwrap();
    
    // 認証トークン取得
    let auth_token = get_test_auth_token(&server).await;
    
    // 1. 文書作成
    let create_response = server
        .post("/graphql")
        .add_header("Authorization", format!("Bearer {}", auth_token))
        .json(&json!({
            "query": r#"
                mutation CreateDocument($input: CreateDocumentInput!) {
                    createDocument(input: $input) {
                        success
                        data {
                            id
                            number
                            title
                            networkPath
                        }
                        errors
                    }
                }
            "#,
            "variables": {
                "input": {
                    "title": "統合テスト文書",
                    "documentTypeId": "1",
                    "createdDate": "2024-12-15",
                    "internalExternal": "INTERNAL",
                    "importanceClass": "CLASS2",
                    "personalInfo": "NONE"
                }
            }
        }))
        .await;
    
    create_response.assert_status_ok();
    let create_data: serde_json::Value = create_response.json();
    
    assert!(create_data["data"]["createDocument"]["success"].as_bool().unwrap());
    let document_id = create_data["data"]["createDocument"]["data"]["id"].as_str().unwrap();
    let document_number = create_data["data"]["createDocument"]["data"]["number"].as_str().unwrap();
    let network_path = create_data["data"]["createDocument"]["data"]["networkPath"].as_str().unwrap();
    
    // 文書番号形式の検証
    assert!(document_number.len() > 0);
    assert!(network_path.starts_with("\\\\"));
    
    // 2. 文書取得
    let get_response = server
        .post("/graphql")
        .add_header("Authorization", format!("Bearer {}", auth_token))
        .json(&json!({
            "query": r#"
                query GetDocument($id: ID!) {
                    document(id: $id) {
                        id
                        number
                        title
                        createdDate
                        creator {
                            name
                        }
                        documentType {
                            name
                        }
                    }
                }
            "#,
            "variables": { "id": document_id }
        }))
        .await;
    
    get_response.assert_status_ok();
    let get_data: serde_json::Value = get_response.json();
    
    assert_eq!(
        get_data["data"]["document"]["number"].as_str().unwrap(),
        document_number
    );
    assert_eq!(
        get_data["data"]["document"]["title"].as_str().unwrap(),
        "統合テスト文書"
    );
    
    // 3. 文書検索
    let search_response = server
        .post("/graphql")
        .add_header("Authorization", format!("Bearer {}", auth_token))
        .json(&json!({
            "query": r#"
                query SearchDocuments($input: DocumentSearchInput!) {
                    searchDocuments(input: $input) {
                        documents {
                            id
                            number
                            title
                        }
                        totalCount
                        hasNextPage
                    }
                }
            "#,
            "variables": {
                "input": {
                    "title": "統合テスト",
                    "pagination": {
                        "offset": 0,
                        "limit": 10
                    }
                }
            }
        }))
        .await;
    
    search_response.assert_status_ok();
    let search_data: serde_json::Value = search_response.json();
    
    let documents = search_data["data"]["searchDocuments"]["documents"].as_array().unwrap();
    assert!(documents.len() >= 1);
    assert_eq!(documents[0]["number"].as_str().unwrap(), document_number);
    
    // 4. 文書更新
    let update_response = server
        .post("/graphql")
        .add_header("Authorization", format!("Bearer {}", auth_token))
        .json(&json!({
            "query": r#"
                mutation UpdateDocument($id: ID!, $input: UpdateDocumentInput!) {
                    updateDocument(id: $id, input: $input) {
                        success
                        data {
                            title
                            notes
                        }
                        errors
                    }
                }
            "#,
            "variables": {
                "id": document_id,
                "input": {
                    "title": "更新された統合テスト文書",
                    "notes": "統合テストで更新されました"
                }
            }
        }))
        .await;
    
    update_response.assert_status_ok();
    let update_data: serde_json::Value = update_response.json();
    
    assert!(update_data["data"]["updateDocument"]["success"].as_bool().unwrap());
    assert_eq!(
        update_data["data"]["updateDocument"]["data"]["title"].as_str().unwrap(),
        "更新された統合テスト文書"
    );
    
    // 5. ファイル存在確認
    let file_check_response = server
        .post("/api/files/check")
        .add_header("Authorization", format!("Bearer {}", auth_token))
        .json(&json!({
            "documentId": document_id.parse::<i32>().unwrap()
        }))
        .await;
    
    file_check_response.assert_status_ok();
    let file_check_data: serde_json::Value = file_check_response.json();
    
    // ファイル確認結果の検証（存在しない可能性があるのでエラーでないことを確認）
    assert!(file_check_data.get("folderExists").is_some());
}

#[sqlx::test]
async fn test_permission_based_access_control(pool: SqlitePool) {
    let app = create_test_app(pool).await;
    let server = TestServer::new(app).unwrap();
    
    // 制限されたユーザーでログイン
    let limited_token = get_limited_user_token(&server).await;
    
    // 管理者機能へのアクセス試行
    let admin_response = server
        .post("/api/admin/users")
        .add_header("Authorization", format!("Bearer {}", limited_token))
        .await;
    
    admin_response.assert_status(StatusCode::FORBIDDEN);
    
    // 機密文書へのアクセス試行
    let confidential_response = server
        .post("/graphql")
        .add_header("Authorization", format!("Bearer {}", limited_token))
        .json(&json!({
            "query": r#"
                query GetConfidentialDocument($id: ID!) {
                    document(id: $id) {
                        id
                        title
                    }
                }
            "#,
            "variables": { "id": "999" } // 機密文書ID
        }))
        .await;
    
    confidential_response.assert_status_ok();
    let data: serde_json::Value = confidential_response.json();
    
    // 権限がない場合はnullが返される
    assert!(data["data"]["document"].is_null());
}
```

#### データベーストランザクションテスト

```rust
// tests/integration/transaction_test.rs
#[sqlx::test]
async fn test_document_creation_transaction_rollback(pool: SqlitePool) {
    let document_service = DocumentService::new(/* ... */);
    
    // 正常なケース
    let valid_request = CreateDocumentRequest {
        title: "有効な文書".to_string(),
        document_type_id: 1,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        ..Default::default()
    };
    
    let result1 = document_service.create_document(valid_request, &UserPermissions::admin()).await;
    assert!(result1.is_ok());
    
    // エラーが発生するケース（重複文書番号）
    let duplicate_request = CreateDocumentRequest {
        number: Some(result1.unwrap().number), // 重複番号
        title: "重複文書".to_string(),
        document_type_id: 1,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        ..Default::default()
    };
    
    let result2 = document_service.create_document(duplicate_request, &UserPermissions::admin()).await;
    assert!(result2.is_err());
    
    // データベースが一貫した状態であることを確認
    let search_result = document_service.search_documents(
        DocumentSearchFilters {
            title: Some("重複文書".to_string()),
            ..Default::default()
        },
        &UserPermissions::admin()
    ).await.unwrap();
    
    assert_eq!(search_result.0.len(), 0); // 重複文書は作成されていない
}
```

#### 成果物(TASK-039)

- API統合テスト
- トランザクションテスト
- 権限制御テスト
- エラーレスポンステスト

---

### TASK-040: 性能テスト

- **説明**: 負荷テスト・性能測定
- **優先度**: High
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-039

#### 実装内容(TASK-040)

1. 負荷テスト実行
2. レスポンス時間測定
3. メモリ使用量監視
4. 性能改善提案

#### ベンチマークテスト

```rust
// benches/document_search_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tokio::runtime::Runtime;

fn document_search_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let service = rt.block_on(create_test_document_service());
    
    // 大量テストデータの準備
    rt.block_on(setup_large_dataset(10000)); // 1万件のテストデータ
    
    let mut group = c.benchmark_group("document_search");
    
    for record_count in [100, 1000, 5000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("search_by_title", record_count),
            record_count,
            |b, &record_count| {
                b.to_async(&rt).iter(|| async {
                    let filters = DocumentSearchFilters {
                        title: Some("テスト".to_string()),
                        pagination: Pagination { offset: 0, limit: 50 },
                        ..Default::default()
                    };
                    
                    let result = service.search_documents(
                        black_box(filters),
                        &UserPermissions::admin()
                    ).await;
                    
                    black_box(result);
                });
            },
        );
    }
    
    group.finish();
}

fn file_existence_check_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let service = rt.block_on(create_test_file_check_service());
    
    c.bench_function("file_existence_check_100", |b| {
        b.to_async(&rt).iter(|| async {
            let documents = get_test_documents(100).await;
            
            for document in black_box(documents) {
                let result = service.check_document_existence(&document).await;
                black_box(result);
            }
        })
    });
}

criterion_group!(benches, document_search_benchmark, file_existence_check_benchmark);
criterion_main!(benches);
```

#### 負荷テストシナリオ

```bash
# ベンチマーク実行
cargo bench

# CPU・メモリプロファイリング
cargo install flamegraph
cargo flamegraph --bench document_search_benchmark

# 性能測定レポート
cargo bench -- --output-format html
```

#### 性能基準

| 機能         | 目標性能   | 測定方法                    |
| ------------ | ---------- | --------------------------- |
| 文書検索     | 2秒以内    | 1000件検索での平均応答時間  |
| 文書作成     | 1秒以内    | 文書番号生成込みの作成時間  |
| ファイル確認 | 5秒以内    | ネットワークアクセス込み    |
| 同時接続     | 10ユーザー | 同時検索での性能劣化20%以内 |

#### 成果物(TASK-040)

- 性能ベンチマーク結果
- 負荷テストレポート
- 性能改善提案
- 監視メトリクス

---

### TASK-041: ドキュメント整備

- **説明**: API仕様・運用手順書
- **優先度**: Medium
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-040

#### 実装内容(TASK-041)

1. API仕様書自動生成
2. 運用手順書作成
3. ユーザーマニュアル
4. トラブルシューティングガイド

#### API仕様書生成

```rust
// src/docs/api_docs.rs
use async_graphql::*;

// GraphQLスキーマからAPI仕様書を自動生成
pub fn generate_api_docs() -> String {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .finish();
    
    schema.sdl()
}

// OpenAPIスペック生成（REST API用）
pub fn generate_openapi_spec() -> utoipa::openapi::OpenApi {
    use utoipa::OpenApi;
    
    #[derive(OpenApi)]
    #[openapi(
        paths(
            health_check,
            file_check,
            batch_run
        ),
        components(
            schemas(FileCheckResponse, BatchExecutionResponse)
        ),
        tags(
            (name = "Health", description = "System health endpoints"),
            (name = "Files", description = "File operation endpoints"),
            (name = "Batch", description = "Batch processing endpoints")
        )
    )]
    struct ApiDoc;
    
    ApiDoc::openapi()
}
```

#### 運用手順書

```markdown
  # 運用手順書
  
  ## 日次運用
  
  ### システム稼働確認
  #### 1. ヘルスチェック確認
  
     ```bash
     curl http://localhost:8080/health
     ```
  
  #### 2. ログ確認
  
     ```bash
     tail -f logs/application.log
     ```
  
  #### 3. データベース接続確認
  
     ```bash
     sqlx migrate info --database-url sqlite://./data/prod.db
     ```
  
  ## 月次運用
  
  ### ファイル存在確認バッチ
  
  - 実行タイミング: 毎月1日 9:00 自動実行
  - 手動実行:
  
    ```bash
    curl -X POST /api/admin/batch/run/file-check
    ```
  
  ### データベースメンテナンス
  
  1. バックアップ作成
  
     ```bash
     cp data/prod.db backup/prod_$(date +%Y%m%d).db
     ```
  
  2. 統計情報更新
  
     ```sql
     ANALYZE;
     ```
  
  ## トラブルシューティング
  
  ### データベース接続エラー
  
  **症状**: `Connection refused` エラー
  **原因**: データベースファイルのロックまたは権限問題
  **対処**:
  
  1. プロセス確認: `ps aux | grep doc_man_db`
  2. ファイル権限確認: `ls -la data/prod.db`
  3. 必要に応じてプロセス再起動
  
  ### ファイル確認エラー
  
  **症状**: ネットワークパスアクセスエラー
  **原因**: ネットワークドライブ接続問題
  **対処**:
  
  1. ネットワーク接続確認
  2. 認証情報確認
  3. パス設定確認

```

#### 成果物(TASK-041)

- API仕様書（自動生成）
- 運用手順書
- ユーザーマニュアル
- トラブルシューティングガイド

---

### TASK-042: デプロイ設定

- **説明**: 本番環境設定・CI/CD
- **優先度**: Medium
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-041

#### 実装内容(TASK-042)

1. 本番環境設定
2. CI/CDパイプライン
3. デプロイスクリプト
4. 環境監視設定

#### GitHub Actions CI/CD

```yaml
# .github/workflows/deploy.yml
name: Deploy to Production

on:
  push:
    branches: [main]
    tags: ['v*']

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
    
    - name: Cache cargo
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Run integration tests
      run: cargo test --test '*'
    
  build:
    needs: test
    runs-on: windows-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Build release
      run: cargo build --release
    
    - name: Create deployment package
      run: |
        mkdir deploy
        Copy-Item target/release/doc_man_db.exe deploy/
        Copy-Item -Recurse migrations deploy/
        Copy-Item .env.production deploy/.env
        Compress-Archive -Path deploy/* -DestinationPath doc_man_db.zip
    
    - name: Upload artifact
      uses: actions/upload-artifact@v3
      with:
        name: deployment-package
        path: doc_man_db.zip
  
  deploy:
    needs: build
    runs-on: windows-latest
    if: startsWith(github.ref, 'refs/tags/v')
    
    steps:
    - name: Download artifact
      uses: actions/download-artifact@v3
      with:
        name: deployment-package
    
    - name: Deploy to production
      run: |
        # PowerShellデプロイスクリプト実行
        ./scripts/deploy.ps1 -Package doc_man_db.zip -Environment production
```

#### デプロイスクリプト

```powershell
# scripts/deploy.ps1
param(
    [Parameter(Mandatory=$true)]
    [string]$Package,
    
    [Parameter(Mandatory=$true)]
    [ValidateSet("staging", "production")]
    [string]$Environment
)

$ErrorActionPreference = "Stop"

Write-Host "Starting deployment to $Environment"

# 設定読み込み
$config = Get-Content "deploy-config.json" | ConvertFrom-Json
$deployPath = $config.$Environment.deployPath
$serviceName = $config.$Environment.serviceName

try {
    # サービス停止
    Write-Host "Stopping service: $serviceName"
    Stop-Service -Name $serviceName -Force -ErrorAction SilentlyContinue
    
    # バックアップ作成
    $backupPath = "$deployPath\backup\$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    Write-Host "Creating backup: $backupPath"
    if (Test-Path $deployPath) {
        Copy-Item -Path $deployPath -Destination $backupPath -Recurse
    }
    
    # 新しいバージョン展開
    Write-Host "Extracting package: $Package"
    Expand-Archive -Path $Package -DestinationPath $deployPath -Force
    
    # データベースマイグレーション実行
    Write-Host "Running database migrations"
    Set-Location $deployPath
    .\doc_man_db.exe migrate
    
    # サービス開始
    Write-Host "Starting service: $serviceName"
    Start-Service -Name $serviceName
    
    # ヘルスチェック
    Write-Host "Performing health check"
    $healthUrl = $config.$Environment.healthUrl
    for ($i = 1; $i -le 30; $i++) {
        try {
            $response = Invoke-RestMethod -Uri $healthUrl -TimeoutSec 10
            if ($response -eq "OK") {
                Write-Host "Health check passed"
                break
            }
        }
        catch {
            if ($i -eq 30) {
                throw "Health check failed after 30 attempts"
            }
            Start-Sleep -Seconds 2
        }
    }
    
    Write-Host "Deployment completed successfully"
}
catch {
    Write-Error "Deployment failed: $_"
    
    # ロールバック
    if (Test-Path $backupPath) {
        Write-Host "Rolling back to previous version"
        Remove-Item -Path $deployPath -Recurse -Force
        Copy-Item -Path $backupPath -Destination $deployPath -Recurse
        Start-Service -Name $serviceName
    }
    
    exit 1
}
```

#### 成果物(TASK-042)

- 本番環境設定
- CI/CDパイプライン
- 自動デプロイスクリプト
- ロールバック機能

---

### TASK-043: 運用監視設定

- **説明**: ログ監視・アラート設定
- **優先度**: Medium
- **見積工数**: 6h
- **状態**: 未着手
- **依存関係**: TASK-042

#### 実装内容(TASK-043)

1. ログ監視設定
2. メトリクス収集
3. アラート設定
4. ダッシュボード構築

#### 成果物(TASK-043)

- ログ監視システム
- メトリクス収集
- アラート通知
- 運用ダッシュボード

---

### TASK-044: ユーザー研修準備

- **説明**: 操作マニュアル・研修資料
- **優先度**: Low
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-043

#### 成果物(TASK-044)

- 操作マニュアル
- 研修資料
- FAQ集

---

### TASK-045: 本番リリース

- **説明**: 本番環境デプロイ・検証
- **優先度**: High
- **見積工数**: 4h
- **状態**: 未着手
- **依存関係**: TASK-044

#### 成果物(TASK-045)

- 本番環境デプロイ完了
- 本番動作検証
- リリース完了報告

## フェーズ完了基準

### 必須条件

- [ ] 単体テストカバレッジ 90%以上
- [ ] 統合テスト全て通過
- [ ] 性能基準を満たす
- [ ] API仕様書完成
- [ ] 本番環境デプロイ成功
- [ ] 運用監視システム稼働

### 検証方法

```bash
# テスト実行
cargo test --all
cargo tarpaulin --out Html

# 性能テスト
cargo bench

# 本番デプロイ
./scripts/deploy.ps1 -Package release.zip -Environment production
```

## 次フェーズへの引き継ぎ事項

- 基本版リリース完了
- 運用体制確立
- 高度機能開発準備
- ユーザーフィードバック収集開始

## リスク・課題

- **テスト時間不足**: 品質保証期間の確保
- **本番環境問題**: 環境差異による不具合
- **性能要件**: 本番データでの性能検証

## 対応策

- 早期テスト開始・自動化
- 本番類似環境でのテスト
- 段階的負荷テスト実施
