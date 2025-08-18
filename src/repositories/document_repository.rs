// Document Repository - データベースアクセス層

use crate::models::{CreateDocumentRequest, Document, DocumentSearchFilters};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

// Repository エラー型
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Resource not found with id: {id}")]
    NotFound { id: String },
    #[error("Validation failed: {0}")]
    Validation(String),
}

// Document Repository trait
#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn create(&self, request: CreateDocumentRequest) -> Result<Document, RepositoryError>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Document>, RepositoryError>;
    async fn search(
        &self,
        filters: DocumentSearchFilters,
    ) -> Result<(Vec<Document>, i64), RepositoryError>;
}

// SQLite実装
pub struct SqliteDocumentRepository {
    pool: SqlitePool,
}

impl SqliteDocumentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// テスト用のインメモリSQLiteリポジトリを作成
    pub async fn new_in_memory() -> Result<Self, RepositoryError> {
        let pool = SqlitePool::connect(":memory:").await?;

        // テーブル作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                number TEXT UNIQUE NOT NULL,
                title TEXT NOT NULL,
                document_type_id INTEGER NOT NULL,
                business_number TEXT,
                created_by INTEGER NOT NULL,
                created_date TEXT NOT NULL,
                internal_external TEXT,
                importance_class TEXT,
                personal_info TEXT,
                notes TEXT,
                network_path TEXT,
                is_active INTEGER DEFAULT 1,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl DocumentRepository for SqliteDocumentRepository {
    async fn create(&self, request: CreateDocumentRequest) -> Result<Document, RepositoryError> {
        // バリデーション実行
        request
            .validate()
            .map_err(|e| RepositoryError::Validation(e.to_string()))?;

        // 文書番号を生成（簡易実装）
        let document_number = format!("DOC-{:06}", chrono::Utc::now().timestamp_millis() % 1000000);

        // データベースに挿入
        let result = sqlx::query(
            r#"
            INSERT INTO documents (number, title, document_type_id, business_number, created_by, created_date, internal_external, importance_class, personal_info, notes, network_path, is_active)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&document_number)
        .bind(&request.title)
        .bind(request.document_type_id)
        .bind(&request.business_number)
        .bind(request.created_by)
        .bind(request.created_date.format("%Y-%m-%d").to_string())
        .bind(&request.internal_external)
        .bind(&request.importance_class)
        .bind(&request.personal_info)
        .bind(&request.notes)
        .bind(&request.number) // network_pathとして使用
        .bind(true)
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        let id = result.last_insert_rowid() as i32;

        // 挿入されたレコードを取得
        let row = sqlx::query(
            "SELECT id, number, title, document_type_id, business_number, created_by, created_date, internal_external, importance_class, personal_info, notes, network_path, is_active, created_at, updated_at FROM documents WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        // レスポンス構築
        use chrono::{NaiveDate, NaiveDateTime};

        let document = Document {
            id: row.get("id"),
            number: row.get("number"),
            title: row.get("title"),
            document_type_id: row.get("document_type_id"),
            business_number: row.get("business_number"),
            created_by: row.get("created_by"),
            created_date: NaiveDate::parse_from_str(
                &row.get::<String, _>("created_date"),
                "%Y-%m-%d",
            )
            .map_err(|e| RepositoryError::Validation(format!("Invalid date format: {e}")))?,
            internal_external: row.get("internal_external"),
            importance_class: row.get("importance_class"),
            personal_info: row.get("personal_info"),
            notes: row.get("notes"),
            network_path: row.get("network_path"),
            is_active: row.get("is_active"),
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::parse_from_str(
                    &row.get::<String, _>("created_at"),
                    "%Y-%m-%d %H:%M:%S",
                )
                .map_err(|e| {
                    RepositoryError::Validation(format!("Invalid datetime format: {e}"))
                })?,
                Utc,
            ),
            updated_at: DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::parse_from_str(
                    &row.get::<String, _>("updated_at"),
                    "%Y-%m-%d %H:%M:%S",
                )
                .map_err(|e| {
                    RepositoryError::Validation(format!("Invalid datetime format: {e}"))
                })?,
                Utc,
            ),
        };

        Ok(document)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Document>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, number, title, document_type_id, business_number, created_by, created_date, internal_external, importance_class, personal_info, notes, network_path, is_active, created_at, updated_at FROM documents WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        if let Some(row) = row {
            use chrono::{NaiveDate, NaiveDateTime};

            let document = Document {
                id: row.get("id"),
                number: row.get("number"),
                title: row.get("title"),
                document_type_id: row.get("document_type_id"),
                business_number: row.get("business_number"),
                created_by: row.get("created_by"),
                created_date: NaiveDate::parse_from_str(
                    &row.get::<String, _>("created_date"),
                    "%Y-%m-%d",
                )
                .map_err(|e| RepositoryError::Validation(format!("Invalid date format: {e}")))?,
                internal_external: row.get("internal_external"),
                importance_class: row.get("importance_class"),
                personal_info: row.get("personal_info"),
                notes: row.get("notes"),
                network_path: row.get("network_path"),
                is_active: row.get("is_active"),
                created_at: DateTime::<Utc>::from_naive_utc_and_offset(
                    NaiveDateTime::parse_from_str(
                        &row.get::<String, _>("created_at"),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .map_err(|e| {
                        RepositoryError::Validation(format!("Invalid datetime format: {e}"))
                    })?,
                    Utc,
                ),
                updated_at: DateTime::<Utc>::from_naive_utc_and_offset(
                    NaiveDateTime::parse_from_str(
                        &row.get::<String, _>("updated_at"),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .map_err(|e| {
                        RepositoryError::Validation(format!("Invalid datetime format: {e}"))
                    })?,
                    Utc,
                ),
            };
            Ok(Some(document))
        } else {
            Ok(None)
        }
    }

    async fn search(
        &self,
        filters: DocumentSearchFilters,
    ) -> Result<(Vec<Document>, i64), RepositoryError> {
        let mut query = "SELECT id, number, title, document_type_id, business_number, created_by, created_date, internal_external, importance_class, personal_info, notes, network_path, is_active, created_at, updated_at FROM documents WHERE 1=1".to_string();
        let mut count_query = "SELECT COUNT(*) as count FROM documents WHERE 1=1".to_string();

        // フィルター条件を構築
        if let Some(ref _title) = filters.title {
            query.push_str(" AND title LIKE ?");
            count_query.push_str(" AND title LIKE ?");
        }

        if let Some(_document_type_id) = filters.document_type_id {
            query.push_str(" AND document_type_id = ?");
            count_query.push_str(" AND document_type_id = ?");
        }

        if let Some(_created_by) = filters.created_by {
            query.push_str(" AND created_by = ?");
            count_query.push_str(" AND created_by = ?");
        }

        // LIMIT/OFFSET追加
        query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

        // カウントクエリ実行
        let mut count_stmt = sqlx::query(&count_query);
        if let Some(ref title) = filters.title {
            count_stmt = count_stmt.bind(format!("%{title}%"));
        }
        if let Some(document_type_id) = filters.document_type_id {
            count_stmt = count_stmt.bind(document_type_id);
        }
        if let Some(created_by) = filters.created_by {
            count_stmt = count_stmt.bind(created_by);
        }

        let count_row = count_stmt
            .fetch_one(&self.pool)
            .await
            .map_err(RepositoryError::Database)?;
        let total: i64 = count_row.get("count");

        // メインクエリ実行
        let mut stmt = sqlx::query(&query);
        if let Some(ref title) = filters.title {
            stmt = stmt.bind(format!("%{title}%"));
        }
        if let Some(document_type_id) = filters.document_type_id {
            stmt = stmt.bind(document_type_id);
        }
        if let Some(created_by) = filters.created_by {
            stmt = stmt.bind(created_by);
        }
        stmt = stmt.bind(filters.limit).bind(filters.offset);

        let rows = stmt
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::Database)?;

        use chrono::{NaiveDate, NaiveDateTime};
        let documents: Result<Vec<Document>, RepositoryError> = rows
            .into_iter()
            .map(|row| {
                Ok(Document {
                    id: row.get("id"),
                    number: row.get("number"),
                    title: row.get("title"),
                    document_type_id: row.get("document_type_id"),
                    business_number: row.get("business_number"),
                    created_by: row.get("created_by"),
                    created_date: NaiveDate::parse_from_str(
                        &row.get::<String, _>("created_date"),
                        "%Y-%m-%d",
                    )
                    .map_err(|e| {
                        RepositoryError::Validation(format!("Invalid date format: {e}"))
                    })?,
                    internal_external: row.get("internal_external"),
                    importance_class: row.get("importance_class"),
                    personal_info: row.get("personal_info"),
                    notes: row.get("notes"),
                    network_path: row.get("network_path"),
                    is_active: row.get("is_active"),
                    created_at: DateTime::<Utc>::from_naive_utc_and_offset(
                        NaiveDateTime::parse_from_str(
                            &row.get::<String, _>("created_at"),
                            "%Y-%m-%d %H:%M:%S",
                        )
                        .map_err(|e| {
                            RepositoryError::Validation(format!("Invalid datetime format: {e}"))
                        })?,
                        Utc,
                    ),
                    updated_at: DateTime::<Utc>::from_naive_utc_and_offset(
                        NaiveDateTime::parse_from_str(
                            &row.get::<String, _>("updated_at"),
                            "%Y-%m-%d %H:%M:%S",
                        )
                        .map_err(|e| {
                            RepositoryError::Validation(format!("Invalid datetime format: {e}"))
                        })?,
                        Utc,
                    ),
                })
            })
            .collect();

        Ok((documents?, total))
    }
}
