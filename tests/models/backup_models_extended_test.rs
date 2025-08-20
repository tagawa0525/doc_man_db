// Backup モデルの実装をカバーする追加テスト

use chrono::Utc;
use doc_man_db::models::backup::*;
use uuid::Uuid;

#[test]
fn test_backup_schedule_creation() {
    let schedule = BackupSchedule {
        id: Uuid::new_v4(),
        name: "Daily Backup".to_string(),
        description: Some("Daily full backup at 2 AM".to_string()),
        backup_type: BackupType::Full,
        cron_expression: "0 2 * * *".to_string(),
        retention_days: 30,
        is_active: true,
        target_tables: vec!["documents".to_string(), "users".to_string()],
        compression_enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert_eq!(schedule.name, "Daily Backup");
    assert_eq!(schedule.backup_type, BackupType::Full);
    assert_eq!(schedule.retention_days, 30);
    assert!(schedule.is_active);
    assert!(schedule.compression_enabled);
    assert_eq!(schedule.target_tables.len(), 2);
}

#[test]
fn test_backup_request_default() {
    let request = BackupRequest::default();

    assert_eq!(request.backup_type, BackupType::Full);
    assert!(request.target_tables.is_none());
    assert_eq!(request.compression_enabled, Some(true));
    assert!(request.description.is_none());
}

#[test]
fn test_backup_request_custom() {
    let request = BackupRequest {
        backup_type: BackupType::Incremental,
        target_tables: Some(vec!["documents".to_string()]),
        compression_enabled: Some(false),
        description: Some("Custom incremental backup".to_string()),
    };

    assert_eq!(request.backup_type, BackupType::Incremental);
    assert_eq!(request.target_tables, Some(vec!["documents".to_string()]));
    assert_eq!(request.compression_enabled, Some(false));
    assert_eq!(
        request.description,
        Some("Custom incremental backup".to_string())
    );
}

#[test]
fn test_restore_request_creation() {
    let backup_id = Uuid::new_v4();
    let restore_point = Utc::now();

    let request = RestoreRequest {
        backup_job_id: backup_id,
        restore_type: RestoreType::PointInTime,
        target_database: Some("test_db".to_string()),
        restore_point: Some(restore_point),
        target_tables: Some(vec!["documents".to_string()]),
    };

    assert_eq!(request.backup_job_id, backup_id);
    assert_eq!(request.restore_type, RestoreType::PointInTime);
    assert_eq!(request.target_database, Some("test_db".to_string()));
    assert_eq!(request.restore_point, Some(restore_point));
    assert_eq!(request.target_tables, Some(vec!["documents".to_string()]));
}

#[test]
fn test_backup_statistics_creation() {
    let last_backup = Utc::now();
    let next_backup = Utc::now();

    let stats = BackupStatistics {
        total_backups: 100,
        successful_backups: 95,
        failed_backups: 5,
        total_size_bytes: 1024 * 1024 * 500, // 500MB
        last_successful_backup: Some(last_backup),
        next_scheduled_backup: Some(next_backup),
        retention_compliance: 0.98,
    };

    assert_eq!(stats.total_backups, 100);
    assert_eq!(stats.successful_backups, 95);
    assert_eq!(stats.failed_backups, 5);
    assert_eq!(stats.total_size_bytes, 524_288_000);
    assert_eq!(stats.retention_compliance, 0.98);
    assert!(stats.last_successful_backup.is_some());
    assert!(stats.next_scheduled_backup.is_some());
}

#[test]
fn test_backup_config_creation() {
    let config = BackupConfig {
        backup_directory: "/var/backups/doc_man".to_string(),
        max_concurrent_jobs: 3,
        default_retention_days: 90,
        compression_level: 6,
        notification_email: Some("admin@company.com".to_string()),
        auto_cleanup_enabled: true,
    };

    assert_eq!(config.backup_directory, "/var/backups/doc_man");
    assert_eq!(config.max_concurrent_jobs, 3);
    assert_eq!(config.default_retention_days, 90);
    assert_eq!(config.compression_level, 6);
    assert_eq!(
        config.notification_email,
        Some("admin@company.com".to_string())
    );
    assert!(config.auto_cleanup_enabled);
}

#[test]
fn test_backup_job_duration_calculation() {
    let mut job = BackupJob::new(BackupType::Full, vec!["documents".to_string()]);

    // 開始前は期間なし
    assert!(job.duration_seconds().is_none());

    job.mark_started();

    // 実行中も完了時間がないため期間なし
    assert!(job.duration_seconds().is_none());

    // 少し待って完了
    std::thread::sleep(std::time::Duration::from_millis(100));
    job.mark_completed("/backup/test.sql".to_string(), 1024);

    // 期間が計算される
    let duration = job.duration_seconds();
    assert!(duration.is_some());
    assert!(duration.unwrap() >= 0);
}

#[test]
fn test_restore_job_lifecycle() {
    let backup_id = Uuid::new_v4();
    let mut restore = RestoreJob::new(backup_id, RestoreType::TableSpecific);

    // 初期状態
    assert_eq!(restore.status, BackupStatus::Pending);
    assert!(restore.started_at.is_none());

    // 開始
    restore.mark_started();
    assert_eq!(restore.status, BackupStatus::Running);
    assert!(restore.started_at.is_some());

    // 完了
    restore.mark_completed();
    assert_eq!(restore.status, BackupStatus::Completed);
    assert!(restore.completed_at.is_some());
    assert!(restore.error_message.is_none());
}

#[test]
fn test_restore_job_failure() {
    let backup_id = Uuid::new_v4();
    let mut restore = RestoreJob::new(backup_id, RestoreType::Full);

    restore.mark_started();
    restore.mark_failed("Table schema mismatch".to_string());

    assert_eq!(restore.status, BackupStatus::Failed);
    assert!(restore.completed_at.is_some());
    assert_eq!(
        restore.error_message,
        Some("Table schema mismatch".to_string())
    );
}

#[test]
fn test_restore_types_serialization() {
    let types = vec![
        RestoreType::Full,
        RestoreType::TableSpecific,
        RestoreType::PointInTime,
    ];

    for restore_type in types {
        let json = serde_json::to_string(&restore_type).unwrap();
        let deserialized: RestoreType = serde_json::from_str(&json).unwrap();
        assert_eq!(restore_type, deserialized);
    }
}

#[test]
fn test_backup_status_serialization() {
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

#[test]
fn test_backup_job_with_schedule() {
    let schedule_id = Uuid::new_v4();
    let mut job = BackupJob::new(BackupType::Differential, vec!["documents".to_string()]);
    job.schedule_id = Some(schedule_id);

    assert_eq!(job.schedule_id, Some(schedule_id));
    assert_eq!(job.backup_type, BackupType::Differential);
}

#[test]
fn test_restore_job_with_restore_point() {
    let backup_id = Uuid::new_v4();
    let restore_point = Utc::now();
    let mut restore = RestoreJob::new(backup_id, RestoreType::PointInTime);
    restore.restore_point = Some(restore_point);
    restore.target_database = "production_db".to_string();

    assert_eq!(restore.restore_point, Some(restore_point));
    assert_eq!(restore.target_database, "production_db");
    assert_eq!(restore.restore_type, RestoreType::PointInTime);
}

#[test]
fn test_backup_job_cancellation() {
    let mut job = BackupJob::new(BackupType::Full, vec!["documents".to_string()]);
    job.mark_started();

    // キャンセル操作をシミュレート
    job.status = BackupStatus::Cancelled;
    job.completed_at = Some(Utc::now());
    job.error_message = Some("Backup cancelled by user".to_string());

    assert_eq!(job.status, BackupStatus::Cancelled);
    assert!(job.completed_at.is_some());
    assert_eq!(
        job.error_message,
        Some("Backup cancelled by user".to_string())
    );
}

#[test]
fn test_backup_config_minimal() {
    let config = BackupConfig {
        backup_directory: "/tmp".to_string(),
        max_concurrent_jobs: 1,
        default_retention_days: 7,
        compression_level: 0,
        notification_email: None,
        auto_cleanup_enabled: false,
    };

    assert_eq!(config.backup_directory, "/tmp");
    assert_eq!(config.max_concurrent_jobs, 1);
    assert_eq!(config.default_retention_days, 7);
    assert_eq!(config.compression_level, 0);
    assert!(config.notification_email.is_none());
    assert!(!config.auto_cleanup_enabled);
}
