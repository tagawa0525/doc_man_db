use crate::models::validation::{ValidationReportFormat, ValidationResult, ValidationSummary};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ReportServiceError {
    #[error("レポート生成エラー: {message}")]
    GenerationError { message: String },

    #[error("レポートフォーマットエラー: {format}")]
    FormatError { format: String },

    #[error("テンプレートエラー: {message}")]
    TemplateError { message: String },

    #[error("ファイル出力エラー: {message}")]
    FileOutputError { message: String },

    #[error("データ変換エラー: {message}")]
    ConversionError { message: String },
}

/// レポート生成リクエスト
#[derive(Debug, Clone)]
pub struct ReportGenerationRequest {
    pub request_id: Uuid,
    pub validation_results: Vec<ValidationResult>,
    pub format: ValidationReportFormat,
    pub include_details: bool,
    pub template_config: Option<ReportTemplateConfig>,
    pub output_path: Option<String>,
}

/// レポートテンプレート設定
#[derive(Debug, Clone)]
pub struct ReportTemplateConfig {
    pub title: String,
    pub subtitle: Option<String>,
    pub logo_path: Option<String>,
    pub custom_styles: Option<String>,
    pub include_charts: bool,
}

/// レポート生成結果
#[derive(Debug, Clone)]
pub struct ReportGenerationResult {
    pub report_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub format: ValidationReportFormat,
    pub content: ReportContent,
    pub file_path: Option<String>,
    pub summary: ValidationSummary,
}

/// レポートコンテンツ
#[derive(Debug, Clone)]
pub enum ReportContent {
    Json(String),
    Html(String),
    Csv(String),
    Pdf(Vec<u8>),
}

#[async_trait]
pub trait ReportService: Send + Sync {
    async fn generate_report(
        &self,
        request: ReportGenerationRequest,
    ) -> Result<ReportGenerationResult, ReportServiceError>;
    async fn generate_summary_report(
        &self,
        validation_results: Vec<ValidationResult>,
    ) -> Result<ValidationSummary, ReportServiceError>;
    async fn export_to_file(
        &self,
        result: &ReportGenerationResult,
        file_path: &str,
    ) -> Result<(), ReportServiceError>;
}

/// レポートサービス実装
pub struct ReportServiceImpl {
    // 将来的な拡張のための設定オプション
    pub default_template: ReportTemplateConfig,
}

impl ReportServiceImpl {
    pub fn new() -> Self {
        Self {
            default_template: ReportTemplateConfig {
                title: "データ検証レポート".to_string(),
                subtitle: Some("文書管理システム".to_string()),
                logo_path: None,
                custom_styles: None,
                include_charts: true,
            },
        }
    }

    fn generate_json_report(
        &self,
        validation_results: &[ValidationResult],
        include_details: bool,
    ) -> Result<String, ReportServiceError> {
        let report_data = if include_details {
            serde_json::json!({
                "summary": self.calculate_summary(validation_results),
                "results": validation_results,
                "generated_at": Utc::now(),
                "total_count": validation_results.len()
            })
        } else {
            serde_json::json!({
                "summary": self.calculate_summary(validation_results),
                "generated_at": Utc::now(),
                "total_count": validation_results.len()
            })
        };

        serde_json::to_string_pretty(&report_data).map_err(|e| {
            ReportServiceError::ConversionError {
                message: format!("JSONシリアライゼーションエラー: {e}"),
            }
        })
    }

    fn generate_html_report(
        &self,
        validation_results: &[ValidationResult],
        template_config: &ReportTemplateConfig,
        include_details: bool,
    ) -> Result<String, ReportServiceError> {
        let summary = self.calculate_summary(validation_results);

        let mut html = format!(
            r#"
<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 40px; line-height: 1.6; }}
        .header {{ text-align: center; border-bottom: 3px solid #2c3e50; padding-bottom: 20px; margin-bottom: 30px; }}
        .title {{ color: #2c3e50; font-size: 2.5em; margin-bottom: 10px; }}
        .subtitle {{ color: #7f8c8d; font-size: 1.2em; }}
        .summary {{ background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 30px; }}
        .summary-item {{ display: inline-block; margin: 10px 20px; text-align: center; }}
        .summary-number {{ font-size: 2em; font-weight: bold; color: #3498db; }}
        .summary-label {{ color: #7f8c8d; font-size: 0.9em; }}
        .status-success {{ color: #27ae60; }}
        .status-warning {{ color: #f39c12; }}
        .status-error {{ color: #e74c3c; }}
        .details {{ margin-top: 30px; }}
        .result-item {{ background: white; border: 1px solid #ecf0f1; padding: 15px; margin: 10px 0; border-radius: 5px; }}
        .result-header {{ font-weight: bold; margin-bottom: 10px; }}
        .error-list {{ background: #fdf2f2; padding: 10px; margin: 10px 0; border-left: 4px solid #e74c3c; }}
        {}
    </style>
</head>
<body>
    <div class="header">
        <h1 class="title">{}</h1>
        <div class="subtitle">{}</div>
        <div style="color: #95a5a6; margin-top: 10px;">生成日時: {}</div>
    </div>
"#,
            template_config.title,
            template_config
                .custom_styles
                .as_ref()
                .unwrap_or(&String::new()),
            template_config.title,
            template_config.subtitle.as_ref().unwrap_or(&String::new()),
            Utc::now().format("%Y年%m月%d日 %H:%M:%S UTC")
        );

        // サマリーセクション
        html.push_str(&format!(
            r#"
    <div class="summary">
        <h2>検証結果サマリー</h2>
        <div class="summary-item">
            <div class="summary-number status-success">{}</div>
            <div class="summary-label">成功</div>
        </div>
        <div class="summary-item">
            <div class="summary-number status-warning">{}</div>
            <div class="summary-label">警告</div>
        </div>
        <div class="summary-item">
            <div class="summary-number status-error">{}</div>
            <div class="summary-label">エラー</div>
        </div>
        <div class="summary-item">
            <div class="summary-number">{}</div>
            <div class="summary-label">総件数</div>
        </div>
        <div class="summary-item">
            <div class="summary-number">{:.1}%</div>
            <div class="summary-label">品質スコア</div>
        </div>
    </div>
"#,
            summary.success_count,
            summary.warning_count,
            summary.error_count,
            summary.total_count,
            summary.quality_score
        ));

        // 詳細セクション（オプション）
        if include_details {
            html.push_str("<div class=\"details\"><h2>検証結果詳細</h2>");

            for result in validation_results {
                let status_class = match result.is_valid {
                    true if result.errors.is_empty() => "status-success",
                    true => "status-warning",
                    false => "status-error",
                };

                html.push_str(&format!(
                    r#"
                <div class="result-item">
                    <div class="result-header {}">{} - {}</div>
                    <div>実行時間: {}ms</div>
                    <div>品質スコア: {:.1}%</div>
"#,
                    status_class,
                    result.rule_type,
                    result.rule_name,
                    result.execution_time_ms,
                    result.quality_score
                ));

                if !result.errors.is_empty() {
                    html.push_str("<div class=\"error-list\"><strong>エラー:</strong><ul>");
                    for error in &result.errors {
                        html.push_str(&format!("<li>{}</li>", error.message));
                    }
                    html.push_str("</ul></div>");
                }

                html.push_str("</div>");
            }

            html.push_str("</div>");
        }

        html.push_str("</body></html>");
        Ok(html)
    }

    fn generate_csv_report(
        &self,
        validation_results: &[ValidationResult],
    ) -> Result<String, ReportServiceError> {
        let mut csv = String::from(
            "ルールタイプ,ルール名,有効,実行時間(ms),品質スコア,エラー数,警告数,エラー詳細\n",
        );

        for result in validation_results {
            let error_details = result
                .errors
                .iter()
                .map(|e| e.message.replace(",", ";").replace("\n", " "))
                .collect::<Vec<_>>()
                .join(" | ");

            csv.push_str(&format!(
                "{},{},{},{},{:.1},{},{},{}\n",
                result.rule_type,
                result.rule_name.replace(",", ";"),
                if result.is_valid { "有効" } else { "無効" },
                result.execution_time_ms,
                result.quality_score,
                result.errors.len(),
                result.warnings.len(),
                error_details
            ));
        }

        Ok(csv)
    }

    fn generate_pdf_report(
        &self,
        _validation_results: &[ValidationResult],
        _template_config: &ReportTemplateConfig,
    ) -> Result<Vec<u8>, ReportServiceError> {
        // PDF生成は将来的な実装として、現在はプレースホルダー
        // 実装時にはwkhtmltopdfやprintpdfなどのライブラリを使用
        Err(ReportServiceError::FormatError {
            format: "PDF生成は現在未実装です".to_string(),
        })
    }

    fn calculate_summary(&self, validation_results: &[ValidationResult]) -> ValidationSummary {
        let total_count = validation_results.len();
        let success_count = validation_results
            .iter()
            .filter(|r| r.is_valid && r.errors.is_empty())
            .count();
        let warning_count = validation_results
            .iter()
            .filter(|r| r.is_valid && !r.errors.is_empty())
            .count();
        let error_count = validation_results.iter().filter(|r| !r.is_valid).count();

        let total_quality_score = validation_results
            .iter()
            .map(|r| r.quality_score)
            .sum::<f64>();
        let average_quality_score = if total_count > 0 {
            total_quality_score / total_count as f64
        } else {
            0.0
        };

        ValidationSummary {
            total_count,
            success_count,
            warning_count,
            error_count,
            quality_score: average_quality_score,
            execution_time_ms: validation_results.iter().map(|r| r.execution_time_ms).sum(),
        }
    }
}

#[async_trait]
impl ReportService for ReportServiceImpl {
    async fn generate_report(
        &self,
        request: ReportGenerationRequest,
    ) -> Result<ReportGenerationResult, ReportServiceError> {
        let template_config = request
            .template_config
            .unwrap_or_else(|| self.default_template.clone());

        let content = match request.format {
            ValidationReportFormat::Json => {
                let json_content = self
                    .generate_json_report(&request.validation_results, request.include_details)?;
                ReportContent::Json(json_content)
            }
            ValidationReportFormat::Html => {
                let html_content = self.generate_html_report(
                    &request.validation_results,
                    &template_config,
                    request.include_details,
                )?;
                ReportContent::Html(html_content)
            }
            ValidationReportFormat::Csv => {
                let csv_content = self.generate_csv_report(&request.validation_results)?;
                ReportContent::Csv(csv_content)
            }
            ValidationReportFormat::Pdf => {
                let pdf_content =
                    self.generate_pdf_report(&request.validation_results, &template_config)?;
                ReportContent::Pdf(pdf_content)
            }
        };

        let summary = self.calculate_summary(&request.validation_results);

        Ok(ReportGenerationResult {
            report_id: Uuid::new_v4(),
            generated_at: Utc::now(),
            format: request.format,
            content,
            file_path: request.output_path,
            summary,
        })
    }

    async fn generate_summary_report(
        &self,
        validation_results: Vec<ValidationResult>,
    ) -> Result<ValidationSummary, ReportServiceError> {
        Ok(self.calculate_summary(&validation_results))
    }

    async fn export_to_file(
        &self,
        result: &ReportGenerationResult,
        file_path: &str,
    ) -> Result<(), ReportServiceError> {
        use std::fs;

        match &result.content {
            ReportContent::Json(content)
            | ReportContent::Html(content)
            | ReportContent::Csv(content) => {
                fs::write(file_path, content).map_err(|e| ReportServiceError::FileOutputError {
                    message: format!("ファイル出力エラー: {e}"),
                })?;
            }
            ReportContent::Pdf(content) => {
                fs::write(file_path, content).map_err(|e| ReportServiceError::FileOutputError {
                    message: format!("PDFファイル出力エラー: {e}"),
                })?;
            }
        }

        Ok(())
    }
}

impl Default for ReportServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
