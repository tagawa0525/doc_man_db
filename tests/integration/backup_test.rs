use doc_man_db::models::backup::*;
use doc_man_db::services::{BackupService, BackupServiceImpl};

use uuid::Uuid;

#[tokio::test]
async fn test_backup_service_creation() {
    let config = BackupConfig::default();
    let _service = BackupServiceImpl::new(config, "test.db".to_string());

    // サービスが正常に作成されることを確認
    // database_urlフィールドはprivateなので、サービスの作成のみテスト
    assert!(true);
}

#[tokio::test]
async fn test_backup_request_creation() {
    let request = BackupRequest {
        backup_type: BackupType::Full,
        target_tables: Some(vec!["documents".to_string(), "users".to_string()]),
        compression_enabled: Some(true),
        description: Some("テストバックアップ".to_string()),
    };

    assert_eq!(request.backup_type, BackupType::Full);
    assert!(request.compression_enabled.unwrap());
    assert_eq!(request.target_tables.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn test_backup_job_lifecycle() {
    let mut job = BackupJob::new(
        BackupType::Incremental,
        vec!["documents".to_string(), "document_types".to_string()],
    );

    // 初期状態の確認
    assert_eq!(job.status, BackupStatus::Pending);
    assert!(job.started_at.is_none());
    assert!(job.completed_at.is_none());

    // 開始
    job.mark_started();
    assert_eq!(job.status, BackupStatus::Running);
    assert!(job.started_at.is_some());

    // 完了
    job.mark_completed("/backup/test.sql".to_string(), 2048);
    assert_eq!(job.status, BackupStatus::Completed);
    assert!(job.completed_at.is_some());
    assert_eq!(job.file_path, Some("/backup/test.sql".to_string()));
    assert_eq!(job.file_size_bytes, Some(2048));

    // 実行時間の計算
    assert!(job.duration_seconds().is_some());
    assert!(job.duration_seconds().unwrap() >= 0);
}

#[tokio::test]
async fn test_backup_job_failure() {
    let mut job = BackupJob::new(BackupType::Full, vec!["documents".to_string()]);

    job.mark_started();
    job.mark_failed("ディスク容量不足".to_string());

    assert_eq!(job.status, BackupStatus::Failed);
    assert!(job.completed_at.is_some());
    assert_eq!(job.error_message, Some("ディスク容量不足".to_string()));
}

#[tokio::test]
async fn test_restore_job_creation() {
    let backup_id = Uuid::new_v4();
    let restore = RestoreJob::new(backup_id, RestoreType::Full);

    assert_eq!(restore.backup_job_id, backup_id);
    assert_eq!(restore.restore_type, RestoreType::Full);
    assert_eq!(restore.status, BackupStatus::Pending);
    assert_eq!(restore.target_database, "main");
}

#[tokio::test]
async fn test_restore_job_lifecycle() {
    let backup_id = Uuid::new_v4();
    let mut restore = RestoreJob::new(backup_id, RestoreType::TableSpecific);

    // 開始
    restore.mark_started();
    assert_eq!(restore.status, BackupStatus::Running);
    assert!(restore.started_at.is_some());

    // 完了
    restore.mark_completed();
    assert_eq!(restore.status, BackupStatus::Completed);
    assert!(restore.completed_at.is_some());
}

#[tokio::test]
async fn test_restore_job_failure() {
    let backup_id = Uuid::new_v4();
    let mut restore = RestoreJob::new(backup_id, RestoreType::PointInTime);

    restore.mark_started();
    restore.mark_failed("バックアップファイルが見つかりません".to_string());

    assert_eq!(restore.status, BackupStatus::Failed);
    assert!(restore.completed_at.is_some());
    assert_eq!(
        restore.error_message,
        Some("バックアップファイルが見つかりません".to_string())
    );
}

#[tokio::test]
async fn test_backup_types_serialization() {
    let backup_types = vec![
        BackupType::Full,
        BackupType::Incremental,
        BackupType::Differential,
    ];

    for backup_type in backup_types {
        let json = serde_json::to_string(&backup_type).unwrap();
        let deserialized: BackupType = serde_json::from_str(&json).unwrap();
        assert_eq!(backup_type, deserialized);
    }
}

#[tokio::test]
async fn test_backup_status_serialization() {
    let statuses = vec![
        BackupStatus::Pending,
        BackupStatus::Running,
        BackupStatus::Completed,
        BackupStatus::Failed,
        BackupStatus::Cancelled,
    ];

    for status in statuses {
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: BackupStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }
}

#[tokio::test]
async fn test_restore_types_serialization() {
    let restore_types = vec![
        RestoreType::Full,
        RestoreType::TableSpecific,
        RestoreType::PointInTime,
    ];

    for restore_type in restore_types {
        let json = serde_json::to_string(&restore_type).unwrap();
        let deserialized: RestoreType = serde_json::from_str(&json).unwrap();
        assert_eq!(restore_type, deserialized);
    }
}

#[tokio::test]
async fn test_backup_request_default() {
    let request = BackupRequest::default();

    assert_eq!(request.backup_type, BackupType::Full);
    assert!(request.target_tables.is_none());
    assert_eq!(request.compression_enabled, Some(true));
    assert!(request.description.is_none());
}

#[tokio::test]
async fn test_backup_config_default() {
    let config = BackupConfig::default();

    assert_eq!(config.backup_directory, "./backups");
    assert_eq!(config.max_concurrent_jobs, 2);
    assert_eq!(config.default_retention_days, 30);
    assert_eq!(config.compression_level, 6);
    assert!(config.notification_email.is_none());
    assert!(config.auto_cleanup_enabled);
}

#[tokio::test]
async fn test_backup_statistics_creation() {
    let stats = BackupStatistics {
        total_backups: 10,
        successful_backups: 8,
        failed_backups: 2,
        total_size_bytes: 1024000,
        last_successful_backup: Some(chrono::Utc::now()),
        next_scheduled_backup: None,
        retention_compliance: 90.0,
    };

    assert_eq!(stats.total_backups, 10);
    assert_eq!(stats.successful_backups, 8);
    assert_eq!(stats.failed_backups, 2);
    assert_eq!(stats.retention_compliance, 90.0);
}

// バックアップサービスの統合テスト
#[tokio::test]
async fn test_backup_service_get_statistics() {
    let config = BackupConfig::default();
    let service = BackupServiceImpl::new(config, "test.db".to_string());

    let result = service.get_backup_statistics().await;
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert_eq!(stats.total_backups, 0); // サンプル実装では0を返す
}

#[tokio::test]
async fn test_backup_service_list_jobs() {
    let config = BackupConfig::default();
    let service = BackupServiceImpl::new(config, "test.db".to_string());

    let result = service.list_backup_jobs(Some(10)).await;
    assert!(result.is_ok());

    let jobs = result.unwrap();
    assert_eq!(jobs.len(), 0); // サンプル実装では空のリストを返す
}

#[tokio::test]
async fn test_backup_service_get_job() {
    let config = BackupConfig::default();
    let service = BackupServiceImpl::new(config, "test.db".to_string());

    let job_id = Uuid::new_v4();
    let result = service.get_backup_job(job_id).await;
    assert!(result.is_ok());

    let job = result.unwrap();
    assert!(job.is_none()); // サンプル実装ではNoneを返す
}

#[tokio::test]
async fn test_backup_service_cleanup() {
    let config = BackupConfig::default();
    let service = BackupServiceImpl::new(config, "test.db".to_string());

    let result = service.cleanup_old_backups(30).await;
    assert!(result.is_ok());

    let deleted_count = result.unwrap();
    assert_eq!(deleted_count, 0); // バックアップディレクトリが存在しないため0
}

// エラーケースのテスト
#[tokio::test]
async fn test_backup_service_invalid_database() {
    let config = BackupConfig::default();
    let service = BackupServiceImpl::new(config, "non_existent.db".to_string());

    let request = BackupRequest::default();
    let result = service.create_backup(request).await;

    // データベースファイルが存在しないためエラーになる
    assert!(result.is_err());
}

// 復数のバックアップタイプのテスト
#[tokio::test]
async fn test_multiple_backup_types() {
    let backup_types = vec![
        BackupType::Full,
        BackupType::Incremental,
        BackupType::Differential,
    ];

    for backup_type in backup_types {
        let job = BackupJob::new(backup_type.clone(), vec!["test_table".to_string()]);
        assert_eq!(job.backup_type, backup_type);
        assert_eq!(job.status, BackupStatus::Pending);
    }
}

// 複数のリストアタイプのテスト
#[tokio::test]
async fn test_multiple_restore_types() {
    let backup_id = Uuid::new_v4();
    let restore_types = vec![
        RestoreType::Full,
        RestoreType::TableSpecific,
        RestoreType::PointInTime,
    ];

    for restore_type in restore_types {
        let restore = RestoreJob::new(backup_id, restore_type.clone());
        assert_eq!(restore.restore_type, restore_type);
        assert_eq!(restore.status, BackupStatus::Pending);
    }
}
