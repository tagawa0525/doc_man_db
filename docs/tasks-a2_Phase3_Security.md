# Phase 3: 認証・セキュリティ実装 (Week 7-8)

## フェーズ概要

- **期間**: Week 7-8 (2週間)
- **目標**: 認証・認可システム、セキュリティ機能の実装
- **成果物**: 認証・認可システム、セキュリティ機能、監査ログ機能

## タスク一覧

### TASK-017: Windows認証実装

- **説明**: AD連携・認証ミドルウェア
- **優先度**: High
- **見積工数**: 16h
- **状態**: 未着手
- **依存関係**: TASK-005

#### 実装内容(TASK-017)

1. Windows AD連携機能
2. 認証ミドルウェア
3. JWT トークン管理
4. フォールバック認証（JSON）

#### ファイル構成

```text
src/auth/
├── mod.rs
├── ad_connector.rs      # AD接続
├── jwt_manager.rs       # JWT管理
├── middleware.rs        # 認証ミドルウェア
└── fallback.rs          # フォールバック認証
```

#### 実装例(TASK-017)

```rust
// src/auth/ad_connector.rs
use ldap3::{LdapConn, Scope, SearchEntry};

pub struct AdConnector {
    server_url: String,
    bind_dn: String,
    bind_password: String,
}

impl AdConnector {
    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AdUserInfo, AuthError> {
        let mut ldap = LdapConn::new(&self.server_url)?;
        
        // サービスアカウントでバインド
        ldap.simple_bind(&self.bind_dn, &self.bind_password)?;
        
        // ユーザー検索
        let user_dn = format!("CN={},OU=Users,DC=corp,DC=local", username);
        let search_result = ldap.search(
            &user_dn,
            Scope::Base,
            "(objectClass=person)",
            vec!["cn", "mail", "employeeID"]
        )?;
        
        if search_result.0.is_empty() {
            return Err(AuthError::UserNotFound);
        }
        
        // ユーザー認証
        match ldap.simple_bind(&user_dn, password) {
            Ok(_) => {
                let entry = SearchEntry::construct(search_result.0[0].clone());
                Ok(AdUserInfo {
                    username: username.to_string(),
                    display_name: entry.attr("cn")[0].clone(),
                    email: entry.attr("mail").get(0).cloned(),
                    employee_id: entry.attr("employeeID").get(0).cloned(),
                })
            }
            Err(_) => Err(AuthError::InvalidCredentials),
        }
    }
    
    pub async fn sync_users(&self) -> Result<Vec<AdUserInfo>, AuthError> {
        let mut ldap = LdapConn::new(&self.server_url)?;
        ldap.simple_bind(&self.bind_dn, &self.bind_password)?;
        
        let (rs, _res) = ldap.search(
            "OU=Users,DC=corp,DC=local",
            Scope::Subtree,
            "(objectClass=person)",
            vec!["cn", "mail", "employeeID", "sAMAccountName"]
        )?;
        
        let users = rs.into_iter()
            .map(|entry| {
                let entry = SearchEntry::construct(entry);
                AdUserInfo {
                    username: entry.attr("sAMAccountName")[0].clone(),
                    display_name: entry.attr("cn")[0].clone(),
                    email: entry.attr("mail").get(0).cloned(),
                    employee_id: entry.attr("employeeID").get(0).cloned(),
                }
            })
            .collect();
        
        Ok(users)
    }
}
```

#### JWT管理

```rust
// src/auth/jwt_manager.rs
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // ユーザーID
    pub name: String,     // 表示名
    pub email: Option<String>,
    pub employee_id: String,
    pub permissions: UserPermissions,
    pub exp: usize,       // 有効期限
    pub iat: usize,       // 発行時刻
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    algorithm: Algorithm,
}

impl JwtManager {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            algorithm: Algorithm::HS256,
        }
    }
    
    pub fn generate_token(&self, user: &AuthenticatedUser) -> Result<String, AuthError> {
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user.id.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            employee_id: user.employee_id.clone(),
            permissions: user.permissions.clone(),
            exp: now + 8 * 3600, // 8時間
            iat: now,
        };
        
        encode(&Header::new(self.algorithm), &claims, &self.encoding_key)
            .map_err(AuthError::JwtError)
    }
    
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::new(self.algorithm);
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(AuthError::JwtError)
    }
}
```

#### 認証ミドルウェア

```rust
// src/auth/middleware.rs
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(jwt_manager): State<JwtManager>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers.get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));
    
    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    
    let claims = match jwt_manager.validate_token(token) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // リクエストにユーザー情報を追加
    req.extensions_mut().insert(CurrentUser {
        id: claims.sub.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        name: claims.name,
        email: claims.email,
        employee_id: claims.employee_id,
        permissions: claims.permissions,
    });
    
    Ok(next.run(req).await)
}
```

#### 成果物(TASK-017)

- Windows AD連携機能
- JWT認証システム
- 認証ミドルウェア
- フォールバック認証

---

### TASK-018: 認可システム

- **説明**: 役割ベースアクセス制御
- **優先度**: High
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-017

#### 実装内容(TASK-018)

1. 役割ベースアクセス制御（RBAC）
2. 権限マトリックス定義
3. リソースレベル認可
4. 動的権限評価

#### データベーステーブル(TASK-017)

```sql
-- 役割定義
CREATE TABLE roles (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ユーザー役割
CREATE TABLE user_roles (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    granted_by INTEGER NOT NULL,
    granted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    is_active BOOLEAN DEFAULT 1,
    FOREIGN KEY (employee_id) REFERENCES employees (id),
    FOREIGN KEY (role_id) REFERENCES roles (id),
    FOREIGN KEY (granted_by) REFERENCES employees (id)
);

-- 権限定義
CREATE TABLE permissions (
    id INTEGER PRIMARY KEY,
    resource TEXT NOT NULL,  -- 'documents', 'employees', etc.
    action TEXT NOT NULL,    -- 'create', 'read', 'update', 'delete'
    conditions TEXT,         -- JSON条件
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 役割権限
CREATE TABLE role_permissions (
    id INTEGER PRIMARY KEY,
    role_id INTEGER NOT NULL,
    permission_id INTEGER NOT NULL,
    FOREIGN KEY (role_id) REFERENCES roles (id),
    FOREIGN KEY (permission_id) REFERENCES permissions (id),
    UNIQUE(role_id, permission_id)
);
```

#### 実装例(TASK-018)

```rust
// src/auth/authorization.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    pub can_create_documents: bool,
    pub can_update_documents: bool,
    pub can_delete_documents: bool,
    pub can_view_confidential: bool,
    pub can_manage_employees: bool,
    pub can_admin_system: bool,
    pub accessible_departments: Vec<i32>,
}

pub struct AuthorizationService {
    repository: Box<dyn AuthorizationRepository>,
}

impl AuthorizationService {
    pub async fn get_user_permissions(
        &self,
        employee_id: i32,
    ) -> Result<UserPermissions, AuthError> {
        let roles = self.repository.get_user_roles(employee_id).await?;
        let permissions = self.repository.get_role_permissions(&roles).await?;
        
        self.compile_permissions(permissions, employee_id).await
    }
    
    pub fn check_permission(
        &self,
        permissions: &UserPermissions,
        resource: &str,
        action: &str,
        context: Option<&serde_json::Value>,
    ) -> bool {
        match (resource, action) {
            ("documents", "create") => permissions.can_create_documents,
            ("documents", "update") => permissions.can_update_documents,
            ("documents", "delete") => permissions.can_delete_documents,
            ("documents", "read") => {
                if let Some(ctx) = context {
                    if let Some(confidential) = ctx.get("confidential") {
                        if confidential.as_bool().unwrap_or(false) {
                            return permissions.can_view_confidential;
                        }
                    }
                }
                true // 非機密文書は誰でも読める
            },
            ("employees", _) => permissions.can_manage_employees,
            ("system", _) => permissions.can_admin_system,
            _ => false,
        }
    }
}
```

#### 成果物(TASK-018)

- RBAC システム
- 権限マトリックス
- 動的権限評価
- 権限管理API

---

### TASK-019: 機密レベル制御

- **説明**: 機密レベル別表示制御
- **優先度**: High
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-018

#### 実装内容(TASK-019)

1. 機密レベル定義（3軸）
2. レベル別アクセス制御
3. 表示内容フィルタリング
4. 機密レベル履歴管理

#### 機密レベル定義

```rust
// src/models/enums.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InternalExternal {
    Internal,  // 社内
    External,  // 社外
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImportanceClass {
    Class1,    // 情報クラスⅠ（重要）
    Class2,    // 情報クラスⅡ（通常）
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PersonalInfo {
    None,      // 個人情報なし
    Present,   // 個人情報あり
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidentialityLevel {
    pub internal_external: InternalExternal,
    pub importance_class: ImportanceClass,
    pub personal_info: PersonalInfo,
}

impl ConfidentialityLevel {
    pub fn access_level(&self) -> u8 {
        let mut level = 0;
        
        if self.internal_external == InternalExternal::External {
            level += 1;
        }
        if self.importance_class == ImportanceClass::Class1 {
            level += 2;
        }
        if self.personal_info == PersonalInfo::Present {
            level += 4;
        }
        
        level
    }
    
    pub fn can_access(&self, user_clearance: u8) -> bool {
        self.access_level() <= user_clearance
    }
}
```

#### フィルタリング実装

```rust
// src/services/confidentiality_service.rs
impl DocumentService {
    pub async fn filter_by_confidentiality(
        &self,
        documents: Vec<Document>,
        user_permissions: &UserPermissions,
    ) -> Vec<Document> {
        documents.into_iter()
            .filter(|doc| {
                let conf_level = ConfidentialityLevel {
                    internal_external: doc.internal_external.clone(),
                    importance_class: doc.importance_class.clone(),
                    personal_info: doc.personal_info.clone(),
                };
                
                conf_level.can_access(user_permissions.clearance_level)
            })
            .map(|mut doc| {
                // ネットワークパス表示制御
                if !user_permissions.can_view_network_paths {
                    doc.network_path = None;
                }
                doc
            })
            .collect()
    }
}
```

#### 成果物(TASK-019)

- 機密レベル制御システム
- 3軸機密レベル定義
- アクセス制御機能
- 表示フィルタリング

---

### TASK-020: セッション管理

- **説明**: セッション・JWT管理
- **優先度**: Medium
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-017

#### 実装内容(TASK-020)

1. セッション管理
2. トークンリフレッシュ
3. セッション無効化
4. 同時セッション制限

#### データベーステーブル(TASK-020)

```sql
-- セッション管理
CREATE TABLE user_sessions (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    session_token TEXT NOT NULL UNIQUE,
    refresh_token TEXT UNIQUE,
    ip_address TEXT,
    user_agent TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    is_active BOOLEAN DEFAULT 1,
    FOREIGN KEY (employee_id) REFERENCES employees (id)
);
```

#### 成果物(TASK-020)

- セッション管理システム
- トークンリフレッシュ機能
- セキュアセッション制御

---

### TASK-021: 監査ログ

- **説明**: アクセス・操作ログ記録
- **優先度**: Medium
- **見積工数**: 10h
- **状態**: 未着手
- **依存関係**: TASK-018

#### 実装内容(TASK-021)

1. 操作ログ記録
2. アクセスログ記録
3. セキュリティイベント記録
4. ログ分析機能

#### データベーステーブル(TASK-021)

```sql
-- 監査ログ
CREATE TABLE audit_logs (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER,
    action TEXT NOT NULL,       -- 'create', 'update', 'delete', 'view'
    resource_type TEXT NOT NULL, -- 'document', 'employee'
    resource_id TEXT,
    old_values TEXT,            -- JSON
    new_values TEXT,            -- JSON
    ip_address TEXT,
    user_agent TEXT,
    success BOOLEAN NOT NULL,
    error_message TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (employee_id) REFERENCES employees (id)
);

-- セキュリティイベント
CREATE TABLE security_events (
    id INTEGER PRIMARY KEY,
    event_type TEXT NOT NULL,   -- 'login_failure', 'permission_denied'
    employee_id INTEGER,
    ip_address TEXT,
    details TEXT,               -- JSON
    severity TEXT,              -- 'low', 'medium', 'high'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### 実装例(TASK-021)

```rust
// src/services/audit_service.rs
pub struct AuditService {
    repository: Box<dyn AuditRepository>,
}

impl AuditService {
    pub async fn log_action(
        &self,
        user_id: i32,
        action: &str,
        resource_type: &str,
        resource_id: Option<&str>,
        old_values: Option<&serde_json::Value>,
        new_values: Option<&serde_json::Value>,
        ip_address: &str,
        success: bool,
        error: Option<&str>,
    ) -> Result<(), AuditError> {
        let log_entry = AuditLogEntry {
            employee_id: user_id,
            action: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: resource_id.map(String::from),
            old_values: old_values.map(|v| v.to_string()),
            new_values: new_values.map(|v| v.to_string()),
            ip_address: ip_address.to_string(),
            success,
            error_message: error.map(String::from),
        };
        
        self.repository.create_log_entry(log_entry).await
    }
    
    pub async fn log_security_event(
        &self,
        event_type: &str,
        employee_id: Option<i32>,
        ip_address: &str,
        details: &serde_json::Value,
        severity: SecuritySeverity,
    ) -> Result<(), AuditError> {
        let event = SecurityEvent {
            event_type: event_type.to_string(),
            employee_id,
            ip_address: ip_address.to_string(),
            details: details.to_string(),
            severity: severity.to_string(),
        };
        
        self.repository.create_security_event(event).await
    }
}
```

#### 成果物(TASK-021)

- 完全な監査ログシステム
- セキュリティイベント記録
- ログ分析基盤

---

### TASK-022: HTTPS設定

- **説明**: TLS設定・証明書管理・セキュリティヘッダー
- **優先度**: High
- **見積工数**: 6h
- **状態**: 未着手
- **依存関係**: TASK-017

#### 実装内容(TASK-022)

1. **TLS設定・証明書管理**
2. **HTTPS強制リダイレクト**  
3. **組断的セキュリティヘッダー**
4. **CSRF/XSS対策**

#### セキュリティヘッダー実装

```rust
// src/middleware/security.rs
use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
    extract::Request,
};

pub async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    
    // HTTPS強制 (HSTS)
    headers.insert(
        HeaderName::from_static("strict-transport-security"),
        HeaderValue::from_static("max-age=31536000; includeSubDomains; preload")
    );
    
    // XSS保護
    headers.insert(
        HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block")
    );
    
    // Content-Type スニッフィング防止
    headers.insert(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff")
    );
    
    // クリックジャッキング防止
    headers.insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY")
    );
    
    // Content Security Policy
    headers.insert(
        HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(
            "default-src 'self'; \
             script-src 'self' 'unsafe-inline'; \
             style-src 'self' 'unsafe-inline'; \
             img-src 'self' data: blob:; \
             connect-src 'self'; \
             font-src 'self'; \
             object-src 'none'; \
             media-src 'self'; \
             frame-src 'none';"
        )
    );
    
    // Referrer Policy
    headers.insert(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin")
    );
    
    // Permissions Policy
    headers.insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static(
            "camera=(), microphone=(), geolocation=(), \
             usb=(), magnetometer=(), accelerometer=(), \
             gyroscope=(), payment=()"
        )
    );
    
    Ok(response)
}

// CSRF保護ミドルウェア
pub async fn csrf_protection_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // POST, PUT, DELETEリクエストのCSRFトークン確認
    if matches!(request.method().as_str(), "POST" | "PUT" | "DELETE") {
        let csrf_token = headers.get("x-csrf-token")
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::FORBIDDEN)?;
        
        // CSRFトークン検証
        if !validate_csrf_token(csrf_token) {
            return Err(StatusCode::FORBIDDEN);
        }
    }
    
    Ok(next.run(request).await)
}

fn validate_csrf_token(token: &str) -> bool {
    // CSRFトークンの検証ロジック
    // 実際にはセッションやJWTから生成されたトークンと比較
    !token.is_empty() && token.len() >= 32
}
```

#### TLS設定

```rust
// src/server/tls.rs
use axum_server::tls_rustls::RustlsConfig;
use std::path::PathBuf;

pub async fn create_tls_config(
    cert_path: PathBuf,
    key_path: PathBuf,
) -> Result<RustlsConfig, Box<dyn std::error::Error>> {
    let config = RustlsConfig::from_pem_file(cert_path, key_path).await?;
    Ok(config)
}

// src/main.rsに追加
pub async fn start_server_with_tls(
    app: Router,
    addr: SocketAddr,
    tls_config: Option<RustlsConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    match tls_config {
        Some(config) => {
            println!("🔒 Starting HTTPS server on {}", addr);
            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
                .await?
        }
        None => {
            println!("⚠️  Starting HTTP server on {} (TLS disabled)", addr);
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?
        }
    }
    
    Ok(())
}
```

#### 成果物(TASK-022)

- **完全なHTTPS通信設定**
- **組断的セキュリティヘッダー**
- **CSRF/XSS完全対策**
- **CSP/HSTSセキュリティポリシー**
- **証明書管理システム**

---

### TASK-023: セキュリティテスト

- **説明**: 脆弱性テスト・ペネトレーション
- **優先度**: Low
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-022

#### 実装内容(TASK-023)

1. 脆弱性スキャン
2. ペネトレーションテスト
3. セキュリティ監査
4. 改善提案

#### 成果物(TASK-023)

- セキュリティテスト結果
- 脆弱性対策
- セキュリティガイドライン

## フェーズ完了基準

### 必須条件

- [ ] Windows AD認証が動作する
- [ ] JWT認証が正常に機能する
- [ ] 役割ベースアクセス制御が動作する
- [ ] 機密レベル制御が適切に機能する
- [ ] 監査ログが正しく記録される
- [ ] HTTPS通信が確立される

### 検証方法

```bash
# 認証テスト
curl -X POST /auth/login -d '{"username":"user","password":"pass"}'

# 認可テスト
curl -H "Authorization: Bearer <token>" /api/admin/users

# 機密レベルテスト
curl -H "Authorization: Bearer <token>" /api/documents/confidential
```

## 次フェーズへの引き継ぎ事項

- 認証・認可システム完成
- セキュリティ基盤確立
- UI開発でのセキュリティ考慮事項
- 運用セキュリティガイドライン

## リスク・課題

- **AD連携**: ネットワーク環境依存
- **性能**: 認証処理のオーバーヘッド
- **証明書**: 本番環境での証明書管理

## 対応策

- フォールバック認証の準備
- 認証キャッシュ戦略
- 証明書自動更新システム
