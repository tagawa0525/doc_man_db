use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// データ検証ルールタイプ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationRuleType {
    ReferentialIntegrity, // 参照整合性
    FileExistence,        // ファイル存在確認
    DuplicateCheck,       // 重複チェック
    MandatoryFields,      // 必須フィールド
    DataFormat,           // データ形式
    BusinessLogic,        // ビジネスロジック
}

impl std::fmt::Display for ValidationRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationRuleType::ReferentialIntegrity => write!(f, "ReferentialIntegrity"),
            ValidationRuleType::FileExistence => write!(f, "FileExistence"),
            ValidationRuleType::DuplicateCheck => write!(f, "DuplicateCheck"),
            ValidationRuleType::MandatoryFields => write!(f, "MandatoryFields"),
            ValidationRuleType::DataFormat => write!(f, "DataFormat"),
            ValidationRuleType::BusinessLogic => write!(f, "BusinessLogic"),
        }
    }
}

/// 検証エラータイプ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationErrorType {
    MissingValue,          // 値不足
    InvalidFormat,         // 不正な形式
    DataInconsistency,     // データ不整合
    ReferenceError,        // 参照エラー
    BusinessRuleViolation, // ビジネスルール違反
    DuplicateEntry,        // 重複エントリ
}

/// 検証重要度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,   // エラー（修正必要）
    Warning, // 警告（推奨修正）
    Info,    // 情報（参考）
}

/// データ検証エラー
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[error("{message}")]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub message: String,
    pub field_name: Option<String>,
    pub entity_id: Option<i32>,
    pub severity: ValidationSeverity,
}

/// データ検証ルール
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rule_type: ValidationRuleType,
    pub severity: ValidationSeverity,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// データ検証結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub rule_type: ValidationRuleType,
    pub rule_name: String,
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
    pub quality_score: f64,
    pub execution_time_ms: u64,
}

/// データ検証実行リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationExecutionRequest {
    pub request_id: Uuid,
    pub rule_type: ValidationRuleType,
    pub target_ids: Vec<i32>,
    pub parameters: HashMap<String, String>,
}

/// 検証レポートフォーマット
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationReportFormat {
    Json,
    Html,
    Csv,
    Pdf,
}

/// 検証サマリー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub total_count: usize,
    pub success_count: usize,
    pub warning_count: usize,
    pub error_count: usize,
    pub quality_score: f64,
    pub execution_time_ms: u64,
}

/// 検証レポート生成リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct ValidationReportRequest {
    pub validation_result_ids: Vec<Uuid>,
    pub format: ValidationReportFormat,
    pub include_resolved: bool,
    pub severity_filter: Option<Vec<ValidationSeverity>>,
    pub table_filter: Option<Vec<String>>,
}

/// データ検証統計
#[derive(Debug, Clone, Serialize)]
pub struct ValidationStatistics {
    pub total_validations: i32,
    pub last_validation: Option<DateTime<Utc>>,
    pub active_critical_errors: i32,
    pub active_warnings: i32,
    pub resolved_errors_last_30_days: i32,
    pub most_frequent_error_types: Vec<ValidationRuleType>,
    pub tables_with_most_errors: Vec<String>,
    pub data_quality_score: f64,
}

/// 検証ステータス
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Pending,   // 保留中
    Running,   // 実行中
    Completed, // 完了
    Failed,    // 失敗
    Cancelled, // キャンセル
}

impl Default for ValidationExecutionRequest {
    fn default() -> Self {
        Self {
            request_id: Uuid::new_v4(),
            rule_type: ValidationRuleType::ReferentialIntegrity,
            target_ids: vec![],
            parameters: HashMap::new(),
        }
    }
}

impl ValidationResult {
    pub fn new(rule_type: ValidationRuleType, rule_name: String) -> Self {
        Self {
            rule_type,
            rule_name,
            is_valid: true,
            errors: vec![],
            warnings: vec![],
            quality_score: 100.0,
            execution_time_ms: 0,
        }
    }

    /// データ品質スコアを計算
    pub fn calculate_quality_score(&self) -> f64 {
        if self.errors.is_empty() && self.warnings.is_empty() {
            return 100.0;
        }

        // エラーと警告に基づいてスコアを計算
        let error_weight = 10.0;
        let warning_weight = 3.0;

        let penalty = (self.errors.len() as f64 * error_weight)
            + (self.warnings.len() as f64 * warning_weight);

        let max_penalty = 100.0;
        ((max_penalty - penalty) / max_penalty * 100.0).max(0.0)
    }

    /// 検証を無効としてマーク
    pub fn mark_invalid(&mut self) {
        self.is_valid = false;
        self.quality_score = self.calculate_quality_score();
    }

    /// エラーを追加
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        if !self.errors.is_empty() {
            self.mark_invalid();
        }
    }

    /// 警告を追加
    pub fn add_warning(&mut self, warning: ValidationError) {
        self.warnings.push(warning);
        self.quality_score = self.calculate_quality_score();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_new() {
        let result = ValidationResult::new(
            ValidationRuleType::ReferentialIntegrity,
            "Test Rule".to_string(),
        );

        assert_eq!(result.rule_type, ValidationRuleType::ReferentialIntegrity);
        assert_eq!(result.rule_name, "Test Rule");
        assert!(result.is_valid);
        assert_eq!(result.errors.len(), 0);
        assert_eq!(result.warnings.len(), 0);
        assert_eq!(result.quality_score, 100.0);
    }

    #[test]
    fn test_quality_score_calculation() {
        let mut result =
            ValidationResult::new(ValidationRuleType::MandatoryFields, "Test".to_string());

        // エラー追加後のスコアテスト
        result.add_error(ValidationError {
            error_type: ValidationErrorType::MissingValue,
            message: "Test error".to_string(),
            field_name: Some("test_field".to_string()),
            entity_id: Some(1),
            severity: ValidationSeverity::Error,
        });

        assert!(!result.is_valid);
        assert!(result.quality_score < 100.0);
    }

    #[test]
    fn test_validation_rule_types() {
        let rule_types = vec![
            ValidationRuleType::ReferentialIntegrity,
            ValidationRuleType::FileExistence,
            ValidationRuleType::DuplicateCheck,
            ValidationRuleType::MandatoryFields,
            ValidationRuleType::DataFormat,
            ValidationRuleType::BusinessLogic,
        ];

        for rule_type in rule_types {
            let json = serde_json::to_string(&rule_type).unwrap();
            let deserialized: ValidationRuleType = serde_json::from_str(&json).unwrap();
            assert_eq!(rule_type, deserialized);
        }
    }

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError {
            error_type: ValidationErrorType::MissingValue,
            message: "Field is required".to_string(),
            field_name: Some("name".to_string()),
            entity_id: Some(123),
            severity: ValidationSeverity::Error,
        };

        assert_eq!(error.error_type, ValidationErrorType::MissingValue);
        assert_eq!(error.message, "Field is required");
        assert_eq!(error.field_name, Some("name".to_string()));
        assert_eq!(error.entity_id, Some(123));
        assert_eq!(error.severity, ValidationSeverity::Error);
    }
}
