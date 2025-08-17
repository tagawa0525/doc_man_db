# データベース詳細設計書

## 1. データベース概要

### 1.1 設計方針

- **正規化**: 第3正規形まで正規化、パフォーマンス重視で一部非正規化
- **互換性**: SQLite → SQL Server移行を考慮した設計
- **拡張性**: 将来的な機能拡張に対応可能な柔軟な構造
- **保守性**: 明確な命名規則・制約・インデックス設計

### 1.2 命名規則

| 項目               | 規則                      | 例                          |
| ------------------ | ------------------------- | --------------------------- |
| **テーブル名**     | snake_case、複数形        | `documents`, `employees`    |
| **カラム名**       | snake_case                | `employee_id`, `created_at` |
| **インデックス名** | `idx_{table}_{column}`    | `idx_documents_number`      |
| **外部キー名**     | `fk_{table}_{ref_table}`  | `fk_documents_employees`    |
| **制約名**         | `ck_{table}_{constraint}` | `ck_documents_status`       |

### 1.3 データ型マッピング

| 用途            | SQLite              | SQL Server      | 備考           |
| --------------- | ------------------- | --------------- | -------------- |
| **ID (主キー)** | INTEGER PRIMARY KEY | BIGINT IDENTITY | 自動増分       |
| **文字列 (短)** | TEXT                | NVARCHAR(255)   | 255文字以内    |
| **文字列 (長)** | TEXT                | NVARCHAR(MAX)   | 制限なし       |
| **日時**        | TEXT (ISO 8601)     | DATETIME2       | UTCで保存      |
| **真偽値**      | INTEGER (0/1)       | BIT             | SQLiteは数値   |
| **JSON**        | TEXT                | NVARCHAR(MAX)   | JSONとして解析 |

## 2. テーブル設計

### 2.1 組織・人員関連テーブル

#### 2.1.1 departments (部署)

```sql
CREATE TABLE departments (
    id INTEGER PRIMARY KEY,                    -- 部署ID
    code TEXT NOT NULL UNIQUE,                 -- 部署コード (1文字)
    name TEXT NOT NULL,                        -- 部署名
    parent_id INTEGER NULL,                    -- 親部署ID (階層対応)
    effective_from TEXT NOT NULL,              -- 有効期間開始日 (ISO 8601)
    effective_to TEXT NULL,                    -- 有効期間終了日 (NULL=現在も有効)
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (parent_id) REFERENCES departments(id),
    CHECK (effective_from <= effective_to OR effective_to IS NULL)
);

-- インデックス
CREATE INDEX idx_departments_code ON departments(code);
CREATE INDEX idx_departments_effective ON departments(effective_from, effective_to);
```

#### 2.1.2 sections (課)

```sql
CREATE TABLE sections (
    id INTEGER PRIMARY KEY,                    -- 課ID
    department_id INTEGER NOT NULL,           -- 所属部署ID
    code TEXT NOT NULL,                       -- 課コード (1文字)
    name TEXT NOT NULL,                       -- 課名
    effective_from TEXT NOT NULL,             -- 有効期間開始日
    effective_to TEXT NULL,                   -- 有効期間終了日
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (department_id) REFERENCES departments(id),
    CHECK (effective_from <= effective_to OR effective_to IS NULL),
    UNIQUE (department_id, code, effective_from)  -- 同一部署・時期での課コード重複防止
);

-- インデックス
CREATE INDEX idx_sections_dept_code ON sections(department_id, code);
CREATE INDEX idx_sections_effective ON sections(effective_from, effective_to);
```

#### 2.1.3 employees (社員)

```sql
CREATE TABLE employees (
    id INTEGER PRIMARY KEY,                    -- 社員ID
    employee_id TEXT NOT NULL UNIQUE,         -- 社員番号
    name TEXT NOT NULL,                       -- 氏名
    email TEXT NULL,                          -- メールアドレス
    ad_username TEXT NULL,                    -- ADユーザー名
    data_source TEXT NOT NULL DEFAULT 'json', -- データソース: 'json' | 'ad_sync'
    is_active INTEGER NOT NULL DEFAULT 1,     -- アクティブフラグ (論理削除)
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    CHECK (data_source IN ('json', 'ad_sync')),
    CHECK (is_active IN (0, 1))
);

-- インデックス
CREATE INDEX idx_employees_employee_id ON employees(employee_id);
CREATE INDEX idx_employees_ad_username ON employees(ad_username);
CREATE INDEX idx_employees_active ON employees(is_active);
```

#### 2.1.4 employee_assignments (社員所属履歴)

```sql
CREATE TABLE employee_assignments (
    id INTEGER PRIMARY KEY,                    -- 所属履歴ID
    employee_id INTEGER NOT NULL,             -- 社員ID
    department_id INTEGER NOT NULL,           -- 部署ID
    section_id INTEGER NULL,                  -- 課ID (課がない場合はNULL)
    assignment_type TEXT NOT NULL DEFAULT 'primary', -- 所属種別: 'primary' | 'concurrent' | 'secondment'
    effective_from TEXT NOT NULL,             -- 所属開始日
    effective_to TEXT NULL,                   -- 所属終了日 (NULL=現在も所属)
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (employee_id) REFERENCES employees(id),
    FOREIGN KEY (department_id) REFERENCES departments(id),
    FOREIGN KEY (section_id) REFERENCES sections(id),
    CHECK (assignment_type IN ('primary', 'concurrent', 'secondment')),
    CHECK (effective_from <= effective_to OR effective_to IS NULL)
);

-- インデックス
CREATE INDEX idx_assignments_employee ON employee_assignments(employee_id);
CREATE INDEX idx_assignments_dept_section ON employee_assignments(department_id, section_id);
CREATE INDEX idx_assignments_effective ON employee_assignments(effective_from, effective_to);
```

### 2.2 文書管理関連テーブル

#### 2.2.1 document_types (文書種別)

```sql
CREATE TABLE document_types (
    id INTEGER PRIMARY KEY,                    -- 文書種別ID
    code TEXT NOT NULL,                       -- 文書種別コード (A, B, C, etc.)
    name TEXT NOT NULL,                       -- 文書種別名 (報告書、議事録、etc.)
    requires_approval INTEGER NOT NULL DEFAULT 0, -- 承認書要否
    department_id INTEGER NULL,               -- 適用部署 (NULL=全部署)
    effective_from TEXT NOT NULL,             -- 有効期間開始日
    effective_to TEXT NULL,                   -- 有効期間終了日
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (department_id) REFERENCES departments(id),
    CHECK (requires_approval IN (0, 1)),
    CHECK (effective_from <= effective_to OR effective_to IS NULL),
    UNIQUE (code, department_id, effective_from)  -- 同一部署・時期での種別コード重複防止
);

-- インデックス
CREATE INDEX idx_document_types_code ON document_types(code);
CREATE INDEX idx_document_types_dept ON document_types(department_id);
CREATE INDEX idx_document_types_effective ON document_types(effective_from, effective_to);
```

#### 2.2.2 document_number_rules (文書番号生成ルール)

```sql
CREATE TABLE document_number_rules (
    id INTEGER PRIMARY KEY,                    -- ルールID
    name TEXT NOT NULL,                       -- ルール名
    template TEXT NOT NULL,                   -- 生成テンプレート
    sequence_digits INTEGER NOT NULL DEFAULT 3, -- 連番桁数
    department_id INTEGER NULL,               -- 適用部署 (NULL=全部署)
    document_type_codes TEXT NULL,            -- 適用文書種別 (JSON配列: ["A","B"])
    priority INTEGER NOT NULL DEFAULT 1,      -- 優先度 (数値が小さいほど高優先)
    effective_from TEXT NOT NULL,             -- 有効期間開始日
    effective_to TEXT NULL,                   -- 有効期間終了日
    is_active INTEGER NOT NULL DEFAULT 1,     -- アクティブフラグ
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (department_id) REFERENCES departments(id),
    CHECK (sequence_digits > 0),
    CHECK (priority > 0),
    CHECK (is_active IN (0, 1)),
    CHECK (effective_from <= effective_to OR effective_to IS NULL)
);

-- インデックス
CREATE INDEX idx_number_rules_dept ON document_number_rules(department_id);
CREATE INDEX idx_number_rules_priority ON document_number_rules(priority, effective_from);
CREATE INDEX idx_number_rules_active ON document_number_rules(is_active);
```

#### 2.2.3 document_sequences (文書番号連番管理)

```sql
CREATE TABLE document_sequences (
    id INTEGER PRIMARY KEY,                    -- 連番管理ID
    rule_id INTEGER NOT NULL,                 -- 適用ルールID
    year INTEGER NOT NULL,                    -- 年
    month INTEGER NULL,                       -- 月 (年単位の場合はNULL)
    department_id INTEGER NOT NULL,           -- 部署ID
    last_sequence INTEGER NOT NULL DEFAULT 0, -- 最後に使用した連番
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (rule_id) REFERENCES document_number_rules(id),
    FOREIGN KEY (department_id) REFERENCES departments(id),
    CHECK (year > 1900),
    CHECK (month IS NULL OR (month >= 1 AND month <= 12)),
    CHECK (last_sequence >= 0),
    UNIQUE (rule_id, year, month, department_id)  -- 同一条件での重複防止
);

-- インデックス
CREATE INDEX idx_sequences_rule_year ON document_sequences(rule_id, year, month);
CREATE INDEX idx_sequences_dept ON document_sequences(department_id);
```

#### 2.2.4 path_generation_rules (パス生成ルール)

```sql
CREATE TABLE path_generation_rules (
    id INTEGER PRIMARY KEY,                    -- パスルールID
    name TEXT NOT NULL,                       -- ルール名
    path_template TEXT NOT NULL,              -- パステンプレート
    department_id INTEGER NULL,               -- 適用部署 (NULL=全部署)
    year_from INTEGER NULL,                   -- 適用年範囲開始
    year_to INTEGER NULL,                     -- 適用年範囲終了
    priority INTEGER NOT NULL DEFAULT 1,      -- 優先度
    effective_from TEXT NOT NULL,             -- 有効期間開始日
    effective_to TEXT NULL,                   -- 有効期間終了日
    is_active INTEGER NOT NULL DEFAULT 1,     -- アクティブフラグ
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (department_id) REFERENCES departments(id),
    CHECK (year_from IS NULL OR year_from > 1900),
    CHECK (year_to IS NULL OR year_to > 1900),
    CHECK (year_from IS NULL OR year_to IS NULL OR year_from <= year_to),
    CHECK (priority > 0),
    CHECK (is_active IN (0, 1)),
    CHECK (effective_from <= effective_to OR effective_to IS NULL)
);

-- インデックス
CREATE INDEX idx_path_rules_dept ON path_generation_rules(department_id);
CREATE INDEX idx_path_rules_year ON path_generation_rules(year_from, year_to);
CREATE INDEX idx_path_rules_priority ON path_generation_rules(priority, effective_from);
```

#### 2.2.5 documents (文書)

```sql
CREATE TABLE documents (
    id INTEGER PRIMARY KEY,                    -- 文書ID
    number TEXT NOT NULL UNIQUE,              -- 文書番号
    title TEXT NOT NULL,                      -- 文書タイトル
    document_type_id INTEGER NOT NULL,        -- 文書種別ID
    business_number TEXT NULL,                -- 業務番号
    created_by INTEGER NOT NULL,              -- 作成者ID
    created_date TEXT NOT NULL,               -- 作成日
    version_type TEXT NOT NULL DEFAULT 'final', -- 版種別: 'draft' | 'revision' | 'final'
    version_number INTEGER NULL,              -- 版番号 (draft:d1,d2... revision:r1,r2...)
    parent_document_id INTEGER NULL,          -- 親文書ID (改訂・ドラフトの場合)
    
    -- 機密レベル (3軸)
    internal_external TEXT NOT NULL DEFAULT 'internal', -- 社内外: 'internal' | 'external'
    importance_class TEXT NOT NULL DEFAULT 'class2',    -- 重要度: 'class1' | 'class2'
    personal_info TEXT NOT NULL DEFAULT 'none',         -- 個人情報: 'none' | 'present'
    
    -- パス情報
    network_path TEXT NULL,                   -- ネットワークパス
    folder_exists INTEGER NULL,               -- フォルダ存在確認結果
    approval_file_exists INTEGER NULL,        -- 承認書ファイル存在確認結果
    last_path_check TEXT NULL,               -- 最終パス確認日時
    
    -- メタデータ
    notes TEXT NULL,                          -- 備考
    is_active INTEGER NOT NULL DEFAULT 1,     -- アクティブフラグ
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (document_type_id) REFERENCES document_types(id),
    FOREIGN KEY (created_by) REFERENCES employees(id),
    FOREIGN KEY (parent_document_id) REFERENCES documents(id),
    
    CHECK (version_type IN ('draft', 'revision', 'final')),
    CHECK (version_number IS NULL OR version_number > 0),
    CHECK (internal_external IN ('internal', 'external')),
    CHECK (importance_class IN ('class1', 'class2')),
    CHECK (personal_info IN ('none', 'present')),
    CHECK (folder_exists IS NULL OR folder_exists IN (0, 1)),
    CHECK (approval_file_exists IS NULL OR approval_file_exists IN (0, 1)),
    CHECK (is_active IN (0, 1))
);

-- インデックス
CREATE INDEX idx_documents_number ON documents(number);
CREATE INDEX idx_documents_type ON documents(document_type_id);
CREATE INDEX idx_documents_creator ON documents(created_by);
CREATE INDEX idx_documents_business ON documents(business_number);
CREATE INDEX idx_documents_created_date ON documents(created_date);
CREATE INDEX idx_documents_confidentiality ON documents(internal_external, importance_class, personal_info);
CREATE INDEX idx_documents_active ON documents(is_active);
```

### 2.3 業務・回覧関連テーブル

#### 2.3.1 businesses (業務)

```sql
CREATE TABLE businesses (
    id INTEGER PRIMARY KEY,                    -- 業務ID
    business_number TEXT NOT NULL UNIQUE,     -- 業務番号
    name TEXT NOT NULL,                       -- 業務名
    description TEXT NULL,                    -- 業務説明
    customer_name TEXT NULL,                  -- 顧客名
    start_date TEXT NULL,                     -- 開始日
    end_date TEXT NULL,                       -- 終了日
    status TEXT NOT NULL DEFAULT 'active',    -- ステータス: 'active' | 'completed' | 'suspended'
    is_active INTEGER NOT NULL DEFAULT 1,     -- アクティブフラグ
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    CHECK (status IN ('active', 'completed', 'suspended')),
    CHECK (start_date IS NULL OR end_date IS NULL OR start_date <= end_date),
    CHECK (is_active IN (0, 1))
);

-- インデックス
CREATE INDEX idx_businesses_number ON businesses(business_number);
CREATE INDEX idx_businesses_customer ON businesses(customer_name);
CREATE INDEX idx_businesses_status ON businesses(status);
CREATE INDEX idx_businesses_active ON businesses(is_active);
```

#### 2.3.2 business_members (業務従事者)

```sql
CREATE TABLE business_members (
    id INTEGER PRIMARY KEY,                    -- 業務従事者ID
    business_id INTEGER NOT NULL,             -- 業務ID
    employee_id INTEGER NOT NULL,             -- 社員ID
    role TEXT NOT NULL DEFAULT 'member',      -- 役割: 'leader' | 'member' | 'support'
    participation_level TEXT NOT NULL DEFAULT 'full', -- 参加レベル: 'full' | 'partial' | 'consultant'
    start_date TEXT NOT NULL,                 -- 従事開始日
    end_date TEXT NULL,                       -- 従事終了日 (NULL=継続中)
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (business_id) REFERENCES businesses(id),
    FOREIGN KEY (employee_id) REFERENCES employees(id),
    CHECK (role IN ('leader', 'member', 'support')),
    CHECK (participation_level IN ('full', 'partial', 'consultant')),
    CHECK (start_date <= end_date OR end_date IS NULL),
    UNIQUE (business_id, employee_id, start_date)  -- 同一業務・社員・開始日での重複防止
);

-- インデックス
CREATE INDEX idx_business_members_business ON business_members(business_id);
CREATE INDEX idx_business_members_employee ON business_members(employee_id);
CREATE INDEX idx_business_members_role ON business_members(role);
CREATE INDEX idx_business_members_dates ON business_members(start_date, end_date);
```

#### 2.3.3 external_contacts (外部連絡先)

```sql
CREATE TABLE external_contacts (
    id INTEGER PRIMARY KEY,                    -- 外部連絡先ID
    business_id INTEGER NOT NULL,             -- 関連業務ID
    contact_type TEXT NOT NULL,               -- 連絡先種別: 'customer' | 'partner' | 'vendor'
    company_name TEXT NOT NULL,               -- 会社名
    contact_name TEXT NULL,                   -- 担当者名
    email TEXT NULL,                          -- メールアドレス
    phone TEXT NULL,                          -- 電話番号
    is_active INTEGER NOT NULL DEFAULT 1,     -- アクティブフラグ
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (business_id) REFERENCES businesses(id),
    CHECK (contact_type IN ('customer', 'partner', 'vendor')),
    CHECK (is_active IN (0, 1))
);

-- インデックス
CREATE INDEX idx_external_contacts_business ON external_contacts(business_id);
CREATE INDEX idx_external_contacts_type ON external_contacts(contact_type);
CREATE INDEX idx_external_contacts_company ON external_contacts(company_name);
CREATE INDEX idx_external_contacts_active ON external_contacts(is_active);
```

### 2.4 システム管理関連テーブル

#### 2.4.1 file_check_exclusions (存在確認除外設定)

```sql
CREATE TABLE file_check_exclusions (
    id INTEGER PRIMARY KEY,                    -- 除外設定ID
    exclusion_date TEXT NOT NULL,             -- 除外基準日 (この日以前を除外)
    department_id INTEGER NULL,               -- 対象部署 (NULL=全部署)
    reason TEXT NOT NULL,                     -- 除外理由
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (department_id) REFERENCES departments(id)
);

-- インデックス
CREATE INDEX idx_exclusions_date ON file_check_exclusions(exclusion_date);
CREATE INDEX idx_exclusions_dept ON file_check_exclusions(department_id);
```

#### 2.4.2 system_logs (システムログ)

```sql
CREATE TABLE system_logs (
    id INTEGER PRIMARY KEY,                    -- ログID
    log_level TEXT NOT NULL,                  -- ログレベル: 'ERROR' | 'WARN' | 'INFO' | 'DEBUG'
    module TEXT NOT NULL,                     -- モジュール名
    message TEXT NOT NULL,                    -- ログメッセージ
    details TEXT NULL,                        -- 詳細情報 (JSON)
    user_id INTEGER NULL,                     -- 関連ユーザーID
    document_id INTEGER NULL,                 -- 関連文書ID
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    FOREIGN KEY (user_id) REFERENCES employees(id),
    FOREIGN KEY (document_id) REFERENCES documents(id),
    CHECK (log_level IN ('ERROR', 'WARN', 'INFO', 'DEBUG'))
);

-- インデックス
CREATE INDEX idx_logs_level ON system_logs(log_level);
CREATE INDEX idx_logs_module ON system_logs(module);
CREATE INDEX idx_logs_created ON system_logs(created_at);
CREATE INDEX idx_logs_user ON system_logs(user_id);
```

## 3. SQL Server移行対応

### 3.1 データ型変換マッピング

```sql
-- SQLite → SQL Server 変換例
-- 
-- SQLite:
-- CREATE TABLE documents (
--     id INTEGER PRIMARY KEY,
--     title TEXT NOT NULL,
--     created_at TEXT NOT NULL DEFAULT (datetime('now'))
-- );
--
-- SQL Server:
CREATE TABLE documents (
    id BIGINT IDENTITY(1,1) PRIMARY KEY,
    title NVARCHAR(255) NOT NULL,
    created_at DATETIME2 NOT NULL DEFAULT GETUTCDATE()
);
```

### 3.2 制約・インデックス変換

```sql
-- SQLite制約をSQL Server制約に変換
-- CHECK制約、UNIQUE制約は同様に動作
-- FOREIGN KEY制約も同様に動作

-- SQLiteのINTEGER PRIMARY KEYは自動的にBIGINT IDENTITYに変換
-- TEXTフィールドは適切なNVARCHARサイズに変換
-- 日時フィールドはDATETIME2に変換し、UTCで統一
```

### 3.3 マイグレーションスクリプト

```sql
-- データ移行用のSQLxマイグレーション
-- migrations/001_initial_schema.sql (SQLite用)
-- migrations/001_initial_schema_mssql.sql (SQL Server用)
-- 
-- 共通の構造を保ちながら、データ型のみ変換
-- SQLxのfeature flagでデータベース切り替え対応
```

## 4. パフォーマンス最適化

### 4.1 インデックス戦略

- **複合インデックス**: よく一緒に検索される列を組み合わせ
- **カバリングインデックス**: SELECT対象列をインデックスに含める
- **部分インデックス**: WHERE条件の多い列に特化

### 4.2 クエリ最適化

```sql
-- 効率的な文書検索クエリ例
SELECT d.id, d.number, d.title, d.created_date,
       dt.name as document_type_name,
       e.name as creator_name
FROM documents d
INNER JOIN document_types dt ON d.document_type_id = dt.id
INNER JOIN employees e ON d.created_by = e.id
WHERE d.is_active = 1
  AND d.created_date >= ?
  AND d.document_type_id = ?
ORDER BY d.created_date DESC
LIMIT 50;
```

### 4.3 接続管理

```rust
// SQLxプール設定例
let pool = SqlitePoolOptions::new()
    .max_connections(10)           // 最大10接続 (同時ユーザー数制限対応)
    .min_connections(2)            // 最小2接続維持
    .acquire_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(300))
    .connect(&database_url)
    .await?;
```

---

**最終更新**: 2024年12月  
**作成者**: 開発チーム  
**承認者**: プロジェクトマネージャー
