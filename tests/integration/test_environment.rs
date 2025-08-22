use chrono::NaiveDate;
use doc_man_db::config::AppConfig;
use doc_man_db::services::*;
use serde_json::Value;
use sqlx::{Pool, Row, Sqlite};
use std::sync::Arc;
use tokio::time::Duration;

/// Test environment for integration tests
#[allow(dead_code)]
pub struct TestEnvironment {
    pub config: AppConfig,
    pub db_pool: Pool<Sqlite>,
    pub metrics_service: Arc<MetricsService>,
    pub cache_service: Arc<CacheService>,
    pub base_url: String,
    pub client: reqwest::Client,
}

impl TestEnvironment {
    /// Create a new test environment with in-memory database
    pub async fn new() -> Self {
        let config = AppConfig::default();

        // Create in-memory SQLite database for testing
        let db_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(10)
            .connect(":memory:")
            .await
            .expect("Failed to create test database pool");

        // Create basic tables for testing
        Self::create_test_tables(&db_pool).await;

        // Initialize services
        let metrics_service = Arc::new(MetricsService::new(60));
        let cache_service = Arc::new(CacheService::new());

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            db_pool,
            metrics_service,
            cache_service,
            base_url: "http://localhost:8080".to_string(),
            client,
        }
    }

    async fn create_test_tables(pool: &Pool<Sqlite>) {
        // Create test tables using raw SQL to avoid SQLx compile-time checks
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS document_types (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                code TEXT NOT NULL UNIQUE,
                prefix TEXT NOT NULL,
                is_active BOOLEAN DEFAULT TRUE
            )
            "#,
        )
        .execute(pool)
        .await
        .expect("Failed to create document_types table");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                display_name TEXT NOT NULL,
                is_admin BOOLEAN DEFAULT FALSE,
                is_active BOOLEAN DEFAULT TRUE
            )
            "#,
        )
        .execute(pool)
        .await
        .expect("Failed to create users table");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                document_type_id INTEGER NOT NULL,
                created_by INTEGER NOT NULL,
                created_date DATE NOT NULL,
                internal_external TEXT NOT NULL,
                importance_class TEXT NOT NULL,
                personal_info TEXT NOT NULL,
                notes TEXT,
                is_active BOOLEAN DEFAULT TRUE,
                FOREIGN KEY (document_type_id) REFERENCES document_types(id),
                FOREIGN KEY (created_by) REFERENCES users(id)
            )
            "#,
        )
        .execute(pool)
        .await
        .expect("Failed to create documents table");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS circulation_workflows (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                is_active BOOLEAN DEFAULT TRUE
            )
            "#,
        )
        .execute(pool)
        .await
        .expect("Failed to create circulation_workflows table");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS circulations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                document_id INTEGER NOT NULL,
                workflow_id INTEGER NOT NULL,
                status TEXT NOT NULL,
                notes TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (document_id) REFERENCES documents(id),
                FOREIGN KEY (workflow_id) REFERENCES circulation_workflows(id)
            )
            "#,
        )
        .execute(pool)
        .await
        .expect("Failed to create circulations table");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS circulation_steps (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                circulation_id INTEGER NOT NULL,
                step_order INTEGER NOT NULL,
                assignee_id INTEGER NOT NULL,
                status TEXT NOT NULL,
                comments TEXT,
                completed_at DATETIME,
                FOREIGN KEY (circulation_id) REFERENCES circulations(id),
                FOREIGN KEY (assignee_id) REFERENCES users(id)
            )
            "#,
        )
        .execute(pool)
        .await
        .expect("Failed to create circulation_steps table");
    }

    /// Setup test data (users, document types, workflows, etc.)
    pub async fn setup_test_data(&self) -> anyhow::Result<()> {
        self.create_test_document_types().await?;
        self.create_test_workflows().await?;
        self.create_test_users().await?;
        Ok(())
    }

    async fn create_test_document_types(&self) -> anyhow::Result<()> {
        let document_types = vec![
            ("技術文書", "technical"),
            ("管理文書", "management"),
            ("契約文書", "contract"),
        ];

        for (name, code) in document_types {
            sqlx::query("INSERT INTO document_types (name, code, prefix) VALUES (?, ?, ?)")
                .bind(name)
                .bind(code)
                .bind(code)
                .execute(&self.db_pool)
                .await?;
        }

        Ok(())
    }

    async fn create_test_workflows(&self) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO circulation_workflows (name, description, is_active) VALUES (?, ?, ?)",
        )
        .bind("標準承認フロー")
        .bind("一般的な文書承認フロー")
        .bind(true)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn create_test_users(&self) -> anyhow::Result<()> {
        let users = vec![
            ("admin", "管理者", true),
            ("user1", "一般ユーザー1", false),
            ("limited_user", "制限ユーザー", false),
        ];

        for (username, display_name, is_admin) in users {
            sqlx::query("INSERT INTO users (username, display_name, is_admin, is_active) VALUES (?, ?, ?, ?)")
                .bind(username)
                .bind(display_name)
                .bind(is_admin)
                .bind(true)
                .execute(&self.db_pool)
                .await?;
        }

        Ok(())
    }

    /// Authenticate user and return token
    pub async fn authenticate_user(
        &self,
        username: &str,
        _password: &str,
    ) -> anyhow::Result<String> {
        // For testing, we'll just return a mock JWT token
        let token = format!("test_token_{}", username);
        Ok(token)
    }

    /// Create a test document
    pub async fn create_document(
        &self,
        request: CreateDocumentRequest,
        _token: &str,
    ) -> anyhow::Result<TestDocument> {
        let result = sqlx::query(
            r#"
            INSERT INTO documents (
                title, document_type_id, created_by, created_date,
                internal_external, importance_class, personal_info,
                notes, is_active
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&request.title)
        .bind(request.document_type_id)
        .bind(request.created_by)
        .bind(request.created_date)
        .bind(&request.confidentiality.internal_external)
        .bind(&request.confidentiality.importance_class)
        .bind(&request.confidentiality.personal_info)
        .bind(&request.notes)
        .bind(true)
        .execute(&self.db_pool)
        .await?;

        let document_id = result.last_insert_rowid();

        Ok(TestDocument {
            id: document_id as i32,
            title: request.title,
            document_type_id: request.document_type_id,
            created_by: request.created_by,
            created_date: request.created_date,
            confidentiality: request.confidentiality,
            notes: request.notes,
            is_active: true,
        })
    }

    /// Check file existence for a document
    pub async fn check_file_existence(
        &self,
        _document: &TestDocument,
    ) -> anyhow::Result<FileExistenceResult> {
        // Mock implementation for testing
        Ok(FileExistenceResult {
            folder_exists: false,
            files_found: vec![],
            approval_file_exists: false,
        })
    }

    /// Start a circulation workflow
    pub async fn start_circulation(
        &self,
        request: StartCirculationRequest,
        _token: &str,
    ) -> anyhow::Result<CirculationResponse> {
        let circulation_result = sqlx::query("INSERT INTO circulations (document_id, workflow_id, status, notes) VALUES (?, ?, ?, ?)")
            .bind(request.document_id)
            .bind(request.workflow_id)
            .bind("active")
            .bind(&request.notes)
            .execute(&self.db_pool)
            .await?;

        let circulation_id = circulation_result.last_insert_rowid();

        // Create first step
        let step_result = sqlx::query("INSERT INTO circulation_steps (circulation_id, step_order, assignee_id, status) VALUES (?, ?, ?, ?)")
            .bind(circulation_id)
            .bind(1)
            .bind(2) // Assign to user1 (id=2)
            .bind("pending")
            .execute(&self.db_pool)
            .await?;

        let step_id = step_result.last_insert_rowid();

        Ok(CirculationResponse {
            id: circulation_id as i32,
            document_id: request.document_id,
            workflow_id: request.workflow_id,
            current_step_id: step_id as i32,
            status: "active".to_string(),
        })
    }

    /// Approve a circulation step
    pub async fn approve_circulation_step(
        &self,
        request: ApproveStepRequest,
        _token: &str,
    ) -> anyhow::Result<ApprovalResult> {
        sqlx::query("UPDATE circulation_steps SET status = ?, comments = ? WHERE id = ?")
            .bind("completed")
            .bind(&request.comments)
            .bind(request.step_id)
            .execute(&self.db_pool)
            .await?;

        Ok(ApprovalResult {
            success: true,
            message: "Approval processed successfully".to_string(),
        })
    }

    /// Search documents with SQL injection protection
    pub async fn search_documents(
        &self,
        input: DocumentSearchInput,
        _token: &str,
    ) -> anyhow::Result<DocumentSearchResult> {
        let mut query = "SELECT * FROM documents WHERE is_active = 1".to_string();

        if let Some(_title) = &input.title {
            query.push_str(" AND title LIKE ?");
        }

        query.push_str(" LIMIT ? OFFSET ?");

        let mut sqlx_query = sqlx::query(&query);

        if let Some(title) = &input.title {
            sqlx_query = sqlx_query.bind(format!("%{}%", title));
        }

        sqlx_query = sqlx_query
            .bind(input.pagination.limit)
            .bind(input.pagination.offset);

        let rows = sqlx_query.fetch_all(&self.db_pool).await?;

        let mut documents = vec![];
        for row in rows {
            documents.push(TestDocument {
                id: row.get("id"),
                title: row.get("title"),
                document_type_id: row.get("document_type_id"),
                created_by: row.get("created_by"),
                created_date: row.get("created_date"),
                confidentiality: TestConfidentiality {
                    internal_external: row.get("internal_external"),
                    importance_class: row.get("importance_class"),
                    personal_info: row.get("personal_info"),
                },
                notes: row.get("notes"),
                is_active: row.get("is_active"),
            });
        }

        let total_count = documents.len() as i64;
        Ok(DocumentSearchResult {
            documents,
            total_count,
            pagination: input.pagination,
        })
    }

    /// Update a document
    pub async fn update_document(
        &self,
        document_id: i32,
        request: UpdateDocumentRequest,
        _token: &str,
    ) -> anyhow::Result<TestDocument> {
        if let Some(title) = &request.title {
            sqlx::query("UPDATE documents SET title = ? WHERE id = ?")
                .bind(title)
                .bind(document_id)
                .execute(&self.db_pool)
                .await?;
        }

        if let Some(notes) = &request.notes {
            sqlx::query("UPDATE documents SET notes = ? WHERE id = ?")
                .bind(notes)
                .bind(document_id)
                .execute(&self.db_pool)
                .await?;
        }

        // Return updated document
        Ok(TestDocument {
            id: document_id,
            title: request.title.unwrap_or("統合テスト文書".to_string()),
            document_type_id: 1,
            created_by: 1,
            created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
            confidentiality: TestConfidentiality::default(),
            notes: request.notes,
            is_active: true,
        })
    }

    /// Get all employees (admin function)
    pub async fn get_all_employees(&self, token: &str) -> anyhow::Result<Vec<Employee>> {
        // Check if user is admin based on token
        if token.contains("admin") {
            Ok(vec![
                Employee {
                    id: 1,
                    name: "管理者".to_string(),
                    department: "管理部".to_string(),
                },
                Employee {
                    id: 2,
                    name: "一般ユーザー1".to_string(),
                    department: "開発部".to_string(),
                },
            ])
        } else {
            Err(anyhow::anyhow!("Access denied"))
        }
    }

    /// Get a specific document
    pub async fn get_document(
        &self,
        document_id: i32,
        token: &str,
    ) -> anyhow::Result<Option<TestDocument>> {
        let row = sqlx::query("SELECT * FROM documents WHERE id = ? AND is_active = 1")
            .bind(document_id)
            .fetch_optional(&self.db_pool)
            .await?;

        if let Some(row) = row {
            let importance_class: String = row.get("importance_class");
            let personal_info: String = row.get("personal_info");

            // Access control: Only admin can access Class1 (highly confidential) documents
            // or documents with High personal info level
            if (importance_class == "Class1" || personal_info == "High") && !token.contains("admin")
            {
                return Ok(None); // Unauthorized access
            }

            let confidentiality = TestConfidentiality {
                internal_external: row.get("internal_external"),
                importance_class: importance_class.clone(),
                personal_info: personal_info.clone(),
            };

            Ok(Some(TestDocument {
                id: row.get("id"),
                title: row.get("title"),
                document_type_id: row.get("document_type_id"),
                created_by: row.get("created_by"),
                created_date: row.get("created_date"),
                confidentiality,
                notes: row.get("notes"),
                is_active: row.get("is_active"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Create a confidential document for testing
    pub async fn create_confidential_document(&self) -> TestDocument {
        let result = sqlx::query(
            r#"
            INSERT INTO documents (
                title, document_type_id, created_by, created_date,
                internal_external, importance_class, personal_info,
                notes, is_active
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind("機密文書")
        .bind(1)
        .bind(1)
        .bind(NaiveDate::from_ymd_opt(2024, 12, 15).unwrap())
        .bind("Internal")
        .bind("Class1")
        .bind("High")
        .bind("高機密文書")
        .bind(true)
        .execute(&self.db_pool)
        .await
        .expect("Failed to create confidential document");

        let document_id = result.last_insert_rowid();

        TestDocument {
            id: document_id as i32,
            title: "機密文書".to_string(),
            document_type_id: 1,
            created_by: 1,
            created_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
            confidentiality: TestConfidentiality {
                internal_external: "Internal".to_string(),
                importance_class: "Class1".to_string(),
                personal_info: "High".to_string(),
            },
            notes: Some("高機密文書".to_string()),
            is_active: true,
        }
    }

    /// Make HTTP request to running server
    #[allow(dead_code)]
    pub async fn make_request(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
    ) -> anyhow::Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = match method {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            _ => return Err(anyhow::anyhow!("Unsupported HTTP method: {}", method)),
        };

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        Ok(response)
    }

    /// Load test helper - generate concurrent requests
    #[allow(dead_code)]
    pub async fn concurrent_requests(
        &self,
        count: usize,
        path: &str,
    ) -> anyhow::Result<Vec<Duration>> {
        let mut handles = vec![];

        for _ in 0..count {
            let client = self.client.clone();
            let url = format!("{}{}", self.base_url, path);

            let handle = tokio::spawn(async move {
                let start = std::time::Instant::now();
                let _response = client.get(&url).send().await;
                start.elapsed()
            });

            handles.push(handle);
        }

        let mut durations = vec![];
        for handle in handles {
            durations.push(handle.await?);
        }

        Ok(durations)
    }
}

// Test data structures
#[derive(Debug)]
pub struct CreateDocumentRequest {
    pub title: String,
    pub document_type_id: i32,
    pub created_by: i32,
    pub created_date: NaiveDate,
    pub confidentiality: TestConfidentiality,
    pub notes: Option<String>,
}

#[derive(Debug, Default)]
pub struct DocumentSearchInput {
    pub title: Option<String>,
    pub pagination: Pagination,
}

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
    pub offset: i64,
    pub limit: i64,
}

impl Pagination {
    pub fn new(page: i32, per_page: i32) -> Self {
        let limit = per_page as i64;
        let offset = ((page - 1) * per_page) as i64;
        Self {
            page,
            per_page,
            offset,
            limit,
        }
    }
}

#[derive(Debug)]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct StartCirculationRequest {
    pub document_id: i32,
    pub workflow_id: i32,
    pub notes: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ApproveStepRequest {
    pub circulation_id: i32,
    pub step_id: i32,
    pub action: String, // Simplified for testing
    pub comments: Option<String>,
}

// Test model structures
#[derive(Debug)]
#[allow(dead_code)]
pub struct TestDocument {
    pub id: i32,
    pub title: String,
    pub document_type_id: i32,
    pub created_by: i32,
    pub created_date: NaiveDate,
    pub confidentiality: TestConfidentiality,
    pub notes: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct TestConfidentiality {
    pub internal_external: String,
    pub importance_class: String,
    pub personal_info: String,
}

impl Default for TestConfidentiality {
    fn default() -> Self {
        Self {
            internal_external: "Internal".to_string(),
            importance_class: "Class2".to_string(),
            personal_info: "None".to_string(),
        }
    }
}

// Response structures
#[derive(Debug)]
pub struct CirculationResponse {
    pub id: i32,
    pub document_id: i32,
    pub workflow_id: i32,
    pub current_step_id: i32,
    pub status: String,
}

#[derive(Debug)]
pub struct ApprovalResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DocumentSearchResult {
    pub documents: Vec<TestDocument>,
    pub total_count: i64,
    pub pagination: Pagination,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FileExistenceResult {
    pub folder_exists: bool,
    pub files_found: Vec<String>,
    pub approval_file_exists: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub department: String,
}
