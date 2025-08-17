# Phase 1: 環境構築・基盤実装 (Week 1-2) - SQLx版

## フェーズ概要

- **期間**: Week 1-2 (2週間)
- **目標**: プロジェクト基盤とデータベース構造の構築
- **成果物**: 動作する基本Webサーバー、データベーススキーマ、基本モデル定義

## タスク一覧

### TASK-001: プロジェクト初期化

- **説明**: Cargoプロジェクト作成・ディレクトリ構成
- **優先度**: High
- **見積工数**: 2h
- **状態**: 未着手
- **依存関係**: -

#### 実装内容

1. `Cargo.toml` の依存関係追加
2. ディレクトリ構造作成 (`src/`, `tests/`, `migrations/`)
3. 基本設定ファイル作成 (`.env.development`, `rustfmt.toml`)
4. `.gitignore` 設定

#### 成果物

- 完全な `Cargo.toml` 設定
- プロジェクト ディレクトリ構造
- 基本設定ファイル

---

### TASK-002: 依存関係設定

- **説明**: Cargo.toml設定・基本ライブラリ追加
- **優先度**: High
- **見積工数**: 4h
- **状態**: 未着手
- **依存関係**: TASK-001

#### 実装内容

```toml
[dependencies]
# Web Framework
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["fs", "cors"] }

# Database - SQLx
sqlx = { version = "0.7", features = ["sqlite", "chrono", "runtime-tokio-rustls", "migrate"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Time
chrono = { version = "0.4", features = ["serde"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Configuration
config = "0.14"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# GraphQL
async-graphql = "7.0"
async-graphql-axum = "7.0"

# Authentication
jsonwebtoken = "9.0"

# File operations
walkdir = "2.0"

[dev-dependencies]
tokio-test = "0.4"
assert_matches = "1.5"
mockall = "0.12"
proptest = "1.0"
```

#### 成果物

- 設定済み `Cargo.toml`
- 基本ライブラリのビルド確認

---

### TASK-003: データベーススキーマ作成

- **説明**: SQLxセットアップ・マイグレーション
- **優先度**: High
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-002

#### 実装内容

1. SQLx CLI インストール・設定
2. 基本テーブル作成マイグレーション
3. インデックス設定
4. サンプルデータ投入スクリプト

#### SQLxコマンド

```bash
# SQLx CLI インストール
cargo install sqlx-cli --no-default-features --features sqlite

# データベース作成
sqlx database create --database-url sqlite://./data/dev.db

# マイグレーション作成
sqlx migrate add create_departments
sqlx migrate add create_employees
sqlx migrate add create_document_types
sqlx migrate add create_documents

# マイグレーション実行
sqlx migrate run --database-url sqlite://./data/dev.db
```

#### マイグレーションファイル例

```sql
-- migrations/001_create_departments.sql
CREATE TABLE departments (
    id INTEGER PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    effective_from DATE NOT NULL,
    effective_to DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- migrations/002_create_employees.sql
CREATE TABLE employees (
    id INTEGER PRIMARY KEY,
    employee_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    email TEXT,
    ad_username TEXT,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- migrations/003_create_document_types.sql
CREATE TABLE document_types (
    id INTEGER PRIMARY KEY,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    requires_approval BOOLEAN DEFAULT 0,
    department_code TEXT,
    effective_from DATE NOT NULL,
    effective_to DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- migrations/004_create_documents.sql
CREATE TABLE documents (
    id INTEGER PRIMARY KEY,
    number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    document_type_id INTEGER NOT NULL,
    business_number TEXT,
    created_by INTEGER NOT NULL,
    created_date DATE NOT NULL,
    internal_external TEXT CHECK(internal_external IN ('internal', 'external')),
    importance_class TEXT CHECK(importance_class IN ('class1', 'class2')),
    personal_info TEXT CHECK(personal_info IN ('none', 'present')),
    notes TEXT,
    network_path TEXT,
    network_path_status TEXT DEFAULT 'pending' CHECK(network_path_status IN ('pending', 'generated', 'failed')),
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_type_id) REFERENCES document_types (id),
    FOREIGN KEY (created_by) REFERENCES employees (id)
);

-- 検索性能用インデックス
CREATE INDEX idx_documents_search ON documents(title, business_number, created_date);
CREATE INDEX idx_documents_type_date ON documents(document_type_id, created_date);
CREATE INDEX idx_documents_creator ON documents(created_by, is_active);
CREATE INDEX idx_documents_number ON documents(number);

-- 全文検索用仮想テーブル（SQLiteのFTS5）
CREATE VIRTUAL TABLE documents_fts USING fts5(
    title, business_number, notes,
    content='documents',
    content_rowid='id'
);

-- FTS5トリガー（自動更新）
CREATE TRIGGER documents_fts_insert AFTER INSERT ON documents
BEGIN
    INSERT INTO documents_fts(rowid, title, business_number, notes)
    VALUES (new.id, new.title, new.business_number, new.notes);
END;

CREATE TRIGGER documents_fts_update AFTER UPDATE ON documents
BEGIN
    UPDATE documents_fts SET 
        title = new.title,
        business_number = new.business_number,
        notes = new.notes
    WHERE rowid = new.id;
END;

CREATE TRIGGER documents_fts_delete AFTER DELETE ON documents
BEGIN
    DELETE FROM documents_fts WHERE rowid = old.id;
END;
```

#### 成果物

- 完全なデータベーススキーマ
- SQLxマイグレーションファイル
- 初期データ投入スクリプト

---

### TASK-004: 基本モデル実装

- **説明**: 主要テーブルのstruct定義
- **優先度**: High
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-003

#### 実装内容

1. SQLx model 定義
2. Serde シリアライゼーション設定
3. バリデーション機能
4. 基本的なクエリ関数

#### ファイル構成

```text
src/models/
├── mod.rs
├── department.rs       # 部署モデル
├── employee.rs         # 社員モデル
├── document_type.rs    # 文書種別モデル
├── document.rs         # 文書モデル
└── enums.rs           # 共通Enum定義
```

#### 実装例

```rust
// src/models/document.rs
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub id: i32,
    pub number: String,
    pub title: String,
    pub document_type_id: i32,
    pub business_number: Option<String>,
    pub created_by: i32,
    pub created_date: NaiveDate,
    pub internal_external: Option<String>,
    pub importance_class: Option<String>,
    pub personal_info: Option<String>,
    pub notes: Option<String>,
    pub network_path: Option<String>,
    pub network_path_status: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct CreateDocumentRequest {
    pub number: Option<String>, // None = 自動生成
    pub title: String,
    pub document_type_id: i32,
    pub business_number: Option<String>,
    pub created_by: i32,
    pub created_date: NaiveDate,
    pub internal_external: Option<String>,
    pub importance_class: Option<String>,
    pub personal_info: Option<String>,
    pub notes: Option<String>,
}

// 統一エラー型定義
#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Permission denied")]
    Unauthorized,
    #[error("Resource not found: {id}")]
    NotFound { id: String },
    #[error("Network path generation failed: {0}")]
    PathGeneration(String),
    #[error("Duplicate resource: {0}")]
    Duplicate(String),
}

// Repository trait (セキュリティ強化版)
#[async_trait::async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn create(&self, document: CreateDocumentRequest) -> Result<Document, DocumentError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Document>, DocumentError>;
    async fn get_by_id_with_permissions(
        &self, 
        id: i32, 
        user_permissions: &UserPermissions
    ) -> Result<Option<Document>, DocumentError>;
    async fn get_by_number(&self, number: &str) -> Result<Option<Document>, DocumentError>;
    async fn search_with_permissions(
        &self, 
        filters: DocumentSearchFilters,
        user_permissions: &UserPermissions
    ) -> Result<(Vec<Document>, i64), DocumentError>;
    async fn update(&self, id: i32, document: UpdateDocumentRequest) -> Result<Document, DocumentError>;
    async fn delete(&self, id: i32) -> Result<(), DocumentError>;
}
```

#### データベース接続設定

```rust
// src/database/mod.rs
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
```

#### 成果物

- 全モデル struct 定義
- Repository trait 定義
- 基本クエリ関数
- データベース接続機能

---

### TASK-005: Webサーバー基盤

- **説明**: Axumサーバー・ミドルウェア設定
- **優先度**: High
- **見積工数**: 6h
- **状態**: 未着手
- **依存関係**: TASK-002

#### 実装内容

1. Axum サーバー設定
2. 基本ミドルウェア (CORS, Logging)
3. ヘルスチェック エンドポイント
4. 基本エラーハンドリング

#### 実装例

```rust
// src/main.rs
use axum::{routing::get, Router, Extension};
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use sqlx::SqlitePool;

mod database;
mod models;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    // データベース接続
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./data/dev.db".to_string());
    
    let pool = database::create_pool(&database_url).await?;
    database::run_migrations(&pool).await?;

    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .layer(Extension(pool))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await?;

    tracing::info!("Server starting on http://127.0.0.1:8080");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

#### 成果物

- 動作するWebサーバー
- データベース接続統合
- 基本ミドルウェア
- ヘルスチェック機能

---

### TASK-006: 設定管理

- **説明**: 環境変数・設定ファイル管理
- **優先度**: Medium
- **見積工数**: 4h
- **状態**: 未着手
- **依存関係**: TASK-005

#### 実装内容

```rust
// src/config/mod.rs
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub file_server: FileServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .add_source(config::File::with_name(".env"))
            .build()?;
        
        settings.try_deserialize()
    }
}
```

#### 成果物

- 設定管理システム
- 環境別設定ファイル
- 設定バリデーション

---

### TASK-007: ログ設定

- **説明**: 構造化ログ・ローテーション設定
- **優先度**: Medium
- **見積工数**: 3h
- **状態**: 未着手
- **依存関係**: TASK-005

#### 成果物

- ログ管理システム
- ログファイル設定
- 運用ログ仕様

---

### TASK-008: 開発環境整備

- **説明**: デバッグ・ホットリロード設定
- **優先度**: Low
- **見積工数**: 2h
- **状態**: 未着手
- **依存関係**: TASK-005

#### 成果物

- 開発環境設定
- デバッグ設定
- 開発用スクリプト

## フェーズ完了基準

### 必須条件

- [ ] Axum サーバーが正常起動する
- [ ] SQLiteデータベース接続が確立する
- [ ] 全マイグレーションが正常実行される
- [ ] ヘルスチェック API が応答する
- [ ] 基本モデルでCRUD操作が可能
- [ ] ログが正常に出力される

### 検証方法

```bash
# サーバー起動確認
cargo run

# ヘルスチェック確認
curl http://localhost:8080/health

# データベース接続確認
sqlx migrate run --database-url sqlite://./data/dev.db

# テスト実行
cargo test
```

## 次フェーズへの引き継ぎ事項

- データベーススキーマ完成
- 基本APIエンドポイント作成準備完了
- 文書番号生成ロジック実装準備
- GraphQL スキーマ設計準備

## リスク・課題

- **SQLite制限**: 大量データ処理時の性能課題
- **Windows環境**: 開発環境とのパス差異
- **マイグレーション**: スキーマ変更時の下位互換性

## 対応策

- SQLite → SQL Server 移行計画
- パス処理の抽象化  
- マイグレーション戦略文書化

---

### TASK-008: セキュリティ・性能強化

- **説明**: SQL注入対策・エラーハンドリング統一・観測可能性向上
- **優先度**: High
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-004

#### 実装内容

1. **SQL注入対策の徹底**
2. **統一エラーハンドリング**
3. **設定管理外部化**
4. **ログ・メトリクス強化**

#### SQL注入対策実装

```rust
// src/repositories/document_repository.rs - セキュア実装例
use sqlx::{QueryBuilder, Sqlite};

impl DocumentRepositoryImpl {
    // ✅ 安全な検索実装
    pub async fn search_secure(
        &self,
        filters: &DocumentSearchFilters,
        user_permissions: &UserPermissions,
    ) -> Result<(Vec<Document>, i64), DocumentError> {
        let mut query = QueryBuilder::<Sqlite>::new(
            "SELECT d.*, dt.name as document_type_name FROM documents d 
             JOIN document_types dt ON d.document_type_id = dt.id WHERE 1=1"
        );
        
        // 権限ベースフィルタリング
        if !user_permissions.can_view_confidential {
            query.push(" AND d.importance_class != ");
            query.push_bind("class1");
        }
        
        // 部署制限
        if !user_permissions.accessible_departments.is_empty() {
            query.push(" AND d.created_by IN (
                SELECT e.id FROM employees e 
                JOIN department_assignments da ON e.id = da.employee_id 
                WHERE da.department_id IN (");
            
            let mut separated = query.separated(", ");
            for dept_id in &user_permissions.accessible_departments {
                separated.push_bind(dept_id);
            }
            query.push("))");
        }
        
        // 動的フィルター（すべてパラメータ化）
        if let Some(title) = &filters.title {
            query.push(" AND d.title LIKE ");
            query.push_bind(format!("%{}%", title));
        }
        
        if let Some(business_number) = &filters.business_number {
            query.push(" AND d.business_number = ");
            query.push_bind(business_number);
        }
        
        if let Some(date_from) = filters.created_date_from {
            query.push(" AND d.created_date >= ");
            query.push_bind(date_from);
        }
        
        if let Some(date_to) = filters.created_date_to {
            query.push(" AND d.created_date <= ");
            query.push_bind(date_to);
        }
        
        // ページング
        query.push(" ORDER BY d.created_date DESC LIMIT ");
        query.push_bind(filters.pagination.limit);
        query.push(" OFFSET ");
        query.push_bind(filters.pagination.offset);
        
        let documents = query
            .build_query_as::<Document>()
            .fetch_all(&self.pool)
            .await
            .map_err(DocumentError::Database)?;
        
        // 件数取得（同じフィルター条件）
        let count = self.count_with_filters(filters, user_permissions).await?;
        
        Ok((documents, count))
    }
    
    // ✅ 全文検索実装（FTS5使用）
    pub async fn search_fulltext(
        &self,
        query_text: &str,
        user_permissions: &UserPermissions,
    ) -> Result<Vec<Document>, DocumentError> {
        let documents = sqlx::query_as::<_, Document>(
            r#"
            SELECT d.* FROM documents d
            JOIN documents_fts fts ON d.id = fts.rowid
            WHERE documents_fts MATCH $1
            AND ($2 OR d.importance_class != 'class1')
            ORDER BY fts.rank
            LIMIT 100
            "#
        )
        .bind(query_text)
        .bind(user_permissions.can_view_confidential)
        .fetch_all(&self.pool)
        .await
        .map_err(DocumentError::Database)?;
        
        Ok(documents)
    }
}
```

#### 設定管理外部化

```rust
// src/config/settings.rs
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
    pub security: SecuritySettings,
    pub logging: LoggingSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub statement_timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecuritySettings {
    pub jwt_secret: String,
    pub jwt_expiration_hours: u32,
    pub bcrypt_cost: u32,
    pub max_login_attempts: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String, // "json" or "pretty"
    pub enable_performance_logs: bool,
}

impl Settings {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut config = Config::builder();
        
        // デフォルト設定
        config = config.add_source(File::with_name("config/default"));
        
        // 環境別設定
        let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());
        config = config.add_source(File::with_name(&format!("config/{}", env)).required(false));
        
        // 環境変数（DOC_MAN_DB プレフィックス）
        config = config.add_source(Environment::with_prefix("DOC_MAN_DB").separator("__"));
        
        config.build()?.try_deserialize()
    }
}
```

#### 観測可能性向上

```rust
// src/observability/mod.rs
use tracing::{instrument, info, warn, error};
use std::time::Instant;

// サービス層での統一ログ実装
impl DocumentService {
    #[instrument(
        skip(self, input),
        fields(
            document_title = %input.title,
            document_type_id = input.document_type_id,
            user_id = user_permissions.user_id
        )
    )]
    pub async fn create_document(
        &self,
        input: CreateDocumentRequest,
        user_permissions: &UserPermissions,
    ) -> Result<Document, DocumentError> {
        let start = Instant::now();
        
        info!("Starting document creation");
        
        // 権限チェック
        if !user_permissions.can_create_documents {
            warn!("Document creation denied - insufficient permissions");
            return Err(DocumentError::Unauthorized);
        }
        
        // ビジネスロジック実行
        let result = self.repository.create(input).await;
        
        match &result {
            Ok(document) => {
                info!(
                    document_id = document.id,
                    document_number = %document.number,
                    duration_ms = start.elapsed().as_millis(),
                    "Document created successfully"
                );
            }
            Err(e) => {
                error!(
                    error = %e,
                    duration_ms = start.elapsed().as_millis(),
                    "Document creation failed"
                );
            }
        }
        
        result
    }
}

// メトリクス収集
pub struct Metrics {
    pub document_creation_counter: prometheus::Counter,
    pub document_creation_duration: prometheus::Histogram,
    pub search_query_duration: prometheus::Histogram,
    pub active_connections: prometheus::Gauge,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            document_creation_counter: prometheus::Counter::new(
                "documents_created_total",
                "Total number of documents created"
            ).unwrap(),
            document_creation_duration: prometheus::Histogram::new(
                prometheus::HistogramOpts::new(
                    "document_creation_duration_seconds",
                    "Time spent creating documents"
                )
            ).unwrap(),
            search_query_duration: prometheus::Histogram::new(
                prometheus::HistogramOpts::new(
                    "search_query_duration_seconds",
                    "Time spent on search queries"
                )
            ).unwrap(),
            active_connections: prometheus::Gauge::new(
                "database_connections_active",
                "Number of active database connections"
            ).unwrap(),
        }
    }
}
```

#### 成果物

- **完全なSQL注入対策**
- **統一エラーハンドリングシステム**
- **外部化された設定管理**
- **包括的ログ・メトリクス**
- **セキュアなデータアクセス層**
