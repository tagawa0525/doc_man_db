use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

// バリデーションエラー型
#[derive(Debug, Error, PartialEq)]
pub enum ValidationError {
    #[error("Title cannot be empty")]
    EmptyTitle,
    #[error("Invalid document type ID")]
    InvalidDocumentTypeId,
    #[error("Invalid created by ID")]
    InvalidCreatedBy,
}

// 文書モデル（データベースから取得用）
#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub document_type_id: i32,
    pub created_by: i32,
    pub created_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// 文書作成リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDocumentRequest {
    pub title: String,
    pub document_type_id: i32,
    pub created_by: i32,
    pub created_date: NaiveDate,
}

impl CreateDocumentRequest {
    /// バリデーションを実行
    pub fn validate(&self) -> Result<(), ValidationError> {
        // タイトルが空でないことをチェック
        if self.title.trim().is_empty() {
            return Err(ValidationError::EmptyTitle);
        }

        // 文書種別IDが有効であることをチェック
        if self.document_type_id < 1 {
            return Err(ValidationError::InvalidDocumentTypeId);
        }

        // 作成者IDが有効であることをチェック
        if self.created_by < 1 {
            return Err(ValidationError::InvalidCreatedBy);
        }

        Ok(())
    }
}

// 文書更新リクエスト（将来的に使用）
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub document_type_id: Option<i32>,
}

// 文書検索フィルター（将来的に使用）
#[derive(Debug, Clone)]
pub struct DocumentSearchFilters {
    pub title: Option<String>,
    pub document_type_id: Option<i32>,
    pub created_by: Option<i32>,
    pub created_date_from: Option<NaiveDate>,
    pub created_date_to: Option<NaiveDate>,
    pub limit: i64,
    pub offset: i64,
}

impl Default for DocumentSearchFilters {
    fn default() -> Self {
        Self {
            title: None,
            document_type_id: None,
            created_by: None,
            created_date_from: None,
            created_date_to: None,
            limit: 50,
            offset: 0,
        }
    }
}
