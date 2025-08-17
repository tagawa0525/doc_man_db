// Document Repository - データベースアクセス層

use crate::models::{CreateDocumentRequest, Document, DocumentSearchFilters};
use async_trait::async_trait;
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
}

#[async_trait]
impl DocumentRepository for SqliteDocumentRepository {
    async fn create(&self, request: CreateDocumentRequest) -> Result<Document, RepositoryError> {
        // バリデーション実行
        request
            .validate()
            .map_err(|e| RepositoryError::Validation(e.to_string()))?;

        // データベースに挿入
        let result = sqlx::query(
            r#"
            INSERT INTO documents (title, document_type_id, created_by, created_date)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&request.title)
        .bind(request.document_type_id)
        .bind(request.created_by)
        .bind(request.created_date.format("%Y-%m-%d").to_string())
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        let id = result.last_insert_rowid() as i32;

        // 挿入されたレコードを取得
        let row = sqlx::query(
            "SELECT id, title, document_type_id, created_by, created_date, created_at, updated_at FROM documents WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        // レスポンス構築
        use chrono::{NaiveDate, NaiveDateTime};

        let document = Document {
            id: row.get("id"),
            title: row.get("title"),
            document_type_id: row.get("document_type_id"),
            created_by: row.get("created_by"),
            created_date: NaiveDate::parse_from_str(
                &row.get::<String, _>("created_date"),
                "%Y-%m-%d",
            )
            .map_err(|e| RepositoryError::Validation(format!("Invalid date format: {e}")))?,
            created_at: NaiveDateTime::parse_from_str(
                &row.get::<String, _>("created_at"),
                "%Y-%m-%d %H:%M:%S",
            )
            .map_err(|e| RepositoryError::Validation(format!("Invalid datetime format: {e}")))?,
            updated_at: NaiveDateTime::parse_from_str(
                &row.get::<String, _>("updated_at"),
                "%Y-%m-%d %H:%M:%S",
            )
            .map_err(|e| RepositoryError::Validation(format!("Invalid datetime format: {e}")))?,
        };

        Ok(document)
    }

    async fn get_by_id(&self, _id: i32) -> Result<Option<Document>, RepositoryError> {
        todo!("Will implement later")
    }

    async fn search(
        &self,
        _filters: DocumentSearchFilters,
    ) -> Result<(Vec<Document>, i64), RepositoryError> {
        todo!("Will implement later")
    }
}
