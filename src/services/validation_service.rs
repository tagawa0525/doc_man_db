use crate::models::validation::{
    ValidationError, ValidationErrorType, ValidationExecutionRequest, ValidationResult,
    ValidationRule, ValidationRuleType, ValidationSeverity,
};
use async_trait::async_trait;
use chrono::Utc;

use tracing::info;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ValidationServiceError {
    #[error("Database error: {message}")]
    DatabaseError { message: String },
    #[error("Rule not found: {rule_id}")]
    RuleNotFound { rule_id: Uuid },
    #[error("Validation failed: {message}")]
    ValidationFailed { message: String },
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}

#[async_trait]
pub trait ValidationService: Send + Sync {
    async fn execute_validation(
        &self,
        request: ValidationExecutionRequest,
    ) -> Result<ValidationResult, ValidationServiceError>;
    async fn get_active_rules(&self) -> Result<Vec<ValidationRule>, ValidationServiceError>;
}

pub struct ValidationServiceImpl {
    pub builtin_rules: Vec<ValidationRule>,
}

impl ValidationServiceImpl {
    pub fn new() -> Self {
        Self {
            builtin_rules: Self::create_builtin_rules(),
        }
    }

    fn create_builtin_rules() -> Vec<ValidationRule> {
        vec![
            ValidationRule {
                id: Uuid::new_v4(),
                name: "参照整合性チェック".to_string(),
                description: "データベースの参照整合性を検証".to_string(),
                rule_type: ValidationRuleType::ReferentialIntegrity,
                severity: ValidationSeverity::Error,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            ValidationRule {
                id: Uuid::new_v4(),
                name: "必須フィールドチェック".to_string(),
                description: "必須フィールドが入力されているかチェック".to_string(),
                rule_type: ValidationRuleType::MandatoryFields,
                severity: ValidationSeverity::Error,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            ValidationRule {
                id: Uuid::new_v4(),
                name: "重複チェック".to_string(),
                description: "重複データがないかチェック".to_string(),
                rule_type: ValidationRuleType::DuplicateCheck,
                severity: ValidationSeverity::Warning,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            ValidationRule {
                id: Uuid::new_v4(),
                name: "ファイル存在確認".to_string(),
                description: "指定されたファイルが存在するかチェック".to_string(),
                rule_type: ValidationRuleType::FileExistence,
                severity: ValidationSeverity::Warning,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            ValidationRule {
                id: Uuid::new_v4(),
                name: "データ形式チェック".to_string(),
                description: "データが正しい形式かチェック".to_string(),
                rule_type: ValidationRuleType::DataFormat,
                severity: ValidationSeverity::Warning,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            ValidationRule {
                id: Uuid::new_v4(),
                name: "業務ロジックチェック".to_string(),
                description: "業務固有のロジックをチェック".to_string(),
                rule_type: ValidationRuleType::BusinessLogic,
                severity: ValidationSeverity::Info,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ]
    }

    async fn execute_referential_integrity_rule(
        &self,
        target_ids: &[i32],
    ) -> Result<ValidationResult, ValidationServiceError> {
        let start_time = std::time::Instant::now();
        let mut result = ValidationResult::new(
            ValidationRuleType::ReferentialIntegrity,
            "参照整合性チェック".to_string(),
        );

        // 模擬的な参照整合性チェック
        for &id in target_ids {
            if id <= 0 || id > 1000 {
                result.add_error(ValidationError {
                    error_type: ValidationErrorType::ReferenceError,
                    message: format!("ID {id} は無効な参照です"),
                    field_name: Some("id".to_string()),
                    entity_id: Some(id),
                    severity: ValidationSeverity::Error,
                });
            }
        }

        result.execution_time_ms = start_time.elapsed().as_millis().max(1) as u64;
        Ok(result)
    }

    async fn execute_mandatory_fields_rule(
        &self,
        target_ids: &[i32],
    ) -> Result<ValidationResult, ValidationServiceError> {
        let start_time = std::time::Instant::now();
        let mut result = ValidationResult::new(
            ValidationRuleType::MandatoryFields,
            "必須フィールドチェック".to_string(),
        );

        // 模擬的な必須フィールドチェック
        for &id in target_ids {
            if id % 5 == 0 {
                // 5で割り切れるIDは必須フィールドが不足と仮定
                result.add_error(ValidationError {
                    error_type: ValidationErrorType::MissingValue,
                    message: format!("エンティティ {id} で必須フィールドが不足しています"),
                    field_name: Some("title".to_string()),
                    entity_id: Some(id),
                    severity: ValidationSeverity::Error,
                });
            }
        }

        result.execution_time_ms = start_time.elapsed().as_millis().max(1) as u64;
        Ok(result)
    }

    async fn execute_duplicate_check_rule(
        &self,
        _target_ids: &[i32],
    ) -> Result<ValidationResult, ValidationServiceError> {
        let start_time = std::time::Instant::now();
        let mut result = ValidationResult::new(
            ValidationRuleType::DuplicateCheck,
            "重複チェック".to_string(),
        );

        // 模擬的な重複チェック（重複なしと仮定）
        result.execution_time_ms = start_time.elapsed().as_millis().max(1) as u64;
        Ok(result)
    }

    async fn execute_file_existence_rule(
        &self,
        target_ids: &[i32],
    ) -> Result<ValidationResult, ValidationServiceError> {
        let start_time = std::time::Instant::now();
        let mut result = ValidationResult::new(
            ValidationRuleType::FileExistence,
            "ファイル存在確認".to_string(),
        );

        // 模擬的なファイル存在チェック
        for &id in target_ids {
            if id % 3 == 0 {
                // 3で割り切れるIDはファイルが存在しないと仮定
                result.add_warning(ValidationError {
                    error_type: ValidationErrorType::DataInconsistency,
                    message: format!("エンティティ {id} のファイルが見つかりません"),
                    field_name: Some("file_path".to_string()),
                    entity_id: Some(id),
                    severity: ValidationSeverity::Warning,
                });
            }
        }

        result.execution_time_ms = start_time.elapsed().as_millis().max(1) as u64;
        Ok(result)
    }

    async fn execute_data_format_rule(
        &self,
        target_ids: &[i32],
    ) -> Result<ValidationResult, ValidationServiceError> {
        let start_time = std::time::Instant::now();
        let mut result = ValidationResult::new(
            ValidationRuleType::DataFormat,
            "データ形式チェック".to_string(),
        );

        // 模擬的なデータ形式チェック
        for &id in target_ids {
            if id % 7 == 0 {
                // 7で割り切れるIDは形式が不正と仮定
                result.add_warning(ValidationError {
                    error_type: ValidationErrorType::InvalidFormat,
                    message: format!("エンティティ {id} のデータ形式が不正です"),
                    field_name: Some("number".to_string()),
                    entity_id: Some(id),
                    severity: ValidationSeverity::Warning,
                });
            }
        }

        result.execution_time_ms = start_time.elapsed().as_millis().max(1) as u64;
        Ok(result)
    }

    async fn execute_business_logic_rule(
        &self,
        target_ids: &[i32],
    ) -> Result<ValidationResult, ValidationServiceError> {
        let start_time = std::time::Instant::now();
        let mut result = ValidationResult::new(
            ValidationRuleType::BusinessLogic,
            "業務ロジックチェック".to_string(),
        );

        // 模擬的な業務ロジックチェック
        for &id in target_ids {
            if id % 11 == 0 {
                // 11で割り切れるIDは業務ルール違反と仮定
                result.add_warning(ValidationError {
                    error_type: ValidationErrorType::BusinessRuleViolation,
                    message: format!("エンティティ {id} で業務ルール違反が検出されました"),
                    field_name: None,
                    entity_id: Some(id),
                    severity: ValidationSeverity::Info,
                });
            }
        }

        result.execution_time_ms = start_time.elapsed().as_millis().max(1) as u64;
        Ok(result)
    }
}

#[async_trait]
impl ValidationService for ValidationServiceImpl {
    async fn execute_validation(
        &self,
        request: ValidationExecutionRequest,
    ) -> Result<ValidationResult, ValidationServiceError> {
        info!(
            "Executing validation for rule type: {:?}",
            request.rule_type
        );

        let result = match request.rule_type {
            ValidationRuleType::ReferentialIntegrity => {
                self.execute_referential_integrity_rule(&request.target_ids)
                    .await?
            }
            ValidationRuleType::MandatoryFields => {
                self.execute_mandatory_fields_rule(&request.target_ids)
                    .await?
            }
            ValidationRuleType::DuplicateCheck => {
                self.execute_duplicate_check_rule(&request.target_ids)
                    .await?
            }
            ValidationRuleType::FileExistence => {
                self.execute_file_existence_rule(&request.target_ids)
                    .await?
            }
            ValidationRuleType::DataFormat => {
                self.execute_data_format_rule(&request.target_ids).await?
            }
            ValidationRuleType::BusinessLogic => {
                self.execute_business_logic_rule(&request.target_ids)
                    .await?
            }
        };

        info!(
            "Validation completed. Valid: {}, Errors: {}, Warnings: {}",
            result.is_valid,
            result.errors.len(),
            result.warnings.len()
        );

        Ok(result)
    }

    async fn get_active_rules(&self) -> Result<Vec<ValidationRule>, ValidationServiceError> {
        Ok(self.builtin_rules.clone())
    }
}

impl Default for ValidationServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
