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

    #[error("CSV import error: {0}")]
    CsvImport(#[from] CsvImportError),

    #[error("Deduplication error: {0}")]
    Deduplication(#[from] DeduplicationError),

    #[error("Batch processing error: {0}")]
    Batch(#[from] BatchError),
}

#[derive(thiserror::Error, Debug)]
pub enum CsvImportError {
    #[error("CSV parsing error: {0}")]
    Parsing(#[from] csv::Error),

    #[error("Invalid CSV format: {message}")]
    InvalidFormat { message: String },

    #[error("Header validation failed: missing required columns: {columns:?}")]
    MissingHeaders { columns: Vec<String> },

    #[error("Data validation failed at row {row}: {message}")]
    DataValidation { row: usize, message: String },

    #[error("Resolve error: {0}")]
    Resolve(#[from] ResolveError),
}

#[derive(thiserror::Error, Debug)]
pub enum ResolveError {
    #[error("Not found")]
    NotFound,

    #[error("Ambiguous: multiple candidates found")]
    Ambiguous(Vec<String>),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DeduplicationError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid similarity threshold: {threshold}")]
    InvalidThreshold { threshold: f64 },
}

#[derive(thiserror::Error, Debug)]
pub enum BatchError {
    #[error("Scheduler error: {0}")]
    Scheduler(String),

    #[error("Job execution error: {0}")]
    JobExecution(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
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
            AppError::CsvImport(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::Deduplication(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Batch(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
