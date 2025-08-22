use super::test_environment::*;
use chrono::NaiveDate;

#[tokio::test]
async fn test_complete_document_lifecycle() {
    let test_env = TestEnvironment::new().await;

    // Setup test data
    test_env.setup_test_data().await.unwrap();

    // 1. ユーザー認証
    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();
    let user_token = test_env
        .authenticate_user("user1", "password")
        .await
        .unwrap();

    // 2. 文書作成
    let document = test_env
        .create_document(
            CreateDocumentRequest {
                title: "統合テスト文書".to_string(),
                document_type_id: 1,
                created_by: 1,
                created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                confidentiality: TestConfidentiality {
                    internal_external: "Internal".to_string(),
                    importance_class: "Class2".to_string(),
                    personal_info: "None".to_string(),
                },
                notes: Some("統合テスト用文書".to_string()),
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert_eq!(document.title, "統合テスト文書");
    assert_eq!(document.document_type_id, 1);
    assert!(document.is_active);

    // 3. ファイル存在確認
    let file_check_result = test_env.check_file_existence(&document).await.unwrap();
    assert!(!file_check_result.folder_exists); // 新規作成なので存在しない

    // 4. 回覧開始
    let circulation = test_env
        .start_circulation(
            StartCirculationRequest {
                document_id: document.id,
                workflow_id: 1,
                notes: Some("統合テスト用回覧".to_string()),
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert_eq!(circulation.document_id, document.id);
    assert_eq!(circulation.workflow_id, 1);
    assert_eq!(circulation.status, "active");

    // 5. 承認処理
    let approval_result = test_env
        .approve_circulation_step(
            ApproveStepRequest {
                circulation_id: circulation.id,
                step_id: circulation.current_step_id,
                action: "approve".to_string(),
                comments: Some("承認します".to_string()),
            },
            &user_token,
        )
        .await
        .unwrap();

    assert!(approval_result.success);
    assert_eq!(approval_result.message, "Approval processed successfully");

    // 6. 検索確認
    let search_results = test_env
        .search_documents(
            DocumentSearchInput {
                title: Some("統合テスト".to_string()),
                pagination: Pagination {
                    offset: 0,
                    limit: 10,
                },
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert!(search_results.documents.len() >= 1);
    assert_eq!(search_results.documents[0].id, document.id);
    assert_eq!(search_results.documents[0].title, "統合テスト文書");

    // 7. 更新確認
    let updated_document = test_env
        .update_document(
            document.id,
            UpdateDocumentRequest {
                title: Some("更新された統合テスト文書".to_string()),
                notes: Some("統合テストで更新".to_string()),
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert_eq!(updated_document.title, "更新された統合テスト文書");
    assert_eq!(updated_document.notes, Some("統合テストで更新".to_string()));
}

#[tokio::test]
async fn test_permission_enforcement() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    // 制限ユーザーでのテスト
    let limited_token = test_env
        .authenticate_user("limited_user", "password")
        .await
        .unwrap();
    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();

    // 管理者機能へのアクセス試行（失敗すべき）
    let admin_result = test_env.get_all_employees(&limited_token).await;
    assert!(admin_result.is_err());

    // 管理者での同じ機能アクセス（成功すべき）
    let admin_success = test_env.get_all_employees(&admin_token).await;
    assert!(admin_success.is_ok());
    let employees = admin_success.unwrap();
    assert!(employees.len() >= 2);

    // 機密文書へのアクセス試行
    let confidential_doc = test_env.create_confidential_document().await;

    // 制限ユーザーでのアクセス（失敗すべき）
    let limited_access_result = test_env
        .get_document(confidential_doc.id, &limited_token)
        .await
        .unwrap();
    assert!(limited_access_result.is_none());

    // 管理者でのアクセス（成功すべき）
    let admin_access_result = test_env
        .get_document(confidential_doc.id, &admin_token)
        .await
        .unwrap();
    assert!(admin_access_result.is_some());
    assert_eq!(admin_access_result.unwrap().title, "機密文書");
}

#[tokio::test]
async fn test_circulation_workflow_complete() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();
    let user_token = test_env
        .authenticate_user("user1", "password")
        .await
        .unwrap();

    // 1. 文書作成
    let document = test_env
        .create_document(
            CreateDocumentRequest {
                title: "回覧テスト文書".to_string(),
                document_type_id: 1,
                created_by: 1,
                created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                confidentiality: TestConfidentiality::default(),
                notes: Some("回覧フローテスト".to_string()),
            },
            &admin_token,
        )
        .await
        .unwrap();

    // 2. 回覧開始
    let circulation = test_env
        .start_circulation(
            StartCirculationRequest {
                document_id: document.id,
                workflow_id: 1,
                notes: Some("承認をお願いします".to_string()),
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert_eq!(circulation.status, "active");

    // 3. 承認者による承認
    let approval_result = test_env
        .approve_circulation_step(
            ApproveStepRequest {
                circulation_id: circulation.id,
                step_id: circulation.current_step_id,
                action: "approve".to_string(),
                comments: Some("内容を確認し、承認いたします".to_string()),
            },
            &user_token,
        )
        .await
        .unwrap();

    assert!(approval_result.success);

    // 4. 回覧状態確認
    // 実際の実装では、回覧ステップの完了後に次のステップに進むか、
    // 全ステップ完了の場合は回覧が完了状態になる
    // ここではステップが正常に完了したことを確認
    assert_eq!(approval_result.message, "Approval processed successfully");
}

#[tokio::test]
async fn test_document_search_functionality() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();

    // 複数の文書を作成
    let documents = vec![
        ("技術仕様書", 1),
        ("設計書", 1),
        ("要件定義書", 2),
        ("テスト計画書", 1),
    ];

    let mut created_docs = vec![];
    for (title, doc_type_id) in documents {
        let doc = test_env
            .create_document(
                CreateDocumentRequest {
                    title: title.to_string(),
                    document_type_id: doc_type_id,
                    created_by: 1,
                    created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                    confidentiality: TestConfidentiality::default(),
                    notes: Some(format!("{}のテスト", title)),
                },
                &admin_token,
            )
            .await
            .unwrap();
        created_docs.push(doc);
    }

    // 1. タイトル検索テスト
    let search_results = test_env
        .search_documents(
            DocumentSearchInput {
                title: Some("技術".to_string()),
                pagination: Pagination::default(),
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert!(search_results.documents.len() >= 1);
    assert!(
        search_results
            .documents
            .iter()
            .any(|d| d.title.contains("技術"))
    );

    // 2. 部分一致検索テスト
    let search_results = test_env
        .search_documents(
            DocumentSearchInput {
                title: Some("書".to_string()),
                pagination: Pagination::default(),
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert!(search_results.documents.len() >= 4); // すべての文書が「書」を含む

    // 3. ページネーションテスト
    let search_results = test_env
        .search_documents(
            DocumentSearchInput {
                title: None,
                pagination: Pagination {
                    offset: 0,
                    limit: 2,
                },
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert!(search_results.documents.len() <= 2);
}

#[tokio::test]
async fn test_document_lifecycle_with_updates() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();

    // 1. 初期文書作成
    let document = test_env
        .create_document(
            CreateDocumentRequest {
                title: "ライフサイクルテスト文書".to_string(),
                document_type_id: 1,
                created_by: 1,
                created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
                confidentiality: TestConfidentiality::default(),
                notes: Some("初期バージョン".to_string()),
            },
            &admin_token,
        )
        .await
        .unwrap();

    let original_id = document.id;
    let original_title = document.title.clone();

    // 2. 文書更新（タイトル変更）
    let updated_document = test_env
        .update_document(
            document.id,
            UpdateDocumentRequest {
                title: Some("更新されたライフサイクルテスト文書".to_string()),
                notes: None,
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert_eq!(updated_document.id, original_id);
    assert_ne!(updated_document.title, original_title);
    assert_eq!(updated_document.title, "更新されたライフサイクルテスト文書");

    // 3. 文書更新（ノート追加）
    let updated_document = test_env
        .update_document(
            document.id,
            UpdateDocumentRequest {
                title: None,
                notes: Some("更新履歴：タイトルを変更しました".to_string()),
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert_eq!(
        updated_document.notes,
        Some("更新履歴：タイトルを変更しました".to_string())
    );

    // 4. 更新後の検索確認
    let search_results = test_env
        .search_documents(
            DocumentSearchInput {
                title: Some("更新された".to_string()),
                pagination: Pagination::default(),
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert!(search_results.documents.len() >= 1);
    let found_doc = search_results
        .documents
        .iter()
        .find(|d| d.id == original_id)
        .unwrap();
    assert_eq!(found_doc.title, "更新されたライフサイクルテスト文書");
}

#[tokio::test]
async fn test_multiple_users_concurrent_operations() {
    let test_env = TestEnvironment::new().await;
    test_env.setup_test_data().await.unwrap();

    let admin_token = test_env
        .authenticate_user("admin", "password")
        .await
        .unwrap();
    let user_token = test_env
        .authenticate_user("user1", "password")
        .await
        .unwrap();

    // 並行して文書を作成
    let admin_doc_future = test_env.create_document(
        CreateDocumentRequest {
            title: "管理者文書".to_string(),
            document_type_id: 1,
            created_by: 1,
            created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
            confidentiality: TestConfidentiality::default(),
            notes: Some("管理者が作成".to_string()),
        },
        &admin_token,
    );

    let user_doc_future = test_env.create_document(
        CreateDocumentRequest {
            title: "ユーザー文書".to_string(),
            document_type_id: 2,
            created_by: 2,
            created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
            confidentiality: TestConfidentiality::default(),
            notes: Some("ユーザーが作成".to_string()),
        },
        &user_token,
    );

    let (admin_doc, user_doc) = tokio::try_join!(admin_doc_future, user_doc_future).unwrap();

    assert_eq!(admin_doc.title, "管理者文書");
    assert_eq!(user_doc.title, "ユーザー文書");
    assert_ne!(admin_doc.id, user_doc.id);

    // 両方の文書が検索で見つかることを確認
    let search_results = test_env
        .search_documents(
            DocumentSearchInput {
                title: None,
                pagination: Pagination {
                    offset: 0,
                    limit: 10,
                },
            },
            &admin_token,
        )
        .await
        .unwrap();

    assert!(search_results.documents.len() >= 2);
    assert!(
        search_results
            .documents
            .iter()
            .any(|d| d.title == "管理者文書")
    );
    assert!(
        search_results
            .documents
            .iter()
            .any(|d| d.title == "ユーザー文書")
    );
}
