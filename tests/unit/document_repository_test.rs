// Document Repository の基本的なテスト（簡易版）

use chrono::NaiveDate;
use doc_man_db::models::document::{CreateDocumentRequest, UpdateDocumentRequest};

#[test]
fn test_create_document_request_validation() {
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

    assert_eq!(request.title, "テスト文書");
    assert_eq!(request.number, Some("TEST-001".to_string()));
    assert_eq!(request.document_type_id, 1);
}

#[test]
fn test_update_document_request_creation() {
    let update_request = UpdateDocumentRequest {
        title: Some("更新後文書".to_string()),
        document_type_id: Some(2),
    };

    assert_eq!(update_request.title, Some("更新後文書".to_string()));
    assert_eq!(update_request.document_type_id, Some(2));
}

#[test]
fn test_create_document_request_required_fields() {
    let request = CreateDocumentRequest {
        number: None,
        title: "必須フィールドテスト".to_string(),
        document_type_id: 1,
        business_number: None,
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
    };

    assert_eq!(request.title, "必須フィールドテスト");
    assert!(request.number.is_none());
    assert!(request.business_number.is_none());
}
