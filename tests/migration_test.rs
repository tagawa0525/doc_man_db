use doc_man_db::models::migration::*;
use doc_man_db::services::{MigrationService, MigrationServiceImpl};

use tokio;
use uuid::Uuid;

#[tokio::test]
async fn test_migration_service_creation() {
    let _service = MigrationServiceImpl::new(
        "sqlite://source.db".to_string(),
        "sqlite://target.db".to_string(),
    );

    // サービスが正常に作成されることを確認
    // フィールドはプライベートなので、サービスの作成のみテスト
    assert!(true);
}

#[tokio::test]
async fn test_migration_plan_creation() {
    let plan = MigrationPlan::new(
        "Test Migration Plan".to_string(),
        MigrationType::DataMigration,
        MigrationEnvironment::Development,
        MigrationEnvironment::Staging,
        "sqlite://source.db".to_string(),
        "sqlite://target.db".to_string(),
        "admin".to_string(),
    );

    assert_eq!(plan.name, "Test Migration Plan");
    assert_eq!(plan.migration_type, MigrationType::DataMigration);
    assert_eq!(plan.source_environment, MigrationEnvironment::Development);
    assert_eq!(plan.target_environment, MigrationEnvironment::Staging);
    assert_eq!(plan.created_by, "admin");
    assert!(!plan.is_approved());
}

#[tokio::test]
async fn test_migration_plan_approval() {
    let mut plan = MigrationPlan::new(
        "Approval Test".to_string(),
        MigrationType::FullMigration,
        MigrationEnvironment::Staging,
        MigrationEnvironment::Production,
        "sqlite://staging.db".to_string(),
        "sqlite://production.db".to_string(),
        "admin".to_string(),
    );

    assert!(!plan.is_approved());

    plan.approve("supervisor".to_string());
    assert!(plan.is_approved());
    assert_eq!(plan.approved_by, Some("supervisor".to_string()));
}

#[tokio::test]
async fn test_migration_job_lifecycle() {
    let mut job = MigrationJob::new(Uuid::new_v4(), 5);

    // 初期状態の確認
    assert_eq!(job.status, MigrationStatus::Pending);
    assert_eq!(job.progress_percentage, 0.0);
    assert_eq!(job.completed_steps, 0);
    assert!(job.started_at.is_none());
    assert!(job.completed_at.is_none());

    // 開始
    job.start();
    assert_eq!(job.status, MigrationStatus::Running);
    assert!(job.started_at.is_some());

    // 進捗更新
    job.update_progress("Step 1: Schema Migration".to_string(), 1, Some(100));
    assert_eq!(job.progress_percentage, 20.0);
    assert_eq!(job.completed_steps, 1);
    assert_eq!(job.current_step, Some("Step 1: Schema Migration".to_string()));
    assert_eq!(job.data_processed_mb, Some(100));

    job.update_progress("Step 2: Data Migration".to_string(), 2, Some(500));
    assert_eq!(job.progress_percentage, 40.0);
    assert_eq!(job.completed_steps, 2);

    // 完了
    job.complete();
    assert_eq!(job.status, MigrationStatus::Completed);
    assert_eq!(job.progress_percentage, 100.0);
    assert_eq!(job.completed_steps, 5);
    assert!(job.completed_at.is_some());

    // 実行時間の計算
    assert!(job.duration_minutes().is_some());
    assert!(job.duration_minutes().unwrap() >= 0);
}

#[tokio::test]
async fn test_migration_job_failure() {
    let mut job = MigrationJob::new(Uuid::new_v4(), 3);

    job.start();
    job.update_progress("Step 1".to_string(), 1, None);
    job.fail("Database connection failed".to_string());

    assert_eq!(job.status, MigrationStatus::Failed);
    assert!(job.completed_at.is_some());
    assert_eq!(
        job.error_message,
        Some("Database connection failed".to_string())
    );
}

#[tokio::test]
async fn test_migration_job_logging() {
    let mut job = MigrationJob::new(Uuid::new_v4(), 2);

    // ログ追加
    job.add_log(
        LogLevel::Info,
        "Migration started".to_string(),
        Some("start".to_string()),
        None,
    );

    job.add_log(
        LogLevel::Warning,
        "Non-critical issue detected".to_string(),
        Some("validation".to_string()),
        Some(serde_json::json!({"table": "documents", "issue": "missing_index"})),
    );

    job.add_log(
        LogLevel::Error,
        "Critical error occurred".to_string(),
        Some("execution".to_string()),
        None,
    );

    assert_eq!(job.logs.len(), 3);
    assert_eq!(job.logs[0].level, LogLevel::Info);
    assert_eq!(job.logs[1].level, LogLevel::Warning);
    assert_eq!(job.logs[2].level, LogLevel::Error);
    assert_eq!(job.logs[0].message, "Migration started");
    assert!(job.logs[1].data.is_some());
}

#[tokio::test]
async fn test_migration_validation_creation() {
    let plan_id = Uuid::new_v4();
    let validation = MigrationValidation::new(
        plan_id,
        ValidationType::PreMigration,
        "validator".to_string(),
    );

    assert_eq!(validation.plan_id, plan_id);
    assert_eq!(validation.validation_type, ValidationType::PreMigration);
    assert_eq!(validation.status, MigrationValidationStatus::InProgress);
    assert_eq!(validation.risk_level, RiskLevel::Medium);
    assert_eq!(validation.validated_by, "validator");
    assert!(validation.issues.is_empty());
}

#[tokio::test]
async fn test_migration_validation_with_issues() {
    let mut validation = MigrationValidation::new(
        Uuid::new_v4(),
        ValidationType::DataIntegrity,
        "system".to_string(),
    );

    // 重要度の低い問題を追加
    let low_issue = ValidationIssue {
        id: Uuid::new_v4(),
        severity: IssueSeverity::Low,
        category: IssueCategory::PerformanceImpact,
        title: "Minor performance issue".to_string(),
        description: "Query optimization recommended".to_string(),
        affected_tables: vec!["documents".to_string()],
        affected_records: Some(100),
        recommendation: Some("Add index on frequently queried column".to_string()),
        auto_fixable: true,
    };

    validation.add_issue(low_issue);
    assert_eq!(validation.risk_level, RiskLevel::Medium); // 変更なし

    // 重要度の高い問題を追加
    let high_issue = ValidationIssue {
        id: Uuid::new_v4(),
        severity: IssueSeverity::High,
        category: IssueCategory::DataInconsistency,
        title: "Data integrity violation".to_string(),
        description: "Foreign key constraints violated".to_string(),
        affected_tables: vec!["documents".to_string(), "document_types".to_string()],
        affected_records: Some(50),
        recommendation: Some("Fix foreign key references before migration".to_string()),
        auto_fixable: false,
    };

    validation.add_issue(high_issue);
    assert_eq!(validation.risk_level, RiskLevel::High);

    // クリティカルな問題を追加
    let critical_issue = ValidationIssue {
        id: Uuid::new_v4(),
        severity: IssueSeverity::Critical,
        category: IssueCategory::SecurityRisk,
        title: "Security vulnerability detected".to_string(),
        description: "Sensitive data exposure risk".to_string(),
        affected_tables: vec!["users".to_string()],
        affected_records: Some(1000),
        recommendation: Some("Encrypt sensitive data before migration".to_string()),
        auto_fixable: false,
    };

    validation.add_issue(critical_issue);
    assert_eq!(validation.risk_level, RiskLevel::Critical);

    // 検証完了
    validation.complete();
    assert_eq!(validation.status, MigrationValidationStatus::Failed); // クリティカルな問題があるため
    assert!(validation.has_critical_issues());
    assert!(validation.has_issues());
    assert_eq!(validation.issues.len(), 3);
}

#[tokio::test]
async fn test_migration_validation_success() {
    let mut validation = MigrationValidation::new(
        Uuid::new_v4(),
        ValidationType::PreMigration,
        "validator".to_string(),
    );

    // 問題なしで完了
    validation.complete();
    assert_eq!(validation.status, MigrationValidationStatus::Passed);
    assert!(!validation.has_critical_issues());
    assert!(!validation.has_issues());
}

#[tokio::test]
async fn test_migration_service_create_plan() {
    let service = MigrationServiceImpl::default();

    let plan = MigrationPlan::new(
        "Service Test Plan".to_string(),
        MigrationType::SchemaMigration,
        MigrationEnvironment::Development,
        MigrationEnvironment::Staging,
        "sqlite://dev.db".to_string(),
        "sqlite://staging.db".to_string(),
        "admin".to_string(),
    );

    let result = service.create_migration_plan(plan).await;
    assert!(result.is_ok());

    let created_plan = result.unwrap();
    assert_eq!(created_plan.name, "Service Test Plan");
    assert_eq!(created_plan.migration_type, MigrationType::SchemaMigration);
}

#[tokio::test]
async fn test_migration_service_create_plan_error() {
    let service = MigrationServiceImpl::default();

    // 同じデータベースを指定してエラーを発生させる
    let invalid_plan = MigrationPlan::new(
        "Invalid Plan".to_string(),
        MigrationType::DataMigration,
        MigrationEnvironment::Development,
        MigrationEnvironment::Development,
        "sqlite://same.db".to_string(),
        "sqlite://same.db".to_string(), // 同じDB
        "admin".to_string(),
    );

    let result = service.create_migration_plan(invalid_plan).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_migration_service_validate_plan() {
    let service = MigrationServiceImpl::default();

    let plan_id = Uuid::new_v4();
    let result = service
        .validate_migration_plan(plan_id, ValidationType::PreMigration, "validator".to_string())
        .await;

    assert!(result.is_ok());

    let validation = result.unwrap();
    assert_eq!(validation.plan_id, plan_id);
    assert_eq!(validation.validation_type, ValidationType::PreMigration);
    assert_eq!(validation.validated_by, "validator");
}

#[tokio::test]
async fn test_migration_service_execute_dry_run() {
    let service = MigrationServiceImpl::default();

    let request = MigrationRequest {
        plan_id: Uuid::new_v4(),
        dry_run: Some(true),
        force_execution: None,
        notification_settings: None,
    };

    let result = service.execute_migration(request).await;
    assert!(result.is_ok());

    let job = result.unwrap();
    assert_eq!(job.status, MigrationStatus::Completed);
}

#[tokio::test]
async fn test_migration_service_get_statistics() {
    let service = MigrationServiceImpl::default();

    let result = service.get_migration_statistics().await;
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert_eq!(stats.total_migrations, 0);
    assert_eq!(stats.success_rate, 100.0);
}

#[tokio::test]
async fn test_migration_service_list_operations() {
    let service = MigrationServiceImpl::default();

    // 計画一覧取得
    let plans_result = service.list_migration_plans().await;
    assert!(plans_result.is_ok());
    let plans = plans_result.unwrap();
    assert_eq!(plans.len(), 0); // サンプル実装では空のリスト

    // ジョブ一覧取得
    let jobs_result = service.list_migration_jobs(Some(10)).await;
    assert!(jobs_result.is_ok());
    let jobs = jobs_result.unwrap();
    assert_eq!(jobs.len(), 0); // サンプル実装では空のリスト

    // ジョブ取得
    let job_id = Uuid::new_v4();
    let job_result = service.get_migration_job(job_id).await;
    assert!(job_result.is_ok());
    assert!(job_result.unwrap().is_none()); // サンプル実装ではNone
}

#[tokio::test]
async fn test_migration_types_serialization() {
    let migration_types = vec![
        MigrationType::DatabaseMigration,
        MigrationType::DataMigration,
        MigrationType::SchemaMigration,
        MigrationType::FullMigration,
    ];

    for migration_type in migration_types {
        let json = serde_json::to_string(&migration_type).unwrap();
        let deserialized: MigrationType = serde_json::from_str(&json).unwrap();
        assert_eq!(migration_type, deserialized);
    }
}

#[tokio::test]
async fn test_migration_status_serialization() {
    let statuses = vec![
        MigrationStatus::Pending,
        MigrationStatus::Planning,
        MigrationStatus::Validating,
        MigrationStatus::Running,
        MigrationStatus::Completed,
        MigrationStatus::Failed,
        MigrationStatus::RollingBack,
        MigrationStatus::RolledBack,
    ];

    for status in statuses {
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: MigrationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }
}

#[tokio::test]
async fn test_migration_environment_serialization() {
    let environments = vec![
        MigrationEnvironment::Development,
        MigrationEnvironment::Staging,
        MigrationEnvironment::Production,
    ];

    for environment in environments {
        let json = serde_json::to_string(&environment).unwrap();
        let deserialized: MigrationEnvironment = serde_json::from_str(&json).unwrap();
        assert_eq!(environment, deserialized);
    }
}

#[tokio::test]
async fn test_validation_types_serialization() {
    let validation_types = vec![
        ValidationType::PreMigration,
        ValidationType::PostMigration,
        ValidationType::DataIntegrity,
        ValidationType::Performance,
        ValidationType::Security,
    ];

    for validation_type in validation_types {
        let json = serde_json::to_string(&validation_type).unwrap();
        let deserialized: ValidationType = serde_json::from_str(&json).unwrap();
        assert_eq!(validation_type, deserialized);
    }
}

#[tokio::test]
async fn test_rollback_request() {
    let job_id = Uuid::new_v4();
    let rollback_request = RollbackRequest {
        job_id,
        target_point: Some(chrono::Utc::now()),
        reason: "Critical error detected".to_string(),
    };

    assert_eq!(rollback_request.job_id, job_id);
    assert!(rollback_request.target_point.is_some());
    assert_eq!(rollback_request.reason, "Critical error detected");
}

#[tokio::test]
async fn test_migration_statistics_creation() {
    let stats = MigrationStatistics {
        total_migrations: 50,
        successful_migrations: 45,
        failed_migrations: 5,
        average_duration_minutes: 120.5,
        total_data_migrated_gb: 256.7,
        success_rate: 90.0,
        last_migration: Some(chrono::Utc::now()),
        upcoming_migrations: 3,
    };

    assert_eq!(stats.total_migrations, 50);
    assert_eq!(stats.successful_migrations, 45);
    assert_eq!(stats.failed_migrations, 5);
    assert_eq!(stats.success_rate, 90.0);
    assert!(stats.last_migration.is_some());
}

#[tokio::test]
async fn test_complex_migration_scenario() {
    let service = MigrationServiceImpl::default();

    // 1. 移行計画作成
    let plan = MigrationPlan::new(
        "Complex Migration".to_string(),
        MigrationType::FullMigration,
        MigrationEnvironment::Staging,
        MigrationEnvironment::Production,
        "sqlite://staging.db".to_string(),
        "sqlite://production.db".to_string(),
        "admin".to_string(),
    );

    let created_plan = service.create_migration_plan(plan).await.unwrap();

    // 2. 事前検証
    let validation = service
        .validate_migration_plan(
            created_plan.id,
            ValidationType::PreMigration,
            "validator".to_string(),
        )
        .await
        .unwrap();

    assert_eq!(validation.plan_id, created_plan.id);

    // 3. ドライラン実行
    let dry_run_request = MigrationRequest {
        plan_id: created_plan.id,
        dry_run: Some(true),
        force_execution: None,
        notification_settings: None,
    };

    let dry_run_job = service.execute_migration(dry_run_request).await.unwrap();
    assert_eq!(dry_run_job.status, MigrationStatus::Completed);

    // 4. 統計確認
    let stats = service.get_migration_statistics().await.unwrap();
    assert!(stats.success_rate >= 0.0);
}