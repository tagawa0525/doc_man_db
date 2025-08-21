use crate::error::SearchError;
use crate::models::{
    AdvancedEmployeeSearchInput, AutocompleteResult, BusinessMember, EmployeeAutocompleteInput,
    EmployeeSearchResult,
};
use crate::repositories::AdvancedSearchRepository;
use crate::services::UserPermissions;
use std::sync::Arc;

pub struct AdvancedSearchService {
    repository: Arc<dyn AdvancedSearchRepository>,
}

impl AdvancedSearchService {
    pub fn new(repository: Arc<dyn AdvancedSearchRepository>) -> Self {
        Self { repository }
    }

    /// 高度社員検索を実行する
    pub async fn search_employees_advanced(
        &self,
        filters: AdvancedEmployeeSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<EmployeeSearchResult, SearchError> {
        // パラメータバリデーション
        self.validate_search_parameters(&filters)?;

        // 権限チェック
        if !user_permissions.can_view_all_businesses
            && user_permissions.accessible_departments.is_empty()
        {
            return Err(SearchError::InvalidParameters(
                "検索権限がありません".to_string(),
            ));
        }

        self.repository
            .search_employees_advanced(filters, user_permissions)
            .await
    }

    /// 社員オートコンプリート機能
    pub async fn employee_autocomplete(
        &self,
        input: EmployeeAutocompleteInput,
        user_permissions: &UserPermissions,
    ) -> Result<AutocompleteResult, SearchError> {
        // クエリの最小長チェック
        if input.query.len() < 2 {
            return Ok(AutocompleteResult {
                suggestions: vec![],
            });
        }

        self.repository
            .employee_autocomplete(input, user_permissions)
            .await
    }

    /// 社員の業務従事履歴を取得
    pub async fn get_employee_business_history(
        &self,
        employee_id: i32,
        user_permissions: &UserPermissions,
    ) -> Result<Vec<BusinessMember>, SearchError> {
        // 権限チェック: 自分の履歴または管理者権限
        if employee_id != user_permissions.employee_id && !user_permissions.can_view_all_businesses
        {
            return Err(SearchError::InvalidParameters(
                "業務履歴を参照する権限がありません".to_string(),
            ));
        }

        // TODO: business repositoryから業務履歴を取得
        // 現在は空のVecを返す
        Ok(vec![])
    }

    // プライベートメソッド

    /// 検索パラメータのバリデーション
    fn validate_search_parameters(
        &self,
        filters: &AdvancedEmployeeSearchInput,
    ) -> Result<(), SearchError> {
        // ページネーション検証
        if filters.pagination.limit <= 0 || filters.pagination.limit > 1000 {
            return Err(SearchError::InvalidParameters(
                "検索結果の上限は1-1000件です".to_string(),
            ));
        }

        if filters.pagination.offset < 0 {
            return Err(SearchError::InvalidParameters(
                "オフセットは0以上である必要があります".to_string(),
            ));
        }

        // Joining date validation removed - fields not available in current model

        // 検索条件が最低一つはあることを確認
        if filters.name.is_none()
            && filters.employee_number.is_none()
            && filters.email.is_none()
            && filters.department_id.is_none()
            && filters.has_business_experience.is_none()
            && filters.is_active.is_none()
            && filters
                .skill_keywords
                .as_ref()
                .is_none_or(|v| v.is_empty())
        {
            return Err(SearchError::InvalidParameters(
                "最低一つの検索条件を指定してください".to_string(),
            ));
        }

        Ok(())
    }
}

// ヘルパー関数とユーティリティ

/// 検索クエリの正規化
pub fn normalize_search_query(query: &str) -> String {
    query
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || "ー－・".contains(*c))
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// 日本語文字列の類似度計算（簡易版）
pub fn calculate_similarity(query: &str, target: &str) -> f64 {
    let query_normalized = normalize_search_query(query);
    let target_normalized = normalize_search_query(target);

    if query_normalized == target_normalized {
        return 1.0;
    }

    if target_normalized.contains(&query_normalized) {
        return 0.8;
    }

    // レーベンシュタイン距離の簡易計算
    let query_chars: Vec<char> = query_normalized.chars().collect();
    let target_chars: Vec<char> = target_normalized.chars().collect();

    if query_chars.is_empty() || target_chars.is_empty() {
        return 0.0;
    }

    let max_len = query_chars.len().max(target_chars.len());
    let distance = levenshtein_distance(&query_chars, &target_chars);

    1.0 - (distance as f64 / max_len as f64)
}

/// レーベンシュタイン距離計算
fn levenshtein_distance(a: &[char], b: &[char]) -> usize {
    let mut matrix = vec![vec![0; b.len() + 1]; a.len() + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=a.len() {
        matrix[i][0] = i;
    }

    for j in 0..=b.len() {
        matrix[0][j] = j;
    }

    for (i, &char_a) in a.iter().enumerate() {
        for (j, &char_b) in b.iter().enumerate() {
            let cost = if char_a == char_b { 0 } else { 1 };
            matrix[i + 1][j + 1] = *[
                matrix[i][j + 1] + 1, // deletion
                matrix[i + 1][j] + 1, // insertion
                matrix[i][j] + cost,  // substitution
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    matrix[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_search_query() {
        assert_eq!(normalize_search_query("  田中  太郎  "), "田中 太郎");
        assert_eq!(normalize_search_query("TANAKA Taro"), "tanaka taro");
        assert_eq!(
            normalize_search_query("システム・エンジニア"),
            "システム エンジニア"
        );
    }

    #[test]
    fn test_calculate_similarity() {
        assert_eq!(calculate_similarity("田中", "田中"), 1.0);
        assert!(calculate_similarity("田中", "田中太郎") > 0.5);
        assert!(calculate_similarity("abc", "xyz") < 0.5);
    }
}
