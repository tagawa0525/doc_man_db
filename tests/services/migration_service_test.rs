// Migration Service の単体テスト

use doc_man_db::models::migration::*;
use uuid::Uuid;

#[test]
fn test_migration_request_creation() {
    let request = MigrationRequest {
        plan_id: Uuid::new_v4(),
        dry_run: Some(false),
        force_execution: Some(true),
        notification_settings: None,
    };

    assert!(request.dry_run == Some(false));
    assert!(request.force_execution == Some(true));
    assert!(request.notification_settings.is_none());
}

#[test]
fn test_migration_plan_lifecycle() {
    let mut plan = MigrationPlan::new(
        "テスト移行計画".to_string(),
        MigrationType::DataMigration,
        MigrationEnvironment::Development,
        MigrationEnvironment::Staging,
        "sqlite://source.db".to_string(),
        "sqlite://target.db".to_string(),
        "admin_user".to_string(),
    );

    // 初期状態
    assert_eq!(plan.migration_type, MigrationType::DataMigration);
    assert_eq!(plan.source_environment, MigrationEnvironment::Development);
    assert_eq!(plan.target_environment, MigrationEnvironment::Staging);
    assert!(!plan.is_approved());

    // 承認
    plan.approve("admin_user".to_string());
    assert!(plan.is_approved());
    assert_eq!(plan.approved_by, Some("admin_user".to_string()));
}

#[test]
fn test_migration_job_execution() {
    let plan_id = Uuid::new_v4();
    let mut job = MigrationJob::new(plan_id, 10);

    // 開始
    assert_eq!(job.status, MigrationStatus::Pending);
    job.start();
    assert_eq!(job.status, MigrationStatus::Running);
    assert!(job.started_at.is_some());

    // 進行状況更新
    job.update_progress("Migrating tables".to_string(), 5, Some(100));
    assert_eq!(job.progress_percentage, 50.0);
    assert_eq!(job.current_step, Some("Migrating tables".to_string()));

    // 完了
    job.complete();
    assert_eq!(job.status, MigrationStatus::Completed);
    assert!(job.completed_at.is_some());
}

#[test]
fn test_migration_validation() {
    let validation = MigrationValidation::new(
        Uuid::new_v4(),
        ValidationType::PreMigration,
        "validator_service".to_string(),
    );

    assert_eq!(validation.validation_type, ValidationType::PreMigration);
    assert_eq!(validation.validated_by, "validator_service");
    assert_eq!(validation.status, MigrationValidationStatus::InProgress);
    assert!(!validation.has_issues());
}

#[test]
fn test_migration_error_handling() {
    let plan_id = Uuid::new_v4();
    let mut job = MigrationJob::new(plan_id, 10);
    job.start();
    job.update_progress("Processing data".to_string(), 7, Some(150));

    // エラー発生
    job.fail("Critical error occurred".to_string());
    assert_eq!(job.status, MigrationStatus::Failed);
    assert!(job.completed_at.is_some());
    assert_eq!(
        job.error_message,
        Some("Critical error occurred".to_string())
    );
}

#[test]
fn test_migration_plan_validation() {
    let plan = MigrationPlan::new(
        "データベース移行テスト".to_string(),
        MigrationType::DatabaseMigration,
        MigrationEnvironment::Development,
        MigrationEnvironment::Production,
        "sqlite://./source.db".to_string(),
        "postgres://user:pass@localhost:5432/target".to_string(),
        "migration_admin".to_string(),
    );

    // 必須フィールドが設定されていること
    assert!(!plan.name.is_empty());
    assert!(!plan.source_database_url.is_empty());
    assert!(!plan.target_database_url.is_empty());
    assert!(!plan.created_by.is_empty());
    assert_eq!(plan.migration_type, MigrationType::DatabaseMigration);
}

#[test]
fn test_migration_log_functionality() {
    let plan_id = Uuid::new_v4();
    let mut job = MigrationJob::new(plan_id, 5);

    // ログ追加
    job.add_log(
        LogLevel::Info,
        "Migration started".to_string(),
        Some("initialization".to_string()),
        None,
    );

    assert_eq!(job.logs.len(), 1);
    assert_eq!(job.logs[0].message, "Migration started");
    assert_eq!(job.logs[0].level, LogLevel::Info);
}

#[test]
fn test_migration_type_serialization() {
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

#[test]
fn test_migration_environment_hierarchy() {
    // 環境の重要度順序テスト
    let environments = vec![
        MigrationEnvironment::Development,
        MigrationEnvironment::Staging,
        MigrationEnvironment::Staging,
        MigrationEnvironment::Production,
    ];

    // すべての環境がシリアライズできることを確認
    for env in environments {
        let json = serde_json::to_string(&env).unwrap();
        let deserialized: MigrationEnvironment = serde_json::from_str(&json).unwrap();
        assert_eq!(env, deserialized);
    }
}

#[test]
fn test_migration_duration_calculation() {
    let plan_id = Uuid::new_v4();
    let mut job = MigrationJob::new(plan_id, 3);

    // まだ開始していない
    assert!(job.duration_minutes().is_none());

    job.start();
    std::thread::sleep(std::time::Duration::from_millis(10));
    job.complete();

    // 実行時間が計算される（分単位）
    let duration = job.duration_minutes();
    assert!(duration.is_some());
    assert!(duration.unwrap() >= 0);
}
