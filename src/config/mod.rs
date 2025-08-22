use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub auth: AuthConfig,
    pub file_system: FileSystemConfig,
    pub notification: NotificationConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub max_request_size: usize,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub default_ttl_seconds: u64,
    pub max_size: usize,
    pub cleanup_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub method: AuthMethod,
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
    pub windows_ad: Option<WindowsAdConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Json,
    WindowsAd,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsAdConfig {
    pub domain: String,
    pub server: String,
    pub port: u16,
    pub base_dn: String,
    pub bind_user: String,
    pub bind_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfig {
    pub base_path: PathBuf,
    pub check_interval_hours: u64,
    pub batch_size: usize,
    pub max_retries: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub email: EmailConfig,
    pub teams: TeamsConfig,
    pub max_retry_attempts: u32,
    pub retry_delay_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub enabled: bool,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsConfig {
    pub enabled: bool,
    pub webhook_url: String,
    pub default_channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub health_check_interval_seconds: u64,
    pub performance_logging: bool,
    pub log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: None,
                max_request_size: 10 * 1024 * 1024, // 10MB
                timeout_seconds: 30,
            },
            database: DatabaseConfig {
                url: "sqlite://./data/dev.db".to_string(),
                max_connections: 10,
                min_connections: 1,
                acquire_timeout_seconds: 30,
                idle_timeout_seconds: 600,
                max_lifetime_seconds: 3600,
            },
            cache: CacheConfig {
                enabled: true,
                default_ttl_seconds: 300, // 5 minutes
                max_size: 10000,
                cleanup_interval_seconds: 60,
            },
            auth: AuthConfig {
                method: AuthMethod::Json,
                jwt_secret: "default-secret-please-change".to_string(),
                jwt_expiration_hours: 24,
                windows_ad: None,
            },
            file_system: FileSystemConfig {
                base_path: PathBuf::from("./files"),
                check_interval_hours: 24,
                batch_size: 1000,
                max_retries: 3,
                timeout_seconds: 10,
            },
            notification: NotificationConfig {
                email: EmailConfig {
                    enabled: false,
                    smtp_server: "localhost".to_string(),
                    smtp_port: 587,
                    smtp_username: "".to_string(),
                    smtp_password: "".to_string(),
                    from_address: "noreply@example.com".to_string(),
                    use_tls: true,
                },
                teams: TeamsConfig {
                    enabled: false,
                    webhook_url: "".to_string(),
                    default_channel: "general".to_string(),
                },
                max_retry_attempts: 3,
                retry_delay_seconds: 5,
            },
            monitoring: MonitoringConfig {
                metrics_enabled: true,
                metrics_port: 9090,
                health_check_interval_seconds: 30,
                performance_logging: true,
                log_level: "info".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            // Start with default values
            .add_source(Config::try_from(&AppConfig::default())?)
            // Add configuration from files
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false));

        // Add environment-specific configuration
        if let Ok(env) = std::env::var("APP_ENV") {
            config = config.add_source(File::with_name(&format!("config/{env}")).required(false));
        }

        // Add environment variables with APP_ prefix
        config = config.add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        );

        config.build()?.try_deserialize()
    }

    pub fn load_from_file(path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(Config::try_from(&AppConfig::default())?)
            .add_source(File::with_name(path))
            .build()?;

        config.try_deserialize()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate database URL
        if self.database.url.is_empty() {
            return Err(ConfigError::Message("Database URL is required".to_string()));
        }

        // Validate server configuration
        if self.server.port == 0 {
            return Err(ConfigError::Message(
                "Server port must be greater than 0".to_string(),
            ));
        }

        // Validate JWT secret
        if self.auth.jwt_secret == "default-secret-please-change" {
            tracing::warn!("Using default JWT secret. Please change this in production!");
        }

        if self.auth.jwt_secret.len() < 32 {
            return Err(ConfigError::Message(
                "JWT secret must be at least 32 characters".to_string(),
            ));
        }

        // Validate file system path
        if !self.file_system.base_path.exists() {
            tracing::warn!(
                "File system base path does not exist: {:?}",
                self.file_system.base_path
            );
        }

        // Validate notification configuration
        if self.notification.email.enabled && self.notification.email.smtp_server.is_empty() {
            return Err(ConfigError::Message(
                "SMTP server is required when email is enabled".to_string(),
            ));
        }

        if self.notification.teams.enabled && self.notification.teams.webhook_url.is_empty() {
            return Err(ConfigError::Message(
                "Teams webhook URL is required when Teams is enabled".to_string(),
            ));
        }

        Ok(())
    }

    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    pub fn is_development(&self) -> bool {
        std::env::var("APP_ENV").unwrap_or_default() == "development"
    }

    pub fn is_production(&self) -> bool {
        std::env::var("APP_ENV").unwrap_or_default() == "production"
    }
}

/// Configuration manager for runtime configuration updates
pub struct ConfigManager {
    config: AppConfig,
}

impl ConfigManager {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn get(&self) -> &AppConfig {
        &self.config
    }

    pub fn update_cache_config(&mut self, cache_config: CacheConfig) {
        self.config.cache = cache_config;
    }

    pub fn update_monitoring_config(&mut self, monitoring_config: MonitoringConfig) {
        self.config.monitoring = monitoring_config;
    }

    pub fn update_notification_config(&mut self, notification_config: NotificationConfig) {
        self.config.notification = notification_config;
    }

    pub fn reload_from_file(&mut self, path: &str) -> Result<(), ConfigError> {
        self.config = AppConfig::load_from_file(path)?;
        self.config.validate()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert!(config.cache.enabled);
    }

    #[test]
    fn test_config_validation() {
        let mut config = AppConfig::default();

        // Update JWT secret to be valid for testing
        config.auth.jwt_secret =
            "this-is-a-long-enough-jwt-secret-for-testing-purposes".to_string();

        // Valid config should pass
        assert!(config.validate().is_ok());

        // Invalid port should fail
        config.server.port = 0;
        assert!(config.validate().is_err());

        // Reset and test JWT secret
        config = AppConfig::default();
        config.auth.jwt_secret = "short".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_load_from_file() {
        let mut file = NamedTempFile::with_suffix(".toml").unwrap();
        writeln!(
            file,
            r#"
[server]
host = "127.0.0.1"
port = 3000

[database]
url = "sqlite://test.db"
max_connections = 5

[cache]
enabled = false
"#
        )
        .unwrap();

        let path_without_ext = file.path().to_str().unwrap().strip_suffix(".toml").unwrap();
        let config = AppConfig::load_from_file(path_without_ext).unwrap();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.database.max_connections, 5);
        assert!(!config.cache.enabled);
    }

    #[test]
    fn test_config_manager() {
        let mut manager = ConfigManager::new(AppConfig::default());

        assert!(manager.get().cache.enabled);

        let mut new_cache_config = manager.get().cache.clone();
        new_cache_config.enabled = false;
        manager.update_cache_config(new_cache_config);

        assert!(!manager.get().cache.enabled);
    }
}
