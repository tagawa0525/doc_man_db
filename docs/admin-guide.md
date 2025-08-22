# 文書管理システム 管理者ガイド

## 目次

1. [システム管理概要](#システム管理概要)
2. [インストール・セットアップ](#インストールセットアップ)
3. [設定管理](#設定管理)
4. [ユーザー管理](#ユーザー管理)
5. [権限管理](#権限管理)
6. [システム監視](#システム監視)
7. [バックアップ・復旧](#バックアップ復旧)
8. [トラブルシューティング](#トラブルシューティング)
9. [メンテナンス](#メンテナンス)

---

## システム管理概要

### 管理者の役割

- システムの安定稼働確保
- ユーザー・権限管理
- データバックアップ管理
- セキュリティ監視
- システム設定変更

### 必要な技術知識

- Rust言語の基礎
- SQLite/SQL Server管理
- Windows Server管理
- ネットワーク設定
- セキュリティ基礎

---

## インストール・セットアップ

### システム要件

#### ハードウェア要件
- **CPU**: Intel Core i5以上または同等
- **メモリ**: 8GB以上（推奨16GB）
- **ストレージ**: SSD 100GB以上
- **ネットワーク**: Gigabit Ethernet

#### ソフトウェア要件
- **OS**: Windows Server 2019以上
- **データベース**: SQLite（開発）/SQL Server 2019以上（本番）
- **Webサーバー**: システム内蔵（Axum）
- **認証**: Windows Active Directory

### インストール手順

#### 1. バイナリデプロイメント

```bash
# リリースパッケージの展開
unzip doc_man_db_v1.0.0.zip -d C:\doc_man_db

# 実行権限の設定
icacls C:\doc_man_db\doc_man_db.exe /grant Everyone:F

# 設定ファイルの配置
copy config\production.toml C:\doc_man_db\config\
```

#### 2. データベース設定

```bash
# SQLiteの場合（開発・テスト環境）
mkdir C:\doc_man_db\data
copy database\schema.sql C:\doc_man_db\data\

# SQL Serverの場合（本番環境）
sqlcmd -S SERVER\INSTANCE -i database\schema.sql
```

#### 3. サービス登録

```powershell
# Windowsサービスとして登録
sc create DocManDB binPath= "C:\doc_man_db\doc_man_db.exe --service" start= auto
sc description DocManDB "Document Management Database Service"
sc start DocManDB
```

#### 4. 初期設定

```bash
# 初期データのインポート
C:\doc_man_db\doc_man_db.exe --setup
C:\doc_man_db\doc_man_db.exe --import-initial-data
```

---

## 設定管理

### 設定ファイル構造

```toml
# config/production.toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "sqlite://./data/production.db"
max_connections = 10
migrate_on_start = true

[auth]
method = "windows_ad"  # or "json"
ad_domain = "company.local"
jwt_secret = "your-secret-key"
token_expiry = 3600

[logging]
level = "info"
file = "logs/app.log"
max_size = "100MB"
max_files = 10

[security]
enable_https = true
cert_file = "certs/server.crt"
key_file = "certs/server.key"
cors_origins = ["https://company.com"]

[cache]
redis_url = "redis://localhost:6379"
default_ttl = 3600

[notifications]
email_smtp = "smtp.company.com"
email_port = 587
teams_webhook = "https://outlook.office.com/webhook/..."
```

### 環境変数設定

```powershell
# 重要な設定は環境変数で管理
setx DOC_MAN_DB_SECRET "your-super-secret-key"
setx DOC_MAN_DATABASE_URL "sqlserver://server/database"
setx DOC_MAN_AD_DOMAIN "company.local"
```

### 設定の変更・適用

1. 設定ファイルを編集
2. 設定検証を実行
3. サービス再起動

```bash
# 設定検証
C:\doc_man_db\doc_man_db.exe --check-config

# サービス再起動
sc stop DocManDB
sc start DocManDB
```

---

## ユーザー管理

### Active Directory連携

#### AD設定

```powershell
# AD グループの作成
New-ADGroup -Name "DocManDB_Admins" -GroupScope Global
New-ADGroup -Name "DocManDB_Users" -GroupScope Global
New-ADGroup -Name "DocManDB_Viewers" -GroupScope Global

# ユーザーをグループに追加
Add-ADGroupMember -Identity "DocManDB_Admins" -Members "admin.user"
Add-ADGroupMember -Identity "DocManDB_Users" -Members "normal.user"
```

#### 権限マッピング

```toml
# config/auth.toml
[ad_groups]
"DocManDB_Admins" = "admin"
"DocManDB_Users" = "user"
"DocManDB_Viewers" = "viewer"
```

### JSON認証（開発・テスト環境）

#### ユーザー追加

```sql
INSERT INTO users (username, display_name, password_hash, role, is_active) 
VALUES ('admin', '管理者', '$2b$12$...', 'admin', true);
```

#### パスワードリセット

```bash
# CLIツールでパスワードリセット
C:\doc_man_db\doc_man_db.exe --reset-password admin
```

---

## 権限管理

### 権限レベル

| レベル | 権限内容 |
|--------|----------|
| admin | 全機能アクセス、システム設定変更 |
| manager | 文書管理、承認フロー管理 |
| user | 文書作成・編集、回覧参加 |
| viewer | 文書閲覧、検索のみ |

### 機密性レベル制御

```sql
-- 機密性レベルとアクセス権限のマッピング
CREATE TABLE confidentiality_access (
    level VARCHAR(20),
    min_role VARCHAR(20),
    department_restriction BOOLEAN
);

INSERT INTO confidentiality_access VALUES 
('Class1', 'admin', false),
('Class2', 'manager', true),
('Class3', 'user', true),
('Class4', 'viewer', false);
```

### 部門別アクセス制御

```sql
-- 部門別アクセス制御の設定
CREATE TABLE department_access (
    user_id INTEGER,
    department_id INTEGER,
    access_level VARCHAR(20)
);
```

---

## システム監視

### ログ監視

#### アプリケーションログ

```bash
# ログファイルの確認
tail -f C:\doc_man_db\logs\app.log

# エラーログの検索
findstr "ERROR" C:\doc_man_db\logs\app.log
```

#### アクセスログ分析

```sql
-- APIアクセス統計
SELECT endpoint, COUNT(*) as requests, AVG(response_time) as avg_time
FROM api_logs 
WHERE created_at >= DATE('now', '-1 day')
GROUP BY endpoint
ORDER BY requests DESC;
```

### パフォーマンス監視

#### システムメトリクス取得

```powershell
# CPU・メモリ使用率
Get-Counter "\Processor(_Total)\% Processor Time"
Get-Counter "\Memory\Available MBytes"

# ディスク使用率
Get-Counter "\LogicalDisk(C:)\% Free Space"
```

#### データベース監視

```sql
-- データベースサイズ監視
SELECT 
    name,
    size * 8 / 1024 as size_mb,
    max_size * 8 / 1024 as max_size_mb
FROM sys.database_files;

-- 遅いクエリの特定
SELECT TOP 10
    query_text,
    execution_count,
    avg_duration,
    avg_cpu_time
FROM slow_query_log
ORDER BY avg_duration DESC;
```

### アラート設定

#### Windows イベントログ監視

```powershell
# カスタムイベントログの作成
New-EventLog -LogName "DocManDB" -Source "DocManDBService"

# 重要なイベントをログに記録
Write-EventLog -LogName "DocManDB" -Source "DocManDBService" -EventId 1001 -EntryType Error -Message "Database connection failed"
```

---

## バックアップ・復旧

### データベースバックアップ

#### SQLiteバックアップ

```bash
# 日次バックアップスクリプト
@echo off
set BACKUP_DIR=C:\backups\docmandb
set DATE=%date:~-4,4%%date:~-10,2%%date:~-7,2%

# データベースファイルのコピー
copy C:\doc_man_db\data\production.db %BACKUP_DIR%\production_%DATE%.db

# 圧縮
7z a %BACKUP_DIR%\production_%DATE%.7z %BACKUP_DIR%\production_%DATE%.db
del %BACKUP_DIR%\production_%DATE%.db
```

#### SQL Serverバックアップ

```sql
-- 完全バックアップ
BACKUP DATABASE DocManDB 
TO DISK = 'C:\backups\DocManDB_Full.bak'
WITH FORMAT, COMPRESSION;

-- 差分バックアップ
BACKUP DATABASE DocManDB 
TO DISK = 'C:\backups\DocManDB_Diff.bak'
WITH DIFFERENTIAL, COMPRESSION;
```

### ファイルシステムバックアップ

```powershell
# Robocopyによるファイルバックアップ
robocopy "\\server\documents" "\\backup\documents" /MIR /R:3 /W:10 /LOG:backup.log
```

### 復旧手順

#### データベース復旧

```sql
-- SQL Server データベース復旧
RESTORE DATABASE DocManDB 
FROM DISK = 'C:\backups\DocManDB_Full.bak'
WITH REPLACE, NORECOVERY;

RESTORE DATABASE DocManDB 
FROM DISK = 'C:\backups\DocManDB_Diff.bak'
WITH RECOVERY;
```

#### システム全体復旧

1. サービス停止
2. データベース復旧
3. ファイルシステム復旧
4. 設定ファイル復旧
5. サービス開始
6. 動作確認

---

## トラブルシューティング

### 一般的な問題と解決方法

#### 1. サービス起動失敗

**症状**: DocManDBサービスが開始されない

**原因と対処**:
```bash
# ログ確認
type C:\doc_man_db\logs\app.log

# 設定ファイル検証
C:\doc_man_db\doc_man_db.exe --check-config

# 権限確認
icacls C:\doc_man_db /T
```

#### 2. データベース接続エラー

**症状**: "Database connection failed"

**原因と対処**:
```sql
-- 接続文字列確認
sqlcmd -S server\instance -U username -P password

-- データベース存在確認
SELECT name FROM sys.databases;

-- 権限確認
SELECT * FROM sys.database_permissions;
```

#### 3. 認証エラー

**症状**: ログインできない、権限エラー

**原因と対処**:
```powershell
# AD接続確認
nltest /dsgetdc:domain.com

# ユーザー権限確認
net user username /domain

# グループメンバーシップ確認
net group "DocManDB_Users" /domain
```

#### 4. パフォーマンス問題

**症状**: レスポンスが遅い、タイムアウト

**原因と対処**:
```sql
-- インデックス確認
SELECT * FROM sys.dm_db_missing_index_details;

-- 統計情報更新
UPDATE STATISTICS;

-- クエリプラン確認
SET STATISTICS IO ON;
```

### ログレベル設定

```toml
# 詳細ログ出力設定
[logging]
level = "debug"  # error, warn, info, debug, trace
modules = ["doc_man_db::handlers", "doc_man_db::repositories"]
```

### デバッグモード

```bash
# デバッグモードでの起動
C:\doc_man_db\doc_man_db.exe --debug --log-level trace
```

---

## メンテナンス

### 定期メンテナンス作業

#### 日次作業

```bash
# ログローテーション
forfiles /p C:\doc_man_db\logs /s /m *.log /d -7 /c "cmd /c del @path"

# 一時ファイル削除
del /q C:\doc_man_db\temp\*.*

# バックアップ実行
C:\scripts\backup_daily.bat
```

#### 週次作業

```sql
-- データベース統計更新
UPDATE STATISTICS;

-- インデックスの再構築
ALTER INDEX ALL ON documents REBUILD;

-- 不要データの削除
DELETE FROM session_logs WHERE created_at < DATEADD(day, -30, GETDATE());
```

#### 月次作業

```bash
# システム更新確認
choco outdated

# 証明書有効期限確認
certlm.msc

# ディスク容量確認
dir C:\ /-c

# パフォーマンスレポート生成
C:\doc_man_db\doc_man_db.exe --performance-report
```

### アップデート手順

#### マイナーアップデート

1. バックアップ実行
2. サービス停止
3. バイナリ置換
4. 設定ファイル確認
5. データベースマイグレーション実行
6. サービス開始
7. 動作確認

```bash
# マイグレーション実行
C:\doc_man_db\doc_man_db.exe --migrate
```

#### メジャーアップデート

1. 完全バックアップ
2. テスト環境での動作確認
3. メンテナンス時間の確保
4. 段階的アップデート
5. ロールバック準備

### 容量管理

#### データベース容量監視

```sql
-- テーブル別サイズ確認
SELECT 
    t.name AS table_name,
    i.rows AS row_count,
    (i.reserved * 8 / 1024) AS reserved_mb,
    (i.data * 8 / 1024) AS data_mb,
    (i.index_size * 8 / 1024) AS index_mb
FROM sys.tables t
INNER JOIN sys.sysindexes i ON t.object_id = i.id
WHERE i.indid < 2
ORDER BY reserved_mb DESC;
```

#### ログファイル管理

```powershell
# ログファイルサイズ制限
$logConfig = @{
    MaximumFileSize = 100MB
    MaximumFileCount = 10
    LogPath = "C:\doc_man_db\logs"
}
```

---

## セキュリティ管理

### SSL/TLS設定

```bash
# 証明書の配置
copy server.crt C:\doc_man_db\certs\
copy server.key C:\doc_man_db\certs\

# 証明書権限設定
icacls C:\doc_man_db\certs\server.key /grant "NT SERVICE\DocManDB:(R)"
```

### 監査ログ

```sql
-- 監査ログの確認
SELECT 
    user_id,
    action,
    resource,
    created_at,
    ip_address
FROM audit_logs 
WHERE created_at >= DATEADD(day, -1, GETDATE())
ORDER BY created_at DESC;
```

### セキュリティ更新

```powershell
# Windowsセキュリティ更新
Install-Module PSWindowsUpdate
Get-WUInstall -AcceptAll -AutoReboot
```

---

## 緊急時対応

### 障害対応フロー

1. **障害検知**
   - 監視アラート確認
   - ユーザー報告受付
   - 影響範囲調査

2. **初期対応**
   - サービス状態確認
   - ログ分析
   - 一次切り分け

3. **復旧作業**
   - 根本原因特定
   - 修正作業実施
   - 動作確認

4. **事後対応**
   - 障害報告書作成
   - 再発防止策検討
   - 監視強化

### 連絡先リスト

- **システム開発チーム**: dev-team@company.com
- **インフラチーム**: infra-team@company.com
- **セキュリティチーム**: security-team@company.com
- **業務部門責任者**: business-manager@company.com

---

## 付録

### 設定ファイル完全版

```toml
# config/production.toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4
timeout_seconds = 30
max_request_size = "10MB"

[database]
url = "sqlserver://server/DocManDB"
max_connections = 20
min_connections = 5
acquire_timeout = 30
idle_timeout = 600
max_lifetime = 1800

[auth]
method = "windows_ad"
ad_domain = "company.local"
ad_server = "dc.company.local"
jwt_secret = "your-jwt-secret"
token_expiry = 3600
refresh_expiry = 86400

[logging]
level = "info"
file = "logs/app.log"
max_size = "100MB"
max_files = 10
json_format = false

[security]
enable_https = true
cert_file = "certs/server.crt"
key_file = "certs/server.key"
cors_origins = ["https://company.com"]
rate_limit_per_minute = 100

[cache]
redis_url = "redis://localhost:6379"
default_ttl = 3600
max_memory = "256MB"

[notifications]
email_smtp = "smtp.company.com"
email_port = 587
email_username = "noreply@company.com"
email_password = "password"
teams_webhook = "https://outlook.office.com/webhook/..."

[storage]
base_path = "\\\\fileserver\\documents"
temp_path = "C:\\doc_man_db\\temp"
max_file_size = "100MB"
allowed_extensions = [".pdf", ".doc", ".docx", ".xls", ".xlsx"]

[backup]
enabled = true
schedule = "0 2 * * *"  # 毎日2時
retention_days = 30
compression = true
```

### CLIコマンド一覧

```bash
# システム管理
doc_man_db.exe --help                  # ヘルプ表示
doc_man_db.exe --version               # バージョン表示
doc_man_db.exe --check-config          # 設定検証
doc_man_db.exe --migrate               # マイグレーション実行

# ユーザー管理
doc_man_db.exe --create-user username  # ユーザー作成
doc_man_db.exe --reset-password user   # パスワードリセット
doc_man_db.exe --list-users            # ユーザー一覧

# データ管理
doc_man_db.exe --backup                # バックアップ実行
doc_man_db.exe --restore backup.db     # リストア実行
doc_man_db.exe --vacuum                # データベース最適化

# 監視・診断
doc_man_db.exe --health-check          # ヘルスチェック
doc_man_db.exe --performance-report    # パフォーマンスレポート
doc_man_db.exe --system-info           # システム情報表示
```

---

**本ガイドに関するお問い合わせ**
- 作成者: Doc Man DB Development Team
- 最終更新: 2024-12-21
- バージョン: v1.0.0