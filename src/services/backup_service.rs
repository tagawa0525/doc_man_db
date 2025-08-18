use crate::models::backup::{
    BackupConfig, BackupJob, BackupRequest, BackupStatistics, BackupType,
    RestoreJob, RestoreRequest,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::path::Path;
use thiserror::Error;
use tokio::fs;
use tokio::process::Command;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum BackupServiceError {
    #[error("バックアップエラー: {message}")]
    BackupError { message: String },

    #[error("リストアエラー: {message}")]
    RestoreError { message: String },

    #[error("ファイルIOエラー: {message}")]
    FileError { message: String },

    #[error("データベースエラー: {message}")]
    DatabaseError { message: String },

    #[error("設定エラー: {message}")]
    ConfigurationError { message: String },

    #[error("スケジュールエラー: {message}")]
    ScheduleError { message: String },

    #[error("圧縮エラー: {message}")]
    CompressionError { message: String },
}

#[async_trait]
pub trait BackupService: Send + Sync {
    async fn create_backup(&self, request: BackupRequest) -> Result<BackupJob, BackupServiceError>;
    async fn restore_backup(&self, request: RestoreRequest) -> Result<RestoreJob, BackupServiceError>;
    async fn get_backup_job(&self, job_id: Uuid) -> Result<Option<BackupJob>, BackupServiceError>;
    async fn list_backup_jobs(&self, limit: Option<usize>) -> Result<Vec<BackupJob>, BackupServiceError>;
    async fn delete_backup(&self, job_id: Uuid) -> Result<(), BackupServiceError>;
    async fn get_backup_statistics(&self) -> Result<BackupStatistics, BackupServiceError>;
    async fn cleanup_old_backups(&self, retention_days: i32) -> Result<i32, BackupServiceError>;
}

pub struct BackupServiceImpl {
    config: BackupConfig,
    database_url: String,
}

impl BackupServiceImpl {
    pub fn new(config: BackupConfig, database_url: String) -> Self {
        Self {
            config,
            database_url,
        }
    }

    async fn ensure_backup_directory(&self) -> Result<(), BackupServiceError> {
        if !Path::new(&self.config.backup_directory).exists() {
            fs::create_dir_all(&self.config.backup_directory)
                .await
                .map_err(|e| BackupServiceError::FileError {
                    message: format!("バックアップディレクトリ作成エラー: {e}"),
                })?;
        }
        Ok(())
    }

    fn generate_backup_filename(&self, backup_type: &BackupType, timestamp: DateTime<Utc>) -> String {
        let type_str = match backup_type {
            BackupType::Full => "full",
            BackupType::Incremental => "incremental",
            BackupType::Differential => "differential",
        };
        
        let extension = if self.config.compression_level > 0 { "gz" } else { "sql" };
        format!(
            "{}/backup_{}_{}.{}",
            self.config.backup_directory,
            type_str,
            timestamp.format("%Y%m%d_%H%M%S"),
            extension
        )
    }

    async fn execute_sqlite_backup(
        &self,
        job: &mut BackupJob,
        file_path: &str,
    ) -> Result<(), BackupServiceError> {
        info!("SQLiteバックアップを開始: {}", file_path);

        // SQLiteのバックアップコマンドを実行
        let output = if job.compression_enabled {
            // 圧縮バックアップ
            Command::new("sqlite3")
                .arg(&self.database_url)
                .arg(".backup")
                .arg("-")
                .output()
                .await
                .map_err(|e| BackupServiceError::BackupError {
                    message: format!("SQLiteバックアップコマンドエラー: {e}"),
                })?
        } else {
            // 非圧縮バックアップ
            Command::new("sqlite3")
                .arg(&self.database_url)
                .arg(".backup")
                .arg(file_path)
                .output()
                .await
                .map_err(|e| BackupServiceError::BackupError {
                    message: format!("SQLiteバックアップコマンドエラー: {e}"),
                })?
        };

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(BackupServiceError::BackupError {
                message: format!("SQLiteバックアップ失敗: {error_msg}"),
            });
        }

        // 圧縮が有効な場合、gzipで圧縮
        if job.compression_enabled {
            self.compress_backup_file(&output.stdout, file_path).await?;
        }

        // ファイルサイズを取得
        let metadata = fs::metadata(file_path).await.map_err(|e| {
            BackupServiceError::FileError {
                message: format!("バックアップファイルメタデータ取得エラー: {e}"),
            }
        })?;

        job.mark_completed(file_path.to_string(), metadata.len() as i64);
        info!("バックアップ完了: {} ({}バイト)", file_path, metadata.len());

        Ok(())
    }

    async fn compress_backup_file(
        &self,
        data: &[u8],
        output_path: &str,
    ) -> Result<(), BackupServiceError> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let file = std::fs::File::create(output_path).map_err(|e| {
            BackupServiceError::CompressionError {
                message: format!("圧縮ファイル作成エラー: {e}"),
            }
        })?;

        let mut encoder = GzEncoder::new(file, Compression::new(self.config.compression_level as u32));
        encoder.write_all(data).map_err(|e| {
            BackupServiceError::CompressionError {
                message: format!("データ圧縮エラー: {e}"),
            }
        })?;

        encoder.finish().map_err(|e| {
            BackupServiceError::CompressionError {
                message: format!("圧縮完了エラー: {e}"),
            }
        })?;

        Ok(())
    }

    async fn execute_restore(
        &self,
        job: &mut RestoreJob,
        backup_file_path: &str,
    ) -> Result<(), BackupServiceError> {
        info!("リストア開始: {}", backup_file_path);

        // バックアップファイルの存在確認
        if !Path::new(backup_file_path).exists() {
            return Err(BackupServiceError::RestoreError {
                message: format!("バックアップファイルが見つかりません: {backup_file_path}"),
            });
        }

        // SQLiteリストアコマンドを実行
        let output = if backup_file_path.ends_with(".gz") {
            // 圧縮ファイルからのリストア
            self.restore_from_compressed_backup(backup_file_path, &job.target_database)
                .await?
        } else {
            // 非圧縮ファイルからのリストア
            Command::new("sqlite3")
                .arg(&job.target_database)
                .arg(".restore")
                .arg(backup_file_path)
                .output()
                .await
                .map_err(|e| BackupServiceError::RestoreError {
                    message: format!("SQLiteリストアコマンドエラー: {e}"),
                })?
        };

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(BackupServiceError::RestoreError {
                message: format!("リストア失敗: {error_msg}"),
            });
        }

        job.mark_completed();
        info!("リストア完了: {}", backup_file_path);

        Ok(())
    }

    async fn restore_from_compressed_backup(
        &self,
        compressed_file_path: &str,
        target_db: &str,
    ) -> Result<std::process::Output, BackupServiceError> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        // 圧縮ファイルを読み込んで展開
        let file = std::fs::File::open(compressed_file_path).map_err(|e| {
            BackupServiceError::RestoreError {
                message: format!("圧縮バックアップファイル読み込みエラー: {e}"),
            }
        })?;

        let mut decoder = GzDecoder::new(file);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data).map_err(|e| {
            BackupServiceError::RestoreError {
                message: format!("バックアップファイル展開エラー: {e}"),
            }
        })?;

        // 一時ファイルに書き出し
        let temp_file_path = format!("{}/temp_restore_{}.sql", 
                                   self.config.backup_directory, 
                                   Uuid::new_v4());
        fs::write(&temp_file_path, decompressed_data).await.map_err(|e| {
            BackupServiceError::RestoreError {
                message: format!("一時ファイル作成エラー: {e}"),
            }
        })?;

        // SQLiteでリストア実行
        let output = Command::new("sqlite3")
            .arg(target_db)
            .arg(".restore")
            .arg(&temp_file_path)
            .output()
            .await
            .map_err(|e| BackupServiceError::RestoreError {
                message: format!("SQLiteリストアコマンドエラー: {e}"),
            })?;

        // 一時ファイルを削除
        let _ = fs::remove_file(&temp_file_path).await;

        Ok(output)
    }

    async fn validate_backup_request(&self, request: &BackupRequest) -> Result<(), BackupServiceError> {
        // バックアップディレクトリの存在確認
        self.ensure_backup_directory().await?;

        // ディスク容量チェック（簡易実装）
        let backup_dir = Path::new(&self.config.backup_directory);
        if !backup_dir.exists() {
            return Err(BackupServiceError::ConfigurationError {
                message: "バックアップディレクトリが存在しません".to_string(),
            });
        }

        // データベースファイルの存在確認
        if !Path::new(&self.database_url).exists() {
            return Err(BackupServiceError::DatabaseError {
                message: "データベースファイルが見つかりません".to_string(),
            });
        }

        info!("バックアップリクエストの検証完了: {:?}", request.backup_type);
        Ok(())
    }
}

#[async_trait]
impl BackupService for BackupServiceImpl {
    async fn create_backup(&self, request: BackupRequest) -> Result<BackupJob, BackupServiceError> {
        // リクエストの検証
        self.validate_backup_request(&request).await?;

        // バックアップジョブを作成
        let target_tables = request.target_tables.unwrap_or_else(|| {
            vec!["documents".to_string(), "document_types".to_string(), "users".to_string()]
        });

        let mut job = BackupJob::new(request.backup_type.clone(), target_tables);
        job.compression_enabled = request.compression_enabled.unwrap_or(true);

        // バックアップ実行
        job.mark_started();
        let timestamp = Utc::now();
        let file_path = self.generate_backup_filename(&request.backup_type, timestamp);

        match self.execute_sqlite_backup(&mut job, &file_path).await {
            Ok(()) => {
                info!("バックアップ作成成功: {}", job.id);
                Ok(job)
            }
            Err(e) => {
                job.mark_failed(e.to_string());
                error!("バックアップ作成失敗: {}", e);
                Err(e)
            }
        }
    }

    async fn restore_backup(&self, request: RestoreRequest) -> Result<RestoreJob, BackupServiceError> {
        // バックアップジョブの存在確認（実際の実装では、DBから取得）
        // 現在は簡略化のため、リクエストから直接ファイルパスを推定

        let mut restore_job = RestoreJob::new(request.backup_job_id, request.restore_type);
        restore_job.target_database = request.target_database.unwrap_or("main.db".to_string());

        restore_job.mark_started();

        // バックアップファイルパスを推定（実際の実装では、DBから取得）
        let backup_file_path = format!("{}/backup_*.sql", self.config.backup_directory);

        match self.execute_restore(&mut restore_job, &backup_file_path).await {
            Ok(()) => {
                info!("リストア成功: {}", restore_job.id);
                Ok(restore_job)
            }
            Err(e) => {
                restore_job.mark_failed(e.to_string());
                error!("リストア失敗: {}", e);
                Err(e)
            }
        }
    }

    async fn get_backup_job(&self, _job_id: Uuid) -> Result<Option<BackupJob>, BackupServiceError> {
        // 実際の実装では、データベースから取得
        // 現在はサンプルデータを返す
        warn!("get_backup_job: サンプル実装");
        Ok(None)
    }

    async fn list_backup_jobs(&self, _limit: Option<usize>) -> Result<Vec<BackupJob>, BackupServiceError> {
        // 実際の実装では、データベースから取得
        // 現在は空のリストを返す
        warn!("list_backup_jobs: サンプル実装");
        Ok(vec![])
    }

    async fn delete_backup(&self, _job_id: Uuid) -> Result<(), BackupServiceError> {
        // 実際の実装では、ファイルとDBレコードを削除
        warn!("delete_backup: サンプル実装");
        Ok(())
    }

    async fn get_backup_statistics(&self) -> Result<BackupStatistics, BackupServiceError> {
        // 実際の実装では、実際の統計を計算
        // 現在はサンプルデータを返す
        Ok(BackupStatistics {
            total_backups: 0,
            successful_backups: 0,
            failed_backups: 0,
            total_size_bytes: 0,
            last_successful_backup: None,
            next_scheduled_backup: None,
            retention_compliance: 100.0,
        })
    }

    async fn cleanup_old_backups(&self, retention_days: i32) -> Result<i32, BackupServiceError> {
        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days as i64);
        let backup_dir = Path::new(&self.config.backup_directory);

        if !backup_dir.exists() {
            return Ok(0);
        }

        let mut deleted_count = 0;
        let mut entries = fs::read_dir(backup_dir).await.map_err(|e| {
            BackupServiceError::FileError {
                message: format!("バックアップディレクトリ読み込みエラー: {e}"),
            }
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            BackupServiceError::FileError {
                message: format!("ディレクトリエントリ読み込みエラー: {e}"),
            }
        })? {
            let path = entry.path();
            if let Ok(metadata) = fs::metadata(&path).await {
                if let Ok(created) = metadata.created() {
                    let created_datetime: DateTime<Utc> = created.into();
                    if created_datetime < cutoff_date {
                        if let Err(e) = fs::remove_file(&path).await {
                            warn!("古いバックアップファイル削除失敗: {}: {}", path.display(), e);
                        } else {
                            deleted_count += 1;
                            info!("古いバックアップファイル削除: {}", path.display());
                        }
                    }
                }
            }
        }

        info!("古いバックアップファイル{}件を削除", deleted_count);
        Ok(deleted_count)
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            backup_directory: "./backups".to_string(),
            max_concurrent_jobs: 2,
            default_retention_days: 30,
            compression_level: 6,
            notification_email: None,
            auto_cleanup_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_backup_service_creation() {
        let config = BackupConfig::default();
        let service = BackupServiceImpl::new(config, "test.db".to_string());
        
        assert_eq!(service.database_url, "test.db");
    }

    #[test]
    fn test_backup_filename_generation() {
        let config = BackupConfig::default();
        let service = BackupServiceImpl::new(config, "test.db".to_string());
        
        let timestamp = DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        
        let filename = service.generate_backup_filename(&BackupType::Full, timestamp);
        assert!(filename.contains("backup_full_20240101_120000"));
    }

    #[tokio::test]
    async fn test_backup_request_validation() {
        let config = BackupConfig::default();
        let service = BackupServiceImpl::new(config, "non_existent.db".to_string());
        
        let request = BackupRequest::default();
        let result = service.validate_backup_request(&request).await;
        
        // データベースファイルが存在しないためエラーになる
        assert!(result.is_err());
    }
}