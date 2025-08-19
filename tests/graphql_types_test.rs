// GraphQL タイプの変換テスト

use chrono::{NaiveDate, Utc};
use doc_man_db::graphql::types::*;
use doc_man_db::models::{CreateDocumentWithNumberRequest, Document as ModelDocument};

#[test]
fn test_document_conversion() {
    let model_document = ModelDocument {
        id: 1,
        number: "TEST-001".to_string(),
        title: "テスト文書".to_string(),
        document_type_id: 1,
        business_number: Some("BIZ-001".to_string()),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 8, 19).unwrap(),
        internal_external: Some("internal".to_string()),
        importance_class: Some("normal".to_string()),
        personal_info: Some("none".to_string()),
        notes: Some("テストノート".to_string()),
        network_path: Some("/test/path.pdf".to_string()),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let graphql_document: doc_man_db::graphql::types::Document = model_document.into();

    assert_eq!(graphql_document.id, 1);
    assert_eq!(graphql_document.title, "テスト文書");
    assert_eq!(graphql_document.document_type_id, 1);
    assert_eq!(graphql_document.created_by, 1);
    assert_eq!(graphql_document.created_date, "2024-08-19");
    assert!(graphql_document.created_at.contains("T"));
    assert!(graphql_document.updated_at.contains("T"));
}

#[test]
fn test_create_document_input_conversion() {
    let input = CreateDocumentInput {
        title: "新規文書".to_string(),
        document_type_code: "技術".to_string(),
        department_code: "DEV".to_string(),
        created_by: 1,
        created_date: "2024-08-19".to_string(),
    };

    let request: CreateDocumentWithNumberRequest = input.into();

    assert_eq!(request.title, "新規文書");
    assert_eq!(request.document_type_code, "技術");
    assert_eq!(request.department_code, "DEV");
    assert_eq!(request.created_by, 1);
    assert_eq!(
        request.created_date,
        NaiveDate::from_ymd_opt(2024, 8, 19).unwrap()
    );
}

#[test]
fn test_document_search_filters_conversion() {
    let filters = DocumentSearchFilters {
        title: Some("テスト".to_string()),
        document_type_id: Some(1),
        created_by: Some(1),
        created_date_from: Some("2024-01-01".to_string()),
        created_date_to: Some("2024-12-31".to_string()),
        limit: Some(20),
        offset: Some(10),
    };

    let model_filters: doc_man_db::models::DocumentSearchFilters = filters.into();

    assert_eq!(model_filters.title, Some("テスト".to_string()));
    assert_eq!(model_filters.document_type_id, Some(1));
    assert_eq!(model_filters.created_by, Some(1));
    assert_eq!(
        model_filters.created_date_from,
        Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
    );
    assert_eq!(
        model_filters.created_date_to,
        Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap())
    );
    assert_eq!(model_filters.limit, 20);
    assert_eq!(model_filters.offset, 10);
}

#[test]
fn test_document_search_filters_defaults() {
    let filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: None,
        created_date_to: None,
        limit: None,
        offset: None,
    };

    let model_filters: doc_man_db::models::DocumentSearchFilters = filters.into();

    assert_eq!(model_filters.title, None);
    assert_eq!(model_filters.document_type_id, None);
    assert_eq!(model_filters.created_by, None);
    assert_eq!(model_filters.created_date_from, None);
    assert_eq!(model_filters.created_date_to, None);
    assert_eq!(model_filters.limit, 10); // デフォルト値
    assert_eq!(model_filters.offset, 0); // デフォルト値
}

#[test]
fn test_generated_document_number_conversion() {
    let model_generated = GeneratedDocumentNumber {
        rule_id: 1,
        sequence_number: 25001,
        template_used: "技術-{YY}{SEQUENCE:5}".to_string(),
    };

    let graphql_generated: doc_man_db::graphql::types::GeneratedDocumentNumber =
        model_generated.into();

    assert_eq!(graphql_generated.rule_id, 1);
    assert_eq!(graphql_generated.sequence_number, 25001);
    assert_eq!(graphql_generated.template_used, "技術-{YY}{SEQUENCE:5}");
}

#[test]
fn test_created_document_with_number_conversion() {
    let model_document = ModelDocument {
        id: 1,
        number: "技術-25001".to_string(),
        title: "技術文書".to_string(),
        document_type_id: 1,
        business_number: Some("BIZ-001".to_string()),
        created_by: 1,
        created_date: NaiveDate::from_ymd_opt(2024, 8, 19).unwrap(),
        internal_external: Some("internal".to_string()),
        importance_class: Some("normal".to_string()),
        personal_info: Some("none".to_string()),
        notes: Some("技術文書のノート".to_string()),
        network_path: Some("/tech/doc.pdf".to_string()),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let model_generated = GeneratedDocumentNumber {
        rule_id: 1,
        sequence_number: 25001,
        template_used: "技術-{YY}{SEQUENCE:5}".to_string(),
    };

    let model_created = CreatedDocumentWithNumber {
        document: model_document.into(),
        document_number: "技術-25001".to_string(),
        generated_number: model_generated,
    };

    let graphql_created: doc_man_db::graphql::types::CreatedDocumentWithNumber =
        model_created.into();

    assert_eq!(graphql_created.document.id, 1);
    assert_eq!(graphql_created.document.title, "技術文書");
    assert_eq!(graphql_created.document_number, "技術-25001");
    assert_eq!(graphql_created.generated_number.rule_id, 1);
    assert_eq!(graphql_created.generated_number.sequence_number, 25001);
}

#[test]
fn test_search_documents_result_creation() {
    let model_documents = vec![
        ModelDocument {
            id: 1,
            number: "TEST-001".to_string(),
            title: "文書1".to_string(),
            document_type_id: 1,
            business_number: None,
            created_by: 1,
            created_date: NaiveDate::from_ymd_opt(2024, 8, 19).unwrap(),
            internal_external: Some("internal".to_string()),
            importance_class: Some("normal".to_string()),
            personal_info: Some("none".to_string()),
            notes: None,
            network_path: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        ModelDocument {
            id: 2,
            number: "TEST-002".to_string(),
            title: "文書2".to_string(),
            document_type_id: 2,
            business_number: None,
            created_by: 2,
            created_date: NaiveDate::from_ymd_opt(2024, 8, 19).unwrap(),
            internal_external: Some("internal".to_string()),
            importance_class: Some("normal".to_string()),
            personal_info: Some("none".to_string()),
            notes: None,
            network_path: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ];

    let graphql_documents: Vec<doc_man_db::graphql::types::Document> =
        model_documents.into_iter().map(|doc| doc.into()).collect();

    let result = SearchDocumentsResult {
        documents: graphql_documents,
        total: 2,
    };

    assert_eq!(result.documents.len(), 2);
    assert_eq!(result.total, 2);
    assert_eq!(result.documents[0].title, "文書1");
    assert_eq!(result.documents[1].title, "文書2");
}

#[test]
fn test_invalid_date_format_handling() {
    let input = CreateDocumentInput {
        title: "テスト文書".to_string(),
        document_type_code: "技術".to_string(),
        department_code: "DEV".to_string(),
        created_by: 1,
        created_date: "invalid-date".to_string(),
    };

    // 無効な日付形式の場合パニックするかテスト
    let result = std::panic::catch_unwind(|| {
        let _: CreateDocumentWithNumberRequest = input.into();
    });

    assert!(result.is_err());
}

#[test]
fn test_document_search_filters_with_invalid_dates() {
    let filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: Some("invalid-date".to_string()),
        created_date_to: Some("2024-12-31".to_string()),
        limit: None,
        offset: None,
    };

    let model_filters: doc_man_db::models::DocumentSearchFilters = filters.into();

    // 無効な日付は None になる
    assert_eq!(model_filters.created_date_from, None);
    assert_eq!(
        model_filters.created_date_to,
        Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap())
    );
}
