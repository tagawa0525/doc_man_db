pub mod loader;
pub mod models;
pub mod seeder;

pub use seeder::{SeedData, SeedError, Seeder};

/// Seedingで使用する環境名
#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Test,
    Production,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Test => write!(f, "test"),
            Environment::Production => write!(f, "production"),
        }
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "test" => Ok(Environment::Test),
            "production" | "prod" => Ok(Environment::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}
