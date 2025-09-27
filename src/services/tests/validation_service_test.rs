use super::super::validation_service::{ValidationService, ValidationServiceImpl};
use crate::models::validation::{ValidationExecutionRequest, ValidationRuleType};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::test]
async fn test_validation_service_creation() {
    let service = ValidationServiceImpl::new();
    assert!(!service.builtin_rules.is_empty());
}

#[tokio::test]
async fn test_get_active_rules() {
    let service = ValidationServiceImpl::new();
    let rules = service.get_active_rules().await.unwrap();
    assert!(!rules.is_empty());
    assert!(rules.iter().all(|r| r.is_active));
}

#[tokio::test]
async fn test_execute_referential_integrity() {
    let service = ValidationServiceImpl::new();
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::ReferentialIntegrity,
        target_ids: vec![1, 2, 1001], // 1001は無効とみなされる
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();
    assert_eq!(result.rule_type, ValidationRuleType::ReferentialIntegrity);
    assert!(!result.errors.is_empty()); // 1001が無効なのでエラーが発生
}

#[tokio::test]
async fn test_execute_mandatory_fields() {
    let service = ValidationServiceImpl::new();
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::MandatoryFields,
        target_ids: vec![1, 5, 10], // 5と10は5で割り切れるのでエラー
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();
    assert_eq!(result.rule_type, ValidationRuleType::MandatoryFields);
    assert_eq!(result.errors.len(), 2); // 5と10でエラー
}

#[tokio::test]
async fn test_execute_file_existence() {
    let service = ValidationServiceImpl::new();
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::FileExistence,
        target_ids: vec![1, 2, 3, 6], // 3と6は3で割り切れるので警告
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();
    assert_eq!(result.rule_type, ValidationRuleType::FileExistence);
    assert_eq!(result.warnings.len(), 2); // 3と6で警告
}
