use crate::error::BatchError;
use crate::batch::{BatchExecution, BatchType, BatchStatus};
use crate::models::Document;
use std::path::Path;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};
use tokio::fs;

/// ファイル存在確認サービス
pub struct FileCheckService {
    // TODO: 実際のリポジトリ依存関係を追加
}

/// ファイル確認結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCheckResult {
    pub document_id: i32,
    pub document_number: String,
    pub document_path: String,
    pub folder_exists: bool,
    pub main_file_exists: bool,
    pub approval_file_exists: Option<bool>, // Some(bool) if approval required, None if not
    pub last_checked: DateTime<Utc>,
    pub error_message: Option<String>,
}

/// ファイル確認統計
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCheckStatistics {
    pub total_documents: i32,
    pub checked_documents: i32,
    pub existing_folders: i32,
    pub missing_folders: i32,
    pub existing_files: i32,
    pub missing_files: i32,
    pub existing_approvals: i32,
    pub missing_approvals: i32,
    pub check_errors: i32,
}

impl FileCheckService {
    pub fn new() -> Self {
        Self {}
    }
    
    /// 月次ファイル確認を実行
    pub async fn run_monthly_check(&self, execution_id: Uuid) -> Result<BatchExecution, BatchError> {
        info!("Starting monthly file check: {}", execution_id);
        
        let start_time = Utc::now();
        let mut execution = BatchExecution {
            id: execution_id,
            batch_type: BatchType::FileCheck,
            status: BatchStatus::Running,
            total_items: 0,
            processed_items: 0,
            success_count: 0,
            error_count: 0,
            start_time,
            end_time: None,
            started_by: None,
            result_summary: None,
            error_details: None,
        };
        
        // 対象文書を取得
        let documents = self.get_check_target_documents().await?;
        execution.total_items = documents.len() as i32;
        
        info!("Found {} documents to check", documents.len());
        
        let mut results = Vec::new();
        let mut statistics = FileCheckStatistics {
            total_documents: documents.len() as i32,
            checked_documents: 0,
            existing_folders: 0,
            missing_folders: 0,
            existing_files: 0,
            missing_files: 0,
            existing_approvals: 0,
            missing_approvals: 0,
            check_errors: 0,
        };
        
        // 各文書のファイル存在確認
        for document in documents {
            match self.check_document_files(&document).await {
                Ok(result) => {
                    execution.processed_items += 1;
                    statistics.checked_documents += 1;
                    
                    // 統計更新
                    if result.folder_exists {
                        statistics.existing_folders += 1;
                    } else {
                        statistics.missing_folders += 1;
                    }
                    
                    if result.main_file_exists {
                        statistics.existing_files += 1;
                    } else {
                        statistics.missing_files += 1;
                    }
                    
                    match result.approval_file_exists {
                        Some(true) => statistics.existing_approvals += 1,
                        Some(false) => statistics.missing_approvals += 1,
                        None => {} // 承認不要
                    }
                    
                    if result.folder_exists && result.main_file_exists 
                        && result.approval_file_exists.unwrap_or(true) {
                        execution.success_count += 1;
                    }
                    
                    results.push(result);
                }
                Err(error) => {
                    execution.error_count += 1;
                    statistics.check_errors += 1;
                    
                    warn!("Failed to check document {}: {}", document.id, error);
                    
                    results.push(FileCheckResult {
                        document_id: document.id,
                        document_number: document.number.clone(),
                        document_path: document.network_path.clone().unwrap_or_default(),
                        folder_exists: false,
                        main_file_exists: false,
                        approval_file_exists: None,
                        last_checked: Utc::now(),
                        error_message: Some(error.to_string()),
                    });
                }
            }
            
            // 進捗ログ（100件毎）
            if execution.processed_items % 100 == 0 {
                info!("File check progress: {}/{}", execution.processed_items, execution.total_items);
            }
        }
        
        // 実行結果の更新
        execution.end_time = Some(Utc::now());
        execution.status = if execution.error_count == 0 {
            BatchStatus::Completed
        } else if execution.success_count > 0 {
            BatchStatus::Completed
        } else {
            BatchStatus::Failed
        };
        
        execution.result_summary = Some(serde_json::to_string(&statistics)?);
        
        // 結果をデータベースに保存
        self.save_check_results(&results).await?;
        
        info!(
            "Monthly file check completed: {}/{} successful, {} errors, duration: {:?}",
            execution.success_count,
            execution.total_items,
            execution.error_count,
            execution.end_time.unwrap() - execution.start_time
        );
        
        Ok(execution)
    }
    
    /// 個別文書のファイル確認を実行
    pub async fn check_document_files(&self, document: &Document) -> Result<FileCheckResult, BatchError> {
        let network_path = document.network_path.as_ref()
            .ok_or_else(|| BatchError::JobExecution("Document has no network path".to_string()))?;
        
        let document_folder = Path::new(network_path);
        
        // フォルダ存在確認
        let folder_exists = document_folder.exists();
        
        // メインファイル存在確認
        let main_file_exists = if folder_exists {
            self.check_main_file_exists(document_folder, &document.number).await?
        } else {
            false
        };
        
        // 承認ファイル存在確認
        let approval_file_exists = if folder_exists && self.requires_approval(document) {
            Some(self.check_approval_file_exists(document_folder, &document.number).await?)
        } else {
            None
        };
        
        Ok(FileCheckResult {
            document_id: document.id,
            document_number: document.number.clone(),
            document_path: network_path.clone(),
            folder_exists,
            main_file_exists,
            approval_file_exists,
            last_checked: Utc::now(),
            error_message: None,
        })
    }
    
    /// メインファイルの存在確認
    async fn check_main_file_exists(&self, folder_path: &Path, document_number: &str) -> Result<bool, BatchError> {
        // 複数の拡張子を確認
        let extensions = vec!["pdf", "docx", "xlsx", "pptx", "doc", "xls", "ppt"];
        
        for ext in &extensions {
            let file_path = folder_path.join(format!("{}.{}", document_number, ext));
            if file_path.exists() {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// 承認ファイルの存在確認
    async fn check_approval_file_exists(&self, folder_path: &Path, document_number: &str) -> Result<bool, BatchError> {
        // 承認ファイルの命名規則: [document_number]-審査承認.pdf
        let approval_file_path = folder_path.join(format!("{}-審査承認.pdf", document_number));
        Ok(approval_file_path.exists())
    }
    
    /// 承認が必要な文書かチェック
    fn requires_approval(&self, document: &Document) -> bool {
        // TODO: 文書種別に基づく承認要否判定
        // 現在は重要度クラスⅠの文書は承認必要とする
        document.importance_class.as_ref().map_or(false, |class| class == "class1")
    }
    
    /// 確認対象文書を取得
    async fn get_check_target_documents(&self) -> Result<Vec<Document>, BatchError> {
        // TODO: 実際のデータベースから取得
        // 現在は仮のデータを返す
        Ok(vec![
            Document {
                id: 1,
                number: "CTA-2508001".to_string(),
                title: "システム設計書 v2.1".to_string(),
                document_type_id: 1,
                business_number: Some("PJ2024-001".to_string()),
                created_by: 1,
                created_date: chrono::NaiveDate::from_ymd_opt(2024, 8, 15).unwrap(),
                internal_external: Some("internal".to_string()),
                importance_class: Some("class1".to_string()),
                personal_info: Some("none".to_string()),
                notes: None,
                network_path: Some("\\\\server\\documents\\2024\\08\\CTA-2508001".to_string()),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Document {
                id: 2,
                number: "技術-25001".to_string(),
                title: "データベース移行計画書".to_string(),
                document_type_id: 2,
                business_number: Some("PJ2024-002".to_string()),
                created_by: 2,
                created_date: chrono::NaiveDate::from_ymd_opt(2024, 8, 10).unwrap(),
                internal_external: Some("internal".to_string()),
                importance_class: Some("class2".to_string()),
                personal_info: Some("none".to_string()),
                notes: None,
                network_path: Some("\\\\server\\documents\\2024\\08\\技術-25001".to_string()),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Document {
                id: 3,
                number: "REP-2508001".to_string(),
                title: "月次運用レポート 2024年7月".to_string(),
                document_type_id: 3,
                business_number: None,
                created_by: 3,
                created_date: chrono::NaiveDate::from_ymd_opt(2024, 8, 1).unwrap(),
                internal_external: Some("internal".to_string()),
                importance_class: Some("class2".to_string()),
                personal_info: Some("none".to_string()),
                notes: None,
                network_path: Some("\\\\server\\documents\\2024\\08\\REP-2508001".to_string()),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ])
    }
    
    /// 確認結果をデータベースに保存
    async fn save_check_results(&self, results: &[FileCheckResult]) -> Result<(), BatchError> {
        // TODO: 実際のデータベース保存処理
        info!("Saving {} file check results to database", results.len());
        Ok(())
    }
    
    /// 最新の確認結果を取得
    pub async fn get_latest_check_results(&self, _limit: Option<i32>) -> Result<Vec<FileCheckResult>, BatchError> {
        // TODO: データベースから最新結果を取得
        Ok(vec![])
    }
    
    /// 不存在ファイル一覧を取得
    pub async fn get_missing_files(&self) -> Result<Vec<FileCheckResult>, BatchError> {
        // TODO: データベースから不存在ファイルを取得
        Ok(vec![])
    }
    
    /// ファイル確認統計を取得
    pub async fn get_check_statistics(&self, _date_range: Option<(DateTime<Utc>, DateTime<Utc>)>) -> Result<FileCheckStatistics, BatchError> {
        // TODO: データベースから統計を計算
        Ok(FileCheckStatistics {
            total_documents: 0,
            checked_documents: 0,
            existing_folders: 0,
            missing_folders: 0,
            existing_files: 0,
            missing_files: 0,
            existing_approvals: 0,
            missing_approvals: 0,
            check_errors: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;

    #[tokio::test]
    async fn test_check_main_file_exists() {
        let service = FileCheckService::new();
        let temp_dir = TempDir::new().unwrap();
        
        // テスト用ファイルを作成
        let test_file = temp_dir.path().join("DOC-001.pdf");
        File::create(&test_file).unwrap();
        
        let exists = service.check_main_file_exists(temp_dir.path(), "DOC-001").await.unwrap();
        assert!(exists);
        
        let not_exists = service.check_main_file_exists(temp_dir.path(), "DOC-999").await.unwrap();
        assert!(!not_exists);
    }
    
    #[tokio::test]
    async fn test_check_approval_file_exists() {
        let service = FileCheckService::new();
        let temp_dir = TempDir::new().unwrap();
        
        // 承認ファイルを作成
        let approval_file = temp_dir.path().join("DOC-001-審査承認.pdf");
        File::create(&approval_file).unwrap();
        
        let exists = service.check_approval_file_exists(temp_dir.path(), "DOC-001").await.unwrap();
        assert!(exists);
        
        let not_exists = service.check_approval_file_exists(temp_dir.path(), "DOC-999").await.unwrap();
        assert!(!not_exists);
    }
    
    #[test]
    fn test_requires_approval() {
        let service = FileCheckService::new();
        
        let document_with_approval = Document {
            id: 1,
            number: "DOC-001".to_string(),
            title: "Test".to_string(),
            document_type_id: 1,
            business_number: None,
            created_by: 1,
            created_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            internal_external: None,
            importance_class: Some("class1".to_string()),
            personal_info: None,
            notes: None,
            network_path: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        assert!(service.requires_approval(&document_with_approval));
        
        let document_without_approval = Document {
            importance_class: Some("class2".to_string()),
            ..document_with_approval
        };
        
        assert!(!service.requires_approval(&document_without_approval));
    }
}