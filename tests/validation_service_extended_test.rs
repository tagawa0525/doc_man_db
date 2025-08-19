// Validation Service の実動作をカバーする追加テスト

use doc_man_db::models::validation::*;
use doc_man_db::services::validation_service::{
    ValidationService, ValidationServiceError, ValidationServiceImpl,
};
use uuid::Uuid;
use std::collections::HashMap;

#[tokio::test]
async fn test_validation_service_get_active_rules() {
    let service = ValidationServiceImpl::new();

    let rules = service.get_active_rules().await.unwrap();

    // ルールが取得できること
    assert!(!rules.is_empty());

    // 各ルールがアクティブであること
    for rule in &rules {
        assert!(rule.is_active);
        assert!(!rule.name.is_empty());
        assert!(!rule.description.is_empty());
    }

    // 期待するルールタイプが含まれること
    let rule_types: Vec<ValidationRuleType> = rules.iter().map(|r| r.rule_type.clone()).collect();

    assert!(rule_types.contains(&ValidationRuleType::MandatoryFields));
    assert!(rule_types.contains(&ValidationRuleType::ReferentialIntegrity));
    assert!(rule_types.contains(&ValidationRuleType::FileExistence));
}

#[tokio::test]
async fn test_validation_service_execute_referential_integrity() {
    let service = ValidationServiceImpl::new();
    let target_ids = vec![-1, 0, 1001, 2000]; // 無効な参照ID

    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::ReferentialIntegrity,
        target_ids,
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();

    // 無効なIDに対してエラーが発生すること
    assert!(!result.errors.is_empty());
    assert_eq!(result.rule_type, ValidationRuleType::ReferentialIntegrity);

    // -1, 0, 1001, 2000は全て無効範囲
    assert!(result.errors.len() >= 3);
}

#[tokio::test]
async fn test_validation_service_execute_mandatory_fields() {
    let service = ValidationServiceImpl::new();
    let target_ids = vec![5, 10, 15, 20]; // 5で割り切れるIDは必須フィールド不足

    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::MandatoryFields,
        target_ids,
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();

    // 5で割り切れるIDには全てエラーが発生すること
    assert_eq!(result.errors.len(), 4);
    assert_eq!(result.rule_type, ValidationRuleType::MandatoryFields);

    for error in &result.errors {
        assert_eq!(error.field_name, Some("title".to_string()));
        assert_eq!(error.error_type, ValidationErrorType::MissingValue);
    }
}

#[tokio::test]
async fn test_validation_service_execute_file_existence() {
    let service = ValidationServiceImpl::new();
    let target_ids = vec![3, 6, 9, 12]; // 3で割り切れるIDはファイル存在せず

    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::FileExistence,
        target_ids,
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();

    // 3で割り切れるIDに対してワーニングが発生すること
    assert_eq!(result.warnings.len(), 4);
    assert_eq!(result.rule_type, ValidationRuleType::FileExistence);

    for warning in &result.warnings {
        assert_eq!(warning.field_name, Some("file_path".to_string()));
        assert_eq!(warning.severity, ValidationSeverity::Warning);
    }
}

#[tokio::test]
async fn test_validation_service_execute_duplicate_check() {
    let service = ValidationServiceImpl::new();
    let target_ids = vec![1, 2, 7, 8];

    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::DuplicateCheck,
        target_ids,
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();

    // 重複がない場合はエラーなし
    assert_eq!(result.errors.len(), 0);
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.rule_type, ValidationRuleType::DuplicateCheck);
}

#[tokio::test]
async fn test_validation_service_execute_data_format() {
    let service = ValidationServiceImpl::new();
    let target_ids = vec![7, 14, 21]; // 7で割り切れるIDはフォーマット不正

    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::DataFormat,
        target_ids,
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();

    // 7で割り切れるIDに対してワーニングが発生すること
    assert_eq!(result.warnings.len(), 3);
    assert_eq!(result.rule_type, ValidationRuleType::DataFormat);
    assert!(result.execution_time_ms > 0);
    
    for warning in &result.warnings {
        assert_eq!(warning.error_type, ValidationErrorType::InvalidFormat);
        assert_eq!(warning.severity, ValidationSeverity::Warning);
    }
}

#[tokio::test]
async fn test_validation_service_execute_business_logic() {
    let service = ValidationServiceImpl::new();
    let target_ids = vec![11, 22, 33]; // 11で割り切れるIDは業務ルール違反

    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::BusinessLogic,
        target_ids,
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();

    // 11で割り切れるIDに対してワーニングが発生すること
    assert_eq!(result.warnings.len(), 3);
    assert_eq!(result.rule_type, ValidationRuleType::BusinessLogic);
    assert!(result.execution_time_ms > 0);
    
    for warning in &result.warnings {
        assert_eq!(warning.error_type, ValidationErrorType::BusinessRuleViolation);
        assert_eq!(warning.severity, ValidationSeverity::Info);
    }
}

#[tokio::test]
async fn test_validation_service_empty_target_ids() {
    let service = ValidationServiceImpl::new();
    let target_ids: Vec<i32> = vec![];

    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::MandatoryFields,
        target_ids,
        parameters: HashMap::new(),
    };

    let result = service.execute_validation(request).await.unwrap();

    // 空のターゲットの場合はエラーなし
    assert_eq!(result.errors.len(), 0);
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.rule_type, ValidationRuleType::MandatoryFields);
}

#[tokio::test]
async fn test_validation_service_large_target_set() {
    let service = ValidationServiceImpl::new();
    let target_ids: Vec<i32> = (1..=100).collect(); // 大量のターゲットID

    // 必須フィールドチェック
    let mandatory_request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::MandatoryFields,
        target_ids: target_ids.clone(),
        parameters: HashMap::new(),
    };

    let mandatory_result = service.execute_validation(mandatory_request).await.unwrap();

    // 実行時間が記録されていること
    assert!(mandatory_result.execution_time_ms > 0);

    // 必須フィールドエラーの確認（5で割り切れるIDの数）
    assert_eq!(mandatory_result.errors.len(), 20); // 5, 10, 15, ..., 100

    // ファイル存在チェック
    let file_request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::FileExistence,
        target_ids: target_ids,
        parameters: HashMap::new(),
    };

    let file_result = service.execute_validation(file_request).await.unwrap();

    // ファイル存在警告の確認（3で割り切れるIDの数）
    assert_eq!(file_result.warnings.len(), 33); // 3, 6, 9, ..., 99
}

#[tokio::test]
async fn test_validation_service_get_active_rules_extended() {
    let service = ValidationServiceImpl::new();
    let rules = service.get_active_rules().await.unwrap();

    // アクティブなルールが取得できること
    assert!(!rules.is_empty());

    for rule in &rules {
        assert!(rule.is_active);
        assert!(!rule.name.is_empty());
        assert!(!rule.description.is_empty());
    }

    // 各ルールタイプが含まれていること
    let rule_types: Vec<ValidationRuleType> = rules.iter().map(|r| r.rule_type.clone()).collect();

    assert!(rule_types.contains(&ValidationRuleType::MandatoryFields));
    assert!(rule_types.contains(&ValidationRuleType::ReferentialIntegrity));
    assert!(rule_types.contains(&ValidationRuleType::FileExistence));
}

#[tokio::test]
async fn test_validation_service_mixed_results() {
    let service = ValidationServiceImpl::new();
    // 様々な条件を満たすIDの組み合わせ
    let target_ids = vec![1, 3, 5, 6, 10, 15, 1001];

    // 参照整合性チェック
    let ref_integrity_request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::ReferentialIntegrity,
        target_ids: target_ids.clone(),
        parameters: HashMap::new(),
    };

    let ref_integrity_result = service.execute_validation(ref_integrity_request).await.unwrap();
    assert_eq!(ref_integrity_result.errors.len(), 1); // 1001

    // 必須フィールドチェック
    let mandatory_request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::MandatoryFields,
        target_ids: target_ids.clone(),
        parameters: HashMap::new(),
    };

    let mandatory_result = service.execute_validation(mandatory_request).await.unwrap();
    assert_eq!(mandatory_result.errors.len(), 3); // 5, 10, 15

    // ファイル存在チェック
    let file_request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::FileExistence,
        target_ids: target_ids,
        parameters: HashMap::new(),
    };

    let file_result = service.execute_validation(file_request).await.unwrap();
    assert_eq!(file_result.warnings.len(), 3); // 3, 6, 15
}

#[tokio::test]
async fn test_validation_service_execution_time_recording() {
    let service = ValidationServiceImpl::new();
    let target_ids = vec![1, 2, 3];

    let start = std::time::Instant::now();
    
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::MandatoryFields,
        target_ids,
        parameters: HashMap::new(),
    };
    
    let result = service.execute_validation(request).await.unwrap();
    let total_duration = start.elapsed();

    // 実行時間が記録されていること
    assert!(result.execution_time_ms > 0);
    assert!(result.execution_time_ms <= total_duration.as_millis() as u64 + 100); // 多少の誤差許容
}

#[tokio::test]
async fn test_validation_service_quality_score_calculation() {
    let service = ValidationServiceImpl::new();
    let target_ids = vec![1, 2, 4, 8]; // エラーが出ないID群

    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::DuplicateCheck, // エラーが出ないルール
        target_ids,
        parameters: HashMap::new(),
    };
    
    let result = service.execute_validation(request).await.unwrap();

    // 品質スコアが計算されること
    if result.errors.is_empty() && result.warnings.is_empty() {
        assert_eq!(result.quality_score, 100.0);
    } else {
        assert!(result.quality_score < 100.0);
        assert!(result.quality_score >= 0.0);
    }
}

#[test]
fn test_validation_service_error_types() {
    // ValidationServiceError の各バリアントをテスト
    let database_error = ValidationServiceError::DatabaseError {
        message: "DB connection failed".to_string(),
    };
    let config_error = ValidationServiceError::ConfigurationError {
        message: "Invalid config".to_string(),
    };
    let validation_error = ValidationServiceError::ValidationFailed {
        message: "Validation failed".to_string(),
    };

    assert!(matches!(
        database_error,
        ValidationServiceError::DatabaseError { .. }
    ));
    assert!(matches!(
        config_error,
        ValidationServiceError::ConfigurationError { .. }
    ));
    assert!(matches!(
        validation_error,
        ValidationServiceError::ValidationFailed { .. }
    ));
}
