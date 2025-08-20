use async_trait::async_trait;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::Utc;
use doc_man_db::handlers::backup::{BackupHandlers, BackupQueryParams, CleanupRequest};
use doc_man_db::models::backup::{
    BackupJob, BackupRequest, BackupStatistics, BackupStatus, BackupType, RestoreJob,
    RestoreRequest, RestoreType,
};
use doc_man_db::services::{BackupService, BackupServiceError};
use std::sync::Arc;
use tokio;
use uuid::Uuid;

// モックバックアップサービス
#[derive(Debug)]
struct MockBackupService {
    should_fail: bool,
}

impl MockBackupService {
    fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

#[async_trait]
impl BackupService for MockBackupService {
    async fn create_backup(
        &self,
        _request: BackupRequest,
    ) -> Result<BackupJob, BackupServiceError> {
        if self.should_fail {
            return Err(BackupServiceError::BackupError {
                message: "バックアップ作成に失敗しました".to_string(),
            });
        }

        Ok(BackupJob {
            id: Uuid::new_v4(),
            schedule_id: None,
            backup_type: BackupType::Full,
            status: BackupStatus::Running,
            file_path: Some("./backups/test.sql".to_string()),
            file_size_bytes: None,
            target_tables: vec!["documents".to_string()],
            compression_enabled: true,
            started_at: Some(Utc::now()),
            completed_at: None,
            error_message: None,
            created_at: Utc::now(),
        })
    }

    async fn restore_backup(
        &self,
        _request: RestoreRequest,
    ) -> Result<RestoreJob, BackupServiceError> {
        if self.should_fail {
            return Err(BackupServiceError::RestoreError {
                message: "リストア作成に失敗しました".to_string(),
            });
        }

        Ok(RestoreJob {
            id: Uuid::new_v4(),
            backup_job_id: Uuid::new_v4(),
            restore_type: RestoreType::Full,
            status: BackupStatus::Running,
            target_database: "test_db".to_string(),
            restore_point: None,
            started_at: Some(Utc::now()),
            completed_at: None,
            error_message: None,
            created_at: Utc::now(),
        })
    }

    async fn get_backup_job(&self, job_id: Uuid) -> Result<Option<BackupJob>, BackupServiceError> {
        if self.should_fail {
            return Err(BackupServiceError::DatabaseError {
                message: "ジョブ取得に失敗しました".to_string(),
            });
        }

        if job_id == Uuid::nil() {
            return Ok(None);
        }

        Ok(Some(BackupJob {
            id: job_id,
            schedule_id: None,
            backup_type: BackupType::Full,
            status: BackupStatus::Completed,
            file_path: Some("./backups/test.sql".to_string()),
            file_size_bytes: Some(1024),
            target_tables: vec!["documents".to_string()],
            compression_enabled: true,
            started_at: Some(Utc::now()),
            completed_at: Some(Utc::now()),
            error_message: None,
            created_at: Utc::now(),
        }))
    }

    async fn list_backup_jobs(
        &self,
        _limit: Option<usize>,
    ) -> Result<Vec<BackupJob>, BackupServiceError> {
        if self.should_fail {
            return Err(BackupServiceError::DatabaseError {
                message: "ジョブ一覧取得に失敗しました".to_string(),
            });
        }

        Ok(vec![
            BackupJob {
                id: Uuid::new_v4(),
                schedule_id: None,
                backup_type: BackupType::Full,
                status: BackupStatus::Completed,
                file_path: Some("./backups/test1.sql".to_string()),
                file_size_bytes: Some(1024),
                target_tables: vec!["documents".to_string()],
                compression_enabled: true,
                started_at: Some(Utc::now()),
                completed_at: Some(Utc::now()),
                error_message: None,
                created_at: Utc::now(),
            },
            BackupJob {
                id: Uuid::new_v4(),
                schedule_id: None,
                backup_type: BackupType::Incremental,
                status: BackupStatus::Running,
                file_path: None,
                file_size_bytes: None,
                target_tables: vec!["documents".to_string()],
                compression_enabled: true,
                started_at: Some(Utc::now()),
                completed_at: None,
                error_message: None,
                created_at: Utc::now(),
            },
        ])
    }

    async fn delete_backup(&self, _job_id: Uuid) -> Result<(), BackupServiceError> {
        if self.should_fail {
            return Err(BackupServiceError::FileError {
                message: "バックアップ削除に失敗しました".to_string(),
            });
        }
        Ok(())
    }

    async fn get_backup_statistics(&self) -> Result<BackupStatistics, BackupServiceError> {
        if self.should_fail {
            return Err(BackupServiceError::DatabaseError {
                message: "統計取得に失敗しました".to_string(),
            });
        }

        Ok(BackupStatistics {
            total_backups: 5,
            successful_backups: 4,
            failed_backups: 1,
            total_size_bytes: 5120,
            last_successful_backup: Some(Utc::now()),
            next_scheduled_backup: Some(Utc::now()),
            retention_compliance: 0.8,
        })
    }

    async fn cleanup_old_backups(&self, _retention_days: i32) -> Result<i32, BackupServiceError> {
        if self.should_fail {
            return Err(BackupServiceError::FileError {
                message: "クリーンアップに失敗しました".to_string(),
            });
        }
        Ok(3)
    }
}

#[tokio::test]
async fn test_create_backup_success() {
    let backup_service = Arc::new(MockBackupService::new(false));
    let handlers = Arc::new(BackupHandlers::new(backup_service));

    let request = BackupRequest {
        backup_type: BackupType::Full,
        target_tables: Some(vec!["documents".to_string()]),
        compression_enabled: Some(true),
        description: Some("Test backup".to_string()),
    };

    let result = BackupHandlers::create_backup(State(handlers), Json(request)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0.status, "started");
}

#[tokio::test]
async fn test_create_backup_failure() {
    let backup_service = Arc::new(MockBackupService::new(true));
    let handlers = Arc::new(BackupHandlers::new(backup_service));

    let request = BackupRequest::default();
    let result = BackupHandlers::create_backup(State(handlers), Json(request)).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_backup_job_success() {
    let backup_service = Arc::new(MockBackupService::new(false));
    let handlers = Arc::new(BackupHandlers::new(backup_service));
    let job_id = Uuid::new_v4();

    let result = BackupHandlers::get_backup_job(State(handlers), Path(job_id)).await;

    assert!(result.is_ok());
    let job = result.unwrap();
    assert_eq!(job.0.id, job_id);
}

#[tokio::test]
async fn test_get_backup_job_not_found() {
    let backup_service = Arc::new(MockBackupService::new(false));
    let handlers = Arc::new(BackupHandlers::new(backup_service));

    let result = BackupHandlers::get_backup_job(State(handlers), Path(Uuid::nil())).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_backup_jobs_success() {
    let backup_service = Arc::new(MockBackupService::new(false));
    let handlers = Arc::new(BackupHandlers::new(backup_service));

    let params = BackupQueryParams {
        limit: Some(10),
        offset: Some(0),
        status: None,
    };

    let result = BackupHandlers::list_backup_jobs(State(handlers), Query(params)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0.jobs.len(), 2);
}

#[tokio::test]
async fn test_delete_backup_success() {
    let backup_service = Arc::new(MockBackupService::new(false));
    let handlers = Arc::new(BackupHandlers::new(backup_service));
    let job_id = Uuid::new_v4();

    let result = BackupHandlers::delete_backup(State(handlers), Path(job_id)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0["message"], "バックアップを削除しました");
}

#[tokio::test]
async fn test_get_backup_statistics_success() {
    let backup_service = Arc::new(MockBackupService::new(false));
    let handlers = Arc::new(BackupHandlers::new(backup_service));

    let result = BackupHandlers::get_backup_statistics(State(handlers)).await;

    assert!(result.is_ok());
    let stats = result.unwrap();
    assert_eq!(stats.0.total_backups, 5);
}

#[tokio::test]
async fn test_cleanup_old_backups_success() {
    let backup_service = Arc::new(MockBackupService::new(false));
    let handlers = Arc::new(BackupHandlers::new(backup_service));

    let request = CleanupRequest {
        retention_days: Some(30),
        dry_run: Some(false),
    };

    let result = BackupHandlers::cleanup_old_backups(State(handlers), Json(request)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0.deleted_count, 3);
}

#[tokio::test]
async fn test_health_check_healthy() {
    let backup_service = Arc::new(MockBackupService::new(false));
    let handlers = Arc::new(BackupHandlers::new(backup_service));

    let result = BackupHandlers::health_check(State(handlers)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.0["status"], "healthy");
}
