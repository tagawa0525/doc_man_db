use crate::models::validation::{
    ValidationExecutionRequest, ValidationReportFormat, ValidationResult, ValidationSummary,
};
use crate::services::{ReportGenerationRequest, ReportService, ValidationService};
use axum::{
    Json as JsonBody,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// 検証実行レスポンス
#[derive(Debug, Serialize)]
pub struct ValidationExecutionResponse {
    pub request_id: Uuid,
    pub result: ValidationResult,
    pub summary: ValidationSummary,
}

/// 検証ルール一覧レスポンス
#[derive(Debug, Serialize)]
pub struct ValidationRulesResponse {
    pub rules: Vec<crate::models::validation::ValidationRule>,
    pub total_count: usize,
}

/// レポート生成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateReportRequest {
    pub validation_result_ids: Vec<Uuid>,
    pub format: ValidationReportFormat,
    pub include_details: Option<bool>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
}

/// レポート生成レスポンス
#[derive(Debug, Serialize)]
pub struct CreateReportResponse {
    pub report_id: Uuid,
    pub download_url: String,
    pub format: ValidationReportFormat,
    pub summary: ValidationSummary,
}

/// バッチ検証リクエスト
#[derive(Debug, Deserialize)]
pub struct BatchValidationRequest {
    pub rule_types: Option<Vec<crate::models::validation::ValidationRuleType>>,
    pub target_document_ids: Option<Vec<i32>>,
    pub parallel_execution: Option<bool>,
}

/// バッチ検証レスポンス
#[derive(Debug, Serialize)]
pub struct BatchValidationResponse {
    pub batch_id: Uuid,
    pub results: Vec<ValidationResult>,
    pub summary: ValidationSummary,
    pub execution_time_ms: u64,
}

/// クエリパラメータ
#[derive(Debug, Deserialize)]
pub struct ValidationQueryParams {
    pub rule_type: Option<crate::models::validation::ValidationRuleType>,
    pub include_inactive: Option<bool>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub struct ValidationHandlers {
    validation_service: Arc<dyn ValidationService>,
    report_service: Arc<dyn ReportService>,
}

impl ValidationHandlers {
    pub fn new(
        validation_service: Arc<dyn ValidationService>,
        report_service: Arc<dyn ReportService>,
    ) -> Self {
        Self {
            validation_service,
            report_service,
        }
    }

    /// 検証実行
    pub async fn execute_validation(
        State(_handlers): State<Arc<ValidationHandlers>>,
        JsonBody(request): JsonBody<ValidationExecutionRequest>,
    ) -> Result<Json<ValidationExecutionResponse>, StatusCode> {
        let result = _handlers
            .validation_service
            .execute_validation(request.clone())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let summary = _handlers
            .report_service
            .generate_summary_report(vec![result.clone()])
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ValidationExecutionResponse {
            request_id: request.request_id,
            result,
            summary,
        }))
    }

    /// バッチ検証実行
    pub async fn execute_batch_validation(
        State(_handlers): State<Arc<ValidationHandlers>>,
        JsonBody(request): JsonBody<BatchValidationRequest>,
    ) -> Result<Json<BatchValidationResponse>, StatusCode> {
        let batch_id = Uuid::new_v4();
        let start_time = std::time::Instant::now();

        // 実行するルールを取得
        let active_rules = _handlers
            .validation_service
            .get_active_rules()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut results = Vec::new();

        // 指定されたルールタイプでフィルタリング
        let filtered_rules = if let Some(ref rule_types) = request.rule_types {
            active_rules
                .into_iter()
                .filter(|rule| rule_types.contains(&rule.rule_type))
                .collect()
        } else {
            active_rules
        };

        // 各ルールを実行
        for rule in filtered_rules {
            let validation_request = ValidationExecutionRequest {
                request_id: Uuid::new_v4(),
                rule_type: rule.rule_type.clone(),
                target_ids: request.target_document_ids.clone().unwrap_or_default(),
                parameters: HashMap::new(),
            };

            match _handlers
                .validation_service
                .execute_validation(validation_request)
                .await
            {
                Ok(result) => results.push(result),
                Err(_) => continue, // エラーは無視して継続
            }
        }

        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        let summary = _handlers
            .report_service
            .generate_summary_report(results.clone())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(BatchValidationResponse {
            batch_id,
            results,
            summary,
            execution_time_ms,
        }))
    }

    /// アクティブな検証ルール一覧取得
    pub async fn get_validation_rules(
        State(_handlers): State<Arc<ValidationHandlers>>,
        Query(params): Query<ValidationQueryParams>,
    ) -> Result<Json<ValidationRulesResponse>, StatusCode> {
        let mut rules = _handlers
            .validation_service
            .get_active_rules()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // ルールタイプでフィルタリング
        if let Some(rule_type) = params.rule_type {
            rules.retain(|rule| rule.rule_type == rule_type);
        }

        // 非アクティブルールを含める場合の処理（将来的な拡張）
        if params.include_inactive.unwrap_or(false) {
            // 現在は全てアクティブルールのみ
        }

        // ページネーション
        let total_count = rules.len();
        if let Some(offset) = params.offset {
            if offset < rules.len() {
                rules = rules.into_iter().skip(offset).collect();
            } else {
                rules.clear();
            }
        }

        if let Some(limit) = params.limit {
            rules.truncate(limit);
        }

        Ok(Json(ValidationRulesResponse { rules, total_count }))
    }

    /// レポート生成
    pub async fn create_report(
        State(_handlers): State<Arc<ValidationHandlers>>,
        JsonBody(request): JsonBody<CreateReportRequest>,
    ) -> Result<Json<CreateReportResponse>, StatusCode> {
        // 実際の実装では、validation_result_idsから検証結果を取得する
        // 現在はモックデータを使用
        let validation_results = Vec::new(); // 実装時に実際のデータを取得

        let report_request = ReportGenerationRequest {
            request_id: Uuid::new_v4(),
            validation_results,
            format: request.format.clone(),
            include_details: request.include_details.unwrap_or(true),
            template_config: Some(crate::services::ReportTemplateConfig {
                title: request
                    .title
                    .unwrap_or_else(|| "データ検証レポート".to_string()),
                subtitle: request.subtitle,
                logo_path: None,
                custom_styles: None,
                include_charts: true,
            }),
            output_path: None,
        };

        let report_result = _handlers
            .report_service
            .generate_report(report_request)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let download_url = format!("/api/reports/{}/download", report_result.report_id);

        Ok(Json(CreateReportResponse {
            report_id: report_result.report_id,
            download_url,
            format: request.format,
            summary: report_result.summary,
        }))
    }

    /// レポートダウンロード
    pub async fn download_report(
        State(_handlers): State<Arc<ValidationHandlers>>,
        Path(report_id): Path<Uuid>,
    ) -> Result<String, StatusCode> {
        // 実装時には実際のレポートファイルを返す
        // 現在はプレースホルダー
        Ok(format!(
            "レポート {} のダウンロード機能は開発中です",
            report_id
        ))
    }

    /// 検証統計情報取得
    pub async fn get_validation_statistics(
        State(_handlers): State<Arc<ValidationHandlers>>,
    ) -> Result<Json<ValidationSummary>, StatusCode> {
        // 実装時には実際の統計データを計算
        // 現在はサンプルデータ
        let sample_summary = ValidationSummary {
            total_count: 0,
            success_count: 0,
            warning_count: 0,
            error_count: 0,
            quality_score: 0.0,
            execution_time_ms: 0,
        };

        Ok(Json(sample_summary))
    }
}
