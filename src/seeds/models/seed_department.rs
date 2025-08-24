use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Seedデータ用の部署構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedDepartment {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub parent_id: Option<i32>,
    pub level: i32,
    pub manager_id: Option<i32>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub budget: Option<f64>,
    pub is_active: bool,
    pub created_date: String, // JSON内では文字列で管理
}

impl SeedDepartment {
    /// created_dateをNaiveDateに変換
    pub fn created_date_parsed(&self) -> Result<NaiveDate, chrono::ParseError> {
        NaiveDate::parse_from_str(&self.created_date, "%Y-%m-%d")
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
    fn test_seed_department_deserialization() {
        let json = r#"{
            "id": 1,
            "code": "DEV",
            "name": "開発部",
            "parent_id": null,
            "level": 0,
            "manager_id": 1,
            "description": "システム開発とメンテナンス",
            "location": "本社3F",
            "phone_number": "03-1234-5600",
            "email": "dev@company.com",
            "budget": 50000000.00,
            "is_active": true,
            "created_date": "2020-04-01"
        }"#;

        let department: SeedDepartment = serde_json::from_str(json).unwrap();
        assert_eq!(department.id, 1);
        assert_eq!(department.code, "DEV");
        assert_eq!(department.name, "開発部");
        assert_eq!(department.parent_id, None);
        assert_eq!(department.manager_id, Some(1));
        assert_eq!(department.is_active, true);
        assert_eq!(department.is_active_as_int(), 1);

        // created_dateの解析テスト
        let parsed_date = department.created_date_parsed().unwrap();
        assert_eq!(parsed_date.to_string(), "2020-04-01");
    }
}
