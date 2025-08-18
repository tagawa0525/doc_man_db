use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 移行タイプ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MigrationType {
    DatabaseMigration,  // データベース移行
    DataMigration,      // データ移行
    SchemaMigration,    // スキーマ移行
    FullMigration,      // 完全移行
}

/// 移行ステータス
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MigrationStatus {
    Pending,      // 待機中
    Planning,     // 計画中
    Validating,   // 検証中
    Running,      // 実行中
    Completed,    // 完了
    Failed,       // 失敗
    RollingBack,  // ロールバック中
    RolledBack,   // ロールバック完了
}

/// 移行環境
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MigrationEnvironment {
    Development,
    Staging,
    Production,
}

/// 移行計画
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPlan {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub migration_type: MigrationType,
    pub source_environment: MigrationEnvironment,
    pub target_environment: MigrationEnvironment,
    pub source_database_url: String,
    pub target_database_url: String,
    pub estimated_duration_minutes: Option<i32>,
    pub data_size_mb: Option<i64>,
    pub dependencies: Vec<Uuid>,
    pub rollback_plan: Option<String>,
    pub created_by: String,
    pub approved_by: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 移行実行ジョブ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationJob {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub status: MigrationStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub progress_percentage: f64,
    pub current_step: Option<String>,
    pub total_steps: i32,
    pub completed_steps: i32,
    pub data_processed_mb: Option<i64>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub logs: Vec<MigrationLog>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 移行ログ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationLog {
    pub id: Uuid,
    pub job_id: Uuid,
    pub level: LogLevel,
    pub message: String,
    pub step_name: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub data: Option<serde_json::Value>,
}

/// ログレベル
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// 移行検証結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationValidation {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub validation_type: ValidationType,
    pub status: MigrationValidationStatus,
    pub issues: Vec<ValidationIssue>,
    pub recommendations: Vec<String>,
    pub estimated_duration_minutes: Option<i32>,
    pub risk_level: RiskLevel,
    pub validated_at: DateTime<Utc>,
    pub validated_by: String,
}

/// 検証タイプ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationType {
    PreMigration,    // 移行前検証
    PostMigration,   // 移行後検証
    DataIntegrity,   // データ整合性
    Performance,     // パフォーマンス
    Security,        // セキュリティ
}

/// 検証ステータス
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MigrationValidationStatus {
    Passed,      // 合格
    Failed,      // 失敗
    Warning,     // 警告
    InProgress,  // 実行中
}

/// 検証問題
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub id: Uuid,
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub title: String,
    pub description: String,
    pub affected_tables: Vec<String>,
    pub affected_records: Option<i64>,
    pub recommendation: Option<String>,
    pub auto_fixable: bool,
}

/// 問題の重要度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// 問題カテゴリ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssueCategory {
    DataInconsistency,  // データ不整合
    SchemaChange,       // スキーマ変更
    PerformanceImpact,  // パフォーマンス影響
    SecurityRisk,       // セキュリティリスク
    DependencyIssue,    // 依存関係問題
}

/// リスクレベル
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// 移行リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRequest {
    pub plan_id: Uuid,
    pub dry_run: Option<bool>,
    pub force_execution: Option<bool>,
    pub notification_settings: Option<NotificationSettings>,
}

/// 移行停止リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStopRequest {
    pub job_id: Uuid,
    pub reason: String,
    pub force_stop: Option<bool>,
}

/// ロールバックリクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackRequest {
    pub job_id: Uuid,
    pub target_point: Option<DateTime<Utc>>,
    pub reason: String,
}

/// 通知設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email_notifications: bool,
    pub slack_notifications: bool,
    pub sms_notifications: bool,
    pub notification_recipients: Vec<String>,
}

/// 移行統計
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatistics {
    pub total_migrations: i32,
    pub successful_migrations: i32,
    pub failed_migrations: i32,
    pub average_duration_minutes: f64,
    pub total_data_migrated_gb: f64,
    pub success_rate: f64,
    pub last_migration: Option<DateTime<Utc>>,
    pub upcoming_migrations: i32,
}

impl MigrationPlan {
    pub fn new(
        name: String,
        migration_type: MigrationType,
        source_env: MigrationEnvironment,
        target_env: MigrationEnvironment,
        source_db_url: String,
        target_db_url: String,
        created_by: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            migration_type,
            source_environment: source_env,
            target_environment: target_env,
            source_database_url: source_db_url,
            target_database_url: target_db_url,
            estimated_duration_minutes: None,
            data_size_mb: None,
            dependencies: Vec::new(),
            rollback_plan: None,
            created_by,
            approved_by: None,
            scheduled_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn approve(&mut self, approver: String) {
        self.approved_by = Some(approver);
        self.updated_at = Utc::now();
    }

    pub fn schedule(&mut self, scheduled_time: DateTime<Utc>) {
        self.scheduled_at = Some(scheduled_time);
        self.updated_at = Utc::now();
    }

    pub fn is_approved(&self) -> bool {
        self.approved_by.is_some()
    }
}

impl MigrationJob {
    pub fn new(plan_id: Uuid, total_steps: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            plan_id,
            status: MigrationStatus::Pending,
            started_at: None,
            completed_at: None,
            error_message: None,
            progress_percentage: 0.0,
            current_step: None,
            total_steps,
            completed_steps: 0,
            data_processed_mb: None,
            estimated_completion: None,
            logs: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn start(&mut self) {
        self.status = MigrationStatus::Running;
        self.started_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn complete(&mut self) {
        self.status = MigrationStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.progress_percentage = 100.0;
        self.completed_steps = self.total_steps;
        self.updated_at = Utc::now();
    }

    pub fn fail(&mut self, error: String) {
        self.status = MigrationStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error_message = Some(error);
        self.updated_at = Utc::now();
    }

    pub fn update_progress(&mut self, step: String, completed_steps: i32, data_processed: Option<i64>) {
        self.current_step = Some(step);
        self.completed_steps = completed_steps;
        self.progress_percentage = (completed_steps as f64 / self.total_steps as f64) * 100.0;
        if let Some(data) = data_processed {
            self.data_processed_mb = Some(data);
        }
        self.updated_at = Utc::now();
    }

    pub fn add_log(&mut self, level: LogLevel, message: String, step_name: Option<String>, data: Option<serde_json::Value>) {
        let log = MigrationLog {
            id: Uuid::new_v4(),
            job_id: self.id,
            level,
            message,
            step_name,
            timestamp: Utc::now(),
            data,
        };
        self.logs.push(log);
    }

    pub fn duration_minutes(&self) -> Option<i64> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some((end - start).num_minutes()),
            _ => None,
        }
    }
}

impl MigrationValidation {
    pub fn new(
        plan_id: Uuid,
        validation_type: ValidationType,
        validated_by: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            plan_id,
            validation_type,
            status: MigrationValidationStatus::InProgress,
            issues: Vec::new(),
            recommendations: Vec::new(),
            estimated_duration_minutes: None,
            risk_level: RiskLevel::Medium,
            validated_at: Utc::now(),
            validated_by,
        }
    }

    pub fn add_issue(&mut self, issue: ValidationIssue) {
        // リスクレベルを最高の問題に合わせて更新
        match issue.severity {
            IssueSeverity::Critical => self.risk_level = RiskLevel::Critical,
            IssueSeverity::High if self.risk_level != RiskLevel::Critical => {
                self.risk_level = RiskLevel::High
            }
            IssueSeverity::Medium if matches!(self.risk_level, RiskLevel::Low) => {
                self.risk_level = RiskLevel::Medium
            }
            _ => {}
        }
        self.issues.push(issue);
    }

    pub fn complete(&mut self) {
        self.status = if self.has_critical_issues() {
            MigrationValidationStatus::Failed
        } else if self.has_issues() {
            MigrationValidationStatus::Warning
        } else {
            MigrationValidationStatus::Passed
        };
    }

    pub fn has_critical_issues(&self) -> bool {
        self.issues.iter().any(|i| i.severity == IssueSeverity::Critical)
    }

    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_plan_creation() {
        let plan = MigrationPlan::new(
            "Test Migration".to_string(),
            MigrationType::DataMigration,
            MigrationEnvironment::Development,
            MigrationEnvironment::Staging,
            "sqlite://dev.db".to_string(),
            "sqlite://staging.db".to_string(),
            "admin".to_string(),
        );

        assert_eq!(plan.name, "Test Migration");
        assert_eq!(plan.migration_type, MigrationType::DataMigration);
        assert!(!plan.is_approved());
    }

    #[test]
    fn test_migration_job_lifecycle() {
        let mut job = MigrationJob::new(Uuid::new_v4(), 5);

        assert_eq!(job.status, MigrationStatus::Pending);
        assert_eq!(job.progress_percentage, 0.0);

        job.start();
        assert_eq!(job.status, MigrationStatus::Running);
        assert!(job.started_at.is_some());

        job.update_progress("Step 1".to_string(), 1, Some(100));
        assert_eq!(job.progress_percentage, 20.0);
        assert_eq!(job.completed_steps, 1);

        job.complete();
        assert_eq!(job.status, MigrationStatus::Completed);
        assert_eq!(job.progress_percentage, 100.0);
    }

    #[test]
    fn test_migration_validation() {
        let mut validation = MigrationValidation::new(
            Uuid::new_v4(),
            ValidationType::PreMigration,
            "validator".to_string(),
        );

        assert_eq!(validation.status, MigrationValidationStatus::InProgress);
        assert_eq!(validation.risk_level, RiskLevel::Medium);

        let issue = ValidationIssue {
            id: Uuid::new_v4(),
            severity: IssueSeverity::Critical,
            category: IssueCategory::DataInconsistency,
            title: "Critical Issue".to_string(),
            description: "This is critical".to_string(),
            affected_tables: vec!["documents".to_string()],
            affected_records: Some(100),
            recommendation: None,
            auto_fixable: false,
        };

        validation.add_issue(issue);
        assert_eq!(validation.risk_level, RiskLevel::Critical);

        validation.complete();
        assert_eq!(validation.status, MigrationValidationStatus::Failed);
    }
}