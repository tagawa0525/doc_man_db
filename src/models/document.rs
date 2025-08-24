use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// バリデーションエラー型
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DocumentValidationError {
    #[error("Title cannot be empty")]
    EmptyTitle,
    #[error("Invalid document type ID")]
    InvalidDocumentTypeId,
    #[error("Invalid created by ID")]
    InvalidCreatedBy,
    #[error("Document type code cannot be empty")]
    EmptyDocumentTypeCode,
    #[error("Document type code must be 1-10 characters")]
    InvalidDocumentTypeCodeLength,
    #[error("Document type name cannot be empty")]
    EmptyDocumentTypeName,
    #[error("Department code must be 1-10 characters")]
    InvalidDepartmentCodeLength,
    #[error("Effective until date must be after effective from date")]
    InvalidEffectivePeriod,
    #[error("Rule name cannot be empty")]
    EmptyRuleName,
    #[error("Template cannot be empty")]
    EmptyTemplate,
    #[error("Sequence digits must be greater than 0")]
    InvalidSequenceDigits,
    #[error("Document type codes list cannot be empty")]
    EmptyDocumentTypeCodes,
    #[error("Department code cannot be empty")]
    EmptyDepartmentCode,
}

// 文書モデル（データベースから取得用）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub id: i32,
    pub number: String,
    pub title: String,
    pub document_type_id: i32,
    pub business_number: Option<String>,
    pub created_by: i32,
    pub created_date: NaiveDate,
    pub internal_external: Option<String>,
    pub importance_class: Option<String>,
    pub personal_info: Option<String>,
    pub notes: Option<String>,
    pub network_path: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 文書作成リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDocumentRequest {
    pub number: Option<String>,
    pub title: String,
    pub document_type_id: i32,
    pub business_number: Option<String>,
    pub created_by: i32,
    pub created_date: NaiveDate,
    pub internal_external: Option<String>,
    pub importance_class: Option<String>,
    pub personal_info: Option<String>,
    pub notes: Option<String>,
}

impl CreateDocumentRequest {
    /// バリデーションを実行
    pub fn validate(&self) -> Result<(), DocumentValidationError> {
        // タイトルが空でないことをチェック
        if self.title.trim().is_empty() {
            return Err(DocumentValidationError::EmptyTitle);
        }

        // 文書種別IDが有効であることをチェック
        if self.document_type_id < 1 {
            return Err(DocumentValidationError::InvalidDocumentTypeId);
        }

        // 作成者IDが有効であることをチェック
        if self.created_by < 1 {
            return Err(DocumentValidationError::InvalidCreatedBy);
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

// 文書番号付き文書作成リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDocumentWithNumberRequest {
    pub title: String,
    pub document_type_code: String,
    pub department_code: String,
    pub created_by: i32,
    pub created_date: NaiveDate,
}

impl CreateDocumentWithNumberRequest {
    /// バリデーションを実行
    pub fn validate(&self) -> Result<(), DocumentValidationError> {
        // タイトルが空でないことをチェック
        if self.title.trim().is_empty() {
            return Err(DocumentValidationError::EmptyTitle);
        }

        // 文書種別コードが空でないことをチェック
        if self.document_type_code.trim().is_empty() {
            return Err(DocumentValidationError::EmptyDocumentTypeCode);
        }

        // 文書種別コードが適切な長さであることをチェック (1-10文字)
        let doc_type_len = self.document_type_code.trim().len();
        if !(1..=10).contains(&doc_type_len) {
            return Err(DocumentValidationError::InvalidDocumentTypeCodeLength);
        }

        // 部署コードが空でないことをチェック
        if self.department_code.trim().is_empty() {
            return Err(DocumentValidationError::EmptyDepartmentCode);
        }

        // 部署コードが適切な長さであることをチェック (1-10文字)
        let dept_code_len = self.department_code.trim().len();
        if !(1..=10).contains(&dept_code_len) {
            return Err(DocumentValidationError::InvalidDepartmentCodeLength);
        }

        // 作成者IDが有効であることをチェック
        if self.created_by < 1 {
            return Err(DocumentValidationError::InvalidCreatedBy);
        }

        Ok(())
    }
}
