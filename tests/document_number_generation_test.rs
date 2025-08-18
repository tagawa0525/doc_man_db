use chrono::NaiveDate;
use doc_man_db::models::{
    CreateDocumentNumberGenerationRuleRequest, DocumentNumberRequest, DocumentValidationError,
};

#[test]
fn test_document_number_rule_creation_success() {
    // Given: 有効な文書番号生成ルール作成リクエスト
    let request = CreateDocumentNumberGenerationRuleRequest {
        rule_name: "技術部標準形式2025".to_string(),
        template: "{部署コード}-{年下2桁}{連番:3桁}".to_string(),
        sequence_digits: 3,
        department_code: Some("T".to_string()),
        document_type_codes: vec!["A".to_string(), "B".to_string(), "X".to_string()],
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
        priority: 1,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: 成功する
    assert!(result.is_ok());
}

#[test]
fn test_document_number_rule_empty_name() {
    // Given: 空のルール名を持つリクエスト
    let request = CreateDocumentNumberGenerationRuleRequest {
        rule_name: "".to_string(),
        template: "{部署コード}-{年下2桁}{連番:3桁}".to_string(),
        sequence_digits: 3,
        department_code: None,
        document_type_codes: vec!["A".to_string()],
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
        priority: 1,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), DocumentValidationError::EmptyRuleName);
}

#[test]
fn test_document_number_rule_empty_template() {
    // Given: 空のテンプレートを持つリクエスト
    let request = CreateDocumentNumberGenerationRuleRequest {
        rule_name: "テストルール".to_string(),
        template: "".to_string(),
        sequence_digits: 3,
        department_code: None,
        document_type_codes: vec!["A".to_string()],
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
        priority: 1,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), DocumentValidationError::EmptyTemplate);
}

#[test]
fn test_document_number_rule_invalid_sequence_digits() {
    // Given: 無効な連番桁数を持つリクエスト
    let request = CreateDocumentNumberGenerationRuleRequest {
        rule_name: "テストルール".to_string(),
        template: "{連番:3桁}".to_string(),
        sequence_digits: 0, // 無効（1以上であるべき）
        department_code: None,
        document_type_codes: vec!["A".to_string()],
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
        priority: 1,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(
        result.unwrap_err(),
        DocumentValidationError::InvalidSequenceDigits
    );
}

#[test]
fn test_document_number_rule_empty_document_types() {
    // Given: 空の文書種別リストを持つリクエスト
    let request = CreateDocumentNumberGenerationRuleRequest {
        rule_name: "テストルール".to_string(),
        template: "{連番:3桁}".to_string(),
        sequence_digits: 3,
        department_code: None,
        document_type_codes: vec![], // 空のリスト
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
        priority: 1,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(
        result.unwrap_err(),
        DocumentValidationError::EmptyDocumentTypeCodes
    );
}

#[test]
fn test_document_number_request_validation_success() {
    // Given: 有効な文書番号リクエスト
    let request = DocumentNumberRequest {
        document_type_code: "A".to_string(),
        department_code: "T".to_string(),
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        created_by: 1,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: 成功する
    assert!(result.is_ok());
}

#[test]
fn test_document_number_request_empty_document_type() {
    // Given: 空の文書種別コードを持つリクエスト
    let request = DocumentNumberRequest {
        document_type_code: "".to_string(),
        department_code: "T".to_string(),
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        created_by: 1,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(
        result.unwrap_err(),
        DocumentValidationError::EmptyDocumentTypeCode
    );
}

#[test]
fn test_document_number_request_empty_department_code() {
    // Given: 空の部署コードを持つリクエスト
    let request = DocumentNumberRequest {
        document_type_code: "A".to_string(),
        department_code: "".to_string(),
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        created_by: 1,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(
        result.unwrap_err(),
        DocumentValidationError::EmptyDepartmentCode
    );
}

#[test]
fn test_document_number_request_invalid_created_by() {
    // Given: 無効な作成者IDを持つリクエスト
    let request = DocumentNumberRequest {
        document_type_code: "A".to_string(),
        department_code: "T".to_string(),
        created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        created_by: 0, // 無効（1以上であるべき）
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(
        result.unwrap_err(),
        DocumentValidationError::InvalidCreatedBy
    );
}
