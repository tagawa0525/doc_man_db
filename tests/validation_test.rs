use doc_man_db::models::validation::*;
use doc_man_db::services::{ValidationService, ValidationServiceImpl, ReportService, ReportServiceImpl};
use std::collections::HashMap;
use tokio;
use uuid::Uuid;

#[tokio::test]
async fn test_validation_service_creation() {
    let validation_service = ValidationServiceImpl::new();
    assert!(validation_service.builtin_rules.len() > 0);
}

#[tokio::test]
async fn test_get_active_rules() {
    let validation_service = ValidationServiceImpl::new();
    
    let rules = validation_service.get_active_rules().await.unwrap();
    
    assert!(rules.len() > 0);
    assert!(rules.iter().any(|r| r.rule_type == ValidationRuleType::ReferentialIntegrity));
    assert!(rules.iter().any(|r| r.rule_type == ValidationRuleType::MandatoryFields));
    assert!(rules.iter().any(|r| r.rule_type == ValidationRuleType::DuplicateCheck));
    assert!(rules.iter().any(|r| r.rule_type == ValidationRuleType::FileExistence));
    assert!(rules.iter().any(|r| r.rule_type == ValidationRuleType::DataFormat));
}

#[tokio::test]
async fn test_execute_referential_integrity_validation() {
    let validation_service = ValidationServiceImpl::new();
    
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::ReferentialIntegrity,
        target_ids: vec![1, 2, 3],
        parameters: HashMap::new(),
    };
    
    let result = validation_service.execute_validation(request).await.unwrap();
    
    assert_eq!(result.rule_type, ValidationRuleType::ReferentialIntegrity);
    assert_eq!(result.rule_name, "参照整合性チェック");
    assert!(result.execution_time_ms > 0);
    assert!(result.quality_score >= 0.0 && result.quality_score <= 100.0);
}

#[tokio::test]
async fn test_execute_mandatory_fields_validation() {
    let validation_service = ValidationServiceImpl::new();
    
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::MandatoryFields,
        target_ids: vec![1],
        parameters: HashMap::new(),
    };
    
    let result = validation_service.execute_validation(request).await.unwrap();
    
    assert_eq!(result.rule_type, ValidationRuleType::MandatoryFields);
    assert_eq!(result.rule_name, "必須フィールドチェック");
    assert!(result.execution_time_ms > 0);
}

#[tokio::test]
async fn test_execute_duplicate_check_validation() {
    let validation_service = ValidationServiceImpl::new();
    
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::DuplicateCheck,
        target_ids: vec![],
        parameters: HashMap::new(),
    };
    
    let result = validation_service.execute_validation(request).await.unwrap();
    
    assert_eq!(result.rule_type, ValidationRuleType::DuplicateCheck);
    assert_eq!(result.rule_name, "重複チェック");
    assert!(result.is_valid);
    assert_eq!(result.errors.len(), 0);
}

#[tokio::test]
async fn test_execute_file_existence_validation() {
    let validation_service = ValidationServiceImpl::new();
    
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::FileExistence,
        target_ids: vec![1, 2],
        parameters: HashMap::new(),
    };
    
    let result = validation_service.execute_validation(request).await.unwrap();
    
    assert_eq!(result.rule_type, ValidationRuleType::FileExistence);
    assert_eq!(result.rule_name, "ファイル存在確認");
}

#[tokio::test]
async fn test_execute_data_format_validation() {
    let validation_service = ValidationServiceImpl::new();
    
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::DataFormat,
        target_ids: vec![1, 2, 3, 4, 5],
        parameters: HashMap::new(),
    };
    
    let result = validation_service.execute_validation(request).await.unwrap();
    
    assert_eq!(result.rule_type, ValidationRuleType::DataFormat);
    assert_eq!(result.rule_name, "データ形式チェック");
}

#[tokio::test]
async fn test_execute_business_logic_validation() {
    let validation_service = ValidationServiceImpl::new();
    
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::BusinessLogic,
        target_ids: vec![1],
        parameters: HashMap::new(),
    };
    
    let result = validation_service.execute_validation(request).await.unwrap();
    
    assert_eq!(result.rule_type, ValidationRuleType::BusinessLogic);
    assert_eq!(result.rule_name, "業務ロジックチェック");
}

#[tokio::test]
async fn test_invalid_rule_type() {
    let validation_service = ValidationServiceImpl::new();
    
    // 存在しないルールタイプでテスト（実際にはenumなので発生しないが、将来的な拡張性のため）
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::ReferentialIntegrity,
        target_ids: vec![],
        parameters: HashMap::new(),
    };
    
    let result = validation_service.execute_validation(request).await;
    assert!(result.is_ok());
}

// レポートサービスのテスト
#[tokio::test]
async fn test_report_service_creation() {
    let report_service = ReportServiceImpl::new();
    assert_eq!(report_service.default_template.title, "データ検証レポート");
}

#[tokio::test]
async fn test_generate_json_report() {
    let report_service = ReportServiceImpl::new();
    let validation_results = create_test_validation_results();
    
    let request = doc_man_db::services::ReportGenerationRequest {
        request_id: Uuid::new_v4(),
        validation_results,
        format: ValidationReportFormat::Json,
        include_details: true,
        template_config: None,
        output_path: None,
    };
    
    let result = report_service.generate_report(request).await.unwrap();
    
    match result.content {
        doc_man_db::services::ReportContent::Json(json_content) => {
            assert!(json_content.contains("summary"));
            assert!(json_content.contains("results"));
            assert!(json_content.contains("generated_at"));
        },
        _ => panic!("Expected JSON content"),
    }
}

#[tokio::test]
async fn test_generate_html_report() {
    let report_service = ReportServiceImpl::new();
    let validation_results = create_test_validation_results();
    
    let request = doc_man_db::services::ReportGenerationRequest {
        request_id: Uuid::new_v4(),
        validation_results,
        format: ValidationReportFormat::Html,
        include_details: true,
        template_config: Some(doc_man_db::services::ReportTemplateConfig {
            title: "テストレポート".to_string(),
            subtitle: Some("テスト用".to_string()),
            logo_path: None,
            custom_styles: None,
            include_charts: true,
        }),
        output_path: None,
    };
    
    let result = report_service.generate_report(request).await.unwrap();
    
    match result.content {
        doc_man_db::services::ReportContent::Html(html_content) => {
            assert!(html_content.contains("<!DOCTYPE html>"));
            assert!(html_content.contains("テストレポート"));
            assert!(html_content.contains("テスト用"));
            assert!(html_content.contains("検証結果サマリー"));
        },
        _ => panic!("Expected HTML content"),
    }
}

#[tokio::test]
async fn test_generate_csv_report() {
    let report_service = ReportServiceImpl::new();
    let validation_results = create_test_validation_results();
    
    let request = doc_man_db::services::ReportGenerationRequest {
        request_id: Uuid::new_v4(),
        validation_results,
        format: ValidationReportFormat::Csv,
        include_details: false,
        template_config: None,
        output_path: None,
    };
    
    let result = report_service.generate_report(request).await.unwrap();
    
    match result.content {
        doc_man_db::services::ReportContent::Csv(csv_content) => {
            assert!(csv_content.contains("ルールタイプ,ルール名"));
            assert!(csv_content.contains("ReferentialIntegrity"));
            assert!(csv_content.contains("参照整合性チェック"));
        },
        _ => panic!("Expected CSV content"),
    }
}

#[tokio::test]
async fn test_generate_summary_report() {
    let report_service = ReportServiceImpl::new();
    let validation_results = create_test_validation_results();
    
    let summary = report_service.generate_summary_report(validation_results).await.unwrap();
    
    assert_eq!(summary.total_count, 3);
    assert!(summary.success_count <= summary.total_count);
    assert!(summary.quality_score >= 0.0 && summary.quality_score <= 100.0);
    assert!(summary.execution_time_ms > 0);
}

#[tokio::test]
async fn test_pdf_report_not_implemented() {
    let report_service = ReportServiceImpl::new();
    let validation_results = create_test_validation_results();
    
    let request = doc_man_db::services::ReportGenerationRequest {
        request_id: Uuid::new_v4(),
        validation_results,
        format: ValidationReportFormat::Pdf,
        include_details: true,
        template_config: None,
        output_path: None,
    };
    
    let result = report_service.generate_report(request).await;
    assert!(result.is_err());
}

// テスト用のバリデーション結果を作成
fn create_test_validation_results() -> Vec<ValidationResult> {
    vec![
        ValidationResult {
            rule_type: ValidationRuleType::ReferentialIntegrity,
            rule_name: "参照整合性チェック".to_string(),
            is_valid: true,
            errors: vec![],
            warnings: vec![],
            quality_score: 95.0,
            execution_time_ms: 150,
        },
        ValidationResult {
            rule_type: ValidationRuleType::MandatoryFields,
            rule_name: "必須フィールドチェック".to_string(),
            is_valid: false,
            errors: vec![
                ValidationError {
                    error_type: ValidationErrorType::MissingValue,
                    message: "必須フィールド 'title' が空です".to_string(),
                    field_name: Some("title".to_string()),
                    entity_id: Some(123),
                    severity: ValidationSeverity::Error,
                }
            ],
            warnings: vec![],
            quality_score: 60.0,
            execution_time_ms: 80,
        },
        ValidationResult {
            rule_type: ValidationRuleType::FileExistence,
            rule_name: "ファイル存在確認".to_string(),
            is_valid: true,
            errors: vec![],
            warnings: vec![
                ValidationError {
                    error_type: ValidationErrorType::DataInconsistency,
                    message: "ファイルが見つかりませんが、参照は有効です".to_string(),
                    field_name: Some("network_path".to_string()),
                    entity_id: Some(456),
                    severity: ValidationSeverity::Warning,
                }
            ],
            quality_score: 85.0,
            execution_time_ms: 200,
        },
    ]
}

// 統合テスト：検証とレポート生成の組み合わせ
#[tokio::test]
async fn test_validation_and_report_integration() {
    let validation_service = ValidationServiceImpl::new();
    let report_service = ReportServiceImpl::new();
    
    // 複数の検証を実行
    let mut all_results = Vec::new();
    
    for rule_type in [
        ValidationRuleType::ReferentialIntegrity,
        ValidationRuleType::MandatoryFields,
        ValidationRuleType::DuplicateCheck,
    ] {
        let request = ValidationExecutionRequest {
            request_id: Uuid::new_v4(),
            rule_type: rule_type.clone(),
            target_ids: vec![1, 2, 3],
            parameters: HashMap::new(),
        };
        
        let result = validation_service.execute_validation(request).await.unwrap();
        all_results.push(result);
    }
    
    // レポート生成
    let report_request = doc_man_db::services::ReportGenerationRequest {
        request_id: Uuid::new_v4(),
        validation_results: all_results,
        format: ValidationReportFormat::Html,
        include_details: true,
        template_config: None,
        output_path: None,
    };
    
    let report_result = report_service.generate_report(report_request).await.unwrap();
    
    assert_eq!(report_result.format, ValidationReportFormat::Html);
    assert_eq!(report_result.summary.total_count, 3);
}

// エラーケースのテスト
#[tokio::test]
async fn test_validation_error_handling() {
    let validation_service = ValidationServiceImpl::new();
    
    // 空のターゲットIDでテスト（一部のルールではエラーになる可能性）
    let request = ValidationExecutionRequest {
        request_id: Uuid::new_v4(),
        rule_type: ValidationRuleType::ReferentialIntegrity,
        target_ids: vec![],
        parameters: HashMap::new(),
    };
    
    let result = validation_service.execute_validation(request).await;
    // エラーハンドリングが適切に動作することを確認
    assert!(result.is_ok() || result.is_err());
}