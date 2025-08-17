# テスト戦略設計書

## 1. テスト戦略概要

### 1.1 設計方針

- **TDD (Test-Driven Development)**: 機能実装前のテスト作成
- **テストピラミッド**: 単体テスト重視、統合テスト・E2Eテストで補完
- **自動化重視**: CI/CDでの自動テスト実行
- **カバレッジ目標**: 単体テスト 85%以上、統合テスト 主要機能100%

### 1.2 テストレベル定義

```text
┌─────────────────────────────────────────┐
│           E2E テスト (5%)                │  ← ユーザーシナリオ
├─────────────────────────────────────────┤
│        統合テスト (15%)                  │  ← API・DB連携
├─────────────────────────────────────────┤
│        単体テスト (80%)                  │  ← 関数・メソッド
└─────────────────────────────────────────┘
```

### 1.3 技術スタック

```toml
[dev-dependencies]
# Rust バックエンド
tokio-test = "0.4"              # 非同期テスト
sqlx = { version = "0.7", features = ["testing"] }
mockall = "0.12"                # モック生成
wiremock = "0.6"                # HTTP モック
assert_matches = "1.5"          # パターンマッチング
proptest = "1.4"                # プロパティベーステスト
criterion = "0.5"               # ベンチマーク

# Frontend (package.json)
"@testing-library/svelte": "^4.0.0"
"@testing-library/jest-dom": "^6.0.0"
"vitest": "^1.0.0"
"playwright": "^1.40.0"
"jsdom": "^23.0.0"
```

## 2. 単体テスト設計

### 2.1 Rust バックエンド単体テスト

#### 2.1.1 Repository レイヤーテスト

```rust
// tests/repositories/document_repository_test.rs
use sqlx::SqlitePool;
use crate::repositories::DocumentRepository;
use crate::models::Document;
use chrono::Utc;

#[sqlx::test]
async fn test_create_document_success(pool: SqlitePool) {
    // Arrange
    let repo = DocumentRepository::new(pool);
    let document = CreateDocumentRequest {
        title: "テスト文書".to_string(),
        document_type_id: 1,
        created_by: 1,
        created_date: Utc::now().naive_utc(),
        // ...
    };

    // Act
    let result = repo.create(document).await;

    // Assert
    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.title, "テスト文書");
    assert!(!created.number.is_empty());
}

#[sqlx::test]
async fn test_search_documents_with_filters(pool: SqlitePool) {
    // Arrange
    let repo = DocumentRepository::new(pool);
    setup_test_documents(&pool).await;
    
    let filters = DocumentSearchFilters {
        title: Some("会議".to_string()),
        document_type_id: Some(1),
        pagination: Pagination { offset: 0, limit: 10 },
    };

    // Act
    let result = repo.search(filters).await;

    // Assert
    assert!(result.is_ok());
    let (documents, total) = result.unwrap();
    assert_eq!(documents.len(), 2);
    assert_eq!(total, 2);
    assert!(documents.iter().all(|d| d.title.contains("会議")));
}

#[sqlx::test]
async fn test_update_document_not_found(pool: SqlitePool) {
    // Arrange
    let repo = DocumentRepository::new(pool);
    let update = UpdateDocumentRequest {
        title: Some("更新タイトル".to_string()),
        // ...
    };

    // Act
    let result = repo.update(999, update).await;

    // Assert
    assert!(result.is_err());
    assert_matches!(result.unwrap_err(), RepositoryError::NotFound { .. });
}

async fn setup_test_documents(pool: &SqlitePool) {
    sqlx::query!(
        r#"
        INSERT INTO documents (number, title, document_type_id, created_by, created_date, is_active)
        VALUES 
        ('TEST-001', '月次会議議事録', 1, 1, '2024-12-15', 1),
        ('TEST-002', '定例会議資料', 1, 1, '2024-12-14', 1),
        ('TEST-003', '提案書', 2, 2, '2024-12-13', 1)
        "#
    )
    .execute(pool)
    .await
    .unwrap();
}
```

#### 2.1.2 Service レイヤーテスト

```rust
// tests/services/document_service_test.rs
use mockall::predicate::*;
use crate::services::DocumentService;
use crate::repositories::MockDocumentRepository;

#[tokio::test]
async fn test_create_document_with_number_generation() {
    // Arrange
    let mut mock_repo = MockDocumentRepository::new();
    let mut mock_number_service = MockNumberGenerationService::new();
    
    mock_number_service
        .expect_generate_number()
        .with(eq(1), eq(1), always())
        .times(1)
        .returning(|_, _, _| Ok("CTA-2508001".to_string()));
    
    mock_repo
        .expect_create()
        .with(always())
        .times(1)
        .returning(|req| {
            Ok(Document {
                id: 1,
                number: req.number.unwrap(),
                title: req.title,
                // ...
            })
        });
    
    let service = DocumentService::new(Box::new(mock_repo), Box::new(mock_number_service));
    
    // Act
    let result = service.create_document(CreateDocumentRequest {
        title: "テスト文書".to_string(),
        document_type_id: 1,
        created_by: 1,
        // number は自動生成されるため None
        number: None,
        // ...
    }).await;
    
    // Assert
    assert!(result.is_ok());
    let document = result.unwrap();
    assert_eq!(document.number, "CTA-2508001");
}

#[tokio::test]
async fn test_search_documents_with_permission_filter() {
    // Arrange
    let mut mock_repo = MockDocumentRepository::new();
    
    mock_repo
        .expect_search_with_permissions()
        .with(always(), eq(vec![1, 2]))  // アクセス可能部署
        .times(1)
        .returning(|_, _| Ok((vec![/* 適切な文書のみ */], 5)));
    
    let service = DocumentService::new(Box::new(mock_repo), Box::new(mock_number_service));
    
    let user_permissions = UserPermissions {
        accessible_departments: vec![1, 2],
        // ...
    };
    
    // Act
    let result = service.search_documents(
        DocumentSearchFilters::default(),
        &user_permissions
    ).await;
    
    // Assert
    assert!(result.is_ok());
    // 権限フィルタリングが適用されていることを確認
}
```

#### 2.1.3 プロパティベーステスト

```rust
// tests/property/document_number_test.rs
use proptest::prelude::*;
use crate::services::NumberGenerationService;

proptest! {
    #[test]
    fn test_document_number_format_always_valid(
        year in 2020u32..2030u32,
        department_code in "[A-Z]",
        sequence in 1u32..99999u32
    ) {
        let service = NumberGenerationService::new();
        let number = service.format_number(&department_code, year, sequence, 5);
        
        // 生成された番号が常に正しい形式であることを検証
        let re = regex::Regex::new(r"^[A-Z]-\d{7}$").unwrap();
        prop_assert!(re.is_match(&number));
        
        // 年が正しく含まれていることを検証
        let year_str = format!("{:02}", year % 100);
        prop_assert!(number.contains(&year_str));
    }
}

proptest! {
    #[test]
    fn test_search_filters_never_sql_injection(
        title in r"[a-zA-Z0-9\s\-_'\"\\;()]*",
        business_number in r"[A-Z0-9\-]*"
    ) {
        let filters = DocumentSearchFilters {
            title: if title.is_empty() { None } else { Some(title.clone()) },
            business_number: if business_number.is_empty() { None } else { Some(business_number.clone()) },
            // ...
        };
        
        // フィルタがSQLインジェクション攻撃を含んでいても安全に処理されることを検証
        let result = tokio_test::block_on(async {
            // テスト用データベースでの検索実行
            test_repository.search(filters).await
        });
        
        // エラーが発生してもSQLインジェクションによるものではないことを確認
        prop_assert!(result.is_ok() || matches!(result.unwrap_err(), RepositoryError::InvalidInput { .. }));
    }
}
```

### 2.2 フロントエンド単体テスト

#### 2.2.1 Svelte コンポーネントテスト

```typescript
// tests/components/Button.test.ts
import { render, fireEvent } from '@testing-library/svelte';
import { expect, test, vi } from 'vitest';
import Button from '../src/components/Button.svelte';

test('renders button with correct text', () => {
  const { getByRole } = render(Button, {
    props: { variant: 'primary' },
    slots: { default: 'Click me' }
  });

  const button = getByRole('button');
  expect(button).toBeInTheDocument();
  expect(button).toHaveTextContent('Click me');
  expect(button).toHaveClass('bg-primary-500');
});

test('calls click handler when clicked', async () => {
  const handleClick = vi.fn();
  const { getByRole } = render(Button, {
    props: { variant: 'secondary' }
  });

  const button = getByRole('button');
  button.addEventListener('click', handleClick);

  await fireEvent.click(button);
  expect(handleClick).toHaveBeenCalledTimes(1);
});

test('is disabled when loading', () => {
  const { getByRole } = render(Button, {
    props: { loading: true }
  });

  const button = getByRole('button');
  expect(button).toBeDisabled();
  expect(button).toHaveClass('opacity-50');
});

test('shows loading spinner when loading', () => {
  const { container } = render(Button, {
    props: { loading: true }
  });

  const spinner = container.querySelector('.animate-spin');
  expect(spinner).toBeInTheDocument();
});
```

#### 2.2.2 Store テスト

```typescript
// tests/stores/auth.test.ts
import { get } from 'svelte/store';
import { expect, test, vi, beforeEach } from 'vitest';
import { authStore, authActions } from '../src/stores/auth';

// モック fetch
global.fetch = vi.fn();

beforeEach(() => {
  vi.clearAllMocks();
  authStore.set({
    user: null,
    permissions: null,
    isAuthenticated: false,
    isLoading: false,
  });
});

test('login success updates store correctly', async () => {
  // Arrange
  const mockUser = {
    id: 1,
    employeeId: '12345',
    name: '田中太郎',
    email: 'tanaka@example.com',
  };

  const mockPermissions = {
    canCreateDocuments: true,
    canUpdateDocuments: true,
    // ...
  };

  vi.mocked(fetch).mockResolvedValueOnce({
    ok: true,
    json: async () => ({
      user: mockUser,
      permissions: mockPermissions,
    }),
  } as Response);

  // Act
  await authActions.login();

  // Assert
  const state = get(authStore);
  expect(state.user).toEqual(mockUser);
  expect(state.permissions).toEqual(mockPermissions);
  expect(state.isAuthenticated).toBe(true);
  expect(state.isLoading).toBe(false);
});

test('login failure handles error correctly', async () => {
  // Arrange
  vi.mocked(fetch).mockRejectedValueOnce(new Error('Network error'));

  // Act
  await authActions.login();

  // Assert
  const state = get(authStore);
  expect(state.user).toBeNull();
  expect(state.isAuthenticated).toBe(false);
  expect(state.isLoading).toBe(false);
});
```

## 3. 統合テスト設計

### 3.1 API統合テスト

```rust
// tests/integration/api_test.rs
use axum_test::TestServer;
use sqlx::SqlitePool;
use serde_json::json;

#[sqlx::test]
async fn test_document_crud_flow(pool: SqlitePool) {
    // Arrange
    let app = create_test_app(pool).await;
    let server = TestServer::new(app).unwrap();
    
    // テストユーザーでログイン
    let auth_token = setup_test_user(&server).await;
    
    // Act & Assert: 文書作成
    let create_response = server
        .post("/graphql")
        .add_header("Authorization", format!("Bearer {}", auth_token))
        .json(&json!({
            "query": r#"
                mutation CreateDocument($input: CreateDocumentInput!) {
                    createDocument(input: $input) {
                        success
                        data {
                            id
                            number
                            title
                        }
                        errors
                    }
                }
            "#,
            "variables": {
                "input": {
                    "title": "統合テスト文書",
                    "documentTypeId": "1",
                    "createdDate": "2024-12-15",
                    "internalExternal": "INTERNAL",
                    "importanceClass": "CLASS2",
                    "personalInfo": "NONE"
                }
            }
        }))
        .await;
    
    create_response.assert_status_ok();
    let create_data: serde_json::Value = create_response.json();
    
    assert!(create_data["data"]["createDocument"]["success"].as_bool().unwrap());
    let document_id = create_data["data"]["createDocument"]["data"]["id"].as_str().unwrap();
    let document_number = create_data["data"]["createDocument"]["data"]["number"].as_str().unwrap();
    
    // Act & Assert: 文書取得
    let get_response = server
        .post("/graphql")
        .add_header("Authorization", format!("Bearer {}", auth_token))
        .json(&json!({
            "query": r#"
                query GetDocument($id: ID!) {
                    document(id: $id) {
                        id
                        number
                        title
                        networkPath
                    }
                }
            "#,
            "variables": { "id": document_id }
        }))
        .await;
    
    get_response.assert_status_ok();
    let get_data: serde_json::Value = get_response.json();
    
    assert_eq!(
        get_data["data"]["document"]["number"].as_str().unwrap(),
        document_number
    );
    
    // Act & Assert: 文書更新
    let update_response = server
        .post("/graphql")
        .add_header("Authorization", format!("Bearer {}", auth_token))
        .json(&json!({
            "query": r#"
                mutation UpdateDocument($id: ID!, $input: UpdateDocumentInput!) {
                    updateDocument(id: $id, input: $input) {
                        success
                        data {
                            title
                        }
                    }
                }
            "#,
            "variables": {
                "id": document_id,
                "input": {
                    "title": "更新された文書タイトル"
                }
            }
        }))
        .await;
    
    update_response.assert_status_ok();
    let update_data: serde_json::Value = update_response.json();
    
    assert_eq!(
        update_data["data"]["updateDocument"]["data"]["title"].as_str().unwrap(),
        "更新された文書タイトル"
    );
}

#[sqlx::test]
async fn test_permission_based_access_control(pool: SqlitePool) {
    let app = create_test_app(pool).await;
    let server = TestServer::new(app).unwrap();
    
    // 制限されたユーザーでログイン
    let limited_token = setup_limited_user(&server).await;
    
    // 管理者のみアクセス可能な機能にアクセス試行
    let response = server
        .post("/api/admin/sync-ad")
        .add_header("Authorization", format!("Bearer {}", limited_token))
        .await;
    
    response.assert_status(StatusCode::FORBIDDEN);
    
    let error_data: serde_json::Value = response.json();
    assert_eq!(error_data["error"]["code"], "FORBIDDEN");
}
```

### 3.2 データベース統合テスト

```rust
// tests/integration/database_test.rs
#[sqlx::test(migrations = "migrations")]
async fn test_document_search_with_complex_filters(pool: SqlitePool) {
    // Arrange: テストデータ作成
    setup_complex_test_data(&pool).await;
    
    let repo = DocumentRepository::new(pool);
    
    // Test 1: 複合検索（タイトル + 作成日範囲 + 機密レベル）
    let filters = DocumentSearchFilters {
        title: Some("会議".to_string()),
        created_date_from: Some(NaiveDate::from_ymd_opt(2024, 12, 1).unwrap()),
        created_date_to: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
        confidentiality: Some(ConfidentialityFilter {
            internal_external: Some(InternalExternal::Internal),
            importance_class: Some(ImportanceClass::Class2),
            personal_info: None,
        }),
        pagination: Pagination { offset: 0, limit: 10 },
    };
    
    let (documents, total) = repo.search(filters).await.unwrap();
    
    assert_eq!(total, 3);
    assert!(documents.iter().all(|d| {
        d.title.contains("会議") &&
        d.internal_external == InternalExternal::Internal &&
        d.importance_class == ImportanceClass::Class2
    }));
    
    // Test 2: 権限による絞り込み
    let accessible_departments = vec![1, 2];
    let (filtered_docs, filtered_total) = repo
        .search_with_permissions(filters, accessible_departments)
        .await
        .unwrap();
    
    assert!(filtered_total <= total);
    assert!(filtered_docs.iter().all(|d| 
        [1, 2].contains(&d.creator_department_id)
    ));
}

#[sqlx::test]
async fn test_transaction_rollback_on_error(pool: SqlitePool) {
    let repo = DocumentRepository::new(pool);
    
    // 正常なトランザクション
    let mut tx = pool.begin().await.unwrap();
    
    let document1 = repo.create_in_transaction(&mut tx, valid_document_1()).await;
    assert!(document1.is_ok());
    
    let document2 = repo.create_in_transaction(&mut tx, valid_document_2()).await;
    assert!(document2.is_ok());
    
    tx.commit().await.unwrap();
    
    // エラーありトランザクション（ロールバック確認）
    let mut tx = pool.begin().await.unwrap();
    
    let document3 = repo.create_in_transaction(&mut tx, valid_document_3()).await;
    assert!(document3.is_ok());
    
    // 重複文書番号でエラー発生
    let document4 = repo.create_in_transaction(&mut tx, duplicate_number_document()).await;
    assert!(document4.is_err());
    
    tx.rollback().await.unwrap();
    
    // document3 が作成されていないことを確認
    let search_result = repo.search(DocumentSearchFilters {
        number: Some(valid_document_3().number),
        ..Default::default()
    }).await.unwrap();
    
    assert_eq!(search_result.1, 0); // total count = 0
}

async fn setup_complex_test_data(pool: &SqlitePool) {
    // 部署、社員、文書種別、文書データの一括作成
    sqlx::query!(r#"
        INSERT INTO departments (id, code, name, effective_from) VALUES
        (1, 'T', '技術部', '2024-01-01'),
        (2, 'S', '営業部', '2024-01-01'),
        (3, 'A', '管理部', '2024-01-01');
        
        INSERT INTO employees (id, employee_id, name, is_active) VALUES
        (1, 'E001', '田中太郎', 1),
        (2, 'E002', '佐藤花子', 1),
        (3, 'E003', '鈴木次郎', 1);
        
        INSERT INTO documents (
            number, title, document_type_id, created_by, created_date,
            internal_external, importance_class, personal_info, is_active
        ) VALUES
        ('CTA-001', '技術部月次会議議事録', 1, 1, '2024-12-15', 'internal', 'class2', 'none', 1),
        ('CTA-002', '営業部週次会議資料', 1, 2, '2024-12-10', 'internal', 'class2', 'none', 1),
        ('CTA-003', '管理部定例会議', 1, 3, '2024-12-05', 'internal', 'class1', 'none', 1),
        ('CTB-001', '顧客提案書', 2, 1, '2024-12-01', 'external', 'class1', 'present', 1);
    "#).execute(pool).await.unwrap();
}
```

## 4. E2E テスト設計

### 4.1 Playwright E2Eテスト

```typescript
// tests/e2e/document-management.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Document Management Flow', () => {
  test.beforeEach(async ({ page }) => {
    // テストデータベースにリセット
    await page.request.post('/api/test/reset-database');
    
    // ログイン
    await page.goto('/');
    await page.fill('[data-testid="username"]', 'test-user');
    await page.fill('[data-testid="password"]', 'password');
    await page.click('[data-testid="login-button"]');
    
    // ダッシュボードの表示を確認
    await expect(page.locator('[data-testid="dashboard"]')).toBeVisible();
  });

  test('create document end-to-end', async ({ page }) => {
    // 文書登録ページに移動
    await page.click('[data-testid="nav-documents"]');
    await page.click('[data-testid="new-document-button"]');
    
    // 文書情報入力
    await page.fill('[data-testid="document-title"]', 'E2E テスト文書');
    await page.selectOption('[data-testid="document-type"]', '1'); // 報告書
    await page.fill('[data-testid="business-number"]', 'JOB-2024-E2E');
    
    // 機密レベル設定
    await page.check('[data-testid="internal-radio"]');
    await page.check('[data-testid="class2-radio"]');
    await page.check('[data-testid="no-personal-info-radio"]');
    
    // 備考入力
    await page.fill('[data-testid="notes"]', 'E2Eテストで作成された文書です');
    
    // 文書番号自動生成確認
    const documentNumber = await page.locator('[data-testid="generated-number"]').textContent();
    expect(documentNumber).toMatch(/^[A-Z]+-\d{7}$/);
    
    // 保存
    await page.click('[data-testid="save-button"]');
    
    // 成功メッセージ確認
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
    
    // 文書一覧に遷移して確認
    await page.click('[data-testid="nav-documents"]');
    await page.fill('[data-testid="search-title"]', 'E2E テスト文書');
    await page.click('[data-testid="search-button"]');
    
    // 検索結果に表示されることを確認
    await expect(page.locator('[data-testid="document-row"]').first()).toContainText('E2E テスト文書');
    await expect(page.locator('[data-testid="document-row"]').first()).toContainText(documentNumber);
  });

  test('search and filter documents', async ({ page }) => {
    // テストデータの準備（事前に作成済み文書がある前提）
    await page.goto('/documents');
    
    // 基本検索
    await page.fill('[data-testid="search-title"]', '会議');
    await page.click('[data-testid="search-button"]');
    
    // 検索結果確認
    const searchResults = page.locator('[data-testid="document-row"]');
    await expect(searchResults).not.toHaveCount(0);
    
    const firstResult = searchResults.first();
    await expect(firstResult).toContainText('会議');
    
    // 詳細フィルター
    await page.click('[data-testid="advanced-filter-toggle"]');
    await page.selectOption('[data-testid="filter-document-type"]', '1'); // 報告書
    await page.fill('[data-testid="filter-date-from"]', '2024-12-01');
    await page.fill('[data-testid="filter-date-to"]', '2024-12-31');
    await page.click('[data-testid="search-button"]');
    
    // フィルター適用後の結果確認
    await expect(searchResults.first()).toBeVisible();
    
    // 結果がない場合のメッセージ確認
    await page.fill('[data-testid="search-title"]', '存在しない文書');
    await page.click('[data-testid="search-button"]');
    await expect(page.locator('[data-testid="no-results-message"]')).toBeVisible();
  });

  test('file operation integration', async ({ page }) => {
    // 文書詳細ページに移動
    await page.goto('/documents');
    await page.click('[data-testid="document-row"]').first();
    
    // ファイル情報セクション確認
    await expect(page.locator('[data-testid="file-info"]')).toBeVisible();
    
    // ネットワークパス表示確認
    const networkPath = await page.locator('[data-testid="network-path"]').textContent();
    expect(networkPath).toContain('\\\\server01\\docs');
    
    // フォルダ確認ボタン（Tauriアプリの場合のみ）
    if (await page.locator('[data-testid="open-folder-button"]').isVisible()) {
      await page.click('[data-testid="open-folder-button"]');
      // フォルダが開かれることの確認（実際のE2Eでは難しいので、リクエストが送信されることを確認）
      await expect(page.locator('[data-testid="folder-opened-message"]')).toBeVisible();
    }
    
    // ファイル存在確認
    await page.click('[data-testid="check-file-existence-button"]');
    await page.waitForTimeout(2000); // 確認処理の完了を待機
    
    // 結果確認
    const existenceStatus = await page.locator('[data-testid="existence-status"]').textContent();
    expect(['✅ 確認済み', '❌ 不存在', '⏳ 確認中']).toContain(existenceStatus);
  });

  test('permission-based access control', async ({ page }) => {
    // 制限されたユーザーでログイン
    await page.goto('/');
    await page.fill('[data-testid="username"]', 'limited-user');
    await page.fill('[data-testid="password"]', 'password');
    await page.click('[data-testid="login-button"]');
    
    // 管理者メニューが表示されないことを確認
    await expect(page.locator('[data-testid="nav-admin"]')).not.toBeVisible();
    
    // 文書作成ボタンが表示されないことを確認（権限なしの場合）
    await page.goto('/documents');
    await expect(page.locator('[data-testid="new-document-button"]')).not.toBeVisible();
    
    // 機密文書が検索結果に表示されないことを確認
    await page.fill('[data-testid="search-title"]', '機密');
    await page.click('[data-testid="search-button"]');
    
    const results = page.locator('[data-testid="document-row"]');
    await expect(results).toHaveCount(0);
  });
});

test.describe('Responsive Design', () => {
  test('mobile layout adaptation', async ({ page }) => {
    // モバイルサイズに設定
    await page.setViewportSize({ width: 375, height: 667 });
    
    await page.goto('/');
    
    // モバイルメニューの動作確認
    await expect(page.locator('[data-testid="mobile-menu-button"]')).toBeVisible();
    await page.click('[data-testid="mobile-menu-button"]');
    await expect(page.locator('[data-testid="mobile-menu"]')).toBeVisible();
    
    // 文書一覧のモバイル表示確認
    await page.goto('/documents');
    
    // テーブルが適切にスクロール可能であることを確認
    const table = page.locator('[data-testid="documents-table"]');
    await expect(table).toBeVisible();
    
    // モバイル専用の操作ボタンが表示されることを確認
    await expect(page.locator('[data-testid="mobile-actions"]')).toBeVisible();
  });
});
```

### 4.2 アクセシビリティテスト

```typescript
// tests/e2e/accessibility.spec.ts
import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test.describe('Accessibility Tests', () => {
  test('dashboard accessibility', async ({ page }) => {
    await page.goto('/');
    await page.fill('[data-testid="username"]', 'test-user');
    await page.fill('[data-testid="password"]', 'password');
    await page.click('[data-testid="login-button"]');
    
    // ダッシュボードのアクセシビリティチェック
    const accessibilityScanResults = await new AxeBuilder({ page })
      .withTags(['wcag2a', 'wcag2aa', 'wcag21aa'])
      .analyze();
    
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test('keyboard navigation', async ({ page }) => {
    await page.goto('/documents');
    
    // Tabキーでのナビゲーション
    await page.keyboard.press('Tab');
    let focused = await page.evaluate(() => document.activeElement?.getAttribute('data-testid'));
    expect(focused).toBe('nav-dashboard');
    
    await page.keyboard.press('Tab');
    focused = await page.evaluate(() => document.activeElement?.getAttribute('data-testid'));
    expect(focused).toBe('nav-documents');
    
    // Enterキーでの選択
    await page.keyboard.press('Enter');
    await expect(page).toHaveURL(/.*\/documents/);
    
    // 検索フォームへのTabナビゲーション
    await page.keyboard.press('Tab');
    focused = await page.evaluate(() => document.activeElement?.getAttribute('data-testid'));
    expect(focused).toBe('search-title');
  });

  test('screen reader compatibility', async ({ page }) => {
    await page.goto('/documents/new');
    
    // ARIA属性の確認
    const titleInput = page.locator('[data-testid="document-title"]');
    await expect(titleInput).toHaveAttribute('aria-required', 'true');
    await expect(titleInput).toHaveAttribute('aria-describedby');
    
    // フォームエラー時のaria-invalid設定確認
    await page.click('[data-testid="save-button"]'); // 必須項目未入力で保存試行
    await expect(titleInput).toHaveAttribute('aria-invalid', 'true');
    
    // エラーメッセージとの関連付け確認
    const errorId = await titleInput.getAttribute('aria-describedby');
    const errorElement = page.locator(`#${errorId}`);
    await expect(errorElement).toBeVisible();
    await expect(errorElement).toContainText('必須項目');
  });
});
```

## 5. パフォーマンステスト

### 5.1 負荷テスト

```rust
// benches/api_performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

fn document_search_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let app = rt.block_on(create_test_app());
    
    c.bench_function("document_search_1000_records", |b| {
        b.iter(|| {
            rt.block_on(async {
                let response = app
                    .oneshot(
                        Request::builder()
                            .uri("/api/documents/search")
                            .method("POST")
                            .header("content-type", "application/json")
                            .body(Body::from(r#"{"limit": 50, "offset": 0}"#))
                            .unwrap()
                    )
                    .await
                    .unwrap();
                
                black_box(response);
            })
        })
    });
}

fn document_creation_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let app = rt.block_on(create_test_app());
    
    c.bench_function("document_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let document_data = serde_json::json!({
                    "title": format!("Benchmark Document {}", uuid::Uuid::new_v4()),
                    "documentTypeId": 1,
                    "createdDate": "2024-12-15"
                });
                
                let response = app
                    .oneshot(
                        Request::builder()
                            .uri("/api/documents")
                            .method("POST")
                            .header("content-type", "application/json")
                            .body(Body::from(document_data.to_string()))
                            .unwrap()
                    )
                    .await
                    .unwrap();
                
                black_box(response);
            })
        })
    });
}

criterion_group!(benches, document_search_benchmark, document_creation_benchmark);
criterion_main!(benches);
```

### 5.2 フロントエンドパフォーマンステスト

```typescript
// tests/performance/frontend-performance.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Frontend Performance', () => {
  test('page load performance', async ({ page }) => {
    // Performance metrics収集開始
    await page.goto('/documents', { waitUntil: 'networkidle' });
    
    const performanceMetrics = await page.evaluate(() => {
      const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
      return {
        domContentLoaded: navigation.domContentLoadedEventEnd - navigation.navigationStart,
        loadComplete: navigation.loadEventEnd - navigation.navigationStart,
        firstPaint: performance.getEntriesByName('first-paint')[0]?.startTime || 0,
        firstContentfulPaint: performance.getEntriesByName('first-contentful-paint')[0]?.startTime || 0,
      };
    });
    
    // パフォーマンス基準の確認
    expect(performanceMetrics.domContentLoaded).toBeLessThan(3000); // 3秒以内
    expect(performanceMetrics.loadComplete).toBeLessThan(5000); // 5秒以内
    expect(performanceMetrics.firstContentfulPaint).toBeLessThan(2000); // 2秒以内
  });

  test('large dataset rendering performance', async ({ page }) => {
    // 大量データの表示テスト
    await page.route('/api/documents/search', async route => {
      const largeDataset = Array.from({ length: 1000 }, (_, i) => ({
        id: i + 1,
        number: `TEST-${String(i + 1).padStart(6, '0')}`,
        title: `Test Document ${i + 1}`,
        createdDate: '2024-12-15',
        documentType: { name: 'Report' },
        creator: { name: 'Test User' }
      }));

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          documents: largeDataset,
          totalCount: 1000
        })
      });
    });

    await page.goto('/documents');
    
    const startTime = Date.now();
    await page.fill('[data-testid="search-title"]', '');
    await page.click('[data-testid="search-button"]');
    
    // 最初の行が表示されるまでの時間測定
    await page.waitForSelector('[data-testid="document-row"]');
    const renderTime = Date.now() - startTime;
    
    expect(renderTime).toBeLessThan(3000); // 3秒以内でレンダリング完了
    
    // スクロールパフォーマンステスト
    const scrollStartTime = Date.now();
    await page.mouse.wheel(0, 1000);
    await page.waitForTimeout(100);
    const scrollTime = Date.now() - scrollStartTime;
    
    expect(scrollTime).toBeLessThan(500); // スムーズなスクロール
  });
});
```

## 6. テスト実行・CI統合

### 6.1 テスト実行スクリプト

```json
// package.json
{
  "scripts": {
    "test": "npm run test:unit && npm run test:integration && npm run test:e2e",
    "test:unit": "vitest run",
    "test:unit:watch": "vitest",
    "test:integration": "playwright test tests/integration",
    "test:e2e": "playwright test tests/e2e",
    "test:e2e:headed": "playwright test tests/e2e --headed",
    "test:accessibility": "playwright test tests/e2e/accessibility.spec.ts",
    "test:performance": "playwright test tests/performance",
    "test:coverage": "vitest run --coverage"
  }
}
```

```toml
# Cargo.toml
[package]
name = "doc_man_db"

[[bin]]
name = "test-runner"
path = "tests/test_runner.rs"

[dev-dependencies]
tokio-test = "0.4"
sqlx = { version = "0.7", features = ["testing", "sqlite"] }
criterion = { version = "0.5", features = ["html_reports"] }
```

### 6.2 GitHub Actions CI設定

```yaml
# .github/workflows/test.yml
name: Tests

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  rust-tests:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
    
    - name: Cache cargo
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Check clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
    
    - name: Run unit tests
      run: cargo test --lib
    
    - name: Run integration tests
      run: cargo test --test '*'
    
    - name: Generate coverage report
      uses: actions-rs/tarpaulin@v0.1
      with:
        args: '--all-features --workspace --timeout 120 --out Xml'
    
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./cobertura.xml

  frontend-tests:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '20'
        cache: 'npm'
    
    - name: Install dependencies
      run: npm ci
    
    - name: Run unit tests
      run: npm run test:unit
    
    - name: Install Playwright
      run: npx playwright install
    
    - name: Build application
      run: npm run build
    
    - name: Start test server
      run: npm run preview &
    
    - name: Run E2E tests
      run: npm run test:e2e
    
    - name: Run accessibility tests
      run: npm run test:accessibility
    
    - name: Upload test results
      uses: actions/upload-artifact@v3
      if: failure()
      with:
        name: playwright-report
        path: playwright-report/

  performance-tests:
    runs-on: ubuntu-latest
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Run benchmark tests
      run: cargo bench
    
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '20'
        cache: 'npm'
    
    - name: Install dependencies
      run: npm ci
    
    - name: Install Playwright
      run: npx playwright install
    
    - name: Run performance tests
      run: npm run test:performance
    
    - name: Store benchmark results
      uses: benchmark-action/github-action-benchmark@v1
      with:
        tool: 'cargo'
        output-file-path: target/criterion/report/index.html
        github-token: ${{ secrets.GITHUB_TOKEN }}
        auto-push: true
```

---

**最終更新**: 2024年12月  
**作成者**: 開発チーム  
**承認者**: プロジェクトマネージャー
