use async_trait::async_trait;
use chrono::NaiveDate;
use doc_man_db::models::{CreateDocumentRequest, DocumentNumberGenerationRule};
use doc_man_db::repositories::{DocumentNumberRuleRepository, DocumentRepository, RepositoryError};
use doc_man_db::services::DocumentService;

// テスト用の簡易DocumentRepository実装
struct TestDocumentRepository {
    next_id: i32,
}

impl TestDocumentRepository {
    fn new() -> Self {
        Self { next_id: 1 }
    }
}

#[async_trait]
impl DocumentRepository for TestDocumentRepository {
    async fn create(
        &self,
        request: CreateDocumentRequest,
    ) -> Result<doc_man_db::models::Document, RepositoryError> {
        Ok(doc_man_db::models::Document {
            id: self.next_id,
            title: request.title,
            document_type_id: request.document_type_id,
            created_by: request.created_by,
            created_date: request.created_date,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
    }

    async fn get_by_id(
        &self,
        _id: i32,
    ) -> Result<Option<doc_man_db::models::Document>, RepositoryError> {
        unimplemented!()
    }

    async fn search(
        &self,
        _filters: doc_man_db::models::DocumentSearchFilters,
    ) -> Result<(Vec<doc_man_db::models::Document>, i64), RepositoryError> {
        unimplemented!()
    }
}

// テスト用の簡易DocumentNumberRuleRepository実装
struct TestDocumentNumberRuleRepository {
    rule: Option<DocumentNumberGenerationRule>,
    next_sequence: i32,
}

impl TestDocumentNumberRuleRepository {
    fn new() -> Self {
        Self {
            rule: None,
            next_sequence: 1,
        }
    }

    fn with_rule(mut self, rule: DocumentNumberGenerationRule) -> Self {
        self.rule = Some(rule);
        self
    }

    fn with_sequence(mut self, sequence: i32) -> Self {
        self.next_sequence = sequence;
        self
    }
}

#[async_trait]
impl DocumentNumberRuleRepository for TestDocumentNumberRuleRepository {
    async fn find_applicable_rule(
        &self,
        _document_type_code: &str,
        _department_code: &str,
        _date: NaiveDate,
    ) -> Result<Option<DocumentNumberGenerationRule>, RepositoryError> {
        Ok(self.rule.clone())
    }

    async fn get_next_sequence_number(
        &self,
        _rule_id: i32,
        _year: i32,
        _month: i32,
        _department_code: &str,
    ) -> Result<i32, RepositoryError> {
        Ok(self.next_sequence)
    }

    async fn is_document_number_exists(
        &self,
        _document_number: &str,
    ) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn create_rule(
        &self,
        _request: doc_man_db::models::CreateDocumentNumberGenerationRuleRequest,
    ) -> Result<DocumentNumberGenerationRule, RepositoryError> {
        unimplemented!()
    }

    async fn get_rule_by_id(
        &self,
        _id: i32,
    ) -> Result<Option<DocumentNumberGenerationRule>, RepositoryError> {
        unimplemented!()
    }

    async fn search_rules(
        &self,
        _department_code: Option<String>,
        _active_on_date: Option<NaiveDate>,
        _limit: i64,
        _offset: i64,
    ) -> Result<(Vec<DocumentNumberGenerationRule>, i64), RepositoryError> {
        unimplemented!()
    }
}

#[tokio::test]
async fn test_create_document_with_auto_number_generation() {
    // Given: DocumentServiceとテストデータ
    let rule = DocumentNumberGenerationRule {
        id: 1,
        rule_name: "技術部標準形式2025".to_string(),
        template: "{部署コード}-{年下2桁}{連番:3桁}".to_string(),
        sequence_digits: 3,
        department_code: Some("T".to_string()),
        document_type_codes: "[\"A\",\"B\",\"X\"]".to_string(),
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
        priority: 1,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let doc_repo = TestDocumentRepository::new();
    let rule_repo = TestDocumentNumberRuleRepository::new()
        .with_rule(rule)
        .with_sequence(1);

    let service = DocumentService::new(doc_repo, rule_repo);

    let request = doc_man_db::models::CreateDocumentWithNumberRequest {
        title: "技術報告書 - システム設計".to_string(),
        document_type_code: "A".to_string(),
        department_code: "T".to_string(),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
    };

    // When: 文書作成
    let result = service.create_document_with_number(request).await;

    // Then: 成功し、文書番号が自動生成される
    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.document.title, "技術報告書 - システム設計");
    assert_eq!(created.document_number, "T-25001");
    assert_eq!(created.generated_number.rule_id, 1);
    assert_eq!(created.generated_number.sequence_number, 1);
}

#[tokio::test]
async fn test_create_document_no_applicable_rule() {
    // Given: 適用可能なルールがないDocumentService
    let doc_repo = TestDocumentRepository::new();
    let rule_repo = TestDocumentNumberRuleRepository::new(); // ルールなし

    let service = DocumentService::new(doc_repo, rule_repo);

    let request = doc_man_db::models::CreateDocumentWithNumberRequest {
        title: "未対応文書種別".to_string(),
        document_type_code: "Z".to_string(), // 存在しない文書種別
        department_code: "X".to_string(),    // 存在しない部署
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
    };

    // When: 文書作成
    let result = service.create_document_with_number(request).await;

    // Then: NoApplicableRule エラーが返される
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_document_cta_format() {
    // Given: CTA形式のルールを持つDocumentService
    let rule = DocumentNumberGenerationRule {
        id: 2,
        rule_name: "CTA部2025形式".to_string(),
        template: "CTA-{年下2桁}{月:2桁}{連番:3桁}".to_string(),
        sequence_digits: 3,
        department_code: Some("C".to_string()),
        document_type_codes: "[\"A\",\"B\"]".to_string(),
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
        priority: 1,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let doc_repo = TestDocumentRepository::new();
    let rule_repo = TestDocumentNumberRuleRepository::new()
        .with_rule(rule)
        .with_sequence(15);

    let service = DocumentService::new(doc_repo, rule_repo);

    let request = doc_man_db::models::CreateDocumentWithNumberRequest {
        title: "CTA顧客提出文書".to_string(),
        document_type_code: "B".to_string(),
        department_code: "C".to_string(),
        created_by: 2,
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
    };

    // When: 文書作成
    let result = service.create_document_with_number(request).await;

    // Then: CTA形式の文書番号で作成される
    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.document.title, "CTA顧客提出文書");
    assert_eq!(created.document_number, "CTA-2508015");
    assert_eq!(created.generated_number.rule_id, 2);
    assert_eq!(created.generated_number.sequence_number, 15);
}

#[tokio::test]
async fn test_create_document_validation_error() {
    // Given: DocumentService
    let doc_repo = TestDocumentRepository::new();
    let rule_repo = TestDocumentNumberRuleRepository::new();

    let service = DocumentService::new(doc_repo, rule_repo);

    let request = doc_man_db::models::CreateDocumentWithNumberRequest {
        title: "".to_string(), // 空のタイトル（バリデーションエラー）
        document_type_code: "A".to_string(),
        department_code: "T".to_string(),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
    };

    // When: 文書作成
    let result = service.create_document_with_number(request).await;

    // Then: バリデーションエラーが返される
    assert!(result.is_err());
}
