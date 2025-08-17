use chrono::NaiveDate;
use doc_man_db::models::{CreateDocumentTypeRequest, ValidationError};

#[test]
fn test_document_type_creation_success() {
    // Given: 有効な文書種別作成リクエスト
    let request = CreateDocumentTypeRequest {
        code: "A".to_string(),
        name: "報告書".to_string(),
        description: Some("技術報告書・調査報告書等".to_string()),
        requires_approval: true,
        department_code: Some("T".to_string()),
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: 成功する
    assert!(result.is_ok());
}

#[test]
fn test_document_type_empty_code() {
    // Given: 空のコードを持つリクエスト
    let request = CreateDocumentTypeRequest {
        code: "".to_string(),
        name: "報告書".to_string(),
        description: None,
        requires_approval: true,
        department_code: None,
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), ValidationError::EmptyDocumentTypeCode);
}

#[test]
fn test_document_type_invalid_code_length() {
    // Given: 無効な長さのコードを持つリクエスト
    let request = CreateDocumentTypeRequest {
        code: "AB".to_string(), // 2文字（1文字であるべき）
        name: "報告書".to_string(),
        description: None,
        requires_approval: true,
        department_code: None,
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(
        result.unwrap_err(),
        ValidationError::InvalidDocumentTypeCodeLength
    );
}

#[test]
fn test_document_type_empty_name() {
    // Given: 空の名前を持つリクエスト
    let request = CreateDocumentTypeRequest {
        code: "A".to_string(),
        name: "".to_string(),
        description: None,
        requires_approval: true,
        department_code: None,
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), ValidationError::EmptyDocumentTypeName);
}

#[test]
fn test_document_type_invalid_department_code() {
    // Given: 無効な部署コードを持つリクエスト
    let request = CreateDocumentTypeRequest {
        code: "A".to_string(),
        name: "報告書".to_string(),
        description: None,
        requires_approval: true,
        department_code: Some("TA".to_string()), // 2文字（1文字であるべき）
        effective_from: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        effective_until: None,
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(
        result.unwrap_err(),
        ValidationError::InvalidDepartmentCodeLength
    );
}

#[test]
fn test_document_type_effective_until_before_from() {
    // Given: 終了日が開始日より前のリクエスト
    let request = CreateDocumentTypeRequest {
        code: "A".to_string(),
        name: "報告書".to_string(),
        description: None,
        requires_approval: true,
        department_code: None,
        effective_from: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
        effective_until: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()), // 開始日より前
    };

    // When: バリデーション実行
    let result = request.validate();

    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), ValidationError::InvalidEffectivePeriod);
}
