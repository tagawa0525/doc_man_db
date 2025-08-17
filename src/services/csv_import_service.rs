use crate::error::{CsvImportError, ResolveError};
use crate::models::{
    Document, DocumentCsvRecord, ImportError, ImportOptions, ImportResult,
    ImportExecution, ImportStatus, validate_csv_headers,
};
use crate::repositories::DocumentRepository;
use crate::services::DocumentService;
use async_trait::async_trait;
use csv::ReaderBuilder;
use std::io::Read;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;
use tracing::{info, warn, error};

#[async_trait]
pub trait CsvImportService: Send + Sync {
    async fn import_documents_from_bytes(
        &self,
        data: Vec<u8>,
        file_name: String,
        options: ImportOptions,
    ) -> Result<ImportResult, CsvImportError>;
    
    async fn get_import_executions(&self) -> Result<Vec<ImportExecution>, CsvImportError>;
    
    async fn get_import_execution(&self, import_id: Uuid) -> Result<Option<ImportExecution>, CsvImportError>;
}

pub struct CsvImportServiceImpl {
    // TODO: 実際のサービス依存関係を追加
    // document_service: Box<dyn DocumentService>,
    // document_repository: Box<dyn DocumentRepository>,
}

impl CsvImportServiceImpl {
    pub fn new() -> Self {
        Self {
            // TODO: 実際の依存関係を注入
        }
    }

    async fn process_document_record(
        &self,
        record: DocumentCsvRecord,
        row_number: usize,
        options: &ImportOptions,
    ) -> Result<Document, ImportError> {
        // データ検証
        self.validate_record(&record, row_number)?;
        
        // 作成日の解析
        let created_date = self.parse_date(&record.created_date, row_number)?;
        
        // 文書種別の解決
        let document_type = self.resolve_document_type(&record.document_type_code, row_number).await?;
        
        // 作成者の解決
        let creator = self.resolve_creator(&record.creator_name, row_number).await?;
        
        // 重複チェック
        if options.skip_duplicates {
            if let Ok(existing) = self.find_duplicate_document(&record).await {
                return Err(ImportError {
                    row_number,
                    field: None,
                    message: format!("Duplicate document found: {}", existing.number),
                    raw_data: format!("{:?}", record),
                });
            }
        }
        
        // 文書作成リクエスト
        let create_request = crate::models::CreateDocumentRequest {
            number: None, // 自動生成
            title: record.title,
            document_type_id: document_type.id,
            business_number: record.business_number,
            created_by: creator.id,
            created_date,
            internal_external: record.internal_external,
            importance_class: record.importance_class,
            personal_info: record.personal_info,
            notes: record.notes,
        };
        
        // 文書作成（仮実装）
        // TODO: 実際の文書サービスを使用
        let document = Document {
            id: row_number as i32,
            number: format!("DOC-{:06}", row_number),
            title: create_request.title,
            document_type_id: create_request.document_type_id,
            business_number: create_request.business_number,
            created_by: create_request.created_by,
            created_date: create_request.created_date,
            internal_external: create_request.internal_external,
            importance_class: create_request.importance_class,
            personal_info: create_request.personal_info,
            notes: create_request.notes,
            network_path: Some(format!("\\\\server\\docs\\{}", format!("DOC-{:06}", row_number))),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(document)
    }
    
    fn validate_record(&self, record: &DocumentCsvRecord, row_number: usize) -> Result<(), ImportError> {
        // タイトル検証
        if record.title.trim().is_empty() {
            return Err(ImportError {
                row_number,
                field: Some("title".to_string()),
                message: "Title cannot be empty".to_string(),
                raw_data: record.title.clone(),
            });
        }
        
        if record.title.len() > 255 {
            return Err(ImportError {
                row_number,
                field: Some("title".to_string()),
                message: "Title too long (max 255 characters)".to_string(),
                raw_data: record.title.clone(),
            });
        }
        
        // 文書種別コード検証
        if record.document_type_code.trim().is_empty() {
            return Err(ImportError {
                row_number,
                field: Some("document_type_code".to_string()),
                message: "Document type code cannot be empty".to_string(),
                raw_data: record.document_type_code.clone(),
            });
        }
        
        // 作成者名検証
        if record.creator_name.trim().is_empty() {
            return Err(ImportError {
                row_number,
                field: Some("creator_name".to_string()),
                message: "Creator name cannot be empty".to_string(),
                raw_data: record.creator_name.clone(),
            });
        }
        
        // 業務番号検証（省略可能だが、指定された場合は形式チェック）
        if let Some(ref business_number) = record.business_number {
            if !business_number.trim().is_empty() && business_number.len() > 50 {
                return Err(ImportError {
                    row_number,
                    field: Some("business_number".to_string()),
                    message: "Business number too long (max 50 characters)".to_string(),
                    raw_data: business_number.clone(),
                });
            }
        }
        
        Ok(())
    }
    
    fn parse_date(&self, date_str: &str, row_number: usize) -> Result<NaiveDate, ImportError> {
        // 複数の日付形式をサポート
        let formats = vec![
            "%Y-%m-%d",
            "%Y/%m/%d", 
            "%d/%m/%Y",
            "%d-%m-%Y",
            "%Y年%m月%d日",
        ];
        
        for format in &formats {
            if let Ok(date) = NaiveDate::parse_from_str(date_str.trim(), format) {
                return Ok(date);
            }
        }
        
        Err(ImportError {
            row_number,
            field: Some("created_date".to_string()),
            message: format!("Invalid date format. Supported formats: YYYY-MM-DD, YYYY/MM/DD, DD/MM/YYYY, DD-MM-YYYY, YYYY年MM月DD日"),
            raw_data: date_str.to_string(),
        })
    }
    
    async fn resolve_document_type(
        &self,
        type_code: &str,
        row_number: usize,
    ) -> Result<crate::models::DocumentType, ImportError> {
        // TODO: 文書種別リポジトリから検索
        // 仮実装として、固定的な文書種別を返す
        
        let document_type = match type_code.trim().to_lowercase().as_str() {
            "tech" | "技術" => crate::models::DocumentType {
                id: 1,
                code: "TECH".to_string(),
                name: "技術文書".to_string(),
                description: Some("技術関連文書".to_string()),
                requires_approval: true,
                department_code: Some("A".to_string()),
                effective_from: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                effective_until: None,
                number_format: Some("技術-{YYMM}{###}".to_string()),
                is_active: true,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
            "plan" | "計画" => crate::models::DocumentType {
                id: 2,
                code: "PLAN".to_string(),
                name: "計画書".to_string(),
                description: Some("計画関連文書".to_string()),
                requires_approval: true,
                department_code: Some("B".to_string()),
                effective_from: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                effective_until: None,
                number_format: Some("計画-{YYMM}{###}".to_string()),
                is_active: true,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
            "report" | "レポート" => crate::models::DocumentType {
                id: 3,
                code: "REP".to_string(),
                name: "レポート".to_string(),
                description: Some("レポート関連文書".to_string()),
                requires_approval: false,
                department_code: Some("C".to_string()),
                effective_from: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                effective_until: None,
                number_format: Some("REP-{YYMM}{###}".to_string()),
                is_active: true,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
            _ => {
                return Err(ImportError {
                    row_number,
                    field: Some("document_type_code".to_string()),
                    message: format!("Unknown document type: {}", type_code),
                    raw_data: type_code.to_string(),
                });
            }
        };
        
        Ok(document_type)
    }
    
    async fn resolve_creator(
        &self,
        creator_name: &str,
        row_number: usize,
    ) -> Result<crate::models::Employee, ImportError> {
        // TODO: 社員リポジトリから検索
        // 仮実装として、固定的な社員を返す
        
        let employee = crate::models::Employee {
            id: 1,
            employee_number: Some("001".to_string()),
            name: creator_name.to_string(),
            email: Some(format!("{}@company.com", creator_name.replace(" ", ".").to_lowercase())),
            ad_username: Some(creator_name.replace(" ", ".").to_lowercase()),
            department_id: Some(1),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(employee)
    }
    
    async fn find_duplicate_document(
        &self,
        _record: &DocumentCsvRecord,
    ) -> Result<Document, CsvImportError> {
        // TODO: 重複チェックロジック実装
        // タイトル、作成者、作成日が同じ文書があるかチェック
        Err(CsvImportError::Resolve(ResolveError::NotFound))
    }
}

#[async_trait]
impl CsvImportService for CsvImportServiceImpl {
    async fn import_documents_from_bytes(
        &self,
        data: Vec<u8>,
        file_name: String,
        options: ImportOptions,
    ) -> Result<ImportResult, CsvImportError> {
        let import_id = Uuid::new_v4();
        let start_time = Utc::now();
        
        info!("Starting CSV import: {} (ID: {})", file_name, import_id);
        
        let reader = std::io::Cursor::new(data);
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true) // 可変長列に対応
            .from_reader(reader);
        
        let mut result = ImportResult {
            import_id,
            total_records: 0,
            successful_imports: 0,
            failed_imports: 0,
            errors: Vec::new(),
            start_time,
            end_time: start_time,
        };
        
        // ヘッダー検証
        let headers = csv_reader.headers()?;
        validate_csv_headers(headers)?;
        
        info!("CSV headers validated successfully");
        
        // レコード処理
        for (row_index, record_result) in csv_reader.deserialize::<DocumentCsvRecord>().enumerate() {
            result.total_records += 1;
            let row_number = row_index + 2; // ヘッダー行を考慮
            
            match record_result {
                Ok(record) => {
                    match self.process_document_record(record, row_number, &options).await {
                        Ok(document) => {
                            result.successful_imports += 1;
                            info!("Successfully imported document: {} (row {})", document.number, row_number);
                        }
                        Err(error) => {
                            result.failed_imports += 1;
                            result.errors.push(error);
                            warn!("Failed to import row {}: {}", row_number, result.errors.last().unwrap().message);
                        }
                    }
                }
                Err(csv_error) => {
                    result.failed_imports += 1;
                    let error = ImportError {
                        row_number,
                        field: None,
                        message: format!("CSV parsing error: {}", csv_error),
                        raw_data: String::new(),
                    };
                    result.errors.push(error);
                    warn!("CSV parsing error at row {}: {}", row_number, csv_error);
                }
            }
        }
        
        result.end_time = Utc::now();
        
        info!(
            "CSV import completed: {} total, {} successful, {} failed (Duration: {:?})",
            result.total_records,
            result.successful_imports,
            result.failed_imports,
            result.end_time - result.start_time
        );
        
        Ok(result)
    }
    
    async fn get_import_executions(&self) -> Result<Vec<ImportExecution>, CsvImportError> {
        // TODO: データベースから取得
        Ok(vec![])
    }
    
    async fn get_import_execution(&self, _import_id: Uuid) -> Result<Option<ImportExecution>, CsvImportError> {
        // TODO: データベースから取得
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[tokio::test]
    async fn test_csv_import_basic() {
        let csv_data = r#"title,document_type_code,creator_name,created_date
"テスト文書1","tech","山田太郎","2024-01-15"
"テスト文書2","plan","佐藤花子","2024-01-16"
"#;
        
        let reader = Cursor::new(csv_data);
        
        // Mock services would be injected here in real implementation
        // For now, this test demonstrates the structure
    }
    
    #[test]
    fn test_parse_date_formats() {
        let service = CsvImportServiceImpl::new(
            Box::new(MockDocumentService::new()),
            Box::new(MockDocumentRepository::new()),
        );
        
        // Test various date formats
        assert!(service.parse_date("2024-01-15", 1).is_ok());
        assert!(service.parse_date("2024/01/15", 1).is_ok());
        assert!(service.parse_date("15/01/2024", 1).is_ok());
        assert!(service.parse_date("15-01-2024", 1).is_ok());
        assert!(service.parse_date("2024年01月15日", 1).is_ok());
        
        // Invalid format should fail
        assert!(service.parse_date("invalid-date", 1).is_err());
    }
}

// Mock implementations for testing
#[cfg(test)]
struct MockDocumentService;
#[cfg(test)]
struct MockDocumentRepository;

#[cfg(test)]
impl MockDocumentService {
    fn new() -> Self { Self }
}

#[cfg(test)]
impl MockDocumentRepository {
    fn new() -> Self { Self }
}

// TODO: Implement mock traits when DocumentService and DocumentRepository traits are defined