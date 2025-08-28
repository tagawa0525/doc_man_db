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

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Deduplication error: {0}")]
    Deduplication(#[from] DeduplicationError),

    #[error("Batch processing error: {0}")]
    Batch(#[from] BatchError),

    #[error("Business error: {0}")]
    Business(#[from] BusinessError),

    #[error("Search error: {0}")]
    Search(#[from] SearchError),
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

#[derive(thiserror::Error, Debug)]
pub enum BusinessError {
    #[error("Business not found")]
    NotFound,

    #[error("Business number already exists: {0}")]
    BusinessNumberExists(String),

    #[error("Business member already exists")]
    MemberAlreadyExists,

    #[error("Employee not found")]
    EmployeeNotFound,

    #[error("Business not found")]
    BusinessNotFound,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Invalid business status: {0}")]
    InvalidStatus(String),

    #[error("Invalid role: {0}")]
    InvalidRole(String),

    #[error("Invalid participation level: {0}")]
    InvalidParticipationLevel(String),

    #[error("Invalid date range")]
    InvalidDateRange,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum SearchError {
    #[error("Invalid search parameters: {0}")]
    InvalidParameters(String),

    #[error("Search timeout")]
    Timeout,

    #[error("Too many results: {count}")]
    TooManyResults { count: usize },

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

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
            AppError::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::Database(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Deduplication(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Batch(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Business(ref business_error) => match business_error {
                BusinessError::NotFound
                | BusinessError::BusinessNotFound
                | BusinessError::EmployeeNotFound => axum::http::StatusCode::NOT_FOUND,
                BusinessError::PermissionDenied => axum::http::StatusCode::FORBIDDEN,
                BusinessError::BusinessNumberExists(_) | BusinessError::MemberAlreadyExists => {
                    axum::http::StatusCode::CONFLICT
                }
                BusinessError::InvalidStatus(_)
                | BusinessError::InvalidRole(_)
                | BusinessError::InvalidParticipationLevel(_)
                | BusinessError::InvalidDateRange => axum::http::StatusCode::BAD_REQUEST,
                _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::Search(ref search_error) => match search_error {
                SearchError::InvalidParameters(_) => axum::http::StatusCode::BAD_REQUEST,
                SearchError::Timeout => axum::http::StatusCode::REQUEST_TIMEOUT,
                SearchError::TooManyResults { .. } => axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}
