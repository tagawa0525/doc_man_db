# API設計書

## 1. API概要

### 1.1 設計方針

- **GraphQL優先**: 主要なデータ操作はGraphQL経由
- **RESTful補完**: ファイル操作・認証など特殊用途でREST併用
- **型安全性**: Rust型システムとの完全整合
- **エラーハンドリング**: 統一的なエラー処理・レスポンス

### 1.2 APIアーキテクチャ

```text
[Client (SvelteKit/Tauri)]
           ↓ HTTP/HTTPS
    [Axum Web Server]
      ↓         ↓
[GraphQL API] [REST API]
      ↓         ↓
   [Service Layer]
      ↓
   [Repository Layer]
      ↓
   [SQLx + Database]
```

### 1.3 技術スタック

```toml
[dependencies]
axum = "0.7"                    # Web framework
async-graphql-axum = "7.0"      # GraphQL integration
tower = "0.4"                   # Middleware
tower-http = "0.5"              # HTTP utilities
serde = { version = "1.0", features = ["derive"] }
sqlx = { version = "0.7", features = ["sqlite", "chrono", "uuid"] }
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"               # Error handling
tracing = "0.1"                 # Logging
```

## 2. GraphQL API エンドポイント

### 2.1 GraphQL メインエンドポイント

```rust
// POST /graphql
// Content-Type: application/json

// リクエスト例
{
  "query": "query GetDocuments($search: DocumentSearchInput) { documents(search: $search) { documents { id number title createdDate } totalCount } }",
  "variables": {
    "search": {
      "title": "会議",
      "pagination": { "limit": 10, "offset": 0 }
    }
  }
}

// レスポンス例
{
  "data": {
    "documents": {
      "documents": [
        {
          "id": "1",
          "number": "CTA-2508001",
          "title": "月次会議議事録",
          "createdDate": "2024-12-15"
        }
      ],
      "totalCount": 1
    }
  }
}
```

### 2.2 GraphQL Playground

```rust
// GET /graphql/playground
// 開発環境でのGraphQLクエリテスト用UI
```

## 3. REST API エンドポイント

### 3.1 認証関連API

#### 3.1.1 現在ユーザー情報取得

```rust
// GET /api/auth/me
// 認証: Windows統合認証 or Bearer Token

// レスポンス
{
  "user": {
    "id": 1,
    "employeeId": "12345",
    "name": "田中太郎",
    "email": "tanaka@example.com",
    "departments": [
      {
        "id": 1,
        "code": "T",
        "name": "技術部",
        "role": "primary"
      }
    ],
    "permissions": {
      "canCreateDocuments": true,
      "canUpdateDocuments": true,
      "canDeleteDocuments": false,
      "canManageUsers": false,
      "accessibleDepartments": [1, 2],
      "maxConfidentialityLevel": {
        "internalExternal": "internal",
        "importanceClass": "class2",
        "personalInfo": "none"
      }
    }
  }
}
```

#### 3.1.2 ログアウト

```rust
// POST /api/auth/logout
// レスポンス
{
  "success": true,
  "message": "Logged out successfully"
}
```

### 3.2 ファイル操作API

#### 3.2.1 ネットワークパス生成

```rust
// POST /api/files/generate-path
// Content-Type: application/json

// リクエスト
{
  "documentNumber": "CTA-2508001",
  "documentTypeId": 1,
  "departmentId": 1,
  "createdDate": "2024-12-15"
}

// レスポンス
{
  "success": true,
  "networkPath": "\\\\server01\\docs\\2024\\技術部\\報告書\\CTA-2508001",
  "ruleApplied": "技術部標準パス2024",
  "folderExists": false
}
```

#### 3.2.2 ファイル存在確認

```rust
// POST /api/files/check-existence
// Content-Type: application/json

// リクエスト
{
  "documentIds": [1, 2, 3],
  "checkApprovalFiles": true
}

// レスポンス
{
  "results": [
    {
      "documentId": 1,
      "networkPath": "\\\\server01\\docs\\2024\\技術部\\報告書\\CTA-2508001",
      "folderExists": true,
      "approvalFileExists": false,
      "checkedAt": "2024-12-15T10:30:00Z",
      "errors": []
    }
  ]
}
```

#### 3.2.3 フォルダオープン（Tauri専用）

```rust
// POST /api/files/open-folder
// Content-Type: application/json
// 注: Tauriアプリからのみアクセス可能

// リクエスト
{
  "networkPath": "\\\\server01\\docs\\2024\\技術部\\報告書\\CTA-2508001"
}

// レスポンス
{
  "success": true,
  "message": "Folder opened successfully"
}
```

### 3.3 データ管理API

#### 3.3.1 CSVインポート

```rust
// POST /api/data/import/csv
// Content-Type: multipart/form-data

// FormData:
// - file: CSV file
// - type: "documents" | "employees" | "businesses"
// - options: JSON string with import options

// レスポンス
{
  "success": true,
  "imported": 150,
  "skipped": 5,
  "errors": [
    {
      "row": 23,
      "message": "Invalid document number format",
      "data": "CTA-25080XX"
    }
  ]
}
```

#### 3.3.2 データエクスポート

```rust
// POST /api/data/export
// Content-Type: application/json

// リクエスト
{
  "type": "documents",
  "format": "csv" | "json",
  "filters": {
    "dateFrom": "2024-01-01",
    "dateTo": "2024-12-31",
    "departmentIds": [1, 2]
  }
}

// レスポンス (CSV)
// Content-Type: text/csv
// Content-Disposition: attachment; filename="documents_export_20241215.csv"
```

### 3.4 システム管理API

#### 3.4.1 ヘルスチェック

```rust
// GET /api/health

// レスポンス
{
  "status": "healthy",
  "version": "1.0.0",
  "database": {
    "status": "connected",
    "connections": 3,
    "responseTime": "15ms"
  },
  "fileSystem": {
    "networkDriveAccessible": true,
    "lastCheck": "2024-12-15T10:30:00Z"
  }
}
```

#### 3.4.2 システム統計

```rust
// GET /api/stats

// レスポンス
{
  "documents": {
    "total": 5234,
    "active": 4987,
    "thisMonth": 125
  },
  "employees": {
    "total": 45,
    "active": 42
  },
  "businesses": {
    "total": 167,
    "active": 23
  },
  "fileChecks": {
    "lastRun": "2024-12-15T02:00:00Z",
    "totalChecked": 4987,
    "foundIssues": 12
  }
}
```

#### 3.4.3 AD同期

```rust
// POST /api/admin/sync-ad
// 認証: 管理者権限必要

// レスポンス
{
  "success": true,
  "syncedUsers": 45,
  "newUsers": 2,
  "updatedUsers": 3,
  "deactivatedUsers": 1,
  "errors": []
}
```

## 4. エラーハンドリング

### 4.1 エラー型定義

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Authentication required")]
    Unauthorized,
    
    #[error("Insufficient permissions")]
    Forbidden,
    
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Network drive access error: {path}")]
    NetworkDrive { path: String },
    
    #[error("Business logic error: {message}")]
    BusinessLogic { message: String },
    
    #[error("Internal server error: {message}")]
    Internal { message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            ApiError::Validation { message } => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message)
            }
            ApiError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "Authentication required".to_string())
            }
            ApiError::Forbidden => {
                (StatusCode::FORBIDDEN, "FORBIDDEN", "Insufficient permissions".to_string())
            }
            ApiError::NotFound { resource } => {
                (StatusCode::NOT_FOUND, "NOT_FOUND", format!("Resource not found: {}", resource))
            }
            ApiError::NetworkDrive { path } => {
                (StatusCode::SERVICE_UNAVAILABLE, "NETWORK_ERROR", format!("Network drive access failed: {}", path))
            }
            ApiError::BusinessLogic { message } => {
                (StatusCode::UNPROCESSABLE_ENTITY, "BUSINESS_LOGIC_ERROR", message)
            }
            ApiError::Database(_) | ApiError::Internal { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Internal server error".to_string())
            }
        };

        let body = Json(serde_json::json!({
            "error": {
                "code": error_code,
                "message": message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }));

        (status, body).into_response()
    }
}
```

### 4.2 GraphQLエラーハンドリング

```rust
use async_graphql::*;

// GraphQLエラー拡張
impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        let extensions = match &err {
            ApiError::Validation { .. } => {
                ErrorExtensions::new().set("code", "VALIDATION_ERROR")
            }
            ApiError::Unauthorized => {
                ErrorExtensions::new().set("code", "UNAUTHORIZED")
            }
            ApiError::Forbidden => {
                ErrorExtensions::new().set("code", "FORBIDDEN")
            }
            ApiError::NotFound { .. } => {
                ErrorExtensions::new().set("code", "NOT_FOUND")
            }
            _ => ErrorExtensions::new().set("code", "INTERNAL_ERROR"),
        };

        Error::new(err.to_string()).extend_with(|_, e| *e = extensions)
    }
}
```

## 5. ミドルウェア

### 5.1 認証ミドルウェア

```rust
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Windows統合認証またはJWTトークン検証
    let auth_header = request.headers().get(AUTHORIZATION);
    
    match auth_header {
        Some(header) => {
            // Bearer Token処理
            if let Ok(token) = header.to_str() {
                if token.starts_with("Bearer ") {
                    let jwt_token = &token[7..];
                    // JWT検証処理
                    validate_jwt_token(jwt_token)?;
                }
            }
        }
        None => {
            // Windows統合認証処理
            let user = extract_windows_user(&request)?;
            // ユーザー情報をcontextに設定
        }
    }
    
    Ok(next.run(request).await)
}

async fn validate_jwt_token(token: &str) -> Result<Employee, ApiError> {
    // JWT検証・ユーザー情報取得
    todo!()
}

async fn extract_windows_user(request: &Request) -> Result<Employee, ApiError> {
    // Windows統合認証からユーザー情報取得
    todo!()
}
```

### 5.2 ログミドルウェア

```rust
use tracing::{info, warn};

pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();
    
    let response = next.run(request).await;
    
    let status = response.status();
    let elapsed = start.elapsed();
    
    match status.as_u16() {
        200..=299 => info!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?elapsed,
            "Request completed"
        ),
        400..=499 => warn!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?elapsed,
            "Client error"
        ),
        500..=599 => tracing::error!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?elapsed,
            "Server error"
        ),
        _ => info!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?elapsed,
            "Request completed"
        ),
    }
    
    response
}
```

### 5.3 CORS ミドルウェア

```rust
use tower_http::cors::{CorsLayer, Any};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)  // 開発時: Any、本番時: 特定ドメイン
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .max_age(Duration::from_secs(3600))
}
```

## 6. レート制限

### 6.1 レート制限ミドルウェア

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    limits: Arc<Mutex<HashMap<String, (Instant, u32)>>>,
    max_requests: u32,
    window_size: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_size: Duration) -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_size,
        }
    }
    
    pub async fn check_rate_limit(&self, key: &str) -> Result<(), ApiError> {
        let mut limits = self.limits.lock().unwrap();
        let now = Instant::now();
        
        match limits.get_mut(key) {
            Some((last_reset, count)) => {
                if now.duration_since(*last_reset) > self.window_size {
                    *last_reset = now;
                    *count = 1;
                } else {
                    *count += 1;
                    if *count > self.max_requests {
                        return Err(ApiError::BusinessLogic {
                            message: "Rate limit exceeded".to_string(),
                        });
                    }
                }
            }
            None => {
                limits.insert(key.to_string(), (now, 1));
            }
        }
        
        Ok(())
    }
}
```

## 7. APIルーティング

### 7.1 ルーター構成

```rust
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(pool: sqlx::SqlitePool) -> Router {
    let graphql_schema = build_schema(pool.clone());
    
    Router::new()
        // GraphQL
        .route("/graphql", post(graphql_handler))
        .route("/graphql/playground", get(graphql_playground))
        
        // REST API - 認証
        .route("/api/auth/me", get(auth_me))
        .route("/api/auth/logout", post(auth_logout))
        
        // REST API - ファイル操作
        .route("/api/files/generate-path", post(files_generate_path))
        .route("/api/files/check-existence", post(files_check_existence))
        .route("/api/files/open-folder", post(files_open_folder))
        
        // REST API - データ管理
        .route("/api/data/import/csv", post(data_import_csv))
        .route("/api/data/export", post(data_export))
        
        // REST API - システム管理
        .route("/api/health", get(health_check))
        .route("/api/stats", get(system_stats))
        .route("/api/admin/sync-ad", post(admin_sync_ad))
        
        // 静的ファイル（SvelteKitアプリ）
        .nest_service("/", ServeDir::new("dist"))
        
        // ミドルウェア
        .layer(cors_layer())
        .layer(axum::middleware::from_fn(logging_middleware))
        .layer(axum::middleware::from_fn(auth_middleware))
        .with_state(AppState { pool, graphql_schema })
}
```

### 7.2 状態管理

```rust
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub graphql_schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
}
```

## 8. バリデーション

### 8.1 入力バリデーション

```rust
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct CreateDocumentRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
    pub title: String,
    
    #[validate(custom = "validate_document_number")]
    pub number: Option<String>,
    
    #[validate(email)]
    pub creator_email: Option<String>,
}

fn validate_document_number(number: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^[A-Z]{1,3}-\d{4,8}$").unwrap();
    if re.is_match(number) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid document number format"))
    }
}

// 使用例
pub async fn create_document_handler(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<CreateDocumentRequest>,
) -> Result<Json<Document>, ApiError> {
    // requestは既にバリデーション済み
    let document = DocumentService::create(&state.pool, request).await?;
    Ok(Json(document))
}
```

### 8.2 レスポンス型

```rust
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiErrorResponse>,
    pub meta: Option<ResponseMeta>,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct ResponseMeta {
    pub timestamp: String,
    pub request_id: String,
    pub version: String,
}
```

## 9. テスト戦略

### 9.1 API統合テスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    
    #[tokio::test]
    async fn test_create_document() {
        let app = create_test_app().await;
        let server = TestServer::new(app).unwrap();
        
        let response = server
            .post("/api/documents")
            .json(&serde_json::json!({
                "title": "Test Document",
                "documentTypeId": 1,
                "createdDate": "2024-12-15"
            }))
            .await;
        
        response.assert_status_ok();
        
        let document: Document = response.json();
        assert_eq!(document.title, "Test Document");
    }
    
    #[tokio::test]
    async fn test_graphql_query() {
        let app = create_test_app().await;
        let server = TestServer::new(app).unwrap();
        
        let query = r#"
            query {
                documents(search: { title: "Test" }) {
                    documents {
                        id
                        title
                    }
                    totalCount
                }
            }
        "#;
        
        let response = server
            .post("/graphql")
            .json(&serde_json::json!({
                "query": query
            }))
            .await;
        
        response.assert_status_ok();
    }
    
    async fn create_test_app() -> Router {
        let pool = create_test_database().await;
        create_router(pool)
    }
}
```

---

**最終更新**: 2024年12月  
**作成者**: 開発チーム  
**承認者**: プロジェクトマネージャー
