use crate::repositories::RepositoryError;
use crate::services::DocumentServiceError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<DocumentServiceError> for AppError {
    fn from(error: DocumentServiceError) -> Self {
        match error {
            DocumentServiceError::ValidationError(_) => {
                AppError::ValidationError(error.to_string())
            }
            DocumentServiceError::NumberGenerationError(_) => {
                AppError::InternalError(error.to_string())
            }
            DocumentServiceError::RepositoryError(repo_err) => AppError::Repository(repo_err),
        }
    }
}

impl From<AppError> for axum::http::StatusCode {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Repository(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            AppError::InternalError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
