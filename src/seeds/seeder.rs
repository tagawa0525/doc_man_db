use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

use crate::seeds::{Environment, loader::SeedLoader, models::*};

#[derive(thiserror::Error, Debug)]
pub enum SeedError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Loader error: {0}")]
    Loader(#[from] crate::seeds::loader::LoaderError),
    #[error("Dependency error: missing table '{0}'")]
    MissingDependency(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Circular dependency detected in table dependencies")]
    CircularDependency,
}

/// Seedデータをデータベースに投入するトレイト
#[async_trait]
pub trait SeedData {
    /// データをデータベースに投入
    async fn seed(&self, pool: &Pool<Sqlite>, dry_run: bool) -> Result<usize, SeedError>;

    /// テーブルからすべてのデータを削除
    async fn clear(&self, pool: &Pool<Sqlite>, dry_run: bool) -> Result<usize, SeedError>;

    /// テーブル名を取得
    fn table_name(&self) -> &'static str;

    /// 依存テーブル名のリストを取得
    fn dependencies(&self) -> Vec<&'static str>;
}

/// メインのSeedingサービス
pub struct Seeder {
    pool: Pool<Sqlite>,
    loader: SeedLoader,
}

impl Seeder {
    /// 新しいSeederインスタンスを作成
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            loader: SeedLoader::default(),
        }
    }

    /// カスタムローダーを使用してSeederを作成
    pub fn with_loader(pool: Pool<Sqlite>, loader: SeedLoader) -> Self {
        Self { pool, loader }
    }

    /// 指定された環境のすべてのテーブルにシードデータを投入
    pub async fn seed_all(
        &self,
        env: &Environment,
        dry_run: bool,
        reset: bool,
    ) -> Result<(), SeedError> {
        let tables = self.get_seed_order(env)?;

        println!("Seeding environment: {}", env);
        if dry_run {
            println!("DRY RUN MODE - No data will actually be inserted");
        }

        if reset {
            // 逆順でデータを削除
            println!("Resetting database (clearing existing data)...");
            for table_name in tables.iter().rev() {
                self.clear_table(env, table_name, dry_run).await?;
            }
        }

        // 依存関係順でデータを投入
        for table_name in &tables {
            self.seed_table(env, table_name, dry_run).await?;
        }

        println!("Seeding completed successfully!");
        Ok(())
    }

    /// 指定されたテーブルのみシードデータを投入
    pub async fn seed_table(
        &self,
        env: &Environment,
        table_name: &str,
        dry_run: bool,
    ) -> Result<(), SeedError> {
        println!("Seeding table: {}", table_name);

        let count = match table_name {
            "employees" => {
                let seed_file = self
                    .loader
                    .load_seed_file::<SeedEmployee>(env, table_name)?;
                self.seed_employees(&seed_file.data, dry_run).await?
            }
            "departments" => {
                let seed_file = self
                    .loader
                    .load_seed_file::<SeedDepartment>(env, table_name)?;
                self.seed_departments(&seed_file.data, dry_run).await?
            }
            "document_types" => {
                let seed_file = self
                    .loader
                    .load_seed_file::<SeedDocumentType>(env, table_name)?;
                self.seed_document_types(&seed_file.data, dry_run).await?
            }
            "document_number_rules" => {
                let seed_file = self
                    .loader
                    .load_seed_file::<SeedDocumentNumberRule>(env, table_name)?;
                self.seed_document_number_rules(&seed_file.data, dry_run)
                    .await?
            }
            _ => {
                return Err(SeedError::InvalidData(format!(
                    "Unknown table: {}",
                    table_name
                )));
            }
        };

        println!("  -> {} records processed", count);
        Ok(())
    }

    /// 指定されたテーブルのデータを削除
    pub async fn clear_table(
        &self,
        _env: &Environment,
        table_name: &str,
        dry_run: bool,
    ) -> Result<(), SeedError> {
        if dry_run {
            println!("Would clear table: {}", table_name);
            return Ok(());
        }

        let result = match table_name {
            "employees" => {
                sqlx::query("DELETE FROM employees")
                    .execute(&self.pool)
                    .await?
            }
            "departments" => {
                sqlx::query("DELETE FROM departments")
                    .execute(&self.pool)
                    .await?
            }
            "document_types" => {
                sqlx::query("DELETE FROM document_types")
                    .execute(&self.pool)
                    .await?
            }
            "document_number_rules" => {
                sqlx::query("DELETE FROM document_number_generation_rules")
                    .execute(&self.pool)
                    .await?
            }
            _ => {
                return Err(SeedError::InvalidData(format!(
                    "Unknown table: {}",
                    table_name
                )));
            }
        };

        println!(
            "Cleared table: {} ({} rows affected)",
            table_name,
            result.rows_affected()
        );
        Ok(())
    }

    /// 依存関係を考慮したテーブルの投入順序を決定
    fn get_seed_order(&self, env: &Environment) -> Result<Vec<String>, SeedError> {
        let available_tables = self.loader.list_available_tables(env)?;

        // 依存関係マップを構築
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

        for table in &available_tables {
            let deps = match table.as_str() {
                "employees" => vec![],
                "departments" => vec!["employees".to_string()],
                "document_types" => vec!["employees".to_string()],
                "document_number_rules" => vec![
                    "employees".to_string(),
                    "departments".to_string(),
                    "document_types".to_string(),
                ],
                _ => vec![],
            };
            dependencies.insert(table.clone(), deps);
        }

        // トポロジカルソート
        let mut sorted = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut visiting = std::collections::HashSet::new();

        for table in &available_tables {
            Self::topological_sort(
                table,
                &dependencies,
                &mut sorted,
                &mut visited,
                &mut visiting,
            )?;
        }

        Ok(sorted)
    }

    /// トポロジカルソート（循環依存検出付き）
    fn topological_sort(
        table: &str,
        dependencies: &HashMap<String, Vec<String>>,
        sorted: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
        visiting: &mut std::collections::HashSet<String>,
    ) -> Result<(), SeedError> {
        if visiting.contains(table) {
            return Err(SeedError::CircularDependency);
        }

        if visited.contains(table) {
            return Ok(());
        }

        visiting.insert(table.to_string());

        if let Some(deps) = dependencies.get(table) {
            for dep in deps {
                Self::topological_sort(dep, dependencies, sorted, visited, visiting)?;
            }
        }

        visiting.remove(table);
        visited.insert(table.to_string());
        sorted.push(table.to_string());

        Ok(())
    }

    // 各テーブルのシード実装
    async fn seed_employees(
        &self,
        employees: &[SeedEmployee],
        dry_run: bool,
    ) -> Result<usize, SeedError> {
        if dry_run {
            println!("  Would insert {} employees", employees.len());
            return Ok(employees.len());
        }

        let mut count = 0;
        for employee in employees {
            let hire_date = employee
                .hire_date_parsed()
                .map_err(|e| SeedError::InvalidData(format!("Invalid hire_date: {}", e)))?;

            sqlx::query(r#"
                INSERT OR REPLACE INTO employees 
                (id, employee_number, name, department, position, email, hire_date, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            "#)
            .bind(employee.id)
            .bind(&employee.employee_number)
            .bind(&employee.name)
            .bind(&employee.department)
            .bind(&employee.position)
            .bind(&employee.email)
            .bind(hire_date)
            .execute(&self.pool)
            .await?;

            count += 1;
        }

        Ok(count)
    }

    async fn seed_departments(
        &self,
        departments: &[SeedDepartment],
        dry_run: bool,
    ) -> Result<usize, SeedError> {
        if dry_run {
            println!("  Would insert {} departments", departments.len());
            return Ok(departments.len());
        }

        let mut count = 0;
        for department in departments {
            let created_date = department
                .created_date_parsed()
                .map_err(|e| SeedError::InvalidData(format!("Invalid created_date: {}", e)))?;

            sqlx::query(
                r#"
                INSERT OR REPLACE INTO departments 
                (id, code, name, parent_id, level, manager_id, description, location, 
                 phone_number, email, budget, is_active, created_date, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            "#,
            )
            .bind(department.id)
            .bind(&department.code)
            .bind(&department.name)
            .bind(department.parent_id)
            .bind(department.level)
            .bind(department.manager_id)
            .bind(&department.description)
            .bind(&department.location)
            .bind(&department.phone_number)
            .bind(&department.email)
            .bind(department.budget)
            .bind(department.is_active_as_int())
            .bind(created_date)
            .execute(&self.pool)
            .await?;

            count += 1;
        }

        Ok(count)
    }

    async fn seed_document_types(
        &self,
        document_types: &[SeedDocumentType],
        dry_run: bool,
    ) -> Result<usize, SeedError> {
        if dry_run {
            println!("  Would insert {} document types", document_types.len());
            return Ok(document_types.len());
        }

        let mut count = 0;
        for doc_type in document_types {
            let effective_from = doc_type
                .effective_from_parsed()
                .map_err(|e| SeedError::InvalidData(format!("Invalid effective_from: {}", e)))?;

            let effective_until = doc_type
                .effective_until_parsed()
                .map_err(|e| SeedError::InvalidData(format!("Invalid effective_until: {}", e)))?;

            sqlx::query(
                r#"
                INSERT OR REPLACE INTO document_types 
                (id, name, description, department_code, prefix, effective_from, effective_until, 
                 is_active, created_by, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            "#,
            )
            .bind(doc_type.id)
            .bind(&doc_type.name)
            .bind(&doc_type.description)
            .bind(&doc_type.department_code)
            .bind(&doc_type.prefix)
            .bind(effective_from)
            .bind(effective_until)
            .bind(doc_type.is_active_as_int())
            .bind(doc_type.created_by)
            .execute(&self.pool)
            .await?;

            count += 1;
        }

        Ok(count)
    }

    async fn seed_document_number_rules(
        &self,
        rules: &[SeedDocumentNumberRule],
        dry_run: bool,
    ) -> Result<usize, SeedError> {
        if dry_run {
            println!("  Would insert {} document number rules", rules.len());
            return Ok(rules.len());
        }

        let mut count = 0;
        for rule in rules {
            let effective_from = rule
                .effective_from_parsed()
                .map_err(|e| SeedError::InvalidData(format!("Invalid effective_from: {}", e)))?;

            let effective_until = rule
                .effective_until_parsed()
                .map_err(|e| SeedError::InvalidData(format!("Invalid effective_until: {}", e)))?;

            sqlx::query(
                r#"
                INSERT OR REPLACE INTO document_number_generation_rules 
                (id, rule_name, template, sequence_digits, department_code, document_type_codes,
                 effective_from, effective_until, priority, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            "#,
            )
            .bind(rule.id)
            .bind(&rule.rule_name)
            .bind(&rule.template)
            .bind(rule.sequence_digits)
            .bind(&rule.department_code)
            .bind(&rule.document_type_codes)
            .bind(effective_from)
            .bind(effective_until)
            .bind(rule.priority)
            .execute(&self.pool)
            .await?;

            count += 1;
        }

        Ok(count)
    }
}
