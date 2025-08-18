use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// データ検証ルールタイプ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationRuleType {
    ReferentialIntegrity,   // 参照整合性
    FileExistence,         // ファイル存在確認
    DuplicateCheck,        // 重複チェック
    MandatoryFields,       // 必須フィールド
    DataFormat,            // データ形式
    BusinessLogic,         // ビジネスロジック
}

/// 検証重要度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Critical,    // クリティカル（必須修正）
    Warning,     // 警告（推奨修正）
    Info,        // 情報（参考）
}

/// データ検証ルール
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rule_type: ValidationRuleType,
    pub severity: ValidationSeverity,
    pub target_table: String,
    pub target_field: Option<String>,
    pub sql_query: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// データ検証エラー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub target_table: String,
    pub target_record_id: i32,
    pub field_name: Option<String>,
    pub error_message: String,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
    pub severity: ValidationSeverity,
    pub detected_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_note: Option<String>,
}

/// データ検証結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub id: Uuid,
    pub execution_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_rules_executed: i32,
    pub total_records_checked: i32,
    pub critical_errors: i32,
    pub warnings: i32,
    pub info_messages: i32,
    pub validation_errors: Vec<ValidationError>,
    pub status: ValidationStatus,
    pub summary: Option<String>,
}

/// 検証ステータス
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Running,     // 実行中
    Completed,   // 完了
    Failed,      // 失敗
    Cancelled,   // キャンセル
}

/// データ検証実行リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct ValidationExecutionRequest {
    pub rule_ids: Option<Vec<Uuid>>,  // 実行するルールID（なしは全ルール）
    pub target_table: Option<String>, // 対象テーブル（なしは全テーブル）
    pub target_record_id: Option<i32>, // 特定レコードの検証
    pub include_warnings: bool,        // 警告も含める
    pub include_info: bool,           // 情報も含める
}

/// 検証レポートフォーマット
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportFormat {
    Json,
    Html,
    Csv,
    Pdf,
}

/// 検証レポート生成リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct ValidationReportRequest {
    pub validation_result_id: Uuid,
    pub format: ReportFormat,
    pub include_resolved: bool,         // 解決済みエラーも含める
    pub severity_filter: Option<Vec<ValidationSeverity>>, // 重要度フィルター
    pub table_filter: Option<Vec<String>>, // テーブルフィルター
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
    pub data_quality_score: f64, // 0-100のスコア
}

impl Default for ValidationExecutionRequest {
    fn default() -> Self {
        Self {
            rule_ids: None,
            target_table: None,
            target_record_id: None,
            include_warnings: true,
            include_info: false,
        }
    }
}

impl ValidationResult {
    /// データ品質スコアを計算
    pub fn calculate_quality_score(&self) -> f64 {
        if self.total_records_checked == 0 {
            return 100.0;
        }

        let total_issues = self.critical_errors + self.warnings + self.info_messages;
        if total_issues == 0 {
            return 100.0;
        }

        // 重み付けスコア計算（クリティカルは重く、情報は軽く）
        let weighted_issues = (self.critical_errors as f64 * 3.0) 
            + (self.warnings as f64 * 1.5) 
            + (self.info_messages as f64 * 0.5);
        
        let error_rate = weighted_issues / (self.total_records_checked as f64);
        let score = (1.0 - error_rate.min(1.0)) * 100.0;
        score.max(0.0).min(100.0)
    }

    /// 検証完了
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
        self.status = ValidationStatus::Completed;
        
        // サマリー生成
        self.summary = Some(format!(
            "検証完了: {} ルール実行, {} レコード検証, {} クリティカル, {} 警告, {} 情報",
            self.total_rules_executed,
            self.total_records_checked,
            self.critical_errors,
            self.warnings,
            self.info_messages
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_score_calculation() {
        let mut result = ValidationResult {
            id: Uuid::new_v4(),
            execution_id: Uuid::new_v4(),
            started_at: Utc::now(),
            completed_at: None,
            total_rules_executed: 10,
            total_records_checked: 100,
            critical_errors: 2,
            warnings: 5,
            info_messages: 3,
            validation_errors: vec![],
            status: ValidationStatus::Running,
            summary: None,
        };

        let score = result.calculate_quality_score();
        assert!(score >= 0.0 && score <= 100.0);
        
        // エラーなしの場合は100点
        result.critical_errors = 0;
        result.warnings = 0;
        result.info_messages = 0;
        assert_eq!(result.calculate_quality_score(), 100.0);
    }

    #[test]
    fn test_validation_result_complete() {
        let mut result = ValidationResult {
            id: Uuid::new_v4(),
            execution_id: Uuid::new_v4(),
            started_at: Utc::now(),
            completed_at: None,
            total_rules_executed: 5,
            total_records_checked: 50,
            critical_errors: 1,
            warnings: 2,
            info_messages: 0,
            validation_errors: vec![],
            status: ValidationStatus::Running,
            summary: None,
        };

        result.complete();

        assert_eq!(result.status, ValidationStatus::Completed);
        assert!(result.completed_at.is_some());
        assert!(result.summary.is_some());
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
}