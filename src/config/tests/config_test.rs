use super::super::{AppConfig, ConfigManager};
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
    config.auth.jwt_secret = "this-is-a-long-enough-jwt-secret-for-testing-purposes".to_string();

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
