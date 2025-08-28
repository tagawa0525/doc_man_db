use chrono::Datelike;
use std::sync::Arc;

use crate::models::{
    DocumentNumberGenerationError, DocumentNumberRequest, GeneratedDocumentNumber,
};
use crate::repositories::DocumentNumberRuleRepository;

/// テンプレート適用用のパラメータ構造体
struct TemplateParams<'a> {
    template: &'a str,
    department_code: &'a str,
    document_type_code: &'a str,
    year: i32,
    month: i32,
    sequence_number: i32,
    sequence_digits: i32,
}

/// 文書番号生成サービス
#[derive(Clone)]
pub struct DocumentNumberGenerator {
    rule_repository: Arc<dyn DocumentNumberRuleRepository>,
}

impl DocumentNumberGenerator {
    pub fn new(rule_repository: impl DocumentNumberRuleRepository + 'static) -> Self {
        Self {
            rule_repository: Arc::new(rule_repository),
        }
    }

    /// 文書番号を生成する
    pub async fn generate_document_number(
        &self,
        request: DocumentNumberRequest,
    ) -> Result<GeneratedDocumentNumber, DocumentNumberGenerationError> {
        // リクエストのバリデーション
        request
            .validate()
            .map_err(|_| DocumentNumberGenerationError::NoApplicableRule)?;

        // 適用可能なルールを検索
        let rule = self
            .rule_repository
            .find_applicable_rule(
                &request.document_type_code,
                &request.department_code,
                request.created_date,
            )
            .await?;

        let rule = match rule {
            Some(rule) => rule,
            None => return Err(DocumentNumberGenerationError::NoApplicableRule),
        };

        // 年月の情報を取得
        let year = request.created_date.year();
        let month = request.created_date.month() as i32;

        // 最大10回まで重複回避を試行
        for _attempt in 0..10 {
            // 次の連番を取得
            let sequence_number = self
                .rule_repository
                .get_next_sequence_number(rule.id, year, month, &request.department_code)
                .await?;

            // テンプレートから文書番号を生成
            let document_number = self.apply_template(TemplateParams {
                template: &rule.template,
                department_code: &request.department_code,
                document_type_code: &request.document_type_code,
                year,
                month,
                sequence_number,
                sequence_digits: rule.sequence_digits,
            })?;

            // 重複チェック
            let exists = self
                .rule_repository
                .is_document_number_exists(&document_number)
                .await?;

            if !exists {
                return Ok(GeneratedDocumentNumber {
                    document_number,
                    rule_id: rule.id,
                    sequence_number,
                    template_used: rule.template,
                });
            }
        }

        Err(DocumentNumberGenerationError::SequenceExhausted)
    }

    /// テンプレートを適用して文書番号を生成
    fn apply_template(
        &self,
        params: TemplateParams,
    ) -> Result<String, DocumentNumberGenerationError> {
        let mut result = params.template.to_string();

        // 文書種別コード
        result = result.replace("{文書種別コード}", params.document_type_code);

        // 部署コード
        result = result.replace("{部署コード}", params.department_code);

        // 年（下2桁）
        let year_short = params.year % 100;
        result = result.replace("{年下2桁}", &format!("{year_short:02}"));

        // 月（2桁ゼロ埋め）
        result = result.replace("{月:2桁}", &format!("{:02}", params.month));

        // 連番（指定桁数でゼロ埋め）
        let sequence_format = format!("{{連番:{}桁}}", params.sequence_digits);
        let sequence_value = format!(
            "{:0width$}",
            params.sequence_number,
            width = params.sequence_digits as usize
        );
        result = result.replace(&sequence_format, &sequence_value);

        // 未処理のプレースホルダーがないかチェック
        if result.contains('{') && result.contains('}') {
            return Err(DocumentNumberGenerationError::TemplateError(format!(
                "Unresolved placeholders in template: {}",
                params.template
            )));
        }

        Ok(result)
    }
}
