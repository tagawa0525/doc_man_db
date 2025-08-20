use chrono::NaiveDate;
use doc_man_db::models::{CreateDocumentRequest, DocumentValidationError};
use assert_matches::assert_matches;

#[test]
fn test_create_document_request_validation_success() {
    // Given: 有効な文書作成リクエスト
    let request = CreateDocumentRequest {
        number: None,
        title: "テスト文書".to_string(),
        document_type_id: 1,
        business_number: None,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: 成功する
    assert!(result.is_ok());
}

#[test]
fn test_create_document_request_empty_title() {
    // Given: 空のタイトルを持つリクエスト
    let request = CreateDocumentRequest {
        number: None,
        title: "".to_string(),
        document_type_id: 1,
        business_number: None,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: エラーになる
    assert!(result.is_err());
    assert_matches!(result.unwrap_err(), DocumentValidationError::EmptyTitle);
}

#[test]
fn test_create_document_request_whitespace_only_title() {
    // Given: 空白のみのタイトルを持つリクエスト
    let request = CreateDocumentRequest {
        number: None,
        title: "   ".to_string(),
        document_type_id: 1,
        business_number: None,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: エラーになる
    assert!(result.is_err());
    assert_matches!(result.unwrap_err(), DocumentValidationError::EmptyTitle);
}

#[test]
fn test_create_document_request_invalid_document_type_id() {
    // Given: 無効な文書種別IDを持つリクエスト
    let request = CreateDocumentRequest {
        number: None,
        title: "テスト文書".to_string(),
        document_type_id: 0, // 無効なID（1未満）
        business_number: None,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: エラーになる
    assert!(result.is_err());
    assert_matches!(result.unwrap_err(), DocumentValidationError::InvalidDocumentTypeId);
}

#[test]
fn test_create_document_request_invalid_created_by() {
    // Given: 無効な作成者IDを持つリクエスト
    let request = CreateDocumentRequest {
        number: None,
        title: "テスト文書".to_string(),
        document_type_id: 1,
        business_number: None,
        created_by: 0, // 無効なID（1未満）
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: エラーになる
    assert!(result.is_err());
    assert_matches!(result.unwrap_err(), DocumentValidationError::InvalidCreatedBy);
}