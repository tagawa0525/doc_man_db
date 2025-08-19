// Backup Handler の基本的なテスト

use doc_man_db::models::backup::*;

#[test]
fn test_backup_request_creation() {
    let request = BackupRequest {
        backup_type: BackupType::Full,
        target_tables: Some(vec!["documents".to_string(), "users".to_string()]),
        compression_enabled: Some(true),
        description: Some("テストバックアップ".to_string()),
    };

    assert_eq!(request.backup_type, BackupType::Full);
    assert_eq!(
        request.target_tables,
        Some(vec!["documents".to_string(), "users".to_string()])
    );
    assert_eq!(request.compression_enabled, Some(true));
    assert_eq!(request.description, Some("テストバックアップ".to_string()));
}

#[test]
fn test_backup_job_creation() {
    let job = BackupJob::new(BackupType::Incremental, vec!["documents".to_string()]);

    assert_eq!(job.backup_type, BackupType::Incremental);
    assert_eq!(job.status, BackupStatus::Pending);
    assert_eq!(job.target_tables, vec!["documents".to_string()]);
    assert!(job.compression_enabled);
    assert!(job.started_at.is_none());
    assert!(job.completed_at.is_none());
    assert!(job.file_path.is_none());
}

#[test]
fn test_backup_job_lifecycle() {
    let mut job = BackupJob::new(BackupType::Full, vec!["documents".to_string()]);

    // 開始マーク
    job.mark_started();
    assert_eq!(job.status, BackupStatus::Running);
    assert!(job.started_at.is_some());

    // 完了マーク
    job.mark_completed("/backup/test.sql".to_string(), 1024);
    assert_eq!(job.status, BackupStatus::Completed);
    assert!(job.completed_at.is_some());
    assert_eq!(job.file_path, Some("/backup/test.sql".to_string()));
    assert_eq!(job.file_size_bytes, Some(1024));
}

#[test]
fn test_backup_job_failure() {
    let mut job = BackupJob::new(BackupType::Incremental, vec!["documents".to_string()]);

    job.mark_started();
    job.mark_failed("Disk space insufficient".to_string());

    assert_eq!(job.status, BackupStatus::Failed);
    assert!(job.completed_at.is_some());
    assert_eq!(
        job.error_message,
        Some("Disk space insufficient".to_string())
    );
}

#[test]
fn test_backup_type_serialization() {
    use serde_json;

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

#[test]
fn test_backup_request_default() {
    let request = BackupRequest::default();

    assert_eq!(request.backup_type, BackupType::Full);
    assert_eq!(request.target_tables, None);
    assert_eq!(request.compression_enabled, Some(true));
    assert_eq!(request.description, None);
}

#[test]
fn test_backup_status_serialization() {
    use serde_json;

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
fn test_backup_duration_calculation() {
    let mut job = BackupJob::new(BackupType::Full, vec!["documents".to_string()]);

    // まだ開始していない
    assert_eq!(job.duration_seconds(), None);

    job.mark_started();

    // まだ完了していない
    assert_eq!(job.duration_seconds(), None);

    std::thread::sleep(std::time::Duration::from_millis(10));
    job.mark_completed("/test/backup.sql".to_string(), 100);

    // 実行時間が計算される
    let duration = job.duration_seconds();
    assert!(duration.is_some());
    assert!(duration.unwrap() >= 0);
}
