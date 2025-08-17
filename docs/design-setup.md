# 開発環境セットアップガイド

## 1. 前提条件

### 1.1 必要なソフトウェア

| ソフトウェア       | バージョン | 用途                   | インストール先         |
| ------------------ | ---------- | ---------------------- | ---------------------- |
| **Windows Server** | 2019+      | 本番運用環境           | 本番サーバー           |
| **Rust**           | 1.75+      | メイン開発言語         | 開発者PC               |
| **Node.js**        | 18.0+      | フロントエンド開発     | 開発者PC               |
| **Git**            | 2.40+      | バージョン管理         | 開発者PC               |
| **VS Code**        | latest     | 統合開発環境           | 開発者PC               |
| **SQLite**         | 3.40+      | 開発用データベース     | 開発者PC               |
| **SQL Server**     | 2019+      | 本番用データベース     | 本番サーバー           |
| **IIS**            | 10.0+      | Webサーバー            | 本番サーバー           |
| **PowerShell**     | 5.1+       | デプロイスクリプト実行 | 開発者PC・本番サーバー |

### 1.2 アクセス権限

| 対象                     | 必要権限             | 用途                 |
| ------------------------ | -------------------- | -------------------- |
| **Active Directory**     | ユーザー情報読み取り | AD連携・認証         |
| **ネットワークドライブ** | 共有フォルダアクセス | ファイル存在確認     |
| **本番サーバー**         | 管理者権限           | アプリケーション配置 |
| **SQL Server**           | db_owner権限         | データベース操作     |
| **ファイルサーバー**     | 読み取り専用         | 文書ファイル参照     |

## 2. 開発環境セットアップ

### 2.1 Rust開発環境

#### 2.1.1 Rustインストール

```powershell
# Rustupのインストール（Windows）
# https://rustup.rs/ からrustup-init.exeをダウンロード
.\rustup-init.exe

# パスを更新
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# インストール確認
rustc --version
cargo --version

# 推奨コンポーネントのインストール
rustup component add rustfmt clippy
rustup target add x86_64-pc-windows-msvc
```

#### 2.1.2 追加ツールインストール

```powershell
# SQLxクライアント（データベースマイグレーション用）
cargo install sqlx-cli --no-default-features --features sqlite,postgres

# cargo-watch（ホットリロード用）
cargo install cargo-watch

# cargo-audit（セキュリティ監査用）
cargo install cargo-audit

# cargo-tarpaulin（カバレッジ測定用）
cargo install cargo-tarpaulin
```

### 2.2 Node.js開発環境

#### 2.2.1 Node.jsインストール

```powershell
# Node.js LTSバージョンをインストール
# https://nodejs.org/ からLTSバージョンをダウンロード

# インストール確認
node --version
npm --version

# yarnのインストール（推奨パッケージマネージャー）
npm install -g yarn

# yarn確認
yarn --version
```

#### 2.2.2 フロントエンド開発ツール

```powershell
# SvelteKit開発用グローバルツール
npm install -g @sveltejs/kit
npm install -g vite
npm install -g playwright  # E2Eテスト用

# Tauriツール
npm install -g @tauri-apps/cli
```

### 2.3 データベース環境

#### 2.3.1 SQLite設定（開発用）

```powershell
# SQLiteインストール（Chocolatey使用）
# Chocolateyがない場合: https://chocolatey.org/install
choco install sqlite

# または手動インストール
# https://sqlite.org/download.html からprecompiled binariesをダウンロード

# SQLite確認
sqlite3 --version

# 開発用データベース作成
mkdir data
sqlx database create --database-url sqlite://./data/dev.db
```

#### 2.3.2 SQL Server設定（本番用）

```sql
-- SQL Server Management Studio (SSMS)で実行
-- データベース作成
CREATE DATABASE doc_man_db
  COLLATE Japanese_CI_AS;

-- 専用ユーザー作成
CREATE LOGIN doc_man_user WITH PASSWORD = 'SecurePassword123!';
USE doc_man_db;
CREATE USER doc_man_user FOR LOGIN doc_man_user;
ALTER ROLE db_owner ADD MEMBER doc_man_user;

-- 接続確認
-- 接続文字列: Server=localhost;Database=doc_man_db;User Id=doc_man_user;Password=SecurePassword123!;
```

### 2.4 VS Code設定

#### 2.4.1 必須拡張機能

```json
// .vscode/extensions.json
{
  "recommendations": [
    "rust-lang.rust-analyzer",           // Rust言語サポート
    "tamasfe.even-better-toml",          // TOML設定ファイル
    "serayuzgur.crates",                 // Cargoクレート管理
    "vadimcn.vscode-lldb",               // デバッガ
    "svelte.svelte-vscode",              // Svelte言語サポート
    "bradlc.vscode-tailwindcss",         // Tailwind CSS
    "ms-vscode.vscode-typescript-next",  // TypeScript
    "esbenp.prettier-vscode",            // フォーマッター
    "ms-vscode.powershell",              // PowerShell
    "ms-mssql.mssql"                     // SQL Server
  ]
}
```

#### 2.4.2 ワークスペース設定

```json
// .vscode/settings.json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.procMacro.enable": true,
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer",
  "[svelte]": {
    "editor.defaultFormatter": "svelte.svelte-vscode"
  },
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "files.associations": {
    "*.toml": "toml",
    "Dockerfile*": "dockerfile"
  },
  "sqltools.connections": [
    {
      "name": "SQLite (Development)",
      "driver": "SQLite",
      "database": "./data/dev.db"
    }
  ]
}
```

#### 2.4.3 デバッグ設定

```json
// .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Rust App",
      "cargo": {
        "args": ["build", "--bin=doc_man_db"],
        "filter": {
          "name": "doc_man_db",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}",
      "env": {
        "DATABASE_URL": "sqlite://./data/dev.db",
        "RUST_LOG": "debug"
      }
    },
    {
      "type": "node",
      "request": "launch",
      "name": "Debug SvelteKit",
      "program": "${workspaceFolder}/ui/node_modules/@sveltejs/kit/src/exports/vite/dev/index.js",
      "args": ["--port", "3000"],
      "cwd": "${workspaceFolder}/ui",
      "envFile": "${workspaceFolder}/ui/.env.local"
    }
  ]
}
```

### 2.5 タスク自動化設定

#### 2.5.1 VS Codeタスク

```json
// .vscode/tasks.json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "cargo-check",
      "type": "cargo",
      "command": "check",
      "group": "build",
      "presentation": {
        "clear": true
      }
    },
    {
      "label": "cargo-test",
      "type": "cargo",
      "command": "test",
      "group": "test",
      "presentation": {
        "clear": true
      }
    },
    {
      "label": "cargo-run",
      "type": "cargo",
      "command": "run",
      "group": "build",
      "presentation": {
        "clear": true
      }
    },
    {
      "label": "sqlx-migrate",
      "type": "shell",
      "command": "sqlx migrate run",
      "group": "build",
      "presentation": {
        "clear": true
      }
    },
    {
      "label": "ui-dev",
      "type": "shell",
      "command": "yarn dev",
      "options": {
        "cwd": "${workspaceFolder}/ui"
      },
      "group": "build",
      "presentation": {
        "clear": true
      }
    }
  ]
}
```

## 3. プロジェクト初期化

### 3.1 リポジトリのクローン

```powershell
# プロジェクトのクローン
git clone https://github.com/your-org/doc_man_db.git
cd doc_man_db

# ブランチ確認
git branch -a
git checkout main
```

### 3.2 依存関係のインストール

```powershell
# Rust依存関係の確認・ビルド
cargo check
cargo build

# フロントエンド依存関係のインストール
cd ui
yarn install
cd ..

# データベースマイグレーション実行
sqlx migrate run --database-url sqlite://./data/dev.db
```

### 3.3 環境変数設定

#### 3.3.1 開発環境設定ファイル

```env
# .env.development
DATABASE_URL=sqlite://./data/dev.db
RUST_LOG=debug
BIND_ADDRESS=127.0.0.1:8080
CORS_ORIGINS=http://localhost:3000,http://localhost:8080

# Active Directory設定（開発環境では無効化）
AD_ENABLED=false
AD_DOMAIN=corp.local
AD_SERVER=ldap://dc.corp.local:389

# ファイルサーバー設定
FILE_SERVER_ENABLED=false
FILE_SERVER_BASE=\\\\fileserver\\documents

# 通知設定（開発環境では無効化）
EMAIL_ENABLED=false
TEAMS_ENABLED=false

# セキュリティ設定
JWT_SECRET=development_secret_change_in_production
SESSION_TIMEOUT=3600
```

#### 3.3.2 本番環境設定ファイル

```env
# .env.production
DATABASE_URL=mssql://doc_man_user:SecurePassword123!@localhost/doc_man_db
RUST_LOG=info
BIND_ADDRESS=0.0.0.0:8080
CORS_ORIGINS=https://docman.corp.local

# Active Directory設定
AD_ENABLED=true
AD_DOMAIN=corp.local
AD_SERVER=ldap://dc.corp.local:389
AD_BIND_USER=CN=service_account,OU=Service Accounts,DC=corp,DC=local
AD_BIND_PASSWORD=ServiceAccountPassword123!

# ファイルサーバー設定
FILE_SERVER_ENABLED=true
FILE_SERVER_BASE=\\\\fileserver\\documents
FILE_SERVER_CHECK_INTERVAL=3600

# 通知設定
EMAIL_ENABLED=true
EMAIL_SMTP_SERVER=mail.corp.local
EMAIL_SMTP_PORT=587
EMAIL_SMTP_USER=docman@corp.local
EMAIL_SMTP_PASSWORD=EmailPassword123!

TEAMS_ENABLED=true
TEAMS_WEBHOOK_URL=https://corp.webhook.office.com/webhookb2/...

# セキュリティ設定
JWT_SECRET=super_secure_production_secret_32chars+
SESSION_TIMEOUT=28800
HTTPS_CERT_PATH=./certs/docman.corp.local.crt
HTTPS_KEY_PATH=./certs/docman.corp.local.key
```

### 3.4 初期データセットアップ

#### 3.4.1 開発用サンプルデータ

```sql
-- data/sample_data.sql
-- 部署データ
INSERT INTO departments (code, name, created_at, updated_at) VALUES
('T', '技術部', datetime('now'), datetime('now')),
('S', '営業部', datetime('now'), datetime('now')),
('A', '管理部', datetime('now'), datetime('now'));

-- 課データ
INSERT INTO sections (department_code, code, name, created_at, updated_at) VALUES
('T', 'A', '第一課', datetime('now'), datetime('now')),
('T', 'B', '第二課', datetime('now'), datetime('now')),
('S', 'A', '営業一課', datetime('now'), datetime('now'));

-- 文書種別データ
INSERT INTO document_types (code, name, requires_approval, created_at, updated_at) VALUES
('A', '報告書', 1, datetime('now'), datetime('now')),
('B', '顧客提出文書', 1, datetime('now'), datetime('now')),
('X', '議事録', 0, datetime('now'), datetime('now')),
('D', '入手文書', 0, datetime('now'), datetime('now'));

-- サンプル社員データ
INSERT INTO employees (employee_number, name, email, ad_username, is_active, created_at, updated_at) VALUES
('12345', '田中太郎', 'tanaka@corp.local', 'tanaka', 1, datetime('now'), datetime('now')),
('12346', '佐藤花子', 'sato@corp.local', 'sato', 1, datetime('now'), datetime('now')),
('12347', '鈴木次郎', 'suzuki@corp.local', 'suzuki', 1, datetime('now'), datetime('now'));
```

#### 3.4.2 サンプルデータ投入

```powershell
# サンプルデータの投入
sqlite3 ./data/dev.db < ./data/sample_data.sql

# データ確認
sqlite3 ./data/dev.db "SELECT * FROM departments;"
sqlite3 ./data/dev.db "SELECT * FROM employees;"
```

## 4. 動作確認

### 4.1 バックエンド動作確認

```powershell
# 開発サーバー起動
cargo run

# 別のターミナルでAPIテスト
# Health Check
Invoke-RestMethod -Uri "http://localhost:8080/api/health" -Method GET

# GraphQL Schema確認
Invoke-RestMethod -Uri "http://localhost:8080/graphql" -Method POST -Body '{"query":"{ __schema { types { name } } }"}' -ContentType "application/json"
```

### 4.2 フロントエンド動作確認

```powershell
# UIディレクトリに移動
cd ui

# 開発サーバー起動
yarn dev

# ブラウザで確認
# http://localhost:3000
```

### 4.3 統合動作確認

```powershell
# 両方のサーバーを起動した状態で
# バックエンド: http://localhost:8080
# フロントエンド: http://localhost:3000

# E2Eテスト実行
cd ui
npx playwright test
```

## 5. 開発ワークフロー

### 5.1 日常的な開発手順

```powershell
# 1. 最新コードの取得
git pull origin main

# 2. フィーチャーブランチ作成
git checkout -b feature/new-feature

# 3. 依存関係の更新確認
cargo update
cd ui && yarn install && cd ..

# 4. 開発作業
# - コード変更
# - テスト追加

# 5. 動作確認
cargo test
cargo clippy
cargo fmt

cd ui
yarn test
yarn lint
cd ..

# 6. コミット・プッシュ
git add .
git commit -m "feat: add new feature"
git push origin feature/new-feature

# 7. プルリクエスト作成
# GitHub上でPRを作成
```

### 5.2 定期的なメンテナンス

```powershell
# 週次実行推奨

# 1. セキュリティ監査
cargo audit

# 2. 依存関係の更新
cargo update
cd ui && yarn upgrade && cd ..

# 3. データベースクリーンアップ
# 開発用データベースのリセット
rm ./data/dev.db
sqlx migrate run --database-url sqlite://./data/dev.db
sqlite3 ./data/dev.db < ./data/sample_data.sql

# 4. ログファイルのクリーンアップ
Remove-Item ./logs/*.log -Force -ErrorAction SilentlyContinue
```

## 6. トラブルシューティング

### 6.1 よくある問題と対処法

#### 6.1.1 Rustコンパイルエラー

```powershell
# 依存関係の問題
cargo clean
cargo build

# ツールチェーンの問題
rustup update
rustup default stable

# SQLxマイグレーションエラー
sqlx migrate revert --database-url sqlite://./data/dev.db
sqlx migrate run --database-url sqlite://./data/dev.db
```

#### 6.1.2 Node.js/Yarn問題

```powershell
# node_modulesの再インストール
cd ui
Remove-Item node_modules -Recurse -Force
yarn install

# Yarnキャッシュクリア
yarn cache clean

# グローバルパッケージの再インストール
npm uninstall -g @sveltejs/kit
npm install -g @sveltejs/kit
```

#### 6.1.3 データベース接続問題

```powershell
# SQLiteファイルの権限確認
icacls ./data/dev.db

# SQL Serverの接続確認
sqlcmd -S localhost -U doc_man_user -P SecurePassword123! -Q "SELECT @@VERSION"

# 接続文字列の確認
echo $env:DATABASE_URL
```

#### 6.1.4 ポート競合問題

```powershell
# ポート使用状況確認
netstat -ano | findstr :8080
netstat -ano | findstr :3000

# プロセス終了
taskkill /PID <PID> /F

# 別ポートでの起動
$env:PORT = "8081"
cargo run

# UI側
cd ui
yarn dev --port 3001
```

## 7. デプロイ準備

### 7.1 本番ビルド

```powershell
# リリースビルド
cargo build --release

# フロントエンドビルド
cd ui
yarn build
cd ..

# Tauriアプリビルド（必要な場合）
cd ui
yarn tauri build
cd ..
```

### 7.2 設定ファイルの準備

```powershell
# 本番用設定ファイルの準備
Copy-Item .env.development .env.production
# .env.productionを本番設定に編集

# 証明書ファイルの配置
mkdir certs
# SSL証明書ファイルをcertsディレクトリに配置
```

### 7.3 デプロイパッケージ作成

```powershell
# デプロイ用ディレクトリ作成
mkdir deploy
Copy-Item target/release/doc_man_db.exe deploy/
Copy-Item -Recurse ui/build deploy/ui
Copy-Item -Recurse migrations deploy/
Copy-Item .env.production deploy/.env
Copy-Item -Recurse certs deploy/ -ErrorAction SilentlyContinue

# デプロイパッケージ圧縮
Compress-Archive -Path deploy/* -DestinationPath doc_man_db_v1.0.0.zip
```

## 8. 参考資料

### 8.1 公式ドキュメント

- [Rust公式ガイド](https://doc.rust-lang.org/book/)
- [Axumドキュメント](https://docs.rs/axum/latest/axum/)
- [SQLxガイド](https://docs.rs/sqlx/latest/sqlx/)
- [SvelteKitドキュメント](https://kit.svelte.dev/docs)
- [Tauriガイド](https://tauri.app/v1/guides/)

### 8.2 ツール・拡張機能

- [VS Code Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [VS Code Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Chocolatey Package Manager](https://chocolatey.org/)
- [Windows Terminal](https://aka.ms/terminal)

### 8.3 トラブルシューティングリソース

- [Rust Forge](https://forge.rust-lang.org/)
- [SQLx GitHub Issues](https://github.com/launchbadge/sqlx/issues)
- [SvelteKit FAQ](https://kit.svelte.dev/faq)
- [Stack Overflow](https://stackoverflow.com/) (rust, svelte, sqlx tags)

---

**注記**: このガイドは継続的に更新されます。問題や改善提案があれば開発チームまでお知らせください。
