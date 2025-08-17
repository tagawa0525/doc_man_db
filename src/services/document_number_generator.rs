use chrono::Datelike;
use std::sync::Arc;

use crate::models::{
    DocumentNumberGenerationError, DocumentNumberRequest, GeneratedDocumentNumber,
};
use crate::repositories::DocumentNumberRuleRepository;

/// 文書番号生成サービス
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
        for attempt in 0..10 {
            // 次の連番を取得
            let sequence_number = self
                .rule_repository
                .get_next_sequence_number(rule.id, year, month, &request.department_code)
                .await?
                + attempt;

            // テンプレートから文書番号を生成
            let document_number = self.apply_template(
                &rule.template,
                &request.department_code,
                year,
                month,
                sequence_number,
                rule.sequence_digits,
            )?;

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
        template: &str,
        department_code: &str,
        year: i32,
        month: i32,
        sequence_number: i32,
        sequence_digits: i32,
    ) -> Result<String, DocumentNumberGenerationError> {
        let mut result = template.to_string();

        // 部署コード
        result = result.replace("{部署コード}", department_code);

        // 年（下2桁）
        let year_short = year % 100;
        result = result.replace("{年下2桁}", &format!("{year_short:02}"));

        // 月（2桁ゼロ埋め）
        result = result.replace("{月:2桁}", &format!("{month:02}"));

        // 連番（指定桁数でゼロ埋め）
        let sequence_format = format!("{{連番:{sequence_digits}桁}}");
        let sequence_value = format!(
            "{:0width$}",
            sequence_number,
            width = sequence_digits as usize
        );
        result = result.replace(&sequence_format, &sequence_value);

        // 未処理のプレースホルダーがないかチェック
        if result.contains('{') && result.contains('}') {
            return Err(DocumentNumberGenerationError::TemplateError(format!(
                "Unresolved placeholders in template: {template}"
            )));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_template_standard_format() {
        let generator = DocumentNumberGenerator::new(
            crate::repositories::MockDocumentNumberRuleRepository::new(),
        );

        let result = generator
            .apply_template("{部署コード}-{年下2桁}{連番:3桁}", "T", 2025, 8, 1, 3)
            .unwrap();

        assert_eq!(result, "T-25001");
    }

    #[test]
    fn test_apply_template_cta_format() {
        let generator = DocumentNumberGenerator::new(
            crate::repositories::MockDocumentNumberRuleRepository::new(),
        );

        let result = generator
            .apply_template("CTA-{年下2桁}{月:2桁}{連番:3桁}", "C", 2025, 8, 8, 3)
            .unwrap();

        assert_eq!(result, "CTA-2508008");
    }

    #[test]
    fn test_apply_template_different_digits() {
        let generator = DocumentNumberGenerator::new(
            crate::repositories::MockDocumentNumberRuleRepository::new(),
        );

        let result = generator
            .apply_template("技術-{年下2桁}{連番:5桁}", "T", 2025, 8, 25, 5)
            .unwrap();

        assert_eq!(result, "技術-2500025");
    }

    #[test]
    fn test_apply_template_invalid_placeholder() {
        let generator = DocumentNumberGenerator::new(
            crate::repositories::MockDocumentNumberRuleRepository::new(),
        );

        let result =
            generator.apply_template("{不明なプレースホルダー}-{年下2桁}", "T", 2025, 8, 1, 3);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocumentNumberGenerationError::TemplateError(_)
        ));
    }
}
