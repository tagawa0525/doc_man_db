use chrono::NaiveDate;
use doc_man_db::models::{CreateDocumentRequest, ValidationError};

#[test]
fn test_create_document_request_validation_success() {
    // Given: 有効な文書作成リクエスト
    let request = CreateDocumentRequest {
        title: "テスト文書".to_string(),
        document_type_id: 1,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
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
        title: "".to_string(),
        document_type_id: 1,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), ValidationError::EmptyTitle);
}

#[test]
fn test_create_document_request_whitespace_only_title() {
    // Given: 空白のみのタイトルを持つリクエスト
    let request = CreateDocumentRequest {
        title: "   ".to_string(),
        document_type_id: 1,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), ValidationError::EmptyTitle);
}

#[test]
fn test_create_document_request_invalid_document_type_id() {
    // Given: 無効な文書種別IDを持つリクエスト
    let request = CreateDocumentRequest {
        title: "テスト文書".to_string(),
        document_type_id: 0, // 無効なID（1未満）
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), ValidationError::InvalidDocumentTypeId);
}

#[test]
fn test_create_document_request_invalid_created_by() {
    // Given: 無効な作成者IDを持つリクエスト
    let request = CreateDocumentRequest {
        title: "テスト文書".to_string(),
        document_type_id: 1,
        created_by: 0, // 無効なID（1未満）
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
    };
    
    // When: バリデーション実行
    let result = request.validate();
    
    // Then: エラーになる
    assert!(result.is_err());
    assert_matches::assert_matches!(result.unwrap_err(), ValidationError::InvalidCreatedBy);
}