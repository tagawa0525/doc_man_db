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
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_type_id) REFERENCES document_types (id),
    FOREIGN KEY (created_by) REFERENCES employees (id)
);
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

// Repository trait
#[async_trait::async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn create(&self, document: CreateDocumentRequest) -> Result<Document, sqlx::Error>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Document>, sqlx::Error>;
    async fn get_by_number(&self, number: &str) -> Result<Option<Document>, sqlx::Error>;
    async fn search(&self, filters: DocumentSearchFilters) -> Result<(Vec<Document>, i64), sqlx::Error>;
    async fn update(&self, id: i32, document: UpdateDocumentRequest) -> Result<Document, sqlx::Error>;
    async fn delete(&self, id: i32) -> Result<(), sqlx::Error>;
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
