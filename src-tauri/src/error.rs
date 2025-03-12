use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Comprehensive internal error type
#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;

/// Helper function to convert generic errors to our Error type
pub fn map_err<E: std::error::Error>(err: E) -> Error {
    Error::Unknown(err.to_string())
}

/// Serializable error response for client consumption
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

// Direct conversion from sqlx::Error to ErrorResponse for compatibility
impl From<sqlx::Error> for ErrorResponse {
    fn from(err: sqlx::Error) -> Self {
        let error_message = match &err {
            sqlx::Error::RowNotFound => "Record not found",
            sqlx::Error::Database(db_err) => "Database error",
            sqlx::Error::ColumnNotFound(col) => &format!("Column not found: {}", col),
            sqlx::Error::PoolClosed => "Database connection pool closed",
            sqlx::Error::PoolTimedOut => "Database connection timeout",
            _ => "Database error occurred",
        };

        Self {
            code: "DATABASE_ERROR".into(),
            message: error_message.to_string(),
            details: if cfg!(debug_assertions) {
                Some(err.to_string())
            } else {
                None
            },
        }
    }
}

impl From<Error> for ErrorResponse {
    fn from(err: Error) -> Self {
        match err {
            Error::Database(e) => Self::from(e),
            Error::Io(e) => Self {
                code: "IO_ERROR".into(),
                message: "A file system error occurred".into(),
                details: if cfg!(debug_assertions) {
                    Some(e.to_string())
                } else {
                    None
                },
            },
            Error::Config(msg) => Self {
                code: "CONFIG_ERROR".into(),
                message: "A configuration error occurred".into(),
                details: Some(msg),
            },
            Error::Auth(msg) => Self {
                code: "AUTH_ERROR".into(),
                message: "An authentication error occurred".into(),
                details: Some(msg),
            },
            Error::Validation(msg) => Self {
                code: "VALIDATION_ERROR".into(),
                message: "A validation error occurred".into(),
                details: Some(msg),
            },
            Error::NotFound(msg) => Self {
                code: "NOT_FOUND".into(),
                message: "Resource not found".into(),
                details: Some(msg),
            },
            Error::Conflict(msg) => Self {
                code: "CONFLICT_ERROR".into(),
                message: "A conflict occurred".into(),
                details: Some(msg),
            },
            Error::ExternalService(msg) => Self {
                code: "EXTERNAL_SERVICE_ERROR".into(),
                message: "An external service error occurred".into(),
                details: Some(msg),
            },
            Error::Serialization(e) => Self {
                code: "SERIALIZATION_ERROR".into(),
                message: "A data serialization error occurred".into(),
                details: if cfg!(debug_assertions) {
                    Some(e.to_string())
                } else {
                    None
                },
            },
            Error::Migration(msg) => Self {
                code: "MIGRATION_ERROR".into(),
                message: "A database migration error occurred".into(),
                details: Some(msg),
            },
            Error::Unknown(msg) => Self {
                code: "UNKNOWN_ERROR".into(),
                message: "An unknown error occurred".into(),
                details: if cfg!(debug_assertions) {
                    Some(msg)
                } else {
                    None
                },
            },
        }
    }
}

impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}

/// For compatibility with Tauri commands that return Result<T, String>
impl From<ErrorResponse> for String {
    fn from(err: ErrorResponse) -> Self {
        if let Some(details) = err.details {
            format!("{}: {}", err.message, details)
        } else {
            err.message
        }
    }
}

// Shorthand function to create a not found error
pub fn not_found(resource: &str) -> Error {
    Error::NotFound(format!("{} not found", resource))
}

// Shorthand function to create a validation error
pub fn validation_error(message: &str) -> Error {
    Error::Validation(message.to_string())
}
