// src/config.rs
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub app: ApplicationConfig,
    pub security: SecurityConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

/// Application-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub version: String,
    pub log_level: LogLevel,
    pub data_dir: String,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: u64,
    pub hash_cost: u32,
}

/// Log levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl FromStr for LogLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err(Error::Config(format!("Invalid log level: {}", s))),
        }
    }
}

/// Load configuration from file and environment variables
pub fn load_config() -> Result<AppConfig> {
    // Default config path
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "configs/config.json".to_string());

    // Load base configuration from file
    let config: AppConfig = if Path::new(&config_path).exists() {
        let config_str = fs::read_to_string(&config_path)
            .map_err(|e| Error::Config(format!("Failed to read config file: {}", e)))?;

        serde_json::from_str(&config_str)
            .map_err(|e| Error::Config(format!("Failed to parse config file: {}", e)))?
    } else {
        // Return default configuration if file doesn't exist
        default_config()
    };

    // Override with environment variables if present
    let config = override_with_env(config)?;

    Ok(config)
}

/// Create default configuration
fn default_config() -> AppConfig {
    AppConfig {
        database: DatabaseConfig {
            url: "sqlite:data/erp.db".to_string(),
            max_connections: 5,
            timeout_seconds: 30,
        },
        app: ApplicationConfig {
            name: "Rust ERP".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            log_level: LogLevel::Info,
            data_dir: "data".to_string(),
        },
        security: SecurityConfig {
            jwt_secret: "change_me_in_production".to_string(),
            token_expiry_hours: 24,
            hash_cost: 12,
        },
    }
}

/// Override configuration with environment variables
fn override_with_env(mut config: AppConfig) -> Result<AppConfig> {
    // Database overrides
    if let Ok(url) = env::var("DATABASE_URL") {
        config.database.url = url;
    }
    if let Ok(max_conn) = env::var("DATABASE_MAX_CONNECTIONS") {
        config.database.max_connections = max_conn
            .parse()
            .map_err(|_| Error::Config("Invalid DATABASE_MAX_CONNECTIONS value".to_string()))?;
    }

    // App overrides
    if let Ok(log_level) = env::var("LOG_LEVEL") {
        config.app.log_level = LogLevel::from_str(&log_level)?;
    }
    if let Ok(data_dir) = env::var("DATA_DIR") {
        config.app.data_dir = data_dir;
    }

    // Security overrides
    if let Ok(jwt_secret) = env::var("JWT_SECRET") {
        config.security.jwt_secret = jwt_secret;
    }

    Ok(config)
}
