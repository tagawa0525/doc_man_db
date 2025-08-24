use crate::seeds::Environment;
use crate::seeds::models::SeedFile;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum LoaderError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid seed data directory: {0}")]
    InvalidDirectory(String),
}

/// Seedデータファイルローダー
pub struct SeedLoader {
    seeds_dir: PathBuf,
}

impl SeedLoader {
    /// 新しいSeedLoaderインスタンスを作成
    pub fn new<P: AsRef<Path>>(seeds_dir: P) -> Self {
        Self {
            seeds_dir: seeds_dir.as_ref().to_path_buf(),
        }
    }

    /// 指定された環境とテーブル名に対するシードファイルを読み込み
    pub fn load_seed_file<T>(
        &self,
        env: &Environment,
        table_name: &str,
    ) -> Result<SeedFile<T>, LoaderError>
    where
        T: DeserializeOwned,
    {
        let file_path = self.get_seed_file_path(env, table_name);

        if !file_path.exists() {
            return Err(LoaderError::FileNotFound(
                file_path.to_string_lossy().to_string(),
            ));
        }

        let content = fs::read_to_string(&file_path)?;
        let seed_file: SeedFile<T> = serde_json::from_str(&content)?;

        // 環境の整合性チェック
        if seed_file.environment != env.to_string() {
            println!(
                "Warning: Environment mismatch in {}: expected '{}', found '{}'",
                file_path.display(),
                env,
                seed_file.environment
            );
        }

        Ok(seed_file)
    }

    /// 指定された環境で利用可能なすべてのシードファイル名を取得
    pub fn list_available_tables(&self, env: &Environment) -> Result<Vec<String>, LoaderError> {
        let env_dir = self.get_environment_dir(env);

        if !env_dir.exists() {
            return Err(LoaderError::InvalidDirectory(format!(
                "Environment directory not found: {}",
                env_dir.display()
            )));
        }

        let mut tables = Vec::new();
        for entry in fs::read_dir(&env_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && path.extension().map(|s| s == "json").unwrap_or(false)
                && let Some(stem) = path.file_stem()
                && let Some(table_name) = stem.to_str()
            {
                tables.push(table_name.to_string());
            }
        }

        tables.sort();
        Ok(tables)
    }

    /// シードファイルが存在するかチェック
    pub fn seed_file_exists(&self, env: &Environment, table_name: &str) -> bool {
        self.get_seed_file_path(env, table_name).exists()
    }

    /// 環境ディレクトリのパスを取得
    fn get_environment_dir(&self, env: &Environment) -> PathBuf {
        self.seeds_dir.join("data").join(env.to_string())
    }

    /// シードファイルのパスを取得
    fn get_seed_file_path(&self, env: &Environment, table_name: &str) -> PathBuf {
        self.get_environment_dir(env)
            .join(format!("{}.json", table_name))
    }
}

impl Default for SeedLoader {
    fn default() -> Self {
        Self::new("./seeds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_seed_loader_creation() {
        let loader = SeedLoader::new("/tmp/seeds");
        assert!(loader.seeds_dir.to_str().unwrap().contains("seeds"));
    }

    #[test]
    fn test_default_seed_loader() {
        let loader = SeedLoader::default();
        assert_eq!(loader.seeds_dir, PathBuf::from("./seeds"));
    }

    #[test]
    fn test_get_seed_file_path() {
        let loader = SeedLoader::new("/tmp/seeds");
        let path = loader.get_seed_file_path(&Environment::Development, "employees");
        assert_eq!(
            path,
            PathBuf::from("/tmp/seeds/data/development/employees.json")
        );
    }

    #[test]
    fn test_seed_file_exists() {
        let temp_dir = tempdir().unwrap();
        let seeds_dir = temp_dir.path();

        // テスト用ディレクトリ構造を作成
        let dev_dir = seeds_dir.join("data").join("development");
        fs::create_dir_all(&dev_dir).unwrap();

        // テストファイル作成
        let test_file = dev_dir.join("test.json");
        fs::write(&test_file, "{}").unwrap();

        let loader = SeedLoader::new(seeds_dir);

        assert!(loader.seed_file_exists(&Environment::Development, "test"));
        assert!(!loader.seed_file_exists(&Environment::Development, "nonexistent"));
    }

    #[test]
    fn test_list_available_tables() {
        let temp_dir = tempdir().unwrap();
        let seeds_dir = temp_dir.path();

        // テスト用ディレクトリ構造を作成
        let dev_dir = seeds_dir.join("data").join("development");
        fs::create_dir_all(&dev_dir).unwrap();

        // テストファイル作成
        fs::write(dev_dir.join("employees.json"), "{}").unwrap();
        fs::write(dev_dir.join("departments.json"), "{}").unwrap();
        fs::write(dev_dir.join("not_json.txt"), "test").unwrap(); // JSONファイルではない

        let loader = SeedLoader::new(seeds_dir);
        let tables = loader
            .list_available_tables(&Environment::Development)
            .unwrap();

        assert_eq!(tables, vec!["departments", "employees"]);
    }
}
