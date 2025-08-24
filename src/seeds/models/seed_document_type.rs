use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Seedデータ用の文書種別構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedDocumentType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub department_code: Option<String>,
    pub prefix: String,
    pub effective_from: String, // JSON内では文字列で管理
    pub effective_until: Option<String>,
    pub is_active: bool,
    pub created_by: i32,
}

impl SeedDocumentType {
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

    /// is_activeをSQLiteの整数値に変換
    pub fn is_active_as_int(&self) -> i32 {
        if self.is_active { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_document_type_deserialization() {
        let json = r#"{
            "id": 1,
            "name": "技術文書",
            "description": "技術仕様書、設計書等",
            "department_code": null,
            "prefix": "TEC",
            "effective_from": "2024-01-01",
            "effective_until": null,
            "is_active": true,
            "created_by": 1
        }"#;

        let doc_type: SeedDocumentType = serde_json::from_str(json).unwrap();
        assert_eq!(doc_type.id, 1);
        assert_eq!(doc_type.name, "技術文書");
        assert_eq!(doc_type.prefix, "TEC");
        assert_eq!(doc_type.department_code, None);
        assert_eq!(doc_type.is_active, true);
        assert_eq!(doc_type.is_active_as_int(), 1);

        // effective_fromの解析テスト
        let parsed_date = doc_type.effective_from_parsed().unwrap();
        assert_eq!(parsed_date.to_string(), "2024-01-01");

        // effective_untilの解析テスト（null の場合）
        let parsed_until = doc_type.effective_until_parsed().unwrap();
        assert_eq!(parsed_until, None);
    }

    #[test]
    fn test_seed_document_type_with_effective_until() {
        let json = r#"{
            "id": 2,
            "name": "旧形式文書",
            "description": "廃止予定の文書形式",
            "department_code": "DEV",
            "prefix": "OLD",
            "effective_from": "2020-01-01",
            "effective_until": "2024-12-31",
            "is_active": false,
            "created_by": 1
        }"#;

        let doc_type: SeedDocumentType = serde_json::from_str(json).unwrap();
        assert_eq!(doc_type.department_code, Some("DEV".to_string()));
        assert_eq!(doc_type.is_active, false);
        assert_eq!(doc_type.is_active_as_int(), 0);

        // effective_untilの解析テスト（値がある場合）
        let parsed_until = doc_type.effective_until_parsed().unwrap();
        assert_eq!(parsed_until.unwrap().to_string(), "2024-12-31");
    }
}
