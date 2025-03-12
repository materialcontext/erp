use sqlx::postgres::PgPool;

/// Application state that will be shared across Tauri commands
#[derive(Debug)]
pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { db_pool: pool }
    }
}
