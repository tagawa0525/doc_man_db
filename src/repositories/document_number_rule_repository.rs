use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::SqlitePool;

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

// SQLite実装

pub struct SqliteDocumentNumberRuleRepository {
    #[allow(dead_code)]
    pool: SqlitePool,
}

impl SqliteDocumentNumberRuleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// テスト用のインメモリSQLiteリポジトリを作成
    pub async fn new_in_memory() -> Result<Self, RepositoryError> {
        let pool = SqlitePool::connect(":memory:").await?;

        // マイグレーションを実行
        let migrator = sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
            .await
            .map_err(|e| RepositoryError::Validation(format!("Migration setup error: {}", e)))?;
        
        migrator
            .run(&pool)
            .await
            .map_err(|e| RepositoryError::Database(e.into()))?;

        // seedシステムを使用してテストデータを投入
        use crate::seeds::{Environment, Seeder};
        let seeder = Seeder::new(pool.clone());
        seeder.seed_all(&Environment::Test, false, false).await
            .map_err(|e| RepositoryError::Validation(format!("Seed loading error: {}", e)))?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl DocumentNumberRuleRepository for SqliteDocumentNumberRuleRepository {
    async fn find_applicable_rule(
        &self,
        document_type_code: &str,
        department_code: &str,
        date: NaiveDate,
    ) -> Result<Option<DocumentNumberGenerationRule>, RepositoryError> {
        // データベースから適用可能なルールを検索
        println!(
            "DEBUG: find_applicable_rule called with doc_type='{}', dept='{}'",
            document_type_code, department_code
        );

        // 1. 部署固有ルールを優先検索
        let specific_rule = sqlx::query_as::<_, DocumentNumberGenerationRule>(
            r#"
            SELECT id, rule_name, template, sequence_digits, department_code, 
                   document_type_codes, effective_from, effective_until, priority, 
                   created_at, updated_at
            FROM document_number_generation_rules
            WHERE department_code = ? 
              AND JSON_EXTRACT(document_type_codes, '$') LIKE '%' || ? || '%'
              AND effective_from <= ?
              AND (effective_until IS NULL OR effective_until >= ?)
            ORDER BY priority ASC
            LIMIT 1
            "#,
        )
        .bind(department_code)
        .bind(document_type_code)
        .bind(date)
        .bind(date)
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        if specific_rule.is_some() {
            return Ok(specific_rule);
        }

        // 2. 汎用ルール（department_code = NULL）を検索
        let generic_rule = sqlx::query_as::<_, DocumentNumberGenerationRule>(
            r#"
            SELECT id, rule_name, template, sequence_digits, department_code, 
                   document_type_codes, effective_from, effective_until, priority, 
                   created_at, updated_at
            FROM document_number_generation_rules
            WHERE department_code IS NULL
              AND JSON_EXTRACT(document_type_codes, '$') LIKE '%' || ? || '%'
              AND effective_from <= ?
              AND (effective_until IS NULL OR effective_until >= ?)
            ORDER BY priority ASC
            LIMIT 1
            "#,
        )
        .bind(document_type_code)
        .bind(date)
        .bind(date)
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        Ok(generic_rule)
    }

    async fn get_next_sequence_number(
        &self,
        rule_id: i32,
        year: i32,
        month: i32,
        _department_code: &str,
    ) -> Result<i32, RepositoryError> {
        // 該当する年月のパターンで既存の文書番号から最大連番を取得
        let year_month_pattern = format!("{:02}{:02}", year % 100, month);
        let number_pattern = format!("%-{year_month_pattern}%");
        
        let max_sequence = sqlx::query_scalar::<_, Option<i32>>(
            r#"
            SELECT MAX(
                CAST(
                    SUBSTR(number, LENGTH(number) - 2, 3) AS INTEGER
                )
            ) as max_seq
            FROM documents 
            WHERE number LIKE ?
            "#,
        )
        .bind(number_pattern)
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::Database)?
        .flatten()
        .unwrap_or(0);

        let next_sequence = max_sequence + 1;

        // 連番上限チェック（3桁の場合は999まで）
        if next_sequence > 999 {
            return Err(RepositoryError::Validation(
                format!("Sequence numbers exhausted for rule {}, year {}, month {}. Maximum reached: 999", rule_id, year, month)
            ));
        }

        Ok(next_sequence)
    }

    async fn is_document_number_exists(
        &self,
        document_number: &str,
    ) -> Result<bool, RepositoryError> {
        // データベースで文書番号の存在をチェック
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM documents WHERE number = ?)",
        )
        .bind(document_number)
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::Database)?;

        Ok(exists)
    }

    async fn create_rule(
        &self,
        _request: CreateDocumentNumberGenerationRuleRequest,
    ) -> Result<DocumentNumberGenerationRule, RepositoryError> {
        todo!("Not needed for current tests")
    }

    async fn get_rule_by_id(
        &self,
        _id: i32,
    ) -> Result<Option<DocumentNumberGenerationRule>, RepositoryError> {
        todo!("Not needed for current tests")
    }

    async fn search_rules(
        &self,
        _department_code: Option<String>,
        _active_on_date: Option<NaiveDate>,
        _limit: i64,
        _offset: i64,
    ) -> Result<(Vec<DocumentNumberGenerationRule>, i64), RepositoryError> {
        todo!("Not needed for current tests")
    }
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
