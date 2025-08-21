use axum::http::StatusCode;
use doc_man_db::error::{AppError, BatchError, CsvImportError, DeduplicationError, ResolveError};
use std::io;

#[test]
fn test_app_error_variants() {
    let errors = vec![
        AppError::ValidationError("test validation".to_string()),
        AppError::NotFound("test not found".to_string()),
        AppError::InternalError("test internal".to_string()),
    ];

    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}

#[test]
fn test_csv_import_error_variants() {
    let error1 = CsvImportError::InvalidFormat {
        message: "test format".to_string(),
    };
    let error2 = CsvImportError::MissingHeaders {
        columns: vec!["col1".to_string(), "col2".to_string()],
    };
    let error3 = CsvImportError::DataValidation {
        row: 1,
        message: "test validation".to_string(),
    };

    assert!(error1.to_string().contains("test format"));
    assert!(error2.to_string().contains("col1"));
    assert!(error3.to_string().contains("row 1"));
}

#[test]
fn test_resolve_error_variants() {
    let error1 = ResolveError::NotFound;
    let error2 = ResolveError::Ambiguous(vec!["item1".to_string(), "item2".to_string()]);

    assert!(error1.to_string().contains("Not found"));
    assert!(error2.to_string().contains("Ambiguous"));
}

#[test]
fn test_deduplication_error_variants() {
    let error = DeduplicationError::InvalidThreshold { threshold: 0.5 };
    assert!(error.to_string().contains("0.5"));
}

#[test]
fn test_batch_error_variants() {
    let error1 = BatchError::Scheduler("scheduler failed".to_string());
    let error2 = BatchError::JobExecution("job failed".to_string());

    assert!(error1.to_string().contains("scheduler failed"));
    assert!(error2.to_string().contains("job failed"));
}

#[test]
fn test_app_error_from_csv_import_error() {
    let csv_error = CsvImportError::InvalidFormat {
        message: "test".to_string(),
    };
    let app_error: AppError = csv_error.into();

    match app_error {
        AppError::CsvImport(_) => (),
        _ => panic!("Expected CsvImport variant"),
    }
}

#[test]
fn test_app_error_from_deduplication_error() {
    let dedup_error = DeduplicationError::InvalidThreshold { threshold: 0.5 };
    let app_error: AppError = dedup_error.into();

    match app_error {
        AppError::Deduplication(_) => (),
        _ => panic!("Expected Deduplication variant"),
    }
}

#[test]
fn test_app_error_from_batch_error() {
    let batch_error = BatchError::Scheduler("test".to_string());
    let app_error: AppError = batch_error.into();

    match app_error {
        AppError::Batch(_) => (),
        _ => panic!("Expected Batch variant"),
    }
}

#[test]
fn test_app_error_to_status_code() {
    let test_cases = vec![
        (
            AppError::ValidationError("test".to_string()),
            StatusCode::BAD_REQUEST,
        ),
        (
            AppError::NotFound("test".to_string()),
            StatusCode::NOT_FOUND,
        ),
        (
            AppError::InternalError("test".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    ];

    for (error, expected_status) in test_cases {
        let status: StatusCode = error.into();
        assert_eq!(status, expected_status);
    }
}

#[test]
fn test_batch_error_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let batch_error: BatchError = io_error.into();

    match batch_error {
        BatchError::Io(_) => (),
        _ => panic!("Expected Io variant"),
    }
}

#[test]
fn test_batch_error_from_serde_error() {
    let serde_error = serde_json::from_str::<i32>("invalid").unwrap_err();
    let batch_error: BatchError = serde_error.into();

    match batch_error {
        BatchError::Serialization(_) => (),
        _ => panic!("Expected Serialization variant"),
    }
}

#[test]
fn test_error_debug_format() {
    let error = AppError::ValidationError("test validation".to_string());
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("ValidationError"));
    assert!(debug_str.contains("test validation"));
}

#[test]
fn test_error_display_format() {
    let error = CsvImportError::InvalidFormat {
        message: "test format".to_string(),
    };
    let display_str = error.to_string();
    assert!(display_str.contains("Invalid CSV format"));
    assert!(display_str.contains("test format"));
}

#[test]
fn test_resolve_error_ambiguous_display() {
    let candidates = vec!["candidate1".to_string(), "candidate2".to_string()];
    let error = ResolveError::Ambiguous(candidates);
    let display_str = error.to_string();
    assert!(display_str.contains("Ambiguous"));
    assert!(display_str.contains("multiple candidates"));
}
