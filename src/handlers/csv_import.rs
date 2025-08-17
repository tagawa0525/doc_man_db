use crate::error::AppError;
use crate::models::{ImportOptions, ImportResult};
use crate::services::CsvImportService;
use crate::AppState;
use axum::{
    extract::{Multipart, State, Path, Query},
    response::Json,
    Extension,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

/// CSVファイルアップロード＆インポート
pub async fn upload_and_import_csv(
    State(_app_state): State<AppState>,
    Extension(user_id): Extension<i32>, // 認証から取得されるユーザーID
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    info!("Starting CSV upload and import");

    let mut file_name = String::new();
    let mut file_data: Option<Vec<u8>> = None;
    let mut import_options = ImportOptions::default();

    // マルチパート データを処理
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::ValidationError(format!("Failed to read multipart field: {}", e))
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        
        match field_name.as_str() {
            "file" => {
                file_name = field.file_name().unwrap_or("unknown.csv").to_string();
                file_data = Some(field.bytes().await.map_err(|e| {
                    AppError::ValidationError(format!("Failed to read file data: {}", e))
                })?.to_vec());
            }
            "skip_duplicates" => {
                let value = field.text().await.map_err(|e| {
                    AppError::ValidationError(format!("Failed to read skip_duplicates: {}", e))
                })?;
                import_options.skip_duplicates = value.parse().unwrap_or(true);
            }
            "validate_references" => {
                let value = field.text().await.map_err(|e| {
                    AppError::ValidationError(format!("Failed to read validate_references: {}", e))
                })?;
                import_options.validate_references = value.parse().unwrap_or(true);
            }
            "auto_create_references" => {
                let value = field.text().await.map_err(|e| {
                    AppError::ValidationError(format!("Failed to read auto_create_references: {}", e))
                })?;
                import_options.auto_create_references = value.parse().unwrap_or(false);
            }
            _ => {
                warn!("Unknown field in multipart data: {}", field_name);
            }
        }
    }

    // ファイルデータの検証
    let file_data = file_data.ok_or_else(|| {
        AppError::ValidationError("No file uploaded".to_string())
    })?;

    if file_data.is_empty() {
        return Err(AppError::ValidationError("Empty file uploaded".to_string()));
    }

    // CSVファイル形式の簡易チェック
    if !file_name.to_lowercase().ends_with(".csv") {
        return Err(AppError::ValidationError(
            "File must be a CSV file (.csv extension required)".to_string()
        ));
    }

    info!("Processing CSV file: {} ({} bytes)", file_name, file_data.len());

    // ユーザーIDを設定
    import_options.user_id = user_id;

    // TODO: 実際のCSVインポートサービス実装に置き換える
    let result = create_mock_import_result(&file_name);

    info!(
        "CSV import completed for {}: {} total, {} successful, {} failed",
        file_name, result.total_records, result.successful_imports, result.failed_imports
    );

    Ok(Json(json!({
        "import_id": result.import_id,
        "file_name": file_name,
        "total_records": result.total_records,
        "successful_imports": result.successful_imports,
        "failed_imports": result.failed_imports,
        "errors": result.errors,
        "start_time": result.start_time,
        "end_time": result.end_time,
        "duration_seconds": (result.end_time - result.start_time).num_seconds()
    })))
}

/// インポート実行履歴の取得
pub async fn get_import_executions(
    State(_app_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    // TODO: 実際のCSVインポートサービス実装に置き換える
    let executions = get_mock_import_executions();
    
    Ok(Json(json!({
        "executions": executions
    })))
}

/// 特定のインポート実行の詳細取得
pub async fn get_import_execution(
    State(_app_state): State<AppState>,
    Path(import_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    // TODO: 実際のCSVインポートサービス実装に置き換える
    let execution = get_mock_import_execution(import_id)
        .ok_or_else(|| AppError::NotFound(format!("Import execution not found: {}", import_id)))?;
    
    Ok(Json(json!(execution)))
}

/// CSVテンプレートのダウンロード
pub async fn download_csv_template() -> Result<(axum::http::HeaderMap, String), AppError> {
    let template_csv = r#"title,document_type_code,creator_name,created_date,business_number,department_code,internal_external,importance_class,personal_info,notes
"サンプル文書1","tech","山田太郎","2024-01-15","PJ2024-001","A","internal","class2","none","サンプルノート"
"サンプル文書2","plan","佐藤花子","2024-01-16","","B","external","class1","none",""
"#;

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/csv; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        "attachment; filename=\"document_import_template.csv\"".parse().unwrap(),
    );

    Ok((headers, template_csv.to_string()))
}

/// インポート進捗の取得（WebSocket または Server-Sent Events で実装予定）
pub async fn get_import_progress(
    State(_app_state): State<AppState>,
    Path(import_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    // TODO: リアルタイム進捗取得の実装
    // 現在は固定値を返す
    Ok(Json(json!({
        "import_id": import_id,
        "status": "completed",
        "progress": 100,
        "current_record": 0,
        "total_records": 0,
        "message": "Import completed"
    })))
}

/// バリデーションのみ実行（実際のインポートは行わない）
pub async fn validate_csv(
    State(_app_state): State<AppState>,
    Extension(user_id): Extension<i32>,
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    info!("Starting CSV validation");

    let mut file_name = String::new();
    let mut file_data: Option<Vec<u8>> = None;

    // ファイルデータを取得
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::ValidationError(format!("Failed to read multipart field: {}", e))
    })? {
        if let Some(field_name) = field.name() {
            if field_name == "file" {
                file_name = field.file_name().unwrap_or("unknown.csv").to_string();
                file_data = Some(field.bytes().await.map_err(|e| {
                    AppError::ValidationError(format!("Failed to read file data: {}", e))
                })?.to_vec());
                break;
            }
        }
    }

    let file_data = file_data.ok_or_else(|| {
        AppError::ValidationError("No file uploaded".to_string())
    })?;

    // CSVの構造検証のみ実行
    let reader = std::io::Cursor::new(file_data);
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    // ヘッダー検証
    let headers = csv_reader.headers().map_err(|e| {
        AppError::CsvImport(crate::error::CsvImportError::Parsing(e))
    })?;
    
    crate::models::validate_csv_headers(headers)?;

    // レコード数とサンプルレコードを取得
    let mut record_count = 0;
    let mut sample_records = Vec::new();
    let mut errors = Vec::new();

    for (index, record_result) in csv_reader.records().enumerate() {
        record_count += 1;
        
        if record_count <= 5 {
            match record_result {
                Ok(record) => {
                    let record_map: std::collections::HashMap<String, String> = headers
                        .iter()
                        .zip(record.iter())
                        .map(|(h, v)| (h.to_string(), v.to_string()))
                        .collect();
                    sample_records.push(record_map);
                }
                Err(e) => {
                    errors.push(json!({
                        "row": index + 2,
                        "error": format!("CSV parsing error: {}", e)
                    }));
                }
            }
        }
        
        if record_count > 1000 {
            break; // 大きなファイルの場合は制限
        }
    }

    info!("CSV validation completed for {}: {} records", file_name, record_count);

    Ok(Json(json!({
        "file_name": file_name,
        "is_valid": errors.is_empty(),
        "record_count": record_count,
        "headers": headers.iter().collect::<Vec<_>>(),
        "sample_records": sample_records,
        "errors": errors
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    
    #[tokio::test]
    async fn test_download_csv_template() {
        let result = download_csv_template().await;
        assert!(result.is_ok());
        
        let (headers, content) = result.unwrap();
        assert!(headers.contains_key(axum::http::header::CONTENT_TYPE));
        assert!(headers.contains_key(axum::http::header::CONTENT_DISPOSITION));
        assert!(content.contains("title,document_type_code"));
    }
}

// モック関数の実装
fn create_mock_import_result(_file_name: &str) -> serde_json::Value {
    use chrono::Utc;
    
    json!({
        "import_id": uuid::Uuid::new_v4(),
        "total_records": 100,
        "successful_imports": 95,
        "failed_imports": 5,
        "errors": [],
        "start_time": Utc::now(),
        "end_time": Utc::now()
    })
}

fn get_mock_import_executions() -> Vec<serde_json::Value> {
    use chrono::Utc;
    
    vec![
        json!({
            "import_id": uuid::Uuid::new_v4(),
            "file_name": "documents_2024.csv",
            "total_records": 100,
            "successful_imports": 95,
            "failed_imports": 5,
            "start_time": Utc::now(),
            "end_time": Utc::now(),
            "status": "completed"
        })
    ]
}

fn get_mock_import_execution(import_id: Uuid) -> Option<serde_json::Value> {
    use chrono::Utc;
    
    Some(json!({
        "import_id": import_id,
        "file_name": "documents_2024.csv",
        "total_records": 100,
        "successful_imports": 95,
        "failed_imports": 5,
        "start_time": Utc::now(),
        "end_time": Utc::now(),
        "status": "completed",
        "errors": []
    }))
}