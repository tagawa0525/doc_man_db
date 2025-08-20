// Migration Handler の基本的なテスト

use doc_man_db::models::migration::*;
use uuid::Uuid;

#[test]
fn test_migration_request_creation() {
    let plan_id = Uuid::new_v4();
    let request = MigrationRequest {
        plan_id,
        dry_run: Some(true),
        force_execution: None,
        notification_settings: None,
    };

    assert_eq!(request.plan_id, plan_id);
    assert_eq!(request.dry_run, Some(true));
    assert!(request.force_execution.is_none());
}

#[test]
fn test_migration_plan_creation() {
    let plan = MigrationPlan::new(
        "Test Migration".to_string(),
        MigrationType::DataMigration,
        MigrationEnvironment::Development,
        MigrationEnvironment::Staging,
        "sqlite://source.db".to_string(),
        "sqlite://target.db".to_string(),
        "admin".to_string(),
    );

    assert_eq!(plan.name, "Test Migration");
    assert_eq!(plan.migration_type, MigrationType::DataMigration);
    assert_eq!(plan.source_environment, MigrationEnvironment::Development);
    assert_eq!(plan.target_environment, MigrationEnvironment::Staging);
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
fn test_validation_request_creation() {
    let validation = MigrationValidation::new(
        Uuid::new_v4(),
        ValidationType::PreMigration,
        "validator".to_string(),
    );

    assert_eq!(validation.validation_type, ValidationType::PreMigration);
    assert_eq!(validation.status, MigrationValidationStatus::InProgress);
    assert_eq!(validation.validated_by, "validator");
}
