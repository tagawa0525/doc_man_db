use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Seedデータ用の文書番号生成ルール構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedDocumentNumberRule {
    pub id: i32,
    pub rule_name: String,
    pub template: String,
    pub sequence_digits: i32,
    pub department_code: Option<String>,
    pub document_type_codes: String, // JSON文字列として保存
    pub effective_from: String,      // JSON内では文字列で管理
    pub effective_until: Option<String>,
    pub priority: i32,
}

impl SeedDocumentNumberRule {
    /// effective_fromをNaiveDateに変換
    pub fn effective_from_parsed(&self) -> Result<NaiveDate, chrono::ParseError> {
        NaiveDate::parse_from_str(&self.effective_from, "%Y-%m-%d")
    }

    /// effective_untilをNaiveDateに変換（存在する場合）
    pub fn effective_until_parsed(&self) -> Result<Option<NaiveDate>, chrono::ParseError> {
        match &self.effective_until {
            Some(date_str) => {
                let parsed = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    /// document_type_codesをVec<String>として解析
    pub fn parse_document_type_codes(&self) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_str::<Vec<String>>(&self.document_type_codes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_document_number_rule_deserialization() {
        let json = r#"{
            "id": 1,
            "rule_name": "技術文書ルール",
            "template": "{文書種別コード}-{年下2桁}{月:2桁}{連番:3桁}",
            "sequence_digits": 3,
            "department_code": "DEV",
            "document_type_codes": "[\"TEC\"]",
            "effective_from": "2024-01-01",
            "effective_until": null,
            "priority": 1
        }"#;

        let rule: SeedDocumentNumberRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.id, 1);
        assert_eq!(rule.rule_name, "技術文書ルール");
        assert_eq!(
            rule.template,
            "{文書種別コード}-{年下2桁}{月:2桁}{連番:3桁}"
        );
        assert_eq!(rule.sequence_digits, 3);
        assert_eq!(rule.department_code, Some("DEV".to_string()));
        assert_eq!(rule.priority, 1);

        // effective_fromの解析テスト
        let parsed_date = rule.effective_from_parsed().unwrap();
        assert_eq!(parsed_date.to_string(), "2024-01-01");

        // document_type_codesの解析テスト
        let type_codes = rule.parse_document_type_codes().unwrap();
        assert_eq!(type_codes, vec!["TEC"]);
    }

    #[test]
    fn test_seed_document_number_rule_multiple_types() {
        let json = r#"{
            "id": 3,
            "rule_name": "汎用ルール",
            "template": "{文書種別コード}-{年下2桁}{月:2桁}{連番:3桁}",
            "sequence_digits": 3,
            "department_code": null,
            "document_type_codes": "[\"TEC\",\"BUS\",\"CON\"]",
            "effective_from": "2024-01-01",
            "effective_until": null,
            "priority": 9
        }"#;

        let rule: SeedDocumentNumberRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.department_code, None);

        // 複数の文書種別コードの解析テスト
        let type_codes = rule.parse_document_type_codes().unwrap();
        assert_eq!(type_codes, vec!["TEC", "BUS", "CON"]);
    }
}
