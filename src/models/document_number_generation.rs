use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::DocumentValidationError;

// 文書番号生成ルールモデル（データベースから取得用）
#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct DocumentNumberGenerationRule {
    pub id: i32,
    pub rule_name: String,
    pub template: String,
    pub sequence_digits: i32,
    pub department_code: Option<String>,
    pub document_type_codes: String, // JSON配列として保存
    pub effective_from: NaiveDate,
    pub effective_until: Option<NaiveDate>,
    pub priority: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// 文書番号生成ルール作成リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDocumentNumberGenerationRuleRequest {
    pub rule_name: String,
    pub template: String,
    pub sequence_digits: i32,
    pub department_code: Option<String>,
    pub document_type_codes: Vec<String>,
    pub effective_from: NaiveDate,
    pub effective_until: Option<NaiveDate>,
    pub priority: i32,
}

impl CreateDocumentNumberGenerationRuleRequest {
    /// バリデーションを実行
    pub fn validate(&self) -> Result<(), DocumentValidationError> {
        // ルール名が空でないことをチェック
        if self.rule_name.trim().is_empty() {
            return Err(DocumentValidationError::EmptyRuleName);
        }

        // テンプレートが空でないことをチェック
        if self.template.trim().is_empty() {
            return Err(DocumentValidationError::EmptyTemplate);
        }

        // 連番桁数が有効であることをチェック
        if self.sequence_digits <= 0 {
            return Err(DocumentValidationError::InvalidSequenceDigits);
        }

        // 文書種別コードリストが空でないことをチェック
        if self.document_type_codes.is_empty() {
            return Err(DocumentValidationError::EmptyDocumentTypeCodes);
        }

        // 部署コードが指定されている場合、1文字であることをチェック
        if let Some(dept_code) = &self.department_code
            && !dept_code.trim().is_empty() && dept_code.trim().len() != 1 {
                return Err(DocumentValidationError::InvalidDepartmentCodeLength);
            }

        // 有効期間の整合性をチェック
        if let Some(effective_until) = self.effective_until
            && effective_until <= self.effective_from {
                return Err(DocumentValidationError::InvalidEffectivePeriod);
            }

        Ok(())
    }
}

// 文書番号生成リクエスト
#[derive(Debug, Clone)]
pub struct DocumentNumberRequest {
    pub document_type_code: String,
    pub department_code: String,
    pub created_date: NaiveDate,
    pub created_by: i32,
}

impl DocumentNumberRequest {
    /// バリデーションを実行
    pub fn validate(&self) -> Result<(), DocumentValidationError> {
        // 文書種別コードが空でないことをチェック
        if self.document_type_code.trim().is_empty() {
            return Err(DocumentValidationError::EmptyDocumentTypeCode);
        }

        // 文書種別コードが1文字であることをチェック
        if self.document_type_code.trim().len() != 1 {
            return Err(DocumentValidationError::InvalidDocumentTypeCodeLength);
        }

        // 部署コードが空でないことをチェック
        if self.department_code.trim().is_empty() {
            return Err(DocumentValidationError::EmptyDepartmentCode);
        }

        // 部署コードが1文字であることをチェック
        if self.department_code.trim().len() != 1 {
            return Err(DocumentValidationError::InvalidDepartmentCodeLength);
        }

        // 作成者IDが有効であることをチェック
        if self.created_by < 1 {
            return Err(DocumentValidationError::InvalidCreatedBy);
        }

        Ok(())
    }
}

// 生成された文書番号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedDocumentNumber {
    pub document_number: String,
    pub rule_id: i32,
    pub sequence_number: i32,
    pub template_used: String,
}

// 文書番号生成エラー
#[derive(Debug, thiserror::Error)]
pub enum DocumentNumberGenerationError {
    #[error("No applicable rule found for the given criteria")]
    NoApplicableRule,
    #[error("Sequence numbers exhausted for the rule")]
    SequenceExhausted,
    #[error("Generated document number already exists")]
    DuplicateNumber,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::repositories::RepositoryError),
    #[error("Template parsing error: {0}")]
    TemplateError(String),
}

// 文書番号付き文書作成結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedDocumentWithNumber {
    pub document: crate::models::Document,
    pub document_number: String,
    pub generated_number: GeneratedDocumentNumber,
}
