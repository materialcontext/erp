use crate::services::tauri;
use serde::{Deserialize, Serialize};

// Account view model for the frontend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountViewModel {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub is_active: bool,
    pub parent_id: Option<String>,
    pub balance: String,
    pub created_at: String,
    pub updated_at: String,
}

// Data transfer object for creating/updating accounts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountDto {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub parent_id: Option<String>,
}

impl Default for AccountDto {
    fn default() -> Self {
        Self {
            code: String::new(),
            name: String::new(),
            description: None,
            account_type: "ASSET".to_string(),
            category: "CURRENT_ASSET".to_string(),
            subcategory: None,
            parent_id: None,
        }
    }
}

/// Fetches all accounts from the backend
pub async fn get_all() -> Result<Vec<AccountViewModel>, String> {
    tauri::invoke::<(), Vec<AccountViewModel>>("get_accounts", &())
        .await
        .map_err(|e| format!("Failed to fetch accounts: {}", e))
}

/// Fetches a single account by ID
pub async fn get_by_id(id: &str) -> Result<Option<AccountViewModel>, String> {
    tauri::invoke::<_, Option<AccountViewModel>>("get_account", &id)
        .await
        .map_err(|e| format!("Failed to fetch account: {}", e))
}

/// Creates a new account
pub async fn create(account: &AccountDto) -> Result<AccountViewModel, String> {
    tauri::invoke::<_, AccountViewModel>("create_account", account)
        .await
        .map_err(|e| format!("Failed to create account: {}", e))
}

/// Updates an existing account
pub async fn update(id: &str, account: &AccountDto) -> Result<AccountViewModel, String> {
    #[derive(Serialize)]
    struct UpdateArgs<'a> {
        id: &'a str,
        update_data: &'a AccountDto,
    }

    let args = UpdateArgs {
        id,
        update_data: account,
    };

    tauri::invoke::<_, AccountViewModel>("update_account", &args)
        .await
        .map_err(|e| format!("Failed to update account: {}", e))
}

// Deletes an account
pub async fn delete(id: &str) -> Result<(), String> {
    tauri::invoke::<_, ()>("delete_account", &id)
        .await
        .map_err(|e| format!("Failed to delete account: {}", e))
}

/// Toggles the active status of an account
pub async fn toggle_status(id: &str) -> Result<AccountViewModel, String> {
    tauri::invoke::<_, AccountViewModel>("toggle_account_status", &id)
        .await
        .map_err(|e| format!("Failed to toggle account status: {}", e))
}

/// Fetches root (top-level) accounts
pub async fn get_roots() -> Result<Vec<AccountViewModel>, String> {
    tauri::invoke::<(), Vec<AccountViewModel>>("get_root_accounts", &())
        .await
        .map_err(|e| format!("Failed to fetch root accounts: {}", e))
}

/// Fetches child accounts for a parent account
pub async fn get_children(parent_id: &str) -> Result<Vec<AccountViewModel>, String> {
    tauri::invoke::<_, Vec<AccountViewModel>>("get_child_accounts", parent_id)
        .await
        .map_err(|e| format!("Failed to fetch child accounts: {}", e))
}

/// Gets the available account types
pub fn get_account_types() -> Vec<&'static str> {
    vec!["ASSET", "LIABILITY", "EQUITY", "REVENUE", "EXPENSE"]
}

/// Gets available categories for a given account type
pub fn get_categories_for_type(account_type: &str) -> Vec<&'static str> {
    match account_type {
        "ASSET" => vec!["CURRENT_ASSET", "FIXED_ASSET", "OTHER_ASSET"],
        "LIABILITY" => vec![
            "CURRENT_LIABILITY",
            "LONG_TERM_LIABILITY",
            "OTHER_LIABILITY",
        ],
        "EQUITY" => vec!["OWNER_EQUITY", "RETAINED_EARNINGS"],
        "REVENUE" => vec!["OPERATING_REVENUE", "NON_OPERATING_REVENUE"],
        "EXPENSE" => vec!["OPERATING_EXPENSE", "NON_OPERATING_EXPENSE"],
        _ => vec![],
    }
}
