use crate::error::SearchError;
use crate::models::{
    SearchHistory, FavoriteSearch, SearchSuggestion, CreateSearchHistoryRequest,
    CreateFavoriteSearchRequest, UpdateFavoriteSearchRequest, CreateSearchSuggestionRequest,
    SearchHistoryFilters, FavoriteSearchFilters, SearchSuggestionFilters,
    SearchHistoryResponse, FavoriteSearchResponse, SearchSuggestionResponse,
    SearchStatistics, SearchType
};
use crate::services::UserPermissions;
use sqlx::{Row, SqlitePool};
use std::sync::Arc;

pub struct SearchHistoryService {
    pool: SqlitePool,
}

impl SearchHistoryService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 検索履歴を保存
    pub async fn save_search_history(
        &self,
        request: CreateSearchHistoryRequest,
        _user_permissions: &UserPermissions,
    ) -> Result<SearchHistory, SearchError> {
        let search_id = sqlx::query(
            r#"
            INSERT INTO search_history (employee_id, search_type, search_query, result_count, execution_time_ms)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(request.employee_id)
        .bind(String::from(request.search_type.clone()))
        .bind(&request.search_query)
        .bind(request.result_count)
        .bind(request.execution_time_ms)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.get_search_history_by_id(search_id as i32).await?
            .ok_or(SearchError::InvalidParameters("検索履歴の保存に失敗しました".to_string()))
    }

    /// 検索履歴を取得
    pub async fn get_search_history(
        &self,
        filters: SearchHistoryFilters,
        user_permissions: &UserPermissions,
    ) -> Result<SearchHistoryResponse, SearchError> {
        let mut conditions = Vec::new();
        let mut bind_values = Vec::new();

        // 権限チェック: 自分の履歴のみ表示（管理者は全て）
        if !user_permissions.can_view_all_businesses {
            conditions.push("employee_id = ?");
            bind_values.push(user_permissions.employee_id.to_string());
        } else if let Some(employee_id) = filters.employee_id {
            conditions.push("employee_id = ?");
            bind_values.push(employee_id.to_string());
        }

        if let Some(search_type) = &filters.search_type {
            conditions.push("search_type = ?");
            bind_values.push(String::from(search_type.clone()));
        }

        if let Some(date_from) = &filters.date_from {
            conditions.push("created_at >= ?");
            bind_values.push(date_from.to_string());
        }

        if let Some(date_to) = &filters.date_to {
            conditions.push("created_at <= ?");
            bind_values.push(date_to.to_string());
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // 総件数取得
        let count_query = format!("SELECT COUNT(*) as count FROM search_history {}", where_clause);
        let mut count_query_builder = sqlx::query(&count_query);
        for value in &bind_values {
            count_query_builder = count_query_builder.bind(value);
        }
        let total: i64 = count_query_builder.fetch_one(&self.pool).await?.get("count");

        // データ取得
        let data_query = format!(
            "SELECT * FROM search_history {} ORDER BY created_at DESC LIMIT ? OFFSET ?",
            where_clause
        );
        let mut data_query_builder = sqlx::query(&data_query);
        for value in &bind_values {
            data_query_builder = data_query_builder.bind(value);
        }
        data_query_builder = data_query_builder
            .bind(filters.limit.unwrap_or(20))
            .bind(filters.offset.unwrap_or(0));

        let rows = data_query_builder.fetch_all(&self.pool).await?;
        let history = rows.into_iter()
            .map(|row| SearchHistory::from_row(&row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SearchHistoryResponse {
            total_count: total,
            has_next_page: history.len() as i64 == filters.limit.unwrap_or(20) as i64 && total > (filters.offset.unwrap_or(0) + history.len() as i32) as i64,
            history,
        })
    }

    /// お気に入り検索を作成
    pub async fn create_favorite_search(
        &self,
        request: CreateFavoriteSearchRequest,
        user_permissions: &UserPermissions,
    ) -> Result<FavoriteSearch, SearchError> {
        // 権限チェック: 自分のお気に入りのみ作成可能
        if request.employee_id != user_permissions.employee_id && !user_permissions.can_view_all_businesses {
            return Err(SearchError::InvalidParameters("他のユーザーのお気に入りは作成できません".to_string()));
        }

        let favorite_id = sqlx::query(
            r#"
            INSERT INTO favorite_searches (employee_id, name, search_type, search_query)
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(request.employee_id)
        .bind(&request.name)
        .bind(String::from(request.search_type.clone()))
        .bind(&request.search_query)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.get_favorite_search_by_id(favorite_id as i32).await?
            .ok_or(SearchError::InvalidParameters("お気に入り検索の作成に失敗しました".to_string()))
    }

    /// お気に入り検索を取得
    pub async fn get_favorite_searches(
        &self,
        filters: FavoriteSearchFilters,
        user_permissions: &UserPermissions,
    ) -> Result<FavoriteSearchResponse, SearchError> {
        let mut conditions = Vec::new();
        let mut bind_values = Vec::new();

        // 権限チェック: 自分のお気に入りのみ表示
        if !user_permissions.can_view_all_businesses {
            conditions.push("employee_id = ?");
            bind_values.push(user_permissions.employee_id.to_string());
        } else if let Some(employee_id) = filters.employee_id {
            conditions.push("employee_id = ?");
            bind_values.push(employee_id.to_string());
        }

        if let Some(search_type) = &filters.search_type {
            conditions.push("search_type = ?");
            bind_values.push(String::from(search_type.clone()));
        }

        if let Some(is_active) = filters.is_active {
            conditions.push("is_active = ?");
            bind_values.push(is_active.to_string());
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // 総件数とデータを取得
        let count_query = format!("SELECT COUNT(*) as count FROM favorite_searches {}", where_clause);
        let mut count_query_builder = sqlx::query(&count_query);
        for value in &bind_values {
            count_query_builder = count_query_builder.bind(value);
        }
        let total: i64 = count_query_builder.fetch_one(&self.pool).await?.get("count");

        let data_query = format!(
            "SELECT * FROM favorite_searches {} ORDER BY created_at DESC LIMIT ? OFFSET ?",
            where_clause
        );
        let mut data_query_builder = sqlx::query(&data_query);
        for value in &bind_values {
            data_query_builder = data_query_builder.bind(value);
        }
        data_query_builder = data_query_builder
            .bind(filters.limit.unwrap_or(50))
            .bind(filters.offset.unwrap_or(0));

        let rows = data_query_builder.fetch_all(&self.pool).await?;
        let favorites = rows.into_iter()
            .map(|row| FavoriteSearch::from_row(&row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(FavoriteSearchResponse {
            total_count: total,
            has_next_page: favorites.len() as i64 == filters.limit.unwrap_or(50) as i64 && total > (filters.offset.unwrap_or(0) + favorites.len() as i32) as i64,
            favorites,
        })
    }

    /// 検索候補を更新
    pub async fn update_search_suggestion(
        &self,
        request: CreateSearchSuggestionRequest,
    ) -> Result<(), SearchError> {
        // 既存の候補をチェック
        let existing = sqlx::query(
            "SELECT id, frequency FROM search_suggestions WHERE search_type = ? AND field_name = ? AND suggestion = ?"
        )
        .bind(String::from(request.search_type.clone()))
        .bind(&request.field_name)
        .bind(&request.suggestion)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = existing {
            // 既存の候補の頻度を更新
            let id: i32 = row.get("id");
            let frequency: i32 = row.get("frequency");
            
            sqlx::query(
                "UPDATE search_suggestions SET frequency = ?, last_used = CURRENT_TIMESTAMP WHERE id = ?"
            )
            .bind(frequency + 1)
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else {
            // 新しい候補を追加
            sqlx::query(
                "INSERT INTO search_suggestions (search_type, field_name, suggestion) VALUES (?, ?, ?)"
            )
            .bind(String::from(request.search_type))
            .bind(&request.field_name)
            .bind(&request.suggestion)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    /// 検索候補を取得
    pub async fn get_search_suggestions(
        &self,
        filters: SearchSuggestionFilters,
    ) -> Result<SearchSuggestionResponse, SearchError> {
        let mut conditions = Vec::new();
        let mut bind_values = Vec::new();

        if let Some(search_type) = &filters.search_type {
            conditions.push("search_type = ?");
            bind_values.push(String::from(search_type.clone()));
        }

        if let Some(field_name) = &filters.field_name {
            conditions.push("field_name = ?");
            bind_values.push(field_name.clone());
        }

        if let Some(query) = &filters.query {
            conditions.push("suggestion LIKE ?");
            bind_values.push(format!("%{}%", query));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let query = format!(
            "SELECT * FROM search_suggestions {} ORDER BY frequency DESC, last_used DESC LIMIT ?",
            where_clause
        );
        let mut query_builder = sqlx::query(&query);
        for value in &bind_values {
            query_builder = query_builder.bind(value);
        }
        query_builder = query_builder.bind(filters.limit.unwrap_or(10));

        let rows = query_builder.fetch_all(&self.pool).await?;
        let suggestions = rows.into_iter()
            .map(|row| SearchSuggestion::from_row(&row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SearchSuggestionResponse {
            total_count: suggestions.len() as i64,
            suggestions,
        })
    }

    // プライベートメソッド
    async fn get_search_history_by_id(&self, id: i32) -> Result<Option<SearchHistory>, SearchError> {
        let row = sqlx::query("SELECT * FROM search_history WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(SearchHistory::from_row(&row)?))
        } else {
            Ok(None)
        }
    }

    async fn get_favorite_search_by_id(&self, id: i32) -> Result<Option<FavoriteSearch>, SearchError> {
        let row = sqlx::query("SELECT * FROM favorite_searches WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(FavoriteSearch::from_row(&row)?))
        } else {
            Ok(None)
        }
    }
}