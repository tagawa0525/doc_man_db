use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// CSV インポート用の文書レコード
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentCsvRecord {
    pub title: String,
    pub document_type_code: String,
    pub business_number: Option<String>,
    pub creator_name: String,
    pub created_date: String,
    pub department_code: Option<String>,
    pub internal_external: Option<String>,
    pub importance_class: Option<String>,
    pub personal_info: Option<String>,
    pub notes: Option<String>,
}

/// インポート結果
#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub import_id: Uuid,
    pub total_records: usize,
    pub successful_imports: usize,
    pub failed_imports: usize,
    pub errors: Vec<ImportError>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

/// インポートエラー
#[derive(Debug, Serialize)]
pub struct ImportError {
    pub row_number: usize,
    pub field: Option<String>,
    pub message: String,
    pub raw_data: String,
}

/// インポートオプション
#[derive(Debug)]
pub struct ImportOptions {
    pub skip_duplicates: bool,
    pub validate_references: bool,
    pub auto_create_references: bool,
    pub user_id: i32,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            skip_duplicates: true,
            validate_references: true,
            auto_create_references: false,
            user_id: 1, // Default system user
        }
    }
}

/// インポート実行記録
#[derive(Debug, Serialize)]
pub struct ImportExecution {
    pub id: Uuid,
    pub file_name: String,
    pub total_records: usize,
    pub successful_imports: usize,
    pub failed_imports: usize,
    pub status: ImportStatus,
    pub started_by: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_details: Option<String>,
}

#[derive(Debug, Serialize, sqlx::Type)]
#[sqlx(type_name = "import_status", rename_all = "lowercase")]
pub enum ImportStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for ImportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportStatus::Running => write!(f, "running"),
            ImportStatus::Completed => write!(f, "completed"),
            ImportStatus::Failed => write!(f, "failed"),
            ImportStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// CSV ヘッダー検証
pub fn validate_csv_headers(headers: &csv::StringRecord) -> Result<(), crate::error::CsvImportError> {
    let required_headers = vec![
        "title",
        "document_type_code", 
        "creator_name",
        "created_date"
    ];
    
    let mut missing_headers = Vec::new();
    
    for required in &required_headers {
        if !headers.iter().any(|h| h.trim().to_lowercase() == required.to_lowercase()) {
            missing_headers.push(required.to_string());
        }
    }
    
    if !missing_headers.is_empty() {
        return Err(crate::error::CsvImportError::MissingHeaders { 
            columns: missing_headers 
        });
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_csv_headers_success() {
        let mut headers = csv::StringRecord::new();
        headers.push_field("title");
        headers.push_field("document_type_code");
        headers.push_field("creator_name");
        headers.push_field("created_date");
        headers.push_field("optional_field");
        
        assert!(validate_csv_headers(&headers).is_ok());
    }

    #[test]
    fn test_validate_csv_headers_missing() {
        let mut headers = csv::StringRecord::new();
        headers.push_field("title");
        headers.push_field("creator_name");
        // Missing document_type_code and created_date
        
        let result = validate_csv_headers(&headers);
        assert!(result.is_err());
        
        if let Err(crate::error::CsvImportError::MissingHeaders { columns }) = result {
            assert_eq!(columns.len(), 2);
            assert!(columns.contains(&"document_type_code".to_string()));
            assert!(columns.contains(&"created_date".to_string()));
        } else {
            panic!("Expected MissingHeaders error");
        }
    }
}