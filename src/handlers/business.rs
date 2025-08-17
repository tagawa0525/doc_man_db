use crate::error::AppError;
use crate::models::{
    CreateDocumentWithNumberRequest, CreatedDocumentWithNumber, Document, DocumentSearchFilters,
};
use crate::services::DocumentService;

/// ビジネスロジック層のDocumentハンドラー
/// HTTPハンドラーから呼び出される
#[derive(Clone)]
pub struct DocumentHandlers {
    document_service: DocumentService,
}

impl DocumentHandlers {
    pub fn new(document_service: DocumentService) -> Self {
        Self { document_service }
    }

    pub async fn create_document(
        &self,
        request: CreateDocumentWithNumberRequest,
    ) -> Result<CreatedDocumentWithNumber, AppError> {
        // バリデーション
        if request.title.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Title cannot be empty".to_string(),
            ));
        }

        // サービス層呼び出し
        self.document_service
            .create_document_with_number(request)
            .await
            .map_err(AppError::from)
    }

    pub async fn get_document(&self, id: i32) -> Result<Document, AppError> {
        match self.document_service.get_document_by_id(id).await? {
            Some(document) => Ok(document),
            None => Err(AppError::NotFound(format!(
                "Document with id {id} not found"
            ))),
        }
    }

    pub async fn search_documents(
        &self,
        filters: DocumentSearchFilters,
    ) -> Result<(Vec<Document>, i64), AppError> {
        self.document_service
            .search_documents(filters)
            .await
            .map_err(AppError::from)
    }
}

/// ヘルスチェック用ハンドラー
#[derive(Clone)]
pub struct HealthHandler;

impl Default for HealthHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn health_check(&self) -> Result<(), AppError> {
        // ヘルスチェックは常に成功
        Ok(())
    }
}
