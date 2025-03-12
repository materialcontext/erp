// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use erp_lib::commands;
use erp_lib::AppState;
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() {
    // Load .env file from the root directory
    let root_dir = std::env::current_dir().expect("Failed to determine the current directory");
    let env_path = root_dir.join(".env");

    if env_path.exists() {
        println!("Loading environment from: {}", env_path.display());
        dotenv::from_path(env_path).ok();
    } else {
        println!("Warning: .env file not found at {}", env_path.display());
        // Try to load from the current directory as a fallback
        dotenv().ok();
    }

    // Initialize database connection
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    println!("Connecting to database...");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Ensure database is properly set up
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    println!("Database connection established");

    tauri::Builder::default()
        .manage(AppState { db_pool: pool })
        .invoke_handler(tauri::generate_handler![
            commands::get_accounts,
            commands::get_account,
            commands::create_account,
            commands::update_account,
            commands::delete_account,
            commands::toggle_account_status,
            commands::get_root_accounts,
            commands::get_child_accounts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
