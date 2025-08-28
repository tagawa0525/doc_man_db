use super::super::notification_service::NotificationService;
use crate::batch::{BatchExecution, BatchStatus, BatchType};
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_notification_service_creation() {
    let service = NotificationService::new();
    let config = service.get_notification_config().await.unwrap();

    assert!(config.enabled);
    assert!(!config.channels.is_empty());
}

#[tokio::test]
async fn test_file_check_notification() {
    let service = NotificationService::new();

    let batch_result = BatchExecution {
        id: Uuid::new_v4(),
        batch_type: BatchType::FileCheck,
        status: BatchStatus::Completed,
        total_items: 100,
        processed_items: 100,
        success_count: 95,
        error_count: 5,
        start_time: Utc::now(),
        end_time: Some(Utc::now()),
        started_by: None,
        result_summary: None,
        error_details: None,
    };

    let result = service
        .send_file_check_alert(&batch_result, "Test alert")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_security_alert() {
    let service = NotificationService::new();

    let result = service
        .send_security_alert("Unauthorized Access", "Test security alert")
        .await;
    assert!(result.is_ok());
}
