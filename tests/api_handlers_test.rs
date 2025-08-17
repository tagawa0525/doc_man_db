use axum::http::{Request, StatusCode};
use axum::body::Body;
use doc_man_db::handlers::{DocumentHandlers, HealthHandler};
use doc_man_db::models::{CreateDocumentWithNumberRequest, Document};
use doc_man_db::services::DocumentService;
use doc_man_db::repositories::{DocumentRepository, DocumentNumberRuleRepository, RepositoryError};
use async_trait::async_trait;
use chrono::NaiveDate;
use tower::ServiceExt;

// テスト用のモックリポジトリ実装
struct MockDocumentRepository;
struct MockDocumentNumberRuleRepository;

#[async_trait]
impl DocumentRepository for MockDocumentRepository {
    async fn create(&self, request: doc_man_db::models::CreateDocumentRequest) -> Result<Document, RepositoryError> {
        Ok(Document {
            id: 1,
            title: request.title,
            document_type_id: request.document_type_id,
            created_by: request.created_by,
            created_date: request.created_date,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Document>, RepositoryError> {
        if id == 1 {
            Ok(Some(Document {
                id: 1,
                title: "テスト文書".to_string(),
                document_type_id: 1,
                created_by: 1,
                created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn search(&self, filters: doc_man_db::models::DocumentSearchFilters) -> Result<(Vec<Document>, i64), RepositoryError> {
        let documents = vec![
            Document {
                id: 1,
                title: format!("検索結果文書1 - {}", filters.title.unwrap_or_default()),
                document_type_id: 1,
                created_by: 1,
                created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
        ];
        Ok((documents, 1))
    }
}

#[async_trait]
impl DocumentNumberRuleRepository for MockDocumentNumberRuleRepository {
    async fn find_applicable_rule(
        &self,
        _document_type_code: &str,
        _department_code: &str,
        _date: NaiveDate,
    ) -> Result<Option<doc_man_db::models::DocumentNumberGenerationRule>, RepositoryError> {
        Ok(Some(doc_man_db::models::DocumentNumberGenerationRule {
            id: 1,
            rule_name: "テストルール".to_string(),
            template: "{部署コード}-{年下2桁}{連番:3桁}".to_string(),
            sequence_digits: 3,
            department_code: Some("T".to_string()),
            document_type_codes: "[\"A\",\"B\"]".to_string(),
            effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            effective_until: None,
            priority: 1,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }))
    }

    async fn get_next_sequence_number(
        &self,
        _rule_id: i32,
        _year: i32,
        _month: i32,
        _department_code: &str,
    ) -> Result<i32, RepositoryError> {
        Ok(1)
    }

    async fn is_document_number_exists(&self, _document_number: &str) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn create_rule(
        &self,
        _request: doc_man_db::models::CreateDocumentNumberGenerationRuleRequest,
    ) -> Result<doc_man_db::models::DocumentNumberGenerationRule, RepositoryError> {
        unimplemented!()
    }

    async fn get_rule_by_id(&self, _id: i32) -> Result<Option<doc_man_db::models::DocumentNumberGenerationRule>, RepositoryError> {
        unimplemented!()
    }

    async fn search_rules(
        &self,
        _department_code: Option<String>,
        _active_on_date: Option<NaiveDate>,
        _limit: i64,
        _offset: i64,
    ) -> Result<(Vec<doc_man_db::models::DocumentNumberGenerationRule>, i64), RepositoryError> {
        unimplemented!()
    }
}

#[tokio::test]
async fn test_health_handler() {
    // Given: ヘルスハンドラー
    let handler = HealthHandler::new();
    
    // When: ヘルスチェックリクエスト
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    
    let response = handler.health_check(request).await;
    
    // Then: 成功レスポンスが返される
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_document_handler() {
    // Given: DocumentHandlersとモックサービス
    let doc_repo = MockDocumentRepository;
    let rule_repo = MockDocumentNumberRuleRepository;
    let service = DocumentService::new(doc_repo, rule_repo);
    let handlers = DocumentHandlers::new(service);
    
    let request_body = CreateDocumentWithNumberRequest {
        title: "ハンドラーテスト文書".to_string(),
        document_type_code: "A".to_string(),
        department_code: "T".to_string(),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
    };
    
    // When: 文書作成ハンドラー呼び出し
    let result = handlers.create_document(request_body).await;
    
    // Then: 成功レスポンスが返される
    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.document.title, "ハンドラーテスト文書");
    assert_eq!(created.document_number, "T-25001");
}

#[tokio::test]
async fn test_get_document_handler() {
    // Given: DocumentHandlersとモックサービス
    let doc_repo = MockDocumentRepository;
    let rule_repo = MockDocumentNumberRuleRepository;
    let service = DocumentService::new(doc_repo, rule_repo);
    let handlers = DocumentHandlers::new(service);
    
    // When: 文書取得ハンドラー呼び出し（存在するID）
    let result = handlers.get_document(1).await;
    
    // Then: 文書が返される
    assert!(result.is_ok());
    let document = result.unwrap();
    assert_eq!(document.id, 1);
    assert_eq!(document.title, "テスト文書");
}

#[tokio::test]
async fn test_get_document_handler_not_found() {
    // Given: DocumentHandlersとモックサービス
    let doc_repo = MockDocumentRepository;
    let rule_repo = MockDocumentNumberRuleRepository;
    let service = DocumentService::new(doc_repo, rule_repo);
    let handlers = DocumentHandlers::new(service);
    
    // When: 文書取得ハンドラー呼び出し（存在しないID）
    let result = handlers.get_document(999).await;
    
    // Then: NotFoundエラーが返される
    assert!(result.is_err());
}

#[tokio::test]
async fn test_search_documents_handler() {
    // Given: DocumentHandlersとモックサービス
    let doc_repo = MockDocumentRepository;
    let rule_repo = MockDocumentNumberRuleRepository;
    let service = DocumentService::new(doc_repo, rule_repo);
    let handlers = DocumentHandlers::new(service);
    
    let filters = doc_man_db::models::DocumentSearchFilters {
        title: Some("テスト".to_string()),
        document_type_id: None,
        created_by: None,
        created_date_from: None,
        created_date_to: None,
        limit: 10,
        offset: 0,
    };
    
    // When: 文書検索ハンドラー呼び出し
    let result = handlers.search_documents(filters).await;
    
    // Then: 検索結果が返される
    assert!(result.is_ok());
    let (documents, total) = result.unwrap();
    assert_eq!(documents.len(), 1);
    assert_eq!(total, 1);
    assert!(documents[0].title.contains("検索結果文書"));
}

#[tokio::test]
async fn test_create_document_handler_validation_error() {
    // Given: DocumentHandlersとモックサービス
    let doc_repo = MockDocumentRepository;
    let rule_repo = MockDocumentNumberRuleRepository;
    let service = DocumentService::new(doc_repo, rule_repo);
    let handlers = DocumentHandlers::new(service);
    
    let invalid_request = CreateDocumentWithNumberRequest {
        title: "".to_string(), // 空のタイトル（バリデーションエラー）
        document_type_code: "A".to_string(),
        department_code: "T".to_string(),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
    };
    
    // When: 無効なリクエストで文書作成ハンドラー呼び出し
    let result = handlers.create_document(invalid_request).await;
    
    // Then: バリデーションエラーが返される
    assert!(result.is_err());
}