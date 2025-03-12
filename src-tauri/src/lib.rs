// src/lib.rs
pub mod commands;
pub mod config;
pub mod database;
pub mod error;
pub mod models;
pub mod repositories;
pub mod services;
pub mod state;

// Re-export commonly used items
pub use error::{Error, Result};
pub use state::AppState;
