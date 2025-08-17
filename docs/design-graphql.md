# GraphQL スキーマ設計書

## 1. GraphQL概要

### 1.1 設計方針

- **型安全性**: Rustの型システムとGraphQLスキーマの完全な一致
- **効率性**: N+1問題回避、DataLoaderパターン活用
- **拡張性**: 将来的な機能追加に対応可能なスキーマ設計
- **一貫性**: RESTful な設計思想をGraphQLに適用

### 1.2 GraphQLライブラリ選定

```toml
[dependencies]
async-graphql = "7.0"           # GraphQLサーバー実装
async-graphql-axum = "7.0"      # Axum統合
dataloader = "0.17"             # N+1問題解決
```

### 1.3 スキーマ構成

```text
schema/
├── scalars.rs          # カスタムスカラー型
├── types/              # GraphQL型定義
│   ├── employee.rs     # 社員関連型
│   ├── document.rs     # 文書関連型
│   ├── business.rs     # 業務関連型
│   └── system.rs       # システム関連型
├── queries.rs          # Query root
├── mutations.rs        # Mutation root
└── lib.rs             # スキーマ統合
```

## 2. カスタムスカラー型

### 2.1 基本スカラー

```rust
use async_graphql::*;
use chrono::{DateTime, Utc};

/// 日時型 (ISO 8601 UTC)
#[derive(Clone, Debug)]
pub struct DateTimeUtc(pub DateTime<Utc>);

#[Scalar]
impl ScalarType for DateTimeUtc {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| DateTimeUtc(dt.with_timezone(&Utc)))
                    .map_err(|_| InputValueError::custom("Invalid datetime format"))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}

/// 日付型 (YYYY-MM-DD)
#[derive(Clone, Debug)]
pub struct Date(pub chrono::NaiveDate);

#[Scalar]
impl ScalarType for Date {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                    .map(Date)
                    .map_err(|_| InputValueError::custom("Invalid date format (YYYY-MM-DD)"))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.format("%Y-%m-%d").to_string())
    }
}

/// JSON型
#[derive(Clone, Debug)]
pub struct Json(pub serde_json::Value);

#[Scalar]
impl ScalarType for Json {
    fn parse(value: Value) -> InputValueResult<Self> {
        Ok(Json(value.into_json().map_err(|_| {
            InputValueError::custom("Invalid JSON")
        })?))
    }

    fn to_value(&self) -> Value {
        Value::from_json(self.0.clone()).unwrap()
    }
}
```

## 3. 型定義

### 3.1 組織・人員関連型

```rust
use async_graphql::*;
use super::scalars::*;

/// 部署
#[derive(SimpleObject)]
pub struct Department {
    pub id: ID,
    pub code: String,
    pub name: String,
    pub parent_id: Option<ID>,
    pub effective_from: Date,
    pub effective_to: Option<Date>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 課
#[derive(SimpleObject)]
pub struct Section {
    pub id: ID,
    pub department_id: ID,
    pub code: String,
    pub name: String,
    pub effective_from: Date,
    pub effective_to: Option<Date>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 社員
#[derive(SimpleObject)]
pub struct Employee {
    pub id: ID,
    pub employee_id: String,
    pub name: String,
    pub email: Option<String>,
    pub ad_username: Option<String>,
    pub data_source: DataSource,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// データソース
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DataSource {
    Json,
    AdSync,
}

/// 社員所属履歴
#[derive(SimpleObject)]
pub struct EmployeeAssignment {
    pub id: ID,
    pub employee_id: ID,
    pub department_id: ID,
    pub section_id: Option<ID>,
    pub assignment_type: AssignmentType,
    pub effective_from: Date,
    pub effective_to: Option<Date>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 所属種別
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AssignmentType {
    Primary,    // 本配属
    Concurrent, // 兼務
    Secondment, // 出向
}
```

### 3.2 文書関連型

```rust
/// 文書種別
#[derive(SimpleObject)]
pub struct DocumentType {
    pub id: ID,
    pub code: String,
    pub name: String,
    pub requires_approval: bool,
    pub department_id: Option<ID>,
    pub effective_from: Date,
    pub effective_to: Option<Date>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 文書番号生成ルール
#[derive(SimpleObject)]
pub struct DocumentNumberRule {
    pub id: ID,
    pub name: String,
    pub template: String,
    pub sequence_digits: i32,
    pub department_id: Option<ID>,
    pub document_type_codes: Option<Vec<String>>,
    pub priority: i32,
    pub effective_from: Date,
    pub effective_to: Option<Date>,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 文書
#[derive(SimpleObject)]
pub struct Document {
    pub id: ID,
    pub number: String,
    pub title: String,
    pub document_type_id: ID,
    pub business_number: Option<String>,
    pub created_by: ID,
    pub created_date: Date,
    pub version_type: VersionType,
    pub version_number: Option<i32>,
    pub parent_document_id: Option<ID>,
    
    // 機密レベル
    pub internal_external: InternalExternal,
    pub importance_class: ImportanceClass,
    pub personal_info: PersonalInfo,
    
    // パス情報
    pub network_path: Option<String>,
    pub folder_exists: Option<bool>,
    pub approval_file_exists: Option<bool>,
    pub last_path_check: Option<DateTimeUtc>,
    
    pub notes: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 版種別
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum VersionType {
    Draft,     // ドラフト
    Revision,  // 改訂版
    Final,     // 最終版
}

/// 社内外区分
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum InternalExternal {
    Internal,  // 社内
    External,  // 社外
}

/// 重要度クラス
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ImportanceClass {
    Class1,    // 情報クラスⅠ
    Class2,    // 情報クラスⅡ
}

/// 個人情報有無
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PersonalInfo {
    None,      // 個人情報なし
    Present,   // 個人情報あり
}
```

### 3.3 業務関連型

```rust
/// 業務
#[derive(SimpleObject)]
pub struct Business {
    pub id: ID,
    pub business_number: String,
    pub name: String,
    pub description: Option<String>,
    pub customer_name: Option<String>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub status: BusinessStatus,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 業務ステータス
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum BusinessStatus {
    Active,     // 進行中
    Completed,  // 完了
    Suspended,  // 中断
}

/// 業務従事者
#[derive(SimpleObject)]
pub struct BusinessMember {
    pub id: ID,
    pub business_id: ID,
    pub employee_id: ID,
    pub role: BusinessRole,
    pub participation_level: ParticipationLevel,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 業務役割
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum BusinessRole {
    Leader,   // リーダー
    Member,   // メンバー
    Support,  // サポート
}

/// 参加レベル
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ParticipationLevel {
    Full,        // フル参加
    Partial,     // 部分参加
    Consultant,  // コンサルタント
}

/// 外部連絡先
#[derive(SimpleObject)]
pub struct ExternalContact {
    pub id: ID,
    pub business_id: ID,
    pub contact_type: ContactType,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// 連絡先種別
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ContactType {
    Customer,  // 顧客
    Partner,   // パートナー
    Vendor,    // ベンダー
}
```

## 4. 入力型定義

### 4.1 検索入力型

```rust
/// 文書検索入力
#[derive(InputObject)]
pub struct DocumentSearchInput {
    /// 文書番号（部分一致）
    pub number: Option<String>,
    /// タイトル（部分一致）
    pub title: Option<String>,
    /// 文書種別ID
    pub document_type_id: Option<ID>,
    /// 業務番号（部分一致）
    pub business_number: Option<String>,
    /// 作成者ID
    pub created_by: Option<ID>,
    /// 作成日範囲（開始）
    pub created_date_from: Option<Date>,
    /// 作成日範囲（終了）
    pub created_date_to: Option<Date>,
    /// 機密レベル
    pub confidentiality: Option<ConfidentialityInput>,
    /// アクティブフラグ
    pub is_active: Option<bool>,
    /// ページング
    pub pagination: Option<PaginationInput>,
    /// ソート
    pub sort: Option<DocumentSortInput>,
}

/// 機密レベル検索条件
#[derive(InputObject)]
pub struct ConfidentialityInput {
    pub internal_external: Option<InternalExternal>,
    pub importance_class: Option<ImportanceClass>,
    pub personal_info: Option<PersonalInfo>,
}

/// ページング入力
#[derive(InputObject)]
pub struct PaginationInput {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

/// 文書ソート入力
#[derive(InputObject)]
pub struct DocumentSortInput {
    pub field: DocumentSortField,
    pub direction: SortDirection,
}

/// 文書ソートフィールド
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DocumentSortField {
    Number,
    Title,
    CreatedDate,
    UpdatedAt,
}

/// ソート方向
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// 社員検索入力
#[derive(InputObject)]
pub struct EmployeeSearchInput {
    /// 社員番号（部分一致）
    pub employee_id: Option<String>,
    /// 氏名（部分一致）
    pub name: Option<String>,
    /// 部署ID
    pub department_id: Option<ID>,
    /// 課ID
    pub section_id: Option<ID>,
    /// アクティブフラグ
    pub is_active: Option<bool>,
    /// ページング
    pub pagination: Option<PaginationInput>,
}

/// 業務検索入力
#[derive(InputObject)]
pub struct BusinessSearchInput {
    /// 業務番号（部分一致）
    pub business_number: Option<String>,
    /// 業務名（部分一致）
    pub name: Option<String>,
    /// 顧客名（部分一致）
    pub customer_name: Option<String>,
    /// ステータス
    pub status: Option<BusinessStatus>,
    /// 従事者ID
    pub member_id: Option<ID>,
    /// アクティブフラグ
    pub is_active: Option<bool>,
    /// ページング
    pub pagination: Option<PaginationInput>,
}
```

### 4.2 作成・更新入力型

```rust
/// 文書作成入力
#[derive(InputObject)]
pub struct CreateDocumentInput {
    pub title: String,
    pub document_type_id: ID,
    pub business_number: Option<String>,
    pub created_date: Date,
    pub version_type: VersionType,
    pub version_number: Option<i32>,
    pub parent_document_id: Option<ID>,
    
    // 機密レベル
    pub internal_external: InternalExternal,
    pub importance_class: ImportanceClass,
    pub personal_info: PersonalInfo,
    
    pub notes: Option<String>,
}

/// 文書更新入力
#[derive(InputObject)]
pub struct UpdateDocumentInput {
    pub title: Option<String>,
    pub business_number: Option<String>,
    pub internal_external: Option<InternalExternal>,
    pub importance_class: Option<ImportanceClass>,
    pub personal_info: Option<PersonalInfo>,
    pub notes: Option<String>,
    pub is_active: Option<bool>,
}

/// 社員作成入力
#[derive(InputObject)]
pub struct CreateEmployeeInput {
    pub employee_id: String,
    pub name: String,
    pub email: Option<String>,
    pub ad_username: Option<String>,
    pub data_source: DataSource,
}

/// 社員更新入力
#[derive(InputObject)]
pub struct UpdateEmployeeInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub ad_username: Option<String>,
    pub is_active: Option<bool>,
}

/// 業務作成入力
#[derive(InputObject)]
pub struct CreateBusinessInput {
    pub business_number: String,
    pub name: String,
    pub description: Option<String>,
    pub customer_name: Option<String>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
}
```

## 5. 結果型定義

### 5.1 検索結果型

```rust
/// 文書検索結果
#[derive(SimpleObject)]
pub struct DocumentSearchResult {
    pub documents: Vec<Document>,
    pub total_count: i32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

/// 社員検索結果
#[derive(SimpleObject)]
pub struct EmployeeSearchResult {
    pub employees: Vec<Employee>,
    pub total_count: i32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

/// 業務検索結果
#[derive(SimpleObject)]
pub struct BusinessSearchResult {
    pub businesses: Vec<Business>,
    pub total_count: i32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}
```

### 5.2 操作結果型

```rust
/// 共通操作結果
#[derive(SimpleObject)]
pub struct MutationResult<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub errors: Vec<String>,
}

/// 文書番号生成結果
#[derive(SimpleObject)]
pub struct GenerateDocumentNumberResult {
    pub success: bool,
    pub document_number: Option<String>,
    pub rule_applied: Option<String>,
    pub errors: Vec<String>,
}

/// ファイル存在確認結果
#[derive(SimpleObject)]
pub struct FileExistenceCheckResult {
    pub document_id: ID,
    pub network_path: String,
    pub folder_exists: bool,
    pub approval_file_exists: Option<bool>,
    pub checked_at: DateTimeUtc,
    pub errors: Vec<String>,
}
```

## 6. Query Root

```rust
use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 文書検索
    async fn documents(
        &self,
        ctx: &Context<'_>,
        search: Option<DocumentSearchInput>,
    ) -> Result<DocumentSearchResult> {
        // 実装: DocumentService::search_documents
        todo!()
    }

    /// 文書詳細取得
    async fn document(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Document>> {
        // 実装: DocumentService::get_document_by_id
        todo!()
    }

    /// 社員検索
    async fn employees(
        &self,
        ctx: &Context<'_>,
        search: Option<EmployeeSearchInput>,
    ) -> Result<EmployeeSearchResult> {
        // 実装: EmployeeService::search_employees
        todo!()
    }

    /// 社員詳細取得
    async fn employee(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Employee>> {
        // 実装: EmployeeService::get_employee_by_id
        todo!()
    }

    /// 業務検索
    async fn businesses(
        &self,
        ctx: &Context<'_>,
        search: Option<BusinessSearchInput>,
    ) -> Result<BusinessSearchResult> {
        // 実装: BusinessService::search_businesses
        todo!()
    }

    /// 業務詳細取得
    async fn business(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Business>> {
        // 実装: BusinessService::get_business_by_id
        todo!()
    }

    /// 部署一覧取得
    async fn departments(&self, ctx: &Context<'_>) -> Result<Vec<Department>> {
        // 実装: OrganizationService::get_departments
        todo!()
    }

    /// 課一覧取得（部署IDで絞り込み）
    async fn sections(
        &self,
        ctx: &Context<'_>,
        department_id: Option<ID>,
    ) -> Result<Vec<Section>> {
        // 実装: OrganizationService::get_sections
        todo!()
    }

    /// 文書種別一覧取得
    async fn document_types(&self, ctx: &Context<'_>) -> Result<Vec<DocumentType>> {
        // 実装: DocumentService::get_document_types
        todo!()
    }

    /// 現在ユーザー情報取得
    async fn current_user(&self, ctx: &Context<'_>) -> Result<Option<Employee>> {
        // 実装: AuthService::get_current_user
        todo!()
    }
}
```

## 7. Mutation Root

```rust
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// 文書作成
    async fn create_document(
        &self,
        ctx: &Context<'_>,
        input: CreateDocumentInput,
    ) -> Result<MutationResult<Document>> {
        // 実装: DocumentService::create_document
        todo!()
    }

    /// 文書更新
    async fn update_document(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdateDocumentInput,
    ) -> Result<MutationResult<Document>> {
        // 実装: DocumentService::update_document
        todo!()
    }

    /// 文書削除（論理削除）
    async fn delete_document(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<MutationResult<bool>> {
        // 実装: DocumentService::delete_document
        todo!()
    }

    /// 文書番号生成
    async fn generate_document_number(
        &self,
        ctx: &Context<'_>,
        document_type_id: ID,
        department_id: ID,
        created_date: Date,
    ) -> Result<GenerateDocumentNumberResult> {
        // 実装: NumberGenerationService::generate_number
        todo!()
    }

    /// 社員作成
    async fn create_employee(
        &self,
        ctx: &Context<'_>,
        input: CreateEmployeeInput,
    ) -> Result<MutationResult<Employee>> {
        // 実装: EmployeeService::create_employee
        todo!()
    }

    /// 社員更新
    async fn update_employee(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdateEmployeeInput,
    ) -> Result<MutationResult<Employee>> {
        // 実装: EmployeeService::update_employee
        todo!()
    }

    /// 業務作成
    async fn create_business(
        &self,
        ctx: &Context<'_>,
        input: CreateBusinessInput,
    ) -> Result<MutationResult<Business>> {
        // 実装: BusinessService::create_business
        todo!()
    }

    /// ファイル存在確認実行
    async fn check_file_existence(
        &self,
        ctx: &Context<'_>,
        document_ids: Option<Vec<ID>>,
    ) -> Result<Vec<FileExistenceCheckResult>> {
        // 実装: FileCheckService::check_files
        todo!()
    }

    /// AD同期実行
    async fn sync_ad_users(&self, ctx: &Context<'_>) -> Result<MutationResult<i32>> {
        // 実装: AdSyncService::sync_users
        todo!()
    }
}
```

## 8. DataLoader実装

### 8.1 N+1問題対策

```rust
use dataloader::*;
use std::collections::HashMap;

/// 文書種別DataLoader
pub struct DocumentTypeLoader {
    pool: sqlx::SqlitePool,
}

#[async_trait::async_trait]
impl Loader<i64> for DocumentTypeLoader {
    type Value = DocumentType;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let types = sqlx::query_as!(
            DocumentType,
            "SELECT * FROM document_types WHERE id = ANY(?)",
            keys
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(types.into_iter().map(|t| (t.id, t)).collect())
    }
}

/// 社員DataLoader
pub struct EmployeeLoader {
    pool: sqlx::SqlitePool,
}

#[async_trait::async_trait]
impl Loader<i64> for EmployeeLoader {
    type Value = Employee;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let employees = sqlx::query_as!(
            Employee,
            "SELECT * FROM employees WHERE id = ANY(?)",
            keys
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(employees.into_iter().map(|e| (e.id, e)).collect())
    }
}
```

## 9. スキーマ統合

```rust
use async_graphql::*;

/// GraphQLスキーマ構築
pub fn build_schema(
    pool: sqlx::SqlitePool,
) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let document_type_loader = DataLoader::new(
        DocumentTypeLoader { pool: pool.clone() },
        tokio::spawn,
    );
    
    let employee_loader = DataLoader::new(
        EmployeeLoader { pool: pool.clone() },
        tokio::spawn,
    );

    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .data(document_type_loader)
        .data(employee_loader)
        .finish()
}
```

## 10. 認証・認可統合

### 10.1 GraphQL Context

```rust
/// GraphQLコンテキスト
pub struct GraphQLContext {
    pub pool: sqlx::SqlitePool,
    pub current_user: Option<Employee>,
    pub permissions: UserPermissions,
}

/// ユーザー権限
#[derive(Clone)]
pub struct UserPermissions {
    pub can_create_documents: bool,
    pub can_update_documents: bool,
    pub can_delete_documents: bool,
    pub can_manage_users: bool,
    pub accessible_departments: Vec<i64>,
    pub max_confidentiality_level: ConfidentialityLevel,
}

/// 機密レベル総合判定
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConfidentialityLevel {
    pub internal_external: InternalExternal,
    pub importance_class: ImportanceClass,
    pub personal_info: PersonalInfo,
}
```

### 10.2 権限チェック

```rust
/// 権限チェック用ガード
pub struct RequirePermission<T>(pub T);

#[async_trait::async_trait]
impl<T> Guard for RequirePermission<T>
where
    T: Fn(&UserPermissions) -> bool + Send + Sync,
{
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let permissions = ctx.data::<UserPermissions>()?;
        
        if (self.0)(permissions) {
            Ok(())
        } else {
            Err(Error::new("Insufficient permissions"))
        }
    }
}

// 使用例
#[Object]
impl MutationRoot {
    #[graphql(guard = RequirePermission(|p| p.can_create_documents))]
    async fn create_document(
        &self,
        ctx: &Context<'_>,
        input: CreateDocumentInput,
    ) -> Result<MutationResult<Document>> {
        // 実装
        todo!()
    }
}
```

---

**最終更新**: 2024年12月  
**作成者**: 開発チーム  
**承認者**: プロジェクトマネージャー
