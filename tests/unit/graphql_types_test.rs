use chrono::{NaiveDate, Utc};
use doc_man_db::graphql::types::{
    CreateDocumentInput, CreatedDocumentWithNumber, Document, DocumentSearchFilters,
    GeneratedDocumentNumber, SearchDocumentsResult,
};
use doc_man_db::models;

#[test]
fn test_document_from_model_conversion() {
    let now = Utc::now();
    let model_document = models::Document {
        id: 1,
        number: "T-25001".to_string(),
        title: "Test Document".to_string(),
        document_type_id: 1,
        business_number: Some("BIZ-001".to_string()),
        created_by: 123,
        created_by_name: Some("テストユーザー".to_string()),
        created_date: NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(),
        internal_external: Some("内部".to_string()),
        importance_class: Some("重要".to_string()),
        personal_info: Some("個人情報あり".to_string()),
        notes: Some("テスト用ドキュメント".to_string()),
        network_path: Some("\\\\server\\docs\\T-25001.pdf".to_string()),
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    let graphql_document: Document = model_document.into();

    assert_eq!(graphql_document.id, 1);
    assert_eq!(graphql_document.title, "Test Document");
    assert_eq!(graphql_document.document_type_id, 1);
    assert_eq!(graphql_document.created_by, 123);
    assert_eq!(graphql_document.created_date, "2025-08-20");
    assert!(graphql_document.created_at.contains("T"));
    assert!(graphql_document.updated_at.contains("T"));
}

#[test]
fn test_document_date_formatting() {
    let now = Utc::now();
    let model_document = models::Document {
        id: 2,
        number: "DEV-25001".to_string(),
        title: "Development Document".to_string(),
        document_type_id: 2,
        business_number: None,
        created_by: 456,
        created_by_name: Some("テストユーザー".to_string()),
        created_date: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        internal_external: None,
        importance_class: None,
        personal_info: None,
        notes: None,
        network_path: None,
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    let graphql_document: Document = model_document.into();

    // 日付フォーマットが正しいことを確認
    assert_eq!(graphql_document.created_date, "2025-12-31");
    // ISO 8601形式のタイムスタンプを確認
    assert!(graphql_document.created_at.len() >= 19); // YYYY-MM-DDTHH:MM:SS
    assert!(graphql_document.updated_at.len() >= 19);
}

#[test]
fn test_create_document_input_conversion() {
    let input = CreateDocumentInput {
        title: "New Document".to_string(),
        document_type_code: "TECH".to_string(),
        department_code: "DEV".to_string(),
        created_by: 789,
        created_date: "2025-08-20".to_string(),
    };

    let request: models::CreateDocumentWithNumberRequest = input.into();

    assert_eq!(request.title, "New Document");
    assert_eq!(request.document_type_code, "TECH");
    assert_eq!(request.department_code, "DEV");
    assert_eq!(request.created_by, 789);
    assert_eq!(
        request.created_date,
        NaiveDate::from_ymd_opt(2025, 8, 20).unwrap()
    );
}

#[test]
#[should_panic(expected = "Invalid date format")]
fn test_create_document_input_invalid_date() {
    let input = CreateDocumentInput {
        title: "New Document".to_string(),
        document_type_code: "TECH".to_string(),
        department_code: "DEV".to_string(),
        created_by: 789,
        created_date: "invalid-date".to_string(),
    };

    let _: models::CreateDocumentWithNumberRequest = input.into();
}

#[test]
fn test_document_search_filters_conversion() {
    let graphql_filters = DocumentSearchFilters {
        title: Some("Test".to_string()),
        document_type_id: Some(1),
        created_by: Some(123),
        created_date_from: Some("2025-01-01".to_string()),
        created_date_to: Some("2025-12-31".to_string()),
        limit: Some(50),
        offset: Some(0),
    };

    let model_filters: models::DocumentSearchFilters = graphql_filters.into();

    assert_eq!(model_filters.title, Some("Test".to_string()));
    assert_eq!(model_filters.document_type_id, Some(1));
    assert_eq!(model_filters.created_by, Some(123));
    assert_eq!(
        model_filters.created_date_from,
        Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
    );
    assert_eq!(
        model_filters.created_date_to,
        Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
    );
    assert_eq!(model_filters.limit, 50);
    assert_eq!(model_filters.offset, 0);
}

#[test]
fn test_document_search_filters_minimal() {
    let graphql_filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: None,
        created_date_to: None,
        limit: Some(10),
        offset: Some(0),
    };

    let model_filters: models::DocumentSearchFilters = graphql_filters.into();

    assert!(model_filters.title.is_none());
    assert!(model_filters.document_type_id.is_none());
    assert!(model_filters.created_by.is_none());
    assert!(model_filters.created_date_from.is_none());
    assert!(model_filters.created_date_to.is_none());
    assert_eq!(model_filters.limit, 10);
    assert_eq!(model_filters.offset, 0);
}

#[test]
fn test_document_search_filters_invalid_date_from() {
    let graphql_filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: Some("invalid-date".to_string()),
        created_date_to: None,
        limit: Some(10),
        offset: Some(0),
    };

    let model_filters: models::DocumentSearchFilters = graphql_filters.into();
    // 無効な日付はNoneに変換される
    assert!(model_filters.created_date_from.is_none());
}

#[test]
fn test_document_search_filters_invalid_date_to() {
    let graphql_filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: None,
        created_date_to: Some("invalid-date".to_string()),
        limit: Some(10),
        offset: Some(0),
    };

    let model_filters: models::DocumentSearchFilters = graphql_filters.into();
    // 無効な日付はNoneに変換される
    assert!(model_filters.created_date_to.is_none());
}

#[test]
fn test_generated_document_number_conversion() {
    let model_number = models::GeneratedDocumentNumber {
        document_number: "T-25042".to_string(),
        rule_id: 2,
        sequence_number: 42,
        template_used: "{部署コード}-{年下2桁}{連番:3桁}".to_string(),
    };

    let graphql_number: GeneratedDocumentNumber = model_number.into();

    // GeneratedDocumentNumber にはdocument_numberフィールドがないため、このアサーションを削除
    assert_eq!(graphql_number.rule_id, 2);
    assert_eq!(graphql_number.sequence_number, 42);
    assert_eq!(
        graphql_number.template_used,
        "{部署コード}-{年下2桁}{連番:3桁}"
    );
}

#[test]
fn test_created_document_with_number_conversion() {
    let now = Utc::now();
    let model_created = models::CreatedDocumentWithNumber {
        document: models::Document {
            id: 1,
            number: "T-25001".to_string(),
            title: "Test Document".to_string(),
            document_type_id: 1,
            business_number: None,
            created_by: 123,
            created_by_name: Some("テストユーザー".to_string()),
            created_date: NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(),
            internal_external: None,
            importance_class: None,
            personal_info: None,
            notes: None,
            network_path: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        },
        document_number: "T-25001".to_string(),
        generated_number: models::GeneratedDocumentNumber {
            document_number: "T-25001".to_string(),
            rule_id: 1,
            sequence_number: 1,
            template_used: "{部署コード}-{年下2桁}{連番:3桁}".to_string(),
        },
    };

    let graphql_created: CreatedDocumentWithNumber = model_created.into();

    assert_eq!(graphql_created.document.id, 1);
    assert_eq!(graphql_created.document.title, "Test Document");
    assert_eq!(graphql_created.document_number, "T-25001");
    // GeneratedDocumentNumber にはdocument_numberフィールドがないため、このアサーションを削除
    assert_eq!(graphql_created.generated_number.rule_id, 1);
    assert_eq!(graphql_created.generated_number.sequence_number, 1);
}

#[test]
fn test_search_documents_result_creation() {
    let now = Utc::now();
    let documents = vec![
        models::Document {
            id: 1,
            number: "T-25001".to_string(),
            title: "First Document".to_string(),
            document_type_id: 1,
            business_number: None,
            created_by: 123,
            created_by_name: Some("テストユーザー1".to_string()),
            created_date: NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(),
            internal_external: None,
            importance_class: None,
            personal_info: None,
            notes: None,
            network_path: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        },
        models::Document {
            id: 2,
            number: "T-25002".to_string(),
            title: "Second Document".to_string(),
            document_type_id: 1,
            business_number: None,
            created_by: 456,
            created_by_name: Some("テストユーザー2".to_string()),
            created_date: NaiveDate::from_ymd_opt(2025, 8, 21).unwrap(),
            internal_external: None,
            importance_class: None,
            personal_info: None,
            notes: None,
            network_path: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        },
    ];

    let result = SearchDocumentsResult {
        documents: documents.into_iter().map(|d| d.into()).collect(),
        total: 2,
    };

    assert_eq!(result.documents.len(), 2);
    assert_eq!(result.total, 2);
    assert_eq!(result.documents[0].id, 1);
    assert_eq!(result.documents[0].title, "First Document");
    assert_eq!(result.documents[1].id, 2);
    assert_eq!(result.documents[1].title, "Second Document");
}

#[test]
fn test_search_documents_result_empty() {
    let result = SearchDocumentsResult {
        documents: vec![],
        total: 0,
    };

    assert_eq!(result.documents.len(), 0);
    assert_eq!(result.total, 0);
}

#[test]
fn test_create_document_input_validation_scenarios() {
    // 有効なデータでの変換
    let valid_input = CreateDocumentInput {
        title: "Valid Document".to_string(),
        document_type_code: "VALID".to_string(),
        department_code: "DEV".to_string(),
        created_by: 123,
        created_date: "2025-08-20".to_string(),
    };

    let request: models::CreateDocumentWithNumberRequest = valid_input.into();
    assert!(!request.title.is_empty());
    assert!(!request.document_type_code.is_empty());
    assert!(!request.department_code.is_empty());
    assert!(request.created_by > 0);

    // 空のタイトルも構造的には有効（ビジネスロジックで検証）
    let empty_title_input = CreateDocumentInput {
        title: "".to_string(),
        document_type_code: "TEST".to_string(),
        department_code: "DEV".to_string(),
        created_by: 123,
        created_date: "2025-08-20".to_string(),
    };

    let empty_request: models::CreateDocumentWithNumberRequest = empty_title_input.into();
    assert!(empty_request.title.is_empty());
}

#[test]
fn test_document_search_filters_date_edge_cases() {
    // 境界値テスト: 年始と年末
    let filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: Some("2025-01-01".to_string()),
        created_date_to: Some("2025-12-31".to_string()),
        limit: Some(100),
        offset: Some(0),
    };

    let model_filters: models::DocumentSearchFilters = filters.into();
    assert_eq!(
        model_filters.created_date_from,
        Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
    );
    assert_eq!(
        model_filters.created_date_to,
        Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
    );

    // うるう年のテスト
    let leap_year_filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: Some("2024-02-29".to_string()),
        created_date_to: Some("2024-02-29".to_string()),
        limit: Some(10),
        offset: Some(0),
    };

    let leap_model_filters: models::DocumentSearchFilters = leap_year_filters.into();
    assert_eq!(
        leap_model_filters.created_date_from,
        Some(NaiveDate::from_ymd_opt(2024, 2, 29).unwrap())
    );
    assert_eq!(
        leap_model_filters.created_date_to,
        Some(NaiveDate::from_ymd_opt(2024, 2, 29).unwrap())
    );
}

#[test]
fn test_document_search_filters_limit_offset_boundaries() {
    // 最大制限値のテスト
    let max_filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: None,
        created_date_to: None,
        limit: Some(1000),
        offset: Some(9999),
    };

    let model_filters: models::DocumentSearchFilters = max_filters.into();
    assert_eq!(model_filters.limit, 1000);
    assert_eq!(model_filters.offset, 9999);

    // 最小値のテスト
    let min_filters = DocumentSearchFilters {
        title: None,
        document_type_id: None,
        created_by: None,
        created_date_from: None,
        created_date_to: None,
        limit: Some(1),
        offset: Some(0),
    };

    let min_model_filters: models::DocumentSearchFilters = min_filters.into();
    assert_eq!(min_model_filters.limit, 1);
    assert_eq!(min_model_filters.offset, 0);
}

#[test]
fn test_document_conversion_with_long_strings() {
    let now = Utc::now();
    let long_title = "あ".repeat(500); // 長いタイトル
    let model_document = models::Document {
        id: 999,
        number: "LONG-25001".to_string(),
        title: long_title.clone(),
        document_type_id: 99,
        business_number: Some("VERY-LONG-BUSINESS-NUMBER-12345".to_string()),
        created_by: 999999,
        created_by_name: Some("テストユーザー".to_string()),
        created_date: NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(),
        internal_external: Some("非常に長い内部外部分類情報".to_string()),
        importance_class: Some("最重要機密".to_string()),
        personal_info: Some("大量の個人情報を含む文書".to_string()),
        notes: Some("非常に詳細な注釈情報".to_string()),
        network_path: Some(
            "\\\\very-long-server-name\\very\\long\\path\\to\\document.pdf".to_string(),
        ),
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    let graphql_document: Document = model_document.into();

    assert_eq!(graphql_document.id, 999);
    assert_eq!(graphql_document.title, long_title);
    assert_eq!(graphql_document.document_type_id, 99);
    assert_eq!(graphql_document.created_by, 999999);
}

#[test]
fn test_generated_document_number_template_variations() {
    // 異なるテンプレートパターンのテスト
    let templates = [
        "{部署コード}-{年下2桁}{連番:3桁}",
        "{年4桁}-{部署コード}-{連番:5桁}",
        "CTA-{年下2桁}{月:2桁}{連番:3桁}",
        "{部署コード}{年下2桁}{月:2桁}{日:2桁}-{連番:4桁}",
    ];

    for (i, template) in templates.iter().enumerate() {
        let model_number = models::GeneratedDocumentNumber {
            document_number: format!("TEST-{:04}", i + 1),
            rule_id: i as i32 + 1,
            sequence_number: i as i32 + 100,
            template_used: template.to_string(),
        };

        let graphql_number: GeneratedDocumentNumber = model_number.into();

        // GeneratedDocumentNumber にはdocument_numberフィールドがないため、このアサーションを削除
        assert_eq!(graphql_number.rule_id, i as i32 + 1);
        assert_eq!(graphql_number.sequence_number, i as i32 + 100);
        assert_eq!(graphql_number.template_used, *template);
    }
}
