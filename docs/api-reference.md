# 文書管理システム API リファレンス

## 目次

1. [API概要](#api概要)
2. [認証・認可](#認証認可)
3. [共通仕様](#共通仕様)
4. [エンドポイント一覧](#エンドポイント一覧)
5. [GraphQL API](#graphql-api)
6. [エラーハンドリング](#エラーハンドリング)
7. [レート制限](#レート制限)
8. [SDK・ライブラリ](#sdkライブラリ)

---

## API概要

### ベースURL

- **開発環境**: `http://localhost:8080`
- **本番環境**: `https://docman.company.com`

### APIバージョン

- **現在のバージョン**: v1
- **サポート終了予定**: なし（LTS）

### プロトコル

- **REST API**: `/api/v1/*`
- **GraphQL**: `/graphql`
- **WebSocket**: `/ws` (リアルタイム通知)

---

## 認証・認可

### 認証方式

#### JWT Token認証

```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "username": "user@company.com",
  "password": "password"
}
```

**レスポンス**:

```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 3600,
  "token_type": "Bearer"
}
```

#### Windows認証

```http
GET /api/v1/auth/windows
Authorization: Negotiate ...
```

### 認可ヘッダー

```http
Authorization: Bearer <access_token>
```

### リフレッシュトークン

```http
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

---

## 共通仕様

### HTTPステータスコード

| コード | 意味                  | 説明                             |
| ------ | --------------------- | -------------------------------- |
| 200    | OK                    | 正常処理                         |
| 201    | Created               | リソース作成成功                 |
| 204    | No Content            | 処理成功（レスポンスボディなし） |
| 400    | Bad Request           | リクエスト不正                   |
| 401    | Unauthorized          | 認証失敗                         |
| 403    | Forbidden             | 認可失敗                         |
| 404    | Not Found             | リソース未発見                   |
| 409    | Conflict              | リソース競合                     |
| 422    | Unprocessable Entity  | 入力検証エラー                   |
| 429    | Too Many Requests     | レート制限超過                   |
| 500    | Internal Server Error | サーバー内部エラー               |

### レスポンス形式

#### 成功レスポンス

```json
{
  "success": true,
  "data": {
    // レスポンスデータ
  },
  "meta": {
    "timestamp": "2024-12-21T12:00:00Z",
    "request_id": "req_123456789"
  }
}
```

#### エラーレスポンス

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "入力値が不正です",
    "details": [
      {
        "field": "title",
        "message": "タイトルは必須です"
      }
    ]
  },
  "meta": {
    "timestamp": "2024-12-21T12:00:00Z",
    "request_id": "req_123456789"
  }
}
```

### ページネーション

```json
{
  "data": [...],
  "meta": {
    "pagination": {
      "page": 1,
      "per_page": 20,
      "total": 100,
      "total_pages": 5,
      "has_next": true,
      "has_prev": false
    }
  }
}
```

### フィルタリング・ソート

```http
GET /api/v1/documents?title=技術&sort=created_at:desc&page=1&per_page=20
```

---

## エンドポイント一覧

### 認証関連

#### ログイン

```http
POST /api/v1/auth/login
```

**リクエスト**:

```json
{
  "username": "string",
  "password": "string"
}
```

**レスポンス**:

```json
{
  "success": true,
  "data": {
    "access_token": "string",
    "refresh_token": "string",
    "expires_in": 3600,
    "user": {
      "id": 1,
      "username": "user@company.com",
      "display_name": "ユーザー名",
      "role": "user",
      "permissions": ["read", "write"]
    }
  }
}
```

#### ログアウト

```http
POST /api/v1/auth/logout
Authorization: Bearer <token>
```

#### ユーザー情報取得

```http
GET /api/v1/auth/me
Authorization: Bearer <token>
```

### 文書管理

#### 文書一覧取得

```http
GET /api/v1/documents
Authorization: Bearer <token>
```

**クエリパラメータ**:

- `title`: タイトル検索（部分一致）
- `document_type_id`: 文書種別ID
- `created_by`: 作成者ID
- `created_date_from`: 作成日（開始）
- `created_date_to`: 作成日（終了）
- `confidentiality`: 機密度レベル
- `is_active`: 有効フラグ
- `page`: ページ番号（default: 1）
- `per_page`: 1ページあたり件数（default: 20, max: 100）
- `sort`: ソート条件（例: `created_at:desc`）

**レスポンス**:

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "title": "技術仕様書",
      "document_number": "技術-25001",
      "document_type": {
        "id": 1,
        "name": "技術文書",
        "code": "technical"
      },
      "created_by": {
        "id": 1,
        "name": "山田太郎"
      },
      "created_date": "2024-12-21",
      "confidentiality": {
        "internal_external": "Internal",
        "importance_class": "Class2",
        "personal_info": "None"
      },
      "notes": "技術仕様に関する文書",
      "is_active": true,
      "created_at": "2024-12-21T12:00:00Z",
      "updated_at": "2024-12-21T12:00:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "page": 1,
      "per_page": 20,
      "total": 50,
      "total_pages": 3,
      "has_next": true,
      "has_prev": false
    }
  }
}
```

#### 文書詳細取得

```http
GET /api/v1/documents/{id}
Authorization: Bearer <token>
```

#### 文書作成

```http
POST /api/v1/documents
Authorization: Bearer <token>
Content-Type: application/json
```

**リクエスト**:

```json
{
  "title": "新規技術文書",
  "document_type_id": 1,
  "confidentiality": {
    "internal_external": "Internal",
    "importance_class": "Class2",
    "personal_info": "None"
  },
  "notes": "備考欄"
}
```

#### 文書更新

```http
PUT /api/v1/documents/{id}
Authorization: Bearer <token>
Content-Type: application/json
```

**リクエスト**:

```json
{
  "title": "更新されたタイトル",
  "notes": "更新された備考"
}
```

#### 文書削除

```http
DELETE /api/v1/documents/{id}
Authorization: Bearer <token>
```

### 回覧・承認機能

#### 回覧一覧取得

```http
GET /api/v1/circulations
Authorization: Bearer <token>
```

**クエリパラメータ**:

- `status`: 回覧状況（active, completed, cancelled）
- `document_id`: 文書ID
- `workflow_id`: ワークフローID
- `assignee_id`: 担当者ID

**レスポンス**:

```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "document": {
        "id": 1,
        "title": "技術仕様書",
        "document_number": "技術-25001"
      },
      "workflow": {
        "id": 1,
        "name": "標準承認フロー"
      },
      "status": "active",
      "current_step": {
        "id": 1,
        "step_order": 1,
        "assignee": {
          "id": 2,
          "name": "承認者"
        },
        "status": "pending"
      },
      "initiated_by": {
        "id": 1,
        "name": "申請者"
      },
      "notes": "承認をお願いします",
      "created_at": "2024-12-21T12:00:00Z"
    }
  ]
}
```

#### 回覧開始

```http
POST /api/v1/circulations
Authorization: Bearer <token>
Content-Type: application/json
```

**リクエスト**:

```json
{
  "document_id": 1,
  "workflow_id": 1,
  "notes": "承認をお願いします"
}
```

#### 承認・差し戻し

```http
POST /api/v1/circulations/{circulation_id}/steps/{step_id}/complete
Authorization: Bearer <token>
Content-Type: application/json
```

**リクエスト**:

```json
{
  "action": "approve",  // approve, reject, request_changes
  "comments": "承認いたします"
}
```

### 検索機能

#### 文書検索

```http
POST /api/v1/search/documents
Authorization: Bearer <token>
Content-Type: application/json
```

**リクエスト**:

```json
{
  "query": "技術",
  "filters": {
    "document_type_ids": [1, 2],
    "created_date_from": "2024-01-01",
    "created_date_to": "2024-12-31",
    "confidentiality": ["Class2", "Class3"],
    "created_by_ids": [1, 2]
  },
  "sort": {
    "field": "created_at",
    "order": "desc"
  },
  "pagination": {
    "page": 1,
    "per_page": 20
  }
}
```

#### 検索候補取得

```http
GET /api/v1/search/suggestions?q=技術
Authorization: Bearer <token>
```

**レスポンス**:

```json
{
  "success": true,
  "data": {
    "suggestions": [
      "技術仕様書",
      "技術検討書",
      "技術資料"
    ]
  }
}
```

### システム管理

#### システム状態確認

```http
GET /api/v1/system/health
```

**レスポンス**:

```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "1.0.0",
    "uptime": 86400,
    "database": {
      "status": "connected",
      "response_time_ms": 5
    },
    "cache": {
      "status": "connected",
      "hit_rate": 0.95
    }
  }
}
```

#### システムメトリクス

```http
GET /api/v1/system/metrics
Authorization: Bearer <admin_token>
```

**レスポンス**:

```json
{
  "success": true,
  "data": {
    "api": {
      "total_requests": 1000,
      "successful_requests": 950,
      "failed_requests": 50,
      "average_response_time_ms": 120
    },
    "database": {
      "total_queries": 5000,
      "slow_queries": 5,
      "average_query_time_ms": 50
    },
    "memory": {
      "heap_used_mb": 256,
      "heap_total_mb": 512
    }
  }
}
```

---

## GraphQL API

### エンドポイント

```curl
POST /graphql
Authorization: Bearer <token>
Content-Type: application/json
```

### スキーマ

#### Document型

```graphql
type Document {
  id: ID!
  title: String!
  documentNumber: String!
  documentType: DocumentType!
  createdBy: User!
  createdDate: Date!
  confidentiality: Confidentiality!
  notes: String
  isActive: Boolean!
  createdAt: DateTime!
  updatedAt: DateTime!
  
  # リレーション
  circulations: [Circulation!]!
  files: [DocumentFile!]!
}

type DocumentType {
  id: ID!
  name: String!
  code: String!
  prefix: String!
  isActive: Boolean!
}

type Confidentiality {
  internalExternal: String!
  importanceClass: String!
  personalInfo: String!
}

type User {
  id: ID!
  username: String!
  displayName: String!
  role: String!
  isActive: Boolean!
}
```

#### クエリ例

```graphql
query GetDocuments($filter: DocumentFilter, $pagination: PaginationInput) {
  documents(filter: $filter, pagination: $pagination) {
    data {
      id
      title
      documentNumber
      documentType {
        name
        code
      }
      createdBy {
        displayName
      }
      confidentiality {
        importanceClass
      }
      createdDate
    }
    meta {
      pagination {
        total
        totalPages
        hasNext
      }
    }
  }
}
```

#### ミューテーション例

```graphql
mutation CreateDocument($input: CreateDocumentInput!) {
  createDocument(input: $input) {
    id
    title
    documentNumber
    createdAt
  }
}
```

### リアルタイム通知（Subscription）

```graphql
subscription CirculationUpdates($userId: ID!) {
  circulationUpdates(userId: $userId) {
    type  # STEP_ASSIGNED, STEP_COMPLETED, CIRCULATION_COMPLETED
    circulation {
      id
      document {
        title
      }
      currentStep {
        assignee {
          displayName
        }
      }
    }
  }
}
```

---

## エラーハンドリング

### エラーコード一覧

| コード                    | HTTPステータス | 説明                   |
| ------------------------- | -------------- | ---------------------- |
| `AUTHENTICATION_REQUIRED` | 401            | 認証が必要             |
| `INVALID_CREDENTIALS`     | 401            | 認証情報が不正         |
| `ACCESS_DENIED`           | 403            | アクセス権限なし       |
| `RESOURCE_NOT_FOUND`      | 404            | リソースが見つからない |
| `VALIDATION_ERROR`        | 422            | 入力検証エラー         |
| `DUPLICATE_RESOURCE`      | 409            | リソースの重複         |
| `RATE_LIMIT_EXCEEDED`     | 429            | レート制限超過         |
| `INTERNAL_ERROR`          | 500            | サーバー内部エラー     |
| `DATABASE_ERROR`          | 500            | データベースエラー     |
| `EXTERNAL_SERVICE_ERROR`  | 502            | 外部サービスエラー     |

### エラー詳細例

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "入力値にエラーがあります",
    "details": [
      {
        "field": "title",
        "code": "REQUIRED",
        "message": "タイトルは必須です"
      },
      {
        "field": "document_type_id",
        "code": "INVALID",
        "message": "存在しない文書種別IDです"
      }
    ]
  },
  "meta": {
    "timestamp": "2024-12-21T12:00:00Z",
    "request_id": "req_123456789"
  }
}
```

---

## レート制限

### 制限値

| エンドポイント | 制限     | 時間枠       |
| -------------- | -------- | ------------ |
| 認証API        | 10回/分  | ユーザー単位 |
| 文書検索API    | 100回/分 | ユーザー単位 |
| 文書操作API    | 60回/分  | ユーザー単位 |
| その他API      | 200回/分 | ユーザー単位 |

### レスポンスヘッダー

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1640084400
Retry-After: 60
```

### 制限超過時のレスポンス

```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "レート制限を超過しました。しばらく待ってから再試行してください。",
    "details": {
      "limit": 100,
      "reset_time": "2024-12-21T12:01:00Z"
    }
  }
}
```

---

## SDK・ライブラリ

### JavaScript/TypeScript SDK

#### インストール

```bash
npm install @company/doc-man-db-sdk
```

#### 使用例

```typescript
import { DocManDBClient } from '@company/doc-man-db-sdk';

const client = new DocManDBClient({
  baseUrl: 'https://docman.company.com',
  apiKey: 'your-api-key'
});

// 文書一覧取得
const documents = await client.documents.list({
  title: '技術',
  page: 1,
  perPage: 20
});

// 文書作成
const newDocument = await client.documents.create({
  title: '新規文書',
  documentTypeId: 1,
  confidentiality: {
    internalExternal: 'Internal',
    importanceClass: 'Class2',
    personalInfo: 'None'
  }
});

// 回覧開始
const circulation = await client.circulations.create({
  documentId: newDocument.id,
  workflowId: 1,
  notes: '承認をお願いします'
});
```

### Python SDK

#### インストール

```bash
pip install doc-man-db-python-sdk
```

#### 使用例

```python
from doc_man_db_sdk import DocManDBClient

client = DocManDBClient(
    base_url='https://docman.company.com',
    api_key='your-api-key'
)

# 文書一覧取得
documents = client.documents.list(
    title='技術',
    page=1,
    per_page=20
)

# 文書作成
new_document = client.documents.create({
    'title': '新規文書',
    'document_type_id': 1,
    'confidentiality': {
        'internal_external': 'Internal',
        'importance_class': 'Class2',
        'personal_info': 'None'
    }
})
```

### cURL例

#### 認証

```bash
# ログイン
curl -X POST https://docman.company.com/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "user@company.com",
    "password": "password"
  }'
```

#### 文書一覧取得

```bash
curl -X GET "https://docman.company.com/api/v1/documents?title=技術&page=1&per_page=20" \
  -H "Authorization: Bearer <access_token>"
```

#### 文書作成

```bash
curl -X POST https://docman.company.com/api/v1/documents \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "新規技術文書",
    "document_type_id": 1,
    "confidentiality": {
      "internal_external": "Internal",
      "importance_class": "Class2",
      "personal_info": "None"
    },
    "notes": "技術仕様に関する文書"
  }'
```

---

## Webhook

### 設定

```http
POST /api/v1/webhooks
Authorization: Bearer <admin_token>
Content-Type: application/json
```

**リクエスト**:

```json
{
  "url": "https://your-app.com/webhooks/docman",
  "events": ["document.created", "circulation.completed"],
  "secret": "webhook-secret"
}
```

### イベント一覧

- `document.created`: 文書作成時
- `document.updated`: 文書更新時
- `document.deleted`: 文書削除時
- `circulation.started`: 回覧開始時
- `circulation.completed`: 回覧完了時
- `step.assigned`: ステップ割り当て時
- `step.completed`: ステップ完了時

### ペイロード例

```json
{
  "id": "evt_123456789",
  "type": "document.created",
  "data": {
    "document": {
      "id": 1,
      "title": "新規文書",
      "created_by": {
        "id": 1,
        "name": "作成者"
      }
    }
  },
  "created_at": "2024-12-21T12:00:00Z"
}
```

---

## 変更履歴

| バージョン | 日付       | 変更内容     |
| ---------- | ---------- | ------------ |
| v1.0.0     | 2024-12-21 | 初版リリース |

---

**APIに関するお問い合わせ**

- 開発チーム: dev-team@company.com
- API仕様: https://api-docs.company.com
- サポート: api-support@company.com
