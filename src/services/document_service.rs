use std::sync::Arc;

use crate::models::{
    CreateDocumentRequest, CreateDocumentWithNumberRequest, CreatedDocumentWithNumber,
    DocumentNumberGenerationError, DocumentNumberRequest, ValidationError,
};
use crate::repositories::{DocumentNumberRuleRepository, DocumentRepository};
use crate::services::DocumentNumberGenerator;

/// 文書管理ビジネスロジックサービス
pub struct DocumentService {
    document_repository: Arc<dyn DocumentRepository>,
    number_generator: DocumentNumberGenerator,
}

impl DocumentService {
    pub fn new(
        document_repository: impl DocumentRepository + 'static,
        rule_repository: impl DocumentNumberRuleRepository + 'static,
    ) -> Self {
        let document_repository = Arc::new(document_repository);
        let number_generator = DocumentNumberGenerator::new(rule_repository);

        Self {
            document_repository,
            number_generator,
        }
    }

    /// 文書番号を自動生成して文書を作成する
    pub async fn create_document_with_number(
        &self,
        request: CreateDocumentWithNumberRequest,
    ) -> Result<CreatedDocumentWithNumber, DocumentServiceError> {
        // リクエストのバリデーション
        request
            .validate()
            .map_err(DocumentServiceError::ValidationError)?;

        // 文書番号生成リクエストを作成
        let number_request = DocumentNumberRequest {
            document_type_code: request.document_type_code.clone(),
            department_code: request.department_code.clone(),
            created_date: request.created_date,
            created_by: request.created_by,
        };

        // 文書番号を生成
        let generated_number = self
            .number_generator
            .generate_document_number(number_request)
            .await
            .map_err(DocumentServiceError::NumberGenerationError)?;

        // 文書作成リクエストを作成（仮のdocument_type_id=1を使用）
        let doc_request = CreateDocumentRequest {
            title: request.title,
            document_type_id: 1, // 実際にはdocument_type_codeから解決する必要がある
            created_by: request.created_by,
            created_date: request.created_date,
        };

        // 文書を作成
        let document = self
            .document_repository
            .create(doc_request)
            .await
            .map_err(DocumentServiceError::RepositoryError)?;

        Ok(CreatedDocumentWithNumber {
            document_number: generated_number.document_number.clone(),
            document,
            generated_number,
        })
    }
}

/// DocumentServiceのエラー型
#[derive(Debug, thiserror::Error)]
pub enum DocumentServiceError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationError),
    #[error("Document number generation error: {0}")]
    NumberGenerationError(#[from] DocumentNumberGenerationError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::repositories::RepositoryError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_create_document_with_number_request_validation() {
        let request = CreateDocumentWithNumberRequest {
            title: "テスト文書".to_string(),
            document_type_code: "A".to_string(),
            department_code: "T".to_string(),
            created_by: 1,
            created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        };

        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_document_with_number_request_empty_title() {
        let request = CreateDocumentWithNumberRequest {
            title: "".to_string(),
            document_type_code: "A".to_string(),
            department_code: "T".to_string(),
            created_by: 1,
            created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        };

        let result = request.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::EmptyTitle));
    }

    #[test]
    fn test_create_document_with_number_request_invalid_department_code() {
        let request = CreateDocumentWithNumberRequest {
            title: "テスト文書".to_string(),
            document_type_code: "A".to_string(),
            department_code: "TA".to_string(), // 2文字（無効）
            created_by: 1,
            created_date: NaiveDate::from_ymd_opt(2025, 8, 17).unwrap(),
        };

        let result = request.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::InvalidDepartmentCodeLength
        ));
    }
}
