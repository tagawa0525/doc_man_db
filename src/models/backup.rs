use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// バックアップタイプ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BackupType {
    Full,        // フルバックアップ
    Incremental, // 増分バックアップ
    Differential, // 差分バックアップ
}

/// バックアップステータス
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BackupStatus {
    Pending,    // 待機中
    Running,    // 実行中
    Completed,  // 完了
    Failed,     // 失敗
    Cancelled,  // キャンセル
}

/// バックアップスケジュール設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub backup_type: BackupType,
    pub cron_expression: String,  // "0 2 * * *" = 毎日2時
    pub retention_days: i32,
    pub is_active: bool,
    pub target_tables: Vec<String>,
    pub compression_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// バックアップジョブ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJob {
    pub id: Uuid,
    pub schedule_id: Option<Uuid>,
    pub backup_type: BackupType,
    pub status: BackupStatus,
    pub file_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub target_tables: Vec<String>,
    pub compression_enabled: bool,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// リストア操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreJob {
    pub id: Uuid,
    pub backup_job_id: Uuid,
    pub restore_type: RestoreType,
    pub status: BackupStatus,
    pub target_database: String,
    pub restore_point: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// リストアタイプ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RestoreType {
    Full,           // フルリストア
    TableSpecific,  // 特定テーブルのみ
    PointInTime,    // 特定時点へのリストア
}

/// バックアップリクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRequest {
    pub backup_type: BackupType,
    pub target_tables: Option<Vec<String>>,
    pub compression_enabled: Option<bool>,
    pub description: Option<String>,
}

/// リストアリクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRequest {
    pub backup_job_id: Uuid,
    pub restore_type: RestoreType,
    pub target_database: Option<String>,
    pub restore_point: Option<DateTime<Utc>>,
    pub target_tables: Option<Vec<String>>,
}

/// バックアップ統計
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupStatistics {
    pub total_backups: i32,
    pub successful_backups: i32,
    pub failed_backups: i32,
    pub total_size_bytes: i64,
    pub last_successful_backup: Option<DateTime<Utc>>,
    pub next_scheduled_backup: Option<DateTime<Utc>>,
    pub retention_compliance: f64, // 保持ポリシーへの準拠率
}

/// バックアップ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub backup_directory: String,
    pub max_concurrent_jobs: i32,
    pub default_retention_days: i32,
    pub compression_level: i32,
    pub notification_email: Option<String>,
    pub auto_cleanup_enabled: bool,
}

impl Default for BackupRequest {
    fn default() -> Self {
        Self {
            backup_type: BackupType::Full,
            target_tables: None,
            compression_enabled: Some(true),
            description: None,
        }
    }
}

impl BackupJob {
    pub fn new(backup_type: BackupType, target_tables: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            schedule_id: None,
            backup_type,
            status: BackupStatus::Pending,
            file_path: None,
            file_size_bytes: None,
            target_tables,
            compression_enabled: true,
            started_at: None,
            completed_at: None,
            error_message: None,
            created_at: Utc::now(),
        }
    }

    pub fn mark_started(&mut self) {
        self.status = BackupStatus::Running;
        self.started_at = Some(Utc::now());
    }

    pub fn mark_completed(&mut self, file_path: String, file_size: i64) {
        self.status = BackupStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.file_path = Some(file_path);
        self.file_size_bytes = Some(file_size);
    }

    pub fn mark_failed(&mut self, error: String) {
        self.status = BackupStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error_message = Some(error);
    }

    pub fn duration_seconds(&self) -> Option<i64> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some((end - start).num_seconds()),
            _ => None,
        }
    }
}

impl RestoreJob {
    pub fn new(backup_job_id: Uuid, restore_type: RestoreType) -> Self {
        Self {
            id: Uuid::new_v4(),
            backup_job_id,
            restore_type,
            status: BackupStatus::Pending,
            target_database: "main".to_string(),
            restore_point: None,
            started_at: None,
            completed_at: None,
            error_message: None,
            created_at: Utc::now(),
        }
    }

    pub fn mark_started(&mut self) {
        self.status = BackupStatus::Running;
        self.started_at = Some(Utc::now());
    }

    pub fn mark_completed(&mut self) {
        self.status = BackupStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn mark_failed(&mut self, error: String) {
        self.status = BackupStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error_message = Some(error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_job_creation() {
        let job = BackupJob::new(BackupType::Full, vec!["documents".to_string()]);
        
        assert_eq!(job.backup_type, BackupType::Full);
        assert_eq!(job.status, BackupStatus::Pending);
        assert_eq!(job.target_tables.len(), 1);
        assert!(job.compression_enabled);
        assert!(job.started_at.is_none());
    }

    #[test]
    fn test_backup_job_lifecycle() {
        let mut job = BackupJob::new(BackupType::Incremental, vec!["documents".to_string()]);
        
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
        let mut job = BackupJob::new(BackupType::Full, vec!["documents".to_string()]);
        
        job.mark_started();
        job.mark_failed("Disk full".to_string());
        
        assert_eq!(job.status, BackupStatus::Failed);
        assert!(job.completed_at.is_some());
        assert_eq!(job.error_message, Some("Disk full".to_string()));
    }

    #[test]
    fn test_restore_job_creation() {
        let backup_id = Uuid::new_v4();
        let restore = RestoreJob::new(backup_id, RestoreType::Full);
        
        assert_eq!(restore.backup_job_id, backup_id);
        assert_eq!(restore.restore_type, RestoreType::Full);
        assert_eq!(restore.status, BackupStatus::Pending);
        assert_eq!(restore.target_database, "main");
    }

    #[test]
    fn test_backup_types_serialization() {
        let types = vec![
            BackupType::Full,
            BackupType::Incremental,
            BackupType::Differential,
        ];

        for backup_type in types {
            let json = serde_json::to_string(&backup_type).unwrap();
            let deserialized: BackupType = serde_json::from_str(&json).unwrap();
            assert_eq!(backup_type, deserialized);
        }
    }
}