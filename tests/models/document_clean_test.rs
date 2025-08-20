// Document Service の整理されたテスト

use chrono::{NaiveDate, Utc};
use doc_man_db::models::document::*;

#[test]
fn test_document_create_request() {
    let request = CreateDocumentRequest {
        number: Some("TEST-001".to_string()),
        title: "新規文書".to_string(),
        document_type_id: 1,
        business_number: Some("BIZ-001".to_string()),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 8, 19).unwrap(),
        internal_external: Some("internal".to_string()),
        importance_class: Some("normal".to_string()),
        personal_info: Some("none".to_string()),
        notes: Some("テスト文書です".to_string()),
    };

    assert_eq!(request.title, "新規文書");
    assert_eq!(request.document_type_id, 1);
    assert_eq!(request.created_by, 1);
    assert_eq!(request.business_number, Some("BIZ-001".to_string()));
}

#[test]
fn test_document_update_request() {
    let request = UpdateDocumentRequest {
        title: Some("更新後文書".to_string()),
        document_type_id: Some(2),
    };

    assert_eq!(request.title, Some("更新後文書".to_string()));
    assert_eq!(request.document_type_id, Some(2));
}

#[test]
fn test_document_creation() {
    let document = Document {
        id: 1,
        number: "TEST-001".to_string(),
        title: "テスト文書".to_string(),
        document_type_id: 1,
        business_number: Some("BIZ-001".to_string()),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 8, 19).unwrap(),
        internal_external: Some("internal".to_string()),
        importance_class: Some("high".to_string()),
        personal_info: Some("none".to_string()),
        notes: Some("テスト内容".to_string()),
        network_path: Some("/test/path.pdf".to_string()),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert_eq!(document.title, "テスト文書");
    assert_eq!(document.number, "TEST-001");
    assert_eq!(document.document_type_id, 1);
    assert!(document.is_active);
}

#[test]
fn test_document_validation() {
    // 有効なリクエスト
    let valid_request = CreateDocumentRequest {
        number: Some("VALID-001".to_string()),
        title: "有効な文書タイトル".to_string(),
        document_type_id: 1,
        business_number: Some("BIZ-001".to_string()),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 8, 19).unwrap(),
        internal_external: Some("internal".to_string()),
        importance_class: Some("normal".to_string()),
        personal_info: Some("none".to_string()),
        notes: Some("有効な内容".to_string()),
    };

    assert!(valid_request.validate().is_ok());

    // 無効なリクエスト（空のタイトル）
    let invalid_title_request = CreateDocumentRequest {
        number: Some("INVALID-001".to_string()),
        title: "".to_string(),
        document_type_id: 1,
        business_number: None,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 8, 19).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
    };

    assert_eq!(
        invalid_title_request.validate(),
        Err(DocumentValidationError::EmptyTitle)
    );
}
