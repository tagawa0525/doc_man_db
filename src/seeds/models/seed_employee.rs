use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Seedデータ用の従業員構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedEmployee {
    pub id: i32,
    pub employee_number: String,
    pub name: String,
    pub department: String,
    pub position: String,
    pub email: String,
    pub hire_date: String, // JSON内では文字列で管理
}

impl SeedEmployee {
    /// hire_dateをNaiveDateに変換
    pub fn hire_date_parsed(&self) -> Result<NaiveDate, chrono::ParseError> {
        NaiveDate::parse_from_str(&self.hire_date, "%Y-%m-%d")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_employee_deserialization() {
        let json = r#"{
            "id": 1,
            "employee_number": "EMP101",
            "name": "山田太郎",
            "department": "DEV",
            "position": "部長",
            "email": "yamada@company.com",
            "hire_date": "2018-04-01"
        }"#;

        let employee: SeedEmployee = serde_json::from_str(json).unwrap();
        assert_eq!(employee.id, 1);
        assert_eq!(employee.name, "山田太郎");
        assert_eq!(employee.department, "DEV");

        // hire_dateの解析テスト
        let parsed_date = employee.hire_date_parsed().unwrap();
        assert_eq!(parsed_date.to_string(), "2018-04-01");
    }
}
