# Phase 2: コア機能実装 (Week 3-6)

## フェーズ概要

- **期間**: Week 3-6 (4週間)
- **目標**: 文書管理システムのコア機能実装
- **成果物**: 文書管理API（完全版）、ファイル存在確認機能、基本検索機能

## タスク一覧

### TASK-009: 文書番号生成ルール実装

- **説明**: ルールベース文書番号生成・版管理機能
- **優先度**: High
- **見積工数**: 20h
- **状態**: 未着手
- **依存関係**: TASK-004

#### 実装内容(TASK-009)

1. 番号生成ルールエンジン
2. 歴史的形式対応（CTA-2508001、技術-25001、DA-25001等）
3. 版管理（r1, r2, d1, d2等）
4. 連番管理システム

#### ファイル構成

```text
src/services/
├── number_generation/
│   ├── mod.rs
│   ├── rule_engine.rs      # ルールエンジン
│   ├── format_handlers.rs  # 形式別ハンドラー
│   ├── version_manager.rs  # 版管理
│   └── sequence_manager.rs # 連番管理
```

#### 実装例(TASK-009)

```rust
// src/services/number_generation/rule_engine.rs
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct NumberGenerationRule {
    pub id: i32,
    pub name: String,
    pub template: String, // "{department_code}-{year:2}{sequence:5}"
    pub applies_to_departments: Vec<String>,
    pub applies_to_doc_types: Vec<String>,
    pub effective_from: NaiveDate,
    pub effective_to: Option<NaiveDate>,
    pub priority: i32,
}

pub struct NumberGenerationService {
    rules: Vec<NumberGenerationRule>,
}

impl NumberGenerationService {
    pub fn generate_number(
        &self,
        department_code: &str,
        doc_type_code: &str,
        created_date: NaiveDate,
    ) -> Result<String, NumberGenerationError> {
        let applicable_rule = self.find_applicable_rule(
            department_code, 
            doc_type_code, 
            created_date
        )?;
        
        let sequence = self.get_next_sequence(
            &applicable_rule,
            department_code,
            created_date.year()
        )?;
        
        self.format_number(&applicable_rule, department_code, created_date, sequence)
    }
}
```

#### データベーステーブル(TASK-009)

```sql
-- 番号生成ルール
CREATE TABLE number_generation_rules (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    template TEXT NOT NULL,
    sequence_digits INTEGER DEFAULT 5,
    applies_to_departments TEXT, -- JSON array
    applies_to_doc_types TEXT,   -- JSON array
    effective_from DATE NOT NULL,
    effective_to DATE,
    priority INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 連番管理
CREATE TABLE document_sequences (
    id INTEGER PRIMARY KEY,
    rule_id INTEGER NOT NULL,
    department_code TEXT NOT NULL,
    year INTEGER NOT NULL,
    last_sequence INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (rule_id) REFERENCES number_generation_rules (id),
    UNIQUE(rule_id, department_code, year)
);
```

#### 成果物(TASK-009)

- 番号生成エンジン
- 版管理システム
- 連番管理機能
- 歴史的形式対応

---

### TASK-010: 組織・人員管理API

- **説明**: 部署・人員のCRUD API
- **優先度**: High
- **見積工数**: 20h
- **状態**: 未着手
- **依存関係**: TASK-004

#### 実装内容(TASK-010)

1. 部署管理API（階層構造対応）
2. 社員管理API（異動履歴対応）
3. AD同期機能（基本版）
4. 権限管理API

#### API エンドポイント

```yaml
GraphQL API:
- Query:
  - departments: [Department]
  - employees: [Employee]
  - employee(id: ID!): Employee
  - departmentHistory(employeeId: ID!): [DepartmentAssignment]

- Mutation:
  - createEmployee(input: CreateEmployeeInput!): EmployeeResponse
  - updateEmployee(id: ID!, input: UpdateEmployeeInput!): EmployeeResponse
  - assignToDepartment(input: DepartmentAssignmentInput!): AssignmentResponse
```

#### 実装例(TASK-010)

```rust
// src/handlers/graphql/employee.rs
use async_graphql::*;

#[derive(SimpleObject)]
pub struct Employee {
    pub id: ID,
    pub employee_id: String,
    pub name: String,
    pub email: Option<String>,
    pub is_active: bool,
    pub current_department: Option<Department>,
    pub department_history: Vec<DepartmentAssignment>,
}

#[derive(InputObject)]
pub struct CreateEmployeeInput {
    pub employee_id: String,
    pub name: String,
    pub email: Option<String>,
    pub department_id: Option<ID>,
}

pub struct EmployeeQuery;

#[Object]
impl EmployeeQuery {
    async fn employees(&self, ctx: &Context<'_>) -> Result<Vec<Employee>> {
        let service = ctx.data::<EmployeeService>()?;
        service.get_all_employees().await
    }
    
    async fn employee(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Employee>> {
        let service = ctx.data::<EmployeeService>()?;
        service.get_employee_by_id(id.parse()?).await
    }
}
```

#### データベース拡張

```sql
-- 部署配属履歴
CREATE TABLE department_assignments (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    department_id INTEGER NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    is_primary BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (employee_id) REFERENCES employees (id),
    FOREIGN KEY (department_id) REFERENCES departments (id)
);
```

#### 成果物(TASK-010)

- 完全な組織管理API
- 社員管理API
- 異動履歴管理機能
- AD同期基盤

---

### TASK-011: 文書管理API

- **説明**: 文書のCRUD・検索API
- **優先度**: High
- **見積工数**: 24h
- **状態**: 未着手
- **依存関係**: TASK-009

#### 実装内容(TASK-011)

1. 文書CRUD API
2. 複合条件検索API
3. 機密レベル制御
4. 検索結果ページング

#### GraphQL スキーマ

```graphql
type Document {
  id: ID!
  number: String!
  title: String!
  documentType: DocumentType!
  businessNumber: String
  creator: Employee!
  createdDate: Date!
  confidentiality: Confidentiality!
  notes: String
  networkPath: String
  fileExists: Boolean
  lastChecked: DateTime
}

type Confidentiality {
  internalExternal: InternalExternal!
  importanceClass: ImportanceClass!
  personalInfo: PersonalInfo!
}

input DocumentSearchInput {
  title: String
  businessNumber: String
  documentTypeId: ID
  creatorId: ID
  createdDateFrom: Date
  createdDateTo: Date
  confidentiality: ConfidentialityFilter
  pagination: PaginationInput
}

type DocumentSearchResult {
  documents: [Document!]!
  totalCount: Int!
  hasNextPage: Boolean!
}
```

#### 実装例(TASK-011)

```rust
// src/services/document_service.rs
pub struct DocumentService {
    repository: Box<dyn DocumentRepository>,
    number_service: Box<dyn NumberGenerationService>,
    path_service: Box<dyn PathGenerationService>,
}

impl DocumentService {
    pub async fn create_document(
        &self,
        input: CreateDocumentInput,
        user_permissions: &UserPermissions,
    ) -> Result<Document, DocumentError> {
        // 権限チェック
        self.validate_create_permission(&input, user_permissions)?;
        
        // 文書番号生成
        let number = if let Some(manual_number) = input.number {
            self.validate_manual_number(&manual_number)?;
            manual_number
        } else {
            self.number_service.generate_number(
                &input.department_code,
                &input.document_type_code,
                input.created_date,
            )?
        };
        
        // ネットワークパス生成
        let network_path = self.path_service.generate_path(
            &number,
            &input.document_type_code,
            input.created_date,
        )?;
        
        // 文書作成
        let document = NewDocument {
            number,
            title: input.title,
            document_type_id: input.document_type_id,
            business_number: input.business_number,
            created_by: input.created_by,
            created_date: input.created_date,
            internal_external: input.confidentiality.internal_external,
            importance_class: input.confidentiality.importance_class,
            personal_info: input.confidentiality.personal_info,
            notes: input.notes,
            network_path: Some(network_path),
        };
        
        self.repository.create(document).await
    }
    
    pub async fn search_documents(
        &self,
        search_input: DocumentSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<DocumentSearchResult, DocumentError> {
        // 権限に基づく検索条件追加
        let enhanced_filters = self.apply_permission_filters(
            search_input,
            user_permissions
        );
        
        self.repository.search(enhanced_filters).await
    }
}
```

#### 成果物(TASK-011)

- 完全な文書管理API
- 複合条件検索機能
- 機密レベル制御
- ページング対応

---

### TASK-012: ルールベースパス管理

- **説明**: パス生成ルール・動的パス解決機能
- **優先度**: High
- **見積工数**: 18h
- **状態**: 未着手
- **依存関係**: TASK-011

#### 実装内容(TASK-012)

1. パス生成ルールエンジン
2. 動的パス解決
3. サーバー設定管理
4. パス履歴管理

#### データベーステーブル(TASK-012)

```sql
-- パス生成ルール
CREATE TABLE path_generation_rules (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    path_template TEXT NOT NULL, -- "\\server01\docs\{year}\{dept_name}\{doc_type_name}"
    applies_to_departments TEXT,
    applies_to_years TEXT, -- JSON array of year ranges
    server_config_id INTEGER,
    effective_from DATE NOT NULL,
    effective_to DATE,
    priority INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- サーバー設定
CREATE TABLE server_configs (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    server_name TEXT NOT NULL,
    base_path TEXT NOT NULL,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### 実装例(TASK-012)

```rust
// src/services/path_generation_service.rs
pub struct PathGenerationService {
    rules: Vec<PathGenerationRule>,
    servers: Vec<ServerConfig>,
}

impl PathGenerationService {
    pub fn generate_path(
        &self,
        document_number: &str,
        doc_type_code: &str,
        created_date: NaiveDate,
        department_code: &str,
    ) -> Result<String, PathGenerationError> {
        let rule = self.find_applicable_rule(
            department_code,
            created_date.year()
        )?;
        
        let context = PathContext {
            year: created_date.year(),
            department_name: self.get_department_name(department_code)?,
            doc_type_name: self.get_doc_type_name(doc_type_code)?,
            document_number: document_number.to_string(),
        };
        
        self.render_path(&rule.path_template, &context)
    }
}
```

#### 成果物(TASK-012)

- パス生成エンジン
- 動的パス解決機能
- サーバー設定管理
- パス履歴管理

---

### TASK-013: ファイル存在確認

- **説明**: ネットワークドライブアクセス・確認機能
- **優先度**: High
- **見積工数**: 20h
- **状態**: 未着手
- **依存関係**: TASK-012

#### 実装内容(TASK-013)

1. ネットワークドライブアクセス機能
2. ファイル存在確認API
3. 承認書確認機能
4. バッチ処理基盤

#### 実装例(TASK-013)

```rust
// src/services/file_check_service.rs
use std::path::Path;
use tokio::fs;

pub struct FileCheckService {
    path_service: Box<dyn PathGenerationService>,
}

impl FileCheckService {
    pub async fn check_document_existence(
        &self,
        document: &Document,
    ) -> Result<FileExistenceResult, FileCheckError> {
        let network_path = &document.network_path
            .as_ref()
            .ok_or(FileCheckError::NoPathGenerated)?;
        
        // メインフォルダ確認
        let folder_exists = self.check_folder_exists(network_path).await?;
        
        // 承認書確認（必要な場合）
        let approval_exists = if document.requires_approval() {
            let approval_path = format!("{}/{}-審査承認.pdf", 
                network_path, document.number);
            self.check_file_exists(&approval_path).await?
        } else {
            Some(true) // 不要な場合は true
        };
        
        Ok(FileExistenceResult {
            folder_exists,
            approval_exists,
            checked_at: Utc::now(),
        })
    }
    
    /// 🚀 高性能並行処理バッチファイル確認
    pub async fn batch_check_files_concurrent(
        &self,
        documents: Vec<Document>,
        max_concurrent: usize,
    ) -> Result<Vec<FileCheckResult>, FileCheckError> {
        use tokio::sync::Semaphore;
        use futures::stream::{StreamExt, FuturesUnordered};
        
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut futures = FuturesUnordered::new();
        
        for document in documents {
            let semaphore = semaphore.clone();
            let service = self.clone();
            
            let future = async move {
                let _permit = semaphore.acquire().await.map_err(|_| {
                    FileCheckError::ConcurrencyError("Failed to acquire semaphore".to_string())
                })?;
                
                let start = std::time::Instant::now();
                let result = service.check_document_existence(&document).await;
                let duration = start.elapsed();
                
                Ok::<FileCheckResult, FileCheckError>(FileCheckResult {
                    document_id: document.id,
                    document_number: document.number.clone(),
                    result,
                    duration,
                    checked_at: chrono::Utc::now(),
                })
            };
            
            futures.push(future);
        }
        
        let mut results = Vec::new();
        while let Some(result) = futures.next().await {
            results.push(result?);
        }
        
        Ok(results)
    }
    
    /// 📊 統計情報付きバッチ処理
    pub async fn batch_check_with_stats(
        &self,
        documents: Vec<Document>,
    ) -> Result<BatchCheckStats, FileCheckError> {
        let total_count = documents.len();
        let start_time = std::time::Instant::now();
        
        let results = self.batch_check_files_concurrent(documents, 10).await?;
        
        let mut stats = BatchCheckStats {
            total_documents: total_count,
            processed_documents: results.len(),
            missing_folders: 0,
            missing_approvals: 0,
            errors: 0,
            total_duration: start_time.elapsed(),
            average_duration: Duration::from_millis(0),
        };
        
        let mut total_duration_ms = 0u128;
        
        for result in &results {
            total_duration_ms += result.duration.as_millis();
            
            match &result.result {
                Ok(existence_result) => {
                    if !existence_result.folder_exists {
                        stats.missing_folders += 1;
                    }
                    if let Some(false) = existence_result.approval_exists {
                        stats.missing_approvals += 1;
                    }
                }
                Err(_) => stats.errors += 1,
            }
        }
        
        if !results.is_empty() {
            stats.average_duration = Duration::from_millis(
                (total_duration_ms / results.len() as u128) as u64
            );
        }
        
        Ok(stats)
    }
    
    async fn check_folder_exists(&self, path: &str) -> Result<bool, FileCheckError> {
        match fs::metadata(path).await {
            Ok(metadata) => Ok(metadata.is_dir()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(FileCheckError::AccessError(e)),
        }
    }
}
```

#### データベーステーブル(TASK-013)

```sql
-- ファイル確認ログ
CREATE TABLE file_check_logs (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    check_type TEXT NOT NULL, -- 'folder', 'approval'
    exists BOOLEAN NOT NULL,
    error_message TEXT,
    checked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents (id)
);

-- 確認除外設定
CREATE TABLE file_check_exclusions (
    id INTEGER PRIMARY KEY,
    cutoff_date DATE NOT NULL,
    department_code TEXT,
    reason TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### 高性能データ構造

```rust
// src/models/file_check.rs
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCheckResult {
    pub document_id: i32,
    pub document_number: String,
    pub result: Result<FileExistenceResult, FileCheckError>,
    pub duration: Duration,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCheckStats {
    pub total_documents: usize,
    pub processed_documents: usize,
    pub missing_folders: usize,
    pub missing_approvals: usize,
    pub errors: usize,
    pub total_duration: Duration,
    pub average_duration: Duration,
}

#[derive(Debug, thiserror::Error)]
pub enum FileCheckError {
    #[error("Network path not generated for document")]
    NoPathGenerated,
    #[error("File access error: {0}")]
    AccessError(#[from] std::io::Error),
    #[error("Timeout after {seconds} seconds")]
    Timeout { seconds: u64 },
    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
```

#### 成果物(TASK-013)

- **ファイル存在確認機能**
- **承認書確認機能**  
- **高性能並行処理バッチシステム**
- **統計情報・監視機能**
- **エラーログ・リトライ機能**

---

### TASK-014: データベース最適化

- **説明**: インデックス・クエリ最適化
- **優先度**: Medium
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-011

#### 実装内容(TASK-014)

1. 検索用インデックス設計
2. クエリ性能最適化
3. 全文検索対応
4. 統計情報管理

#### インデックス設計

```sql
-- 文書検索用インデックス
CREATE INDEX idx_documents_search ON documents(title, business_number, created_date);
CREATE INDEX idx_documents_type_date ON documents(document_type_id, created_date);
CREATE INDEX idx_documents_creator ON documents(created_by, is_active);

-- 部署履歴用インデックス
CREATE INDEX idx_dept_assignments_employee ON department_assignments(employee_id, start_date);
CREATE INDEX idx_dept_assignments_period ON department_assignments(start_date, end_date);
```

#### 成果物(TASK-014)

- 最適化されたインデックス
- 高速検索クエリ
- 性能監視機能

---

### TASK-015: エラーハンドリング

- **説明**: 統一エラー処理・レスポンス
- **優先度**: Medium
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-013

#### 実装内容(TASK-015)

1. 統一エラー型定義
2. エラーコード体系
3. 多言語エラーメッセージ
4. エラーログ統合

#### 成果物(TASK-015)

- 統一エラー処理システム
- エラーコード体系
- エラーログ機能

---

### TASK-016: バリデーション

- **説明**: 入力検証・データ整合性
- **優先度**: Medium
- **見積工数**: 10h
- **状態**: 未着手
- **依存関係**: TASK-011

#### 実装内容(TASK-016)

1. 入力値バリデーション
2. ビジネスルール検証
3. データ整合性チェック
4. SQL注入対策

#### 成果物(TASK-016)

- 完全なバリデーション機能
- セキュリティ対策
- データ整合性保証

## フェーズ完了基準

### 必須条件

- [ ] 文書番号が正しいルールで生成される
- [ ] 文書のCRUD操作が完全に動作する
- [ ] 複合条件検索が期待通りの結果を返す
- [ ] ネットワークパスが正しく生成される
- [ ] ファイル存在確認が動作する
- [ ] エラーハンドリングが適切に機能する

### 検証方法

```bash
# 文書作成テスト
curl -X POST /graphql -d '{"query":"mutation{createDocument(...)}"}'

# 検索テスト
curl -X POST /graphql -d '{"query":"query{searchDocuments(...)}"}'

# ファイル確認テスト
curl -X POST /api/files/check -d '{"documentId": 1}'
```

## 次フェーズへの引き継ぎ事項

- 文書管理API完成
- 認証・認可機能実装準備
- UI開発準備完了
- 通知システム実装準備

## リスク・課題

- **ネットワークアクセス**: ファイルサーバーへの接続安定性
- **性能**: 大量データでの検索性能
- **パス生成**: 複雑なルールの保守性

## 対応策

- 非同期処理・タイムアウト設定
- インデックス最適化・ページング
- ルールエンジンの単純化・ドキュメント化
