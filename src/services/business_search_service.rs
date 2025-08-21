use crate::error::SearchError;
use crate::models::{
    AdvancedBusinessSearchInput, BusinessAutocompleteInput, BusinessSearchResult,
    AutocompleteResult, EmployeeBusinesses
};
use crate::repositories::BusinessSearchRepository;
use crate::services::UserPermissions;
use std::sync::Arc;

pub struct BusinessSearchService {
    repository: Arc<dyn BusinessSearchRepository>,
}

impl BusinessSearchService {
    pub fn new(repository: Arc<dyn BusinessSearchRepository>) -> Self {
        Self { repository }
    }

    /// 高度業務検索を実行する
    pub async fn search_businesses_advanced(
        &self,
        filters: AdvancedBusinessSearchInput,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessSearchResult, SearchError> {
        // パラメータバリデーション
        self.validate_search_parameters(&filters)?;
        
        // 権限チェック
        if !user_permissions.can_view_all_businesses && user_permissions.accessible_departments.is_empty() {
            return Err(SearchError::InvalidParameters(
                "業務検索権限がありません".to_string()
            ));
        }
        
        self.repository.search_businesses_advanced(filters, user_permissions).await
    }
    
    /// 業務オートコンプリート機能
    pub async fn business_autocomplete(
        &self,
        input: BusinessAutocompleteInput,
        user_permissions: &UserPermissions,
    ) -> Result<AutocompleteResult, SearchError> {
        // クエリの最小長チェック
        if input.query.len() < 2 {
            return Ok(AutocompleteResult {
                suggestions: vec![]
            });
        }
        
        self.repository.business_autocomplete(input, user_permissions).await
    }
    
    /// 社員の業務一覧を取得（逆引き検索）
    pub async fn get_employee_businesses(
        &self,
        employee_id: i32,
        include_completed: bool,
        user_permissions: &UserPermissions,
    ) -> Result<EmployeeBusinesses, SearchError> {
        // 権限チェック: 自分の業務または管理者権限
        if employee_id != user_permissions.employee_id && !user_permissions.can_view_all_businesses {
            return Err(SearchError::InvalidParameters(
                "他の社員の業務履歴を参照する権限がありません".to_string()
            ));
        }
        
        self.repository.get_employee_businesses(employee_id, include_completed, user_permissions).await
    }
    
    /// 業務統計情報を取得
    pub async fn get_business_statistics(
        &self,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessStatistics, SearchError> {
        // 基本統計情報を取得
        let total_businesses = self.count_total_businesses(user_permissions).await?;
        let active_businesses = self.count_active_businesses(user_permissions).await?;
        let my_businesses = if user_permissions.can_view_all_businesses {
            0
        } else {
            self.count_my_businesses(user_permissions.employee_id, user_permissions).await?
        };
        let completed_this_month = self.count_completed_this_month(user_permissions).await?;
        
        Ok(BusinessStatistics {
            total_businesses,
            active_businesses,
            completed_businesses: total_businesses - active_businesses,
            my_businesses,
            completed_this_month,
        })
    }
    
    /// 業務検索の推奨候補を生成
    pub async fn get_search_suggestions(
        &self,
        user_permissions: &UserPermissions,
    ) -> Result<BusinessSearchSuggestions, SearchError> {
        let mut suggestions = BusinessSearchSuggestions::default();
        
        // 最近の顧客
        suggestions.recent_customers = self.get_recent_customers(user_permissions).await?;
        
        // よく検索される業務
        suggestions.popular_searches = self.get_popular_searches(user_permissions).await?;
        
        // 自分が関与している業務
        if !user_permissions.can_view_all_businesses {
            suggestions.my_recent_businesses = self.get_my_recent_businesses(user_permissions.employee_id, user_permissions).await?;
        }
        
        Ok(suggestions)
    }
    
    // プライベートメソッド
    
    /// 検索パラメータのバリデーション
    fn validate_search_parameters(&self, filters: &AdvancedBusinessSearchInput) -> Result<(), SearchError> {
        // ページネーション検証
        if filters.pagination.limit <= 0 || filters.pagination.limit > 1000 {
            return Err(SearchError::InvalidParameters(
                "検索結果の上限は1-1000件です".to_string()
            ));
        }
        
        if filters.pagination.offset < 0 {
            return Err(SearchError::InvalidParameters(
                "オフセットは0以上である必要があります".to_string()
            ));
        }
        
        // 日付範囲検証
        if let (Some(from), Some(to)) = (&filters.start_date_from, &filters.start_date_to) {
            if from > to {
                return Err(SearchError::InvalidParameters(
                    "開始日の範囲が無効です".to_string()
                ));
            }
        }
        
        if let (Some(from), Some(to)) = (&filters.end_date_from, &filters.end_date_to) {
            if from > to {
                return Err(SearchError::InvalidParameters(
                    "終了日の範囲が無効です".to_string()
                ));
            }
        }
        
        // 検索条件が最低一つはあることを確認（緩い制限）
        let has_conditions = filters.business_number.is_some() || 
                           filters.name.is_some() || 
                           filters.customer_name.is_some() ||
                           filters.description.is_some() ||
                           filters.member_employee_id.is_some() ||
                           filters.status.is_some() ||
                           filters.start_date_from.is_some() ||
                           filters.start_date_to.is_some() ||
                           filters.end_date_from.is_some() ||
                           filters.end_date_to.is_some() ||
                           filters.has_documents.is_some() ||
                           filters.created_by.is_some();
        
        if !has_conditions {
            return Err(SearchError::InvalidParameters(
                "最低一つの検索条件を指定してください".to_string()
            ));
        }
        
        Ok(())
    }
    
    // 統計情報取得のヘルパーメソッド（簡略実装）
    async fn count_total_businesses(&self, _user_permissions: &UserPermissions) -> Result<i32, SearchError> {
        // TODO: 実際のカウント実装
        Ok(0)
    }
    
    async fn count_active_businesses(&self, _user_permissions: &UserPermissions) -> Result<i32, SearchError> {
        // TODO: 実際のカウント実装
        Ok(0)
    }
    
    async fn count_my_businesses(&self, _employee_id: i32, _user_permissions: &UserPermissions) -> Result<i32, SearchError> {
        // TODO: 実際のカウント実装
        Ok(0)
    }
    
    async fn count_completed_this_month(&self, _user_permissions: &UserPermissions) -> Result<i32, SearchError> {
        // TODO: 実際のカウント実装
        Ok(0)
    }
    
    async fn get_recent_customers(&self, _user_permissions: &UserPermissions) -> Result<Vec<String>, SearchError> {
        // TODO: 実際の実装
        Ok(vec![])
    }
    
    async fn get_popular_searches(&self, _user_permissions: &UserPermissions) -> Result<Vec<String>, SearchError> {
        // TODO: 実際の実装
        Ok(vec![])
    }
    
    async fn get_my_recent_businesses(&self, _employee_id: i32, _user_permissions: &UserPermissions) -> Result<Vec<String>, SearchError> {
        // TODO: 実際の実装
        Ok(vec![])
    }
}

// レスポンス型定義

#[derive(Debug, Clone, serde::Serialize)]
pub struct BusinessStatistics {
    pub total_businesses: i32,
    pub active_businesses: i32,
    pub completed_businesses: i32,
    pub my_businesses: i32,
    pub completed_this_month: i32,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct BusinessSearchSuggestions {
    pub recent_customers: Vec<String>,
    pub popular_searches: Vec<String>,
    pub my_recent_businesses: Vec<String>,
}

// 検索クエリの最適化とヘルパー関数

/// 業務名の正規化
pub fn normalize_business_name(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || "ー－・".contains(*c))
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// 業務番号の正規化
pub fn normalize_business_number(number: &str) -> String {
    number.trim()
          .to_uppercase()
          .chars()
          .filter(|c| c.is_alphanumeric() || "-_".contains(*c))
          .collect()
}

/// 顧客名の正規化
pub fn normalize_customer_name(customer: &str) -> String {
    customer.trim()
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>()
            .replace("株式会社", "")
            .replace("有限会社", "")
            .replace("(株)", "")
            .replace("（株）", "")
            .trim()
            .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_business_name() {
        assert_eq!(normalize_business_name("  システム  開発  "), "システム 開発");
        assert_eq!(normalize_business_name("Project-A  管理"), "project-a 管理");
    }
    
    #[test]
    fn test_normalize_business_number() {
        assert_eq!(normalize_business_number("bus-25001"), "BUS-25001");
        assert_eq!(normalize_business_number(" proj_2024_01 "), "PROJ_2024_01");
    }
    
    #[test]
    fn test_normalize_customer_name() {
        assert_eq!(normalize_customer_name("株式会社テスト"), "テスト");
        assert_eq!(normalize_customer_name("テスト(株)"), "テスト");
        assert_eq!(normalize_customer_name("有限会社サンプル"), "サンプル");
    }
}