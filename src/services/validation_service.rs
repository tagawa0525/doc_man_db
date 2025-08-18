use crate::models::{
    ValidationError, ValidationExecutionRequest, ValidationResult, ValidationRule, 
    ValidationRuleType, ValidationSeverity, ValidationStatus
};
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ValidationServiceError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Rule not found: {0}")]
    RuleNotFound(Uuid),
    #[error("Validation execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

#[async_trait]
pub trait ValidationService: Send + Sync {
    /// データ検証を実行
    async fn execute_validation(
        &self,
        request: ValidationExecutionRequest,
    ) -> Result<ValidationResult, ValidationServiceError>;

    /// アクティブな検証ルールを取得
    async fn get_active_rules(&self) -> Result<Vec<ValidationRule>, ValidationServiceError>;

    /// 特定の検証ルールを取得
    async fn get_rule(&self, rule_id: Uuid) -> Result<ValidationRule, ValidationServiceError>;

    /// 検証結果を取得
    async fn get_validation_result(
        &self,
        result_id: Uuid,
    ) -> Result<ValidationResult, ValidationServiceError>;

    /// 検証エラーを解決済みにマーク
    async fn resolve_validation_error(
        &self,
        error_id: Uuid,
        resolution_note: String,
    ) -> Result<(), ValidationServiceError>;
}

pub struct ValidationServiceImpl {
    // TODO: 実際のデータベース接続を追加
    built_in_rules: Vec<ValidationRule>,
}

impl Default for ValidationServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationServiceImpl {
    pub fn new() -> Self {
        Self {
            built_in_rules: Self::create_built_in_rules(),
        }
    }

    /// 組み込み検証ルールを作成
    fn create_built_in_rules() -> Vec<ValidationRule> {
        vec![
            // 参照整合性チェック
            ValidationRule {
                id: Uuid::new_v4(),
                name: "文書種別参照整合性".to_string(),
                description: "文書テーブルの document_type_id が document_types テーブルに存在するかチェック".to_string(),
                rule_type: ValidationRuleType::ReferentialIntegrity,
                severity: ValidationSeverity::Critical,
                target_table: "documents".to_string(),
                target_field: Some("document_type_id".to_string()),
                sql_query: Some(
                    r#"
                    SELECT d.id, d.number, d.document_type_id 
                    FROM documents d 
                    LEFT JOIN document_types dt ON d.document_type_id = dt.id 
                    WHERE dt.id IS NULL AND d.is_active = 1
                    "#.to_string()
                ),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            
            // 作成者参照整合性チェック
            ValidationRule {
                id: Uuid::new_v4(),
                name: "作成者参照整合性".to_string(),
                description: "文書テーブルの created_by が employees テーブルに存在するかチェック".to_string(),
                rule_type: ValidationRuleType::ReferentialIntegrity,
                severity: ValidationSeverity::Critical,
                target_table: "documents".to_string(),
                target_field: Some("created_by".to_string()),
                sql_query: Some(
                    r#"
                    SELECT d.id, d.number, d.created_by 
                    FROM documents d 
                    LEFT JOIN employees e ON d.created_by = e.id 
                    WHERE e.id IS NULL AND d.is_active = 1
                    "#.to_string()
                ),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            
            // 必須フィールドチェック
            ValidationRule {
                id: Uuid::new_v4(),
                name: "文書必須フィールド".to_string(),
                description: "文書の必須フィールドが空でないかチェック".to_string(),
                rule_type: ValidationRuleType::MandatoryFields,
                severity: ValidationSeverity::Critical,
                target_table: "documents".to_string(),
                target_field: None,
                sql_query: Some(
                    r#"
                    SELECT id, number, title, document_type_id, created_by 
                    FROM documents 
                    WHERE (title IS NULL OR TRIM(title) = '' 
                           OR number IS NULL OR TRIM(number) = ''
                           OR document_type_id IS NULL 
                           OR created_by IS NULL) 
                          AND is_active = 1
                    "#.to_string()
                ),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            
            // 重複チェック
            ValidationRule {
                id: Uuid::new_v4(),
                name: "文書番号重複チェック".to_string(),
                description: "同じ文書番号の文書が複数存在しないかチェック".to_string(),
                rule_type: ValidationRuleType::DuplicateCheck,
                severity: ValidationSeverity::Critical,
                target_table: "documents".to_string(),
                target_field: Some("number".to_string()),
                sql_query: Some(
                    r#"
                    SELECT number, COUNT(*) as count 
                    FROM documents 
                    WHERE is_active = 1 
                    GROUP BY number 
                    HAVING COUNT(*) > 1
                    "#.to_string()
                ),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            
            // ファイル存在確認
            ValidationRule {
                id: Uuid::new_v4(),
                name: "ネットワークパス存在確認".to_string(),
                description: "文書のnetwork_pathに指定されたファイルが存在するかチェック".to_string(),
                rule_type: ValidationRuleType::FileExistence,
                severity: ValidationSeverity::Warning,
                target_table: "documents".to_string(),
                target_field: Some("network_path".to_string()),
                sql_query: Some(
                    r#"
                    SELECT id, number, network_path 
                    FROM documents 
                    WHERE network_path IS NOT NULL 
                          AND TRIM(network_path) != '' 
                          AND is_active = 1
                    "#.to_string()
                ),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            
            // データ形式チェック
            ValidationRule {
                id: Uuid::new_v4(),
                name: "文書番号形式チェック".to_string(),
                description: "文書番号が正しい形式（XXX-NNNNNN）になっているかチェック".to_string(),
                rule_type: ValidationRuleType::DataFormat,
                severity: ValidationSeverity::Warning,
                target_table: "documents".to_string(),
                target_field: Some("number".to_string()),
                sql_query: Some(
                    r#"
                    SELECT id, number 
                    FROM documents 
                    WHERE is_active = 1 
                          AND (LENGTH(number) < 4 
                               OR number NOT LIKE '%-______' 
                               OR number NOT GLOB '*-[0-9][0-9][0-9][0-9][0-9][0-9]')
                    "#.to_string()
                ),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ]
    }

    /// 特定のルールを実行
    async fn execute_rule(
        &self,
        rule: &ValidationRule,
        target_record_id: Option<i32>,
    ) -> Result<Vec<ValidationError>, ValidationServiceError> {
        info!("Executing validation rule: {}", rule.name);

        let mut validation_errors = Vec::new();

        match rule.rule_type {
            ValidationRuleType::FileExistence => {
                validation_errors.extend(self.check_file_existence(rule, target_record_id).await?);
            }
            ValidationRuleType::ReferentialIntegrity
            | ValidationRuleType::MandatoryFields 
            | ValidationRuleType::DuplicateCheck
            | ValidationRuleType::DataFormat => {
                validation_errors.extend(self.execute_sql_rule(rule, target_record_id).await?);
            }
            ValidationRuleType::BusinessLogic => {
                validation_errors.extend(self.execute_business_logic_rule(rule, target_record_id).await?);
            }
        }

        Ok(validation_errors)
    }

    /// SQLベースのルールを実行（模擬実装）
    async fn execute_sql_rule(
        &self,
        rule: &ValidationRule,
        _target_record_id: Option<i32>,
    ) -> Result<Vec<ValidationError>, ValidationServiceError> {
        // TODO: 実際のデータベース接続で実行
        // 現在は模擬データを返す
        warn!("SQL rule execution is mocked: {}", rule.name);
        
        // 模擬エラーを少し生成
        let mut errors = Vec::new();
        
        if rule.rule_type == ValidationRuleType::MandatoryFields {
            errors.push(ValidationError {
                id: Uuid::new_v4(),
                rule_id: rule.id,
                target_table: rule.target_table.clone(),
                target_record_id: 1,
                field_name: Some("title".to_string()),
                error_message: "タイトルが空です".to_string(),
                expected_value: Some("非空の文字列".to_string()),
                actual_value: Some("".to_string()),
                severity: rule.severity.clone(),
                detected_at: Utc::now(),
                resolved_at: None,
                resolution_note: None,
            });
        }
        
        Ok(errors)
    }

    /// ファイル存在確認ルールを実行
    async fn check_file_existence(
        &self,
        rule: &ValidationRule,
        _target_record_id: Option<i32>,
    ) -> Result<Vec<ValidationError>, ValidationServiceError> {
        use std::path::Path;
        
        // TODO: 実際のデータベースから対象レコードを取得
        // 現在は模擬チェック
        let mut errors = Vec::new();
        
        // サンプルファイルパス
        let sample_paths = vec![
            r"\\server\docs\DOC-000001.pdf",
            r"\\server\docs\DOC-000002.pdf",
            r"\\server\docs\nonexistent.pdf",
        ];
        
        for (index, path_str) in sample_paths.iter().enumerate() {
            let path = Path::new(path_str);
            if !path.exists() {
                errors.push(ValidationError {
                    id: Uuid::new_v4(),
                    rule_id: rule.id,
                    target_table: rule.target_table.clone(),
                    target_record_id: index as i32 + 1,
                    field_name: rule.target_field.clone(),
                    error_message: format!("ファイルが存在しません: {}", path_str),
                    expected_value: Some("存在するファイルパス".to_string()),
                    actual_value: Some(path_str.to_string()),
                    severity: rule.severity.clone(),
                    detected_at: Utc::now(),
                    resolved_at: None,
                    resolution_note: None,
                });
            }
        }
        
        Ok(errors)
    }

    /// ビジネスロジックルールを実行
    async fn execute_business_logic_rule(
        &self,
        _rule: &ValidationRule,
        _target_record_id: Option<i32>,
    ) -> Result<Vec<ValidationError>, ValidationServiceError> {
        // ビジネス固有のロジックをここに実装
        // 例：承認が必要な文書で承認ファイルが存在するかチェック
        Ok(vec![])
    }

    /// 検証統計を集計
    fn calculate_statistics(&self, validation_errors: &[ValidationError]) -> HashMap<String, i32> {
        let mut stats = HashMap::new();
        
        stats.insert("total_errors".to_string(), validation_errors.len() as i32);
        
        let critical_count = validation_errors
            .iter()
            .filter(|e| e.severity == ValidationSeverity::Critical)
            .count() as i32;
        stats.insert("critical_errors".to_string(), critical_count);
        
        let warning_count = validation_errors
            .iter()
            .filter(|e| e.severity == ValidationSeverity::Warning)
            .count() as i32;
        stats.insert("warning_errors".to_string(), warning_count);
        
        let info_count = validation_errors
            .iter()
            .filter(|e| e.severity == ValidationSeverity::Info)
            .count() as i32;
        stats.insert("info_errors".to_string(), info_count);
        
        stats
    }
}

#[async_trait]
impl ValidationService for ValidationServiceImpl {
    async fn execute_validation(
        &self,
        request: ValidationExecutionRequest,
    ) -> Result<ValidationResult, ValidationServiceError> {
        let execution_id = Uuid::new_v4();
        let started_at = Utc::now();
        
        info!(
            "Starting validation execution: {} (rules: {:?}, table: {:?})",
            execution_id,
            request.rule_ids.as_ref().map(|r| r.len()).unwrap_or(0),
            request.target_table
        );

        // 実行するルールを決定
        let rules_to_execute: Vec<&ValidationRule> = if let Some(rule_ids) = &request.rule_ids {
            self.built_in_rules
                .iter()
                .filter(|rule| rule_ids.contains(&rule.id))
                .collect()
        } else {
            self.built_in_rules
                .iter()
                .filter(|rule| {
                    rule.is_active &&
                    request.target_table.as_ref().map_or(true, |table| rule.target_table == *table)
                })
                .collect()
        };

        if rules_to_execute.is_empty() {
            return Err(ValidationServiceError::InvalidConfiguration(
                "No active rules found for execution".to_string(),
            ));
        }

        let mut all_validation_errors = Vec::new();
        let mut total_records_checked = 0;

        // 各ルールを実行
        for rule in &rules_to_execute {
            match self.execute_rule(rule, request.target_record_id).await {
                Ok(mut errors) => {
                    // 重要度フィルタリング
                    errors.retain(|error| {
                        match error.severity {
                            ValidationSeverity::Critical => true,
                            ValidationSeverity::Warning => request.include_warnings,
                            ValidationSeverity::Info => request.include_info,
                        }
                    });
                    
                    total_records_checked += errors.len() as i32;
                    all_validation_errors.extend(errors);
                }
                Err(e) => {
                    error!("Rule execution failed for {}: {}", rule.name, e);
                    // ルールエラーもValidationErrorとして追加
                    all_validation_errors.push(ValidationError {
                        id: Uuid::new_v4(),
                        rule_id: rule.id,
                        target_table: rule.target_table.clone(),
                        target_record_id: 0,
                        field_name: None,
                        error_message: format!("ルール実行エラー: {}", e),
                        expected_value: None,
                        actual_value: None,
                        severity: ValidationSeverity::Critical,
                        detected_at: Utc::now(),
                        resolved_at: None,
                        resolution_note: None,
                    });
                }
            }
        }

        // 統計計算
        let stats = self.calculate_statistics(&all_validation_errors);

        let mut result = ValidationResult {
            id: Uuid::new_v4(),
            execution_id,
            started_at,
            completed_at: None,
            total_rules_executed: rules_to_execute.len() as i32,
            total_records_checked,
            critical_errors: stats.get("critical_errors").copied().unwrap_or(0),
            warnings: stats.get("warning_errors").copied().unwrap_or(0),
            info_messages: stats.get("info_errors").copied().unwrap_or(0),
            validation_errors: all_validation_errors,
            status: ValidationStatus::Running,
            summary: None,
        };

        result.complete();

        info!(
            "Validation execution completed: {} (Total errors: {}, Critical: {}, Warnings: {}, Info: {})",
            execution_id,
            result.validation_errors.len(),
            result.critical_errors,
            result.warnings,
            result.info_messages
        );

        Ok(result)
    }

    async fn get_active_rules(&self) -> Result<Vec<ValidationRule>, ValidationServiceError> {
        Ok(self.built_in_rules
            .iter()
            .filter(|rule| rule.is_active)
            .cloned()
            .collect())
    }

    async fn get_rule(&self, rule_id: Uuid) -> Result<ValidationRule, ValidationServiceError> {
        self.built_in_rules
            .iter()
            .find(|rule| rule.id == rule_id)
            .cloned()
            .ok_or(ValidationServiceError::RuleNotFound(rule_id))
    }

    async fn get_validation_result(
        &self,
        _result_id: Uuid,
    ) -> Result<ValidationResult, ValidationServiceError> {
        // TODO: 実際のストレージから取得
        Err(ValidationServiceError::ExecutionFailed(
            "Result storage not implemented".to_string(),
        ))
    }

    async fn resolve_validation_error(
        &self,
        _error_id: Uuid,
        _resolution_note: String,
    ) -> Result<(), ValidationServiceError> {
        // TODO: 実際のストレージに保存
        info!("Validation error marked as resolved (mock implementation)");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validation_service_creation() {
        let service = ValidationServiceImpl::new();
        let rules = service.get_active_rules().await.unwrap();
        
        assert!(!rules.is_empty());
        assert!(rules.iter().all(|rule| rule.is_active));
    }

    #[tokio::test]
    async fn test_execute_validation_all_rules() {
        let service = ValidationServiceImpl::new();
        let request = ValidationExecutionRequest::default();
        
        let result = service.execute_validation(request).await.unwrap();
        
        assert_eq!(result.status, ValidationStatus::Completed);
        assert!(result.total_rules_executed > 0);
        assert!(result.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_execute_validation_specific_rules() {
        let service = ValidationServiceImpl::new();
        let rules = service.get_active_rules().await.unwrap();
        let rule_id = rules[0].id;
        
        let request = ValidationExecutionRequest {
            rule_ids: Some(vec![rule_id]),
            target_table: None,
            target_record_id: None,
            include_warnings: true,
            include_info: true,
        };
        
        let result = service.execute_validation(request).await.unwrap();
        
        assert_eq!(result.total_rules_executed, 1);
        assert_eq!(result.status, ValidationStatus::Completed);
    }

    #[tokio::test]
    async fn test_quality_score_calculation() {
        let service = ValidationServiceImpl::new();
        let request = ValidationExecutionRequest::default();
        
        let result = service.execute_validation(request).await.unwrap();
        let quality_score = result.calculate_quality_score();
        
        assert!(quality_score >= 0.0 && quality_score <= 100.0);
    }
}