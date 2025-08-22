use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::DocumentValidationError;

// 文書種別モデル（データベースから取得用）
#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct DocumentType {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub requires_approval: bool,
    pub department_code: Option<String>,
    pub effective_from: NaiveDate,
    pub effective_until: Option<NaiveDate>,
    pub number_format: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// 文書種別作成リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDocumentTypeRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub requires_approval: bool,
    pub department_code: Option<String>,
    pub effective_from: NaiveDate,
    pub effective_until: Option<NaiveDate>,
}

impl CreateDocumentTypeRequest {
    /// バリデーションを実行
    pub fn validate(&self) -> Result<(), DocumentValidationError> {
        // コードが空でないことをチェック
        if self.code.trim().is_empty() {
            return Err(DocumentValidationError::EmptyDocumentTypeCode);
        }

        // コードが1文字であることをチェック
        if self.code.trim().len() != 1 {
            return Err(DocumentValidationError::InvalidDocumentTypeCodeLength);
        }

        // 名前が空でないことをチェック
        if self.name.trim().is_empty() {
            return Err(DocumentValidationError::EmptyDocumentTypeName);
        }

        // 部署コードが指定されている場合、1文字であることをチェック
        if let Some(dept_code) = &self.department_code
            && !dept_code.trim().is_empty()
            && dept_code.trim().len() != 1
        {
            return Err(DocumentValidationError::InvalidDepartmentCodeLength);
        }

        // 有効期間の整合性をチェック
        if let Some(effective_until) = self.effective_until
            && effective_until <= self.effective_from
        {
            return Err(DocumentValidationError::InvalidEffectivePeriod);
        }

        Ok(())
    }
}

// 文書種別検索フィルター（将来的に使用）
#[derive(Debug, Clone)]
pub struct DocumentTypeSearchFilters {
    pub code: Option<String>,
    pub name: Option<String>,
    pub department_code: Option<String>,
    pub active_on_date: Option<NaiveDate>,
    pub requires_approval: Option<bool>,
    pub limit: i64,
    pub offset: i64,
}

impl Default for DocumentTypeSearchFilters {
    fn default() -> Self {
        Self {
            code: None,
            name: None,
            department_code: None,
            active_on_date: None,
            requires_approval: None,
            limit: 50,
            offset: 0,
        }
    }
}
