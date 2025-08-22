# 文書管理システム トラブルシューティングガイド

## 目次

1. [一般的な問題と解決方法](#一般的な問題と解決方法)
2. [システム起動・接続問題](#システム起動接続問題)
3. [認証・認可問題](#認証認可問題)
4. [データベース関連問題](#データベース関連問題)
5. [パフォーマンス問題](#パフォーマンス問題)
6. [ファイルアクセス問題](#ファイルアクセス問題)
7. [UI・ブラウザ問題](#uiブラウザ問題)
8. [ログ分析方法](#ログ分析方法)
9. [緊急時対応手順](#緊急時対応手順)
10. [よくある質問](#よくある質問)

---

## 一般的な問題と解決方法

### システムにアクセスできない

#### 症状
- ブラウザでシステムURLにアクセスしても画面が表示されない
- 「サイトに接続できません」エラーが表示される

#### 原因と対処法

**1. サービスが起動していない**
```powershell
# サービス状況確認
sc query DocManDB

# サービス開始
sc start DocManDB

# サービスログ確認
Get-EventLog -LogName Application -Source DocManDB -Newest 10
```

**2. ポートが使用中**
```powershell
# ポート使用状況確認
netstat -an | findstr :8080

# プロセス特定
netstat -ano | findstr :8080
tasklist /fi "pid eq <PID>"
```

**3. ファイアウォール設定**
```powershell
# ファイアウォール規則確認
Get-NetFirewallRule -DisplayName "*DocManDB*"

# ポート許可追加
New-NetFirewallRule -DisplayName "DocManDB" -Direction Inbound -Port 8080 -Protocol TCP -Action Allow
```

### 動作が異常に遅い

#### 症状
- ページの読み込みに30秒以上かかる
- 操作に対するレスポンスが遅い

#### 診断手順

**1. システムリソース確認**
```powershell
# CPU使用率
Get-Counter "\Processor(_Total)\% Processor Time"

# メモリ使用率
Get-Counter "\Memory\Available MBytes"

# ディスクI/O
Get-Counter "\LogicalDisk(C:)\Current Disk Queue Length"
```

**2. データベース確認**
```sql
-- 実行中のクエリ確認
SELECT 
    session_id,
    start_time,
    command,
    database_id,
    wait_type,
    wait_time
FROM sys.dm_exec_requests
WHERE session_id > 50;

-- ブロッキング確認
SELECT 
    blocking_session_id,
    session_id,
    wait_type,
    wait_resource
FROM sys.dm_exec_requests
WHERE blocking_session_id != 0;
```

**3. ネットワーク確認**
```bash
# レスポンス時間測定
ping -t server-hostname

# 帯域幅テスト
iperf3 -c server-hostname
```

---

## システム起動・接続問題

### サービスが開始されない

#### エラー: "The service did not start due to a logon failure"

**原因**: サービスアカウントの権限不足

**対処法**:
```powershell
# サービスアカウント設定確認
sc qc DocManDB

# ローカルサービスアカウントに変更
sc config DocManDB obj= "NT AUTHORITY\LocalService"

# 必要な権限付与
ntrights +r SeServiceLogonRight -u "NT AUTHORITY\LocalService"
```

#### エラー: "Configuration file not found"

**原因**: 設定ファイルのパスまたは内容に問題

**対処法**:
```bash
# 設定ファイル確認
dir C:\doc_man_db\config\

# 設定ファイル検証
C:\doc_man_db\doc_man_db.exe --check-config

# デフォルト設定で起動テスト
C:\doc_man_db\doc_man_db.exe --config config/default.toml
```

### データベース接続エラー

#### エラー: "Failed to connect to database"

**SQLite接続問題**:
```bash
# ファイル存在確認
dir C:\doc_man_db\data\production.db

# ファイル権限確認
icacls C:\doc_man_db\data\production.db

# 権限修正
icacls C:\doc_man_db\data\production.db /grant "NT SERVICE\DocManDB:(F)"
```

**SQL Server接続問題**:
```sql
-- 接続テスト
sqlcmd -S server\instance -U username -P password

-- データベース存在確認
SELECT name FROM sys.databases WHERE name = 'DocManDB';

-- 権限確認
SELECT 
    dp.name AS principal_name,
    dp.type_desc AS principal_type,
    o.name AS object_name,
    p.permission_name,
    p.state_desc AS permission_state
FROM sys.database_permissions p
    LEFT JOIN sys.objects o ON p.major_id = o.object_id
    LEFT JOIN sys.database_principals dp ON p.grantee_principal_id = dp.principal_id;
```

---

## 認証・認可問題

### Windows認証が失敗する

#### 症状
- ブラウザで認証ダイアログが表示されない
- 認証後に403エラーが発生

#### 対処法

**1. IIS設定確認**
```powershell
# Windows認証有効化
Enable-WindowsOptionalFeature -Online -FeatureName IIS-WindowsAuthentication

# 匿名認証無効化
Set-WebConfigurationProperty -Filter "/system.webServer/security/authentication/anonymousAuthentication" -Name enabled -Value False

# Windows認証有効化
Set-WebConfigurationProperty -Filter "/system.webServer/security/authentication/windowsAuthentication" -Name enabled -Value True
```

**2. ブラウザ設定確認**
```
Chrome: chrome://settings/content/automaticDownloads
- 「統合Windows認証を有効にする」をチェック
- 信頼済みサイトに追加

Internet Explorer:
- インターネットオプション → セキュリティ → ローカルイントラネット
- 「統合Windows認証でのみサーバーにログオンする」をチェック
```

**3. Active Directory確認**
```powershell
# ドメインコントローラー接続確認
nltest /dsgetdc:company.local

# ユーザー認証テスト
net user username /domain

# グループメンバーシップ確認
net group "DocManDB_Users" /domain
```

### JWT認証エラー

#### エラー: "Invalid or expired token"

**対処法**:
```bash
# トークンデコード（デバッグ用）
echo "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." | base64 -d

# 設定確認
grep "jwt_secret" config/production.toml

# 新しいトークン生成
C:\doc_man_db\doc_man_db.exe --generate-token username
```

### 権限エラー

#### エラー: "Access denied to resource"

**権限確認**:
```sql
-- ユーザー権限確認
SELECT 
    u.username,
    u.role,
    u.department_id,
    d.name as department_name
FROM users u
LEFT JOIN departments d ON u.department_id = d.id
WHERE u.username = 'target_user';

-- 文書アクセス権限確認
SELECT 
    doc.id,
    doc.title,
    doc.confidentiality_level,
    CASE 
        WHEN doc.confidentiality_level = 'Class1' AND u.role != 'admin' THEN 'DENIED'
        WHEN doc.confidentiality_level = 'Class2' AND u.role NOT IN ('admin', 'manager') THEN 'DENIED'
        ELSE 'ALLOWED'
    END as access_status
FROM documents doc, users u
WHERE u.username = 'target_user' AND doc.id = 123;
```

---

## データベース関連問題

### データベース破損

#### 症状
- "Database is locked" エラー
- データの一部が表示されない
- インデックスエラーが頻発

#### SQLite修復

```bash
# データベース整合性チェック
sqlite3 production.db "PRAGMA integrity_check;"

# データベース修復
sqlite3 production.db "PRAGMA quick_check;"
sqlite3 production.db "REINDEX;"
sqlite3 production.db "VACUUM;"

# バックアップからの復旧
copy backup\production_20241221.db data\production.db
```

#### SQL Server修復

```sql
-- データベース整合性チェック
DBCC CHECKDB('DocManDB');

-- インデックス再構築
ALTER INDEX ALL ON documents REBUILD;

-- 統計情報更新
UPDATE STATISTICS documents;

-- 緊急修復（データ損失の可能性あり）
DBCC CHECKDB('DocManDB', REPAIR_ALLOW_DATA_LOSS);
```

### トランザクションロック

#### 症状
- 更新処理が応答しない
- "Transaction timeout" エラー

#### 対処法

```sql
-- ブロッキング状況確認
SELECT 
    blocking_session_id,
    session_id,
    wait_type,
    wait_resource,
    wait_time,
    command
FROM sys.dm_exec_requests
WHERE blocking_session_id != 0;

-- 長時間実行中のトランザクション確認
SELECT 
    s.session_id,
    s.login_time,
    s.last_request_start_time,
    t.transaction_begin_time,
    DATEDIFF(minute, t.transaction_begin_time, GETDATE()) as duration_minutes
FROM sys.dm_exec_sessions s
JOIN sys.dm_tran_session_transactions st ON s.session_id = st.session_id
JOIN sys.dm_tran_active_transactions t ON st.transaction_id = t.transaction_id
WHERE DATEDIFF(minute, t.transaction_begin_time, GETDATE()) > 5;

-- 問題セッションの強制終了（慎重に実行）
KILL 123; -- セッションIDを指定
```

---

## パフォーマンス問題

### 検索が遅い

#### 対処法

**1. インデックス確認・作成**
```sql
-- 不足インデックスの確認
SELECT 
    mid.statement AS table_name,
    migs.avg_total_user_cost * (migs.avg_user_impact / 100.0) * (migs.user_seeks + migs.user_scans) AS improvement_measure,
    'CREATE INDEX IX_' + REPLACE(REPLACE(REPLACE(mid.statement, '[', ''), ']', ''), '.', '_') + '_' + 
    REPLACE(REPLACE(mid.equality_columns + ISNULL('_' + mid.inequality_columns, ''), ' ', ''), ',', '_') + 
    ' ON ' + mid.statement + ' (' + ISNULL(mid.equality_columns, '') + 
    CASE WHEN mid.inequality_columns IS NOT NULL THEN ',' + mid.inequality_columns ELSE '' END + ')' + 
    ISNULL(' INCLUDE (' + mid.included_columns + ')', '') AS create_index_statement
FROM sys.dm_db_missing_index_groups mig
INNER JOIN sys.dm_db_missing_index_group_stats migs ON migs.group_handle = mig.index_group_handle
INNER JOIN sys.dm_db_missing_index_details mid ON mig.index_handle = mid.index_handle
WHERE migs.avg_total_user_cost * (migs.avg_user_impact / 100.0) * (migs.user_seeks + migs.user_scans) > 10
ORDER BY improvement_measure DESC;

-- 推奨インデックス作成
CREATE INDEX IX_documents_title ON documents (title);
CREATE INDEX IX_documents_created_date ON documents (created_date);
CREATE INDEX IX_documents_type_active ON documents (document_type_id, is_active);
```

**2. クエリ最適化**
```sql
-- 実行プラン確認
SET STATISTICS IO ON;
SET STATISTICS TIME ON;

SELECT * FROM documents 
WHERE title LIKE '%技術%' 
  AND is_active = 1;

-- より効率的なクエリ
SELECT * FROM documents 
WHERE is_active = 1 
  AND title LIKE '技術%'  -- 前方一致に変更
ORDER BY created_date DESC;
```

### メモリ不足

#### 症状
- "Out of memory" エラー
- システムの応答が極端に遅くなる

#### 対処法

**1. メモリ使用量確認**
```powershell
# プロセスメモリ使用量
Get-Process -Name "doc_man_db" | Select-Object Name, WorkingSet, VirtualMemorySize

# システム全体のメモリ使用量
Get-Counter "\Memory\Available MBytes"
Get-Counter "\Memory\Pages/sec"
```

**2. 設定調整**
```toml
# config/production.toml
[database]
max_connections = 10  # 接続数を削減
acquire_timeout = 30
idle_timeout = 300

[cache]
max_memory = "128MB"  # キャッシュサイズ制限

[server]
workers = 2  # ワーカー数を削減
```

**3. 大容量処理の見直し**
```rust
// ページネーションの実装
let limit = 100; // 一度に処理する件数を制限
let offset = page * limit;

// ストリーミング処理
let mut stream = sqlx::query("SELECT * FROM documents")
    .fetch(&pool);

while let Some(row) = stream.try_next().await? {
    // 1行ずつ処理
}
```

---

## ファイルアクセス問題

### ネットワークドライブにアクセスできない

#### 症状
- 「ネットワークパスが見つかりません」エラー
- ファイル存在確認が失敗する

#### 対処法

**1. ネットワーク接続確認**
```powershell
# ネットワークパス接続テスト
Test-Path "\\fileserver\documents"

# ネットワークドライブマッピング
net use Z: \\fileserver\documents /persistent:yes

# UNCパスアクセステスト
dir "\\fileserver\documents"
```

**2. サービスアカウント権限**
```powershell
# サービスアカウントでのアクセステスト
runas /user:serviceaccount "cmd /c dir \\fileserver\documents"

# 権限付与
icacls "\\fileserver\documents" /grant "NT SERVICE\DocManDB:(R)"
```

**3. システム設定調整**
```toml
# config/production.toml
[storage]
base_path = "Z:\\documents"  # マップされたドライブ使用
timeout_seconds = 30
retry_attempts = 3
```

### ファイル権限エラー

#### エラー: "Access to the path is denied"

**対処法**:
```powershell
# フォルダ権限確認
icacls "C:\doc_man_db\data"

# 権限修正
icacls "C:\doc_man_db\data" /grant "NT SERVICE\DocManDB:(OI)(CI)F"

# 所有者変更
takeown /f "C:\doc_man_db\data" /r /d y
```

---

## UI・ブラウザ問題

### 画面が正しく表示されない

#### 症状
- レイアウトが崩れる
- 日本語が文字化けする
- JavaScriptエラーが発生

#### 対処法

**1. ブラウザキャッシュクリア**
```
Chrome: Ctrl+Shift+Delete
Firefox: Ctrl+Shift+Delete
Edge: Ctrl+Shift+Delete
```

**2. ブラウザ設定確認**
```javascript
// 開発者ツール（F12）でエラー確認
console.log("エラー内容を確認");

// ネットワークタブで通信エラー確認
// コンソールタブでJavaScriptエラー確認
```

**3. 互換性問題**
```html
<!-- 文字エンコーディング確認 -->
<meta charset="utf-8">

<!-- ブラウザ互換モード -->
<meta http-equiv="X-UA-Compatible" content="IE=edge">
```

### モバイルデバイスでの表示問題

#### 対処法

**1. ビューポート設定確認**
```html
<meta name="viewport" content="width=device-width, initial-scale=1.0">
```

**2. レスポンシブCSS確認**
```css
/* モバイル用スタイル */
@media (max-width: 768px) {
  .container {
    padding: 10px;
  }
}
```

---

## ログ分析方法

### ログファイルの場所

```
C:\doc_man_db\logs\
├── app.log          # アプリケーションログ
├── error.log        # エラーログ
├── access.log       # アクセスログ
└── audit.log        # 監査ログ
```

### ログレベル

- **ERROR**: システムエラー、予期しない問題
- **WARN**: 警告、性能問題の可能性
- **INFO**: 一般的な処理情報
- **DEBUG**: デバッグ情報
- **TRACE**: 詳細な実行トレース

### ログ分析コマンド

```powershell
# エラーログ抽出
findstr "ERROR" C:\doc_man_db\logs\app.log

# 特定期間のログ抽出
Get-Content C:\doc_man_db\logs\app.log | Where-Object {$_ -match "2024-12-21"}

# アクセス頻度分析
Get-Content C:\doc_man_db\logs\access.log | Group-Object | Sort-Object Count -Descending

# エラー統計
findstr "ERROR" C:\doc_man_db\logs\app.log | findstr /c:"Database" | measure-object -line
```

### ログローテーション設定

```toml
# config/production.toml
[logging]
level = "info"
file = "logs/app.log"
max_size = "100MB"
max_files = 10
rotate_daily = true
```

---

## 緊急時対応手順

### システム停止時の対応

**1. 即座の対応**
```powershell
# サービス状態確認
sc query DocManDB

# 手動起動試行
sc start DocManDB

# プロセス確認
tasklist | findstr doc_man_db
```

**2. 切り分け作業**
```powershell
# ログ確認
tail -f C:\doc_man_db\logs\app.log

# リソース確認
Get-Counter "\Processor(_Total)\% Processor Time"
Get-Counter "\Memory\Available MBytes"

# ネットワーク確認
ping database-server
telnet database-server 1433
```

**3. 一時的な回避策**
```bash
# 最小構成での起動
C:\doc_man_db\doc_man_db.exe --safe-mode

# データベース切り替え
C:\doc_man_db\doc_man_db.exe --database sqlite://./data/backup.db
```

### データ消失時の対応

**1. 被害状況確認**
```sql
-- テーブル存在確認
SELECT name FROM sys.tables;

-- レコード数確認
SELECT COUNT(*) FROM documents;

-- 最新データ確認
SELECT TOP 10 * FROM documents ORDER BY created_at DESC;
```

**2. バックアップからの復旧**
```bash
# サービス停止
sc stop DocManDB

# バックアップからの復旧
copy "\\backup\DocManDB_20241221.bak" "C:\backups\"
sqlcmd -S server -Q "RESTORE DATABASE DocManDB FROM DISK = 'C:\backups\DocManDB_20241221.bak' WITH REPLACE"

# サービス開始
sc start DocManDB
```

---

## よくある質問

### Q1: パスワードをリセットしたい

**A1**: 管理者権限で以下を実行
```bash
C:\doc_man_db\doc_man_db.exe --reset-password <username>
```

### Q2: 文書番号が重複している

**A2**: データベースで重複確認・修正
```sql
-- 重複確認
SELECT document_number, COUNT(*) 
FROM documents 
GROUP BY document_number 
HAVING COUNT(*) > 1;

-- シーケンス番号リセット
UPDATE document_number_sequences 
SET current_number = (SELECT MAX(CAST(SUBSTRING(document_number, 4, 10) AS INT)) FROM documents WHERE document_number LIKE 'TEC-%')
WHERE document_type = 'technical';
```

### Q3: アップロードファイルサイズ制限を変更したい

**A3**: 設定ファイルで制限値変更
```toml
# config/production.toml
[server]
max_request_size = "50MB"

[storage]
max_file_size = "50MB"
```

### Q4: SSL証明書エラーが発生する

**A4**: 証明書の更新・設定確認
```powershell
# 証明書確認
certlm.msc

# 新しい証明書設定
copy new-certificate.crt C:\doc_man_db\certs\server.crt
copy new-certificate.key C:\doc_man_db\certs\server.key

# サービス再起動
sc restart DocManDB
```

### Q5: 大量データのエクスポートが失敗する

**A5**: バッチサイズの調整・分割処理
```toml
# config/production.toml
[export]
batch_size = 1000
timeout_seconds = 300
```

```sql
-- 分割エクスポート
SELECT * FROM documents 
WHERE created_date BETWEEN '2024-01-01' AND '2024-03-31'
ORDER BY id
OFFSET 0 ROWS FETCH NEXT 1000 ROWS ONLY;
```

---

## サポート・連絡先

### 技術サポート

- **メール**: tech-support@company.com
- **電話**: 03-XXXX-XXXX（平日 9:00-18:00）
- **緊急連絡**: emergency-support@company.com

### システム管理者

- **システム管理**: admin@company.com
- **データベース管理**: dba@company.com
- **ネットワーク管理**: network@company.com

### 開発チーム

- **バグ報告**: dev-team@company.com
- **機能要望**: feature-request@company.com
- **GitHub Issues**: https://github.com/company/doc-man-db/issues

---

## チェックリスト

### 日常点検

- [ ] サービス稼働状況確認
- [ ] ログファイル容量確認
- [ ] データベース接続確認
- [ ] バックアップファイル確認
- [ ] 証明書有効期限確認

### 月次点検

- [ ] システムリソース使用状況
- [ ] データベース統計情報更新
- [ ] インデックス断片化確認
- [ ] 監査ログレビュー
- [ ] セキュリティ更新適用

### 緊急時チェック

- [ ] サービス状態確認
- [ ] ログエラー確認
- [ ] ネットワーク接続確認
- [ ] データベース整合性確認
- [ ] バックアップ可用性確認

---

**本ガイドに関するお問い合わせ**
- 作成者: Doc Man DB Development Team
- 最終更新: 2024-12-21
- バージョン: v1.0.0