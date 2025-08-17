use async_trait::async_trait;
use chrono::NaiveDate;

use super::RepositoryError;
use crate::models::{CreateDocumentNumberGenerationRuleRequest, DocumentNumberGenerationRule};

#[async_trait]
pub trait DocumentNumberRuleRepository: Send + Sync {
    /// 指定された条件に適用可能なルールを検索（優先度順）
    async fn find_applicable_rule(
        &self,
        document_type_code: &str,
        department_code: &str,
        date: NaiveDate,
    ) -> Result<Option<DocumentNumberGenerationRule>, RepositoryError>;

    /// 指定されたルール、年月、部署の組み合わせで次の連番を取得
    async fn get_next_sequence_number(
        &self,
        rule_id: i32,
        year: i32,
        month: i32,
        department_code: &str,
    ) -> Result<i32, RepositoryError>;

    /// 文書番号の重複チェック
    async fn is_document_number_exists(
        &self,
        document_number: &str,
    ) -> Result<bool, RepositoryError>;

    /// 文書番号生成ルールを作成
    async fn create_rule(
        &self,
        request: CreateDocumentNumberGenerationRuleRequest,
    ) -> Result<DocumentNumberGenerationRule, RepositoryError>;

    /// IDでルールを取得
    async fn get_rule_by_id(
        &self,
        id: i32,
    ) -> Result<Option<DocumentNumberGenerationRule>, RepositoryError>;

    /// ルールを検索
    async fn search_rules(
        &self,
        department_code: Option<String>,
        active_on_date: Option<NaiveDate>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<DocumentNumberGenerationRule>, i64), RepositoryError>;
}

// テスト用のモックリポジトリ（mockallを使用）
#[cfg(test)]
pub use mock::*;

#[cfg(test)]
mod mock {
    use super::*;

    mockall::mock! {
        pub DocumentNumberRuleRepository {}

    #[async_trait]
    impl DocumentNumberRuleRepository for DocumentNumberRuleRepository {
        async fn find_applicable_rule(
            &self,
            document_type_code: &str,
            department_code: &str,
            date: NaiveDate,
        ) -> Result<Option<DocumentNumberGenerationRule>, RepositoryError>;

        async fn get_next_sequence_number(
            &self,
            rule_id: i32,
            year: i32,
            month: i32,
            department_code: &str,
        ) -> Result<i32, RepositoryError>;

        async fn is_document_number_exists(&self, document_number: &str) -> Result<bool, RepositoryError>;

        async fn create_rule(
            &self,
            request: CreateDocumentNumberGenerationRuleRequest,
        ) -> Result<DocumentNumberGenerationRule, RepositoryError>;

        async fn get_rule_by_id(&self, id: i32) -> Result<Option<DocumentNumberGenerationRule>, RepositoryError>;

        async fn search_rules(
            &self,
            department_code: Option<String>,
            active_on_date: Option<NaiveDate>,
            limit: i64,
            offset: i64,
        ) -> Result<(Vec<DocumentNumberGenerationRule>, i64), RepositoryError>;
    }
    }
}
