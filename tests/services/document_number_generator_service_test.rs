use async_trait::async_trait;
use chrono::NaiveDate;
use doc_man_db::models::{
    CreateDocumentNumberGenerationRuleRequest, DocumentNumberGenerationRule, DocumentNumberRequest,
};
use doc_man_db::repositories::{DocumentNumberRuleRepository, RepositoryError};
use doc_man_db::services::DocumentNumberGenerator;

// テスト用の簡易リポジトリ実装
struct TestDocumentNumberRuleRepository {
    rule: Option<DocumentNumberGenerationRule>,
    next_sequence: i32,
    existing_numbers: Vec<String>,
}

impl TestDocumentNumberRuleRepository {
    fn new() -> Self {
        Self {
            rule: None,
            next_sequence: 1,
            existing_numbers: vec![],
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

    #[allow(dead_code)]
    fn with_existing_numbers(mut self, numbers: Vec<String>) -> Self {
        self.existing_numbers = numbers;
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
        document_number: &str,
    ) -> Result<bool, RepositoryError> {
        Ok(self.existing_numbers.contains(&document_number.to_string()))
    }

    async fn create_rule(
        &self,
        _request: CreateDocumentNumberGenerationRuleRequest,
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
async fn test_generate_document_number_success() {
    // Given: テストリポジトリとジェネレーター
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

    let repo = TestDocumentNumberRuleRepository::new()
        .with_rule(rule)
        .with_sequence(1);

    let generator = DocumentNumberGenerator::new(repo);

    let request = DocumentNumberRequest {
        document_type_code: "A".to_string(),
        department_code: "T".to_string(),
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        created_by: 1,
    };

    // When: 文書番号生成
    let result = generator.generate_document_number(request).await;

    // Then: 成功し、期待される番号が生成される
    assert!(result.is_ok());
    let generated = result.unwrap();
    assert_eq!(generated.document_number, "T-25001");
    assert_eq!(generated.rule_id, 1);
    assert_eq!(generated.sequence_number, 1);
}

#[tokio::test]
async fn test_generate_document_number_no_applicable_rule() {
    // Given: 適用可能なルールが見つからないテストリポジトリ
    let repo = TestDocumentNumberRuleRepository::new(); // ルールなし

    let generator = DocumentNumberGenerator::new(repo);

    let request = DocumentNumberRequest {
        document_type_code: "Z".to_string(), // 存在しない文書種別
        department_code: "X".to_string(),    // 存在しない部署
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        created_by: 1,
    };

    // When: 文書番号生成
    let result = generator.generate_document_number(request).await;

    // Then: NoApplicableRule エラーが返される
    assert!(result.is_err());
}

#[tokio::test]
async fn test_generate_historical_format_cta() {
    // Given: CTA形式のルール
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

    let repo = TestDocumentNumberRuleRepository::new()
        .with_rule(rule)
        .with_sequence(8);

    let generator = DocumentNumberGenerator::new(repo);

    let request = DocumentNumberRequest {
        document_type_code: "A".to_string(),
        department_code: "C".to_string(),
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        created_by: 1,
    };

    // When: 文書番号生成
    let result = generator.generate_document_number(request).await;

    // Then: CTA形式の番号が生成される
    assert!(result.is_ok());
    let generated = result.unwrap();
    assert_eq!(generated.document_number, "CTA-2508008");
    assert_eq!(generated.rule_id, 2);
    assert_eq!(generated.sequence_number, 8);
}
