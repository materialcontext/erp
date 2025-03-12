use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{not_found, validation_error, Error, ErrorResponse, Result};
use crate::models::account::{Account, AccountCategory, AccountType, NewAccount};
use crate::repositories::accounts::AccountRepository;
use crate::AppState;

// View models for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAccountDto {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub parent_id: Option<String>,
}

impl From<Account> for AccountViewModel {
    fn from(account: Account) -> Self {
        Self {
            id: account.id.to_string(),
            code: account.code,
            name: account.name,
            description: account.description,
            account_type: account.account_type.to_string(),
            category: account.category.to_string(),
            subcategory: account.subcategory,
            is_active: account.is_active,
            parent_id: account.parent_id.map(|id| id.to_string()),
            balance: account.balance.to_string(),
            created_at: account.created_at.to_rfc3339(),
            updated_at: account.updated_at.to_rfc3339(),
        }
    }
}

// Command to get all accounts
#[tauri::command]
pub async fn get_accounts(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<Vec<AccountViewModel>, String> {
    let db_pool = &state.db_pool;
    let repo = AccountRepository::new(db_pool);

    match repo.find_all().await {
        Ok(accounts) => Ok(accounts.into_iter().map(AccountViewModel::from).collect()),
        Err(err) => Err(ErrorResponse::from(Error::Database(err)).into()),
    }
}

// Command to get an account by ID
#[tauri::command]
pub async fn get_account(
    id: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<Option<AccountViewModel>, String> {
    let db_pool = &state.db_pool;
    let repo = AccountRepository::new(db_pool);

    // Parse the UUID
    let account_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid UUID format: {}", e)),
    };

    match repo.find_by_id(account_id).await {
        Ok(Some(account)) => Ok(Some(AccountViewModel::from(account))),
        Ok(None) => Ok(None),
        Err(err) => Err(ErrorResponse::from(Error::Database(err)).into()),
    }
}

// Command to create a new account
#[tauri::command]
pub async fn create_account(
    new_account: NewAccountDto,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<AccountViewModel, String> {
    let db_pool = &state.db_pool;
    let repo = AccountRepository::new(db_pool);

    // Parse the account type
    let account_type = match AccountType::from_str(&new_account.account_type) {
        Some(t) => t,
        None => return Err(ErrorResponse::from(validation_error("Invalid account type")).into()),
    };

    // Parse the category
    let category = match AccountCategory::from_str(&new_account.category) {
        Some(c) => c,
        None => {
            return Err(ErrorResponse::from(validation_error("Invalid account category")).into())
        }
    };

    // Parse the parent ID if present
    let parent_id = if let Some(parent_id_str) = new_account.parent_id {
        if parent_id_str.is_empty() {
            None
        } else {
            match Uuid::parse_str(&parent_id_str) {
                Ok(id) => Some(id),
                Err(e) => return Err(format!("Invalid parent UUID format: {}", e)),
            }
        }
    } else {
        None
    };

    // Create the new account domain model
    let domain_new_account = NewAccount {
        code: new_account.code,
        name: new_account.name,
        description: new_account.description,
        account_type,
        category,
        subcategory: new_account.subcategory,
        parent_id,
    };

    // Create the account
    match repo.create(domain_new_account).await {
        Ok(account) => Ok(AccountViewModel::from(account)),
        Err(err) => Err(ErrorResponse::from(Error::Database(err)).into()),
    }
}

// Command to update an account
#[tauri::command]
pub async fn update_account(
    id: String,
    update_data: NewAccountDto,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<AccountViewModel, String> {
    let db_pool = &state.db_pool;
    let repo = AccountRepository::new(db_pool);

    // Parse the UUID
    let account_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid UUID format: {}", e)),
    };

    // Retrieve the existing account
    let mut account = match repo.find_by_id(account_id).await {
        Ok(Some(account)) => account,
        Ok(None) => return Err(ErrorResponse::from(not_found("Account")).into()),
        Err(err) => return Err(ErrorResponse::from(Error::Database(err)).into()),
    };

    // Parse the account type
    let account_type = match AccountType::from_str(&update_data.account_type) {
        Some(t) => t,
        None => return Err(ErrorResponse::from(validation_error("Invalid account type")).into()),
    };

    // Parse the category
    let category = match AccountCategory::from_str(&update_data.category) {
        Some(c) => c,
        None => {
            return Err(ErrorResponse::from(validation_error("Invalid account category")).into())
        }
    };

    // Parse the parent ID if present
    let parent_id = if let Some(parent_id_str) = update_data.parent_id {
        if parent_id_str.is_empty() {
            None
        } else {
            match Uuid::parse_str(&parent_id_str) {
                Ok(id) => Some(id),
                Err(e) => return Err(format!("Invalid parent UUID format: {}", e)),
            }
        }
    } else {
        None
    };

    // Update the account fields
    account.code = update_data.code;
    account.name = update_data.name;
    account.description = update_data.description;
    account.account_type = account_type;
    account.category = category;
    account.subcategory = update_data.subcategory;
    account.parent_id = parent_id;
    account.updated_at = Utc::now();

    // Save the updated account
    match repo.update(&account).await {
        Ok(()) => Ok(AccountViewModel::from(account)),
        Err(err) => Err(ErrorResponse::from(Error::Database(err)).into()),
    }
}

// Command to delete an account
#[tauri::command]
pub async fn delete_account(
    id: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<(), String> {
    let db_pool = &state.db_pool;
    let repo = AccountRepository::new(db_pool);

    // Parse the UUID
    let account_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid UUID format: {}", e)),
    };

    match repo.delete(account_id).await {
        Ok(()) => Ok(()),
        Err(err) => Err(ErrorResponse::from(Error::Database(err)).into()),
    }
}

// Command to toggle account active status
#[tauri::command]
pub async fn toggle_account_status(
    id: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<AccountViewModel, String> {
    let db_pool = &state.db_pool;
    let repo = AccountRepository::new(db_pool);

    // Parse the UUID
    let account_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid UUID format: {}", e)),
    };

    // Retrieve the existing account
    let mut account = match repo.find_by_id(account_id).await {
        Ok(Some(account)) => account,
        Ok(None) => return Err(ErrorResponse::from(not_found("Account")).into()),
        Err(err) => return Err(ErrorResponse::from(Error::Database(err)).into()),
    };

    // Toggle the active status
    account.is_active = !account.is_active;
    account.updated_at = Utc::now();

    // Save the updated account
    match repo.update(&account).await {
        Ok(()) => Ok(AccountViewModel::from(account)),
        Err(err) => Err(ErrorResponse::from(Error::Database(err)).into()),
    }
}

// Command to get root accounts (top-level)
#[tauri::command]
pub async fn get_root_accounts(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<Vec<AccountViewModel>, String> {
    let db_pool = &state.db_pool;
    let repo = AccountRepository::new(db_pool);

    match repo.find_roots().await {
        Ok(accounts) => Ok(accounts.into_iter().map(AccountViewModel::from).collect()),
        Err(err) => Err(ErrorResponse::from(Error::Database(err)).into()),
    }
}

// Command to get child accounts
#[tauri::command]
pub async fn get_child_accounts(
    parent_id: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<Vec<AccountViewModel>, String> {
    let db_pool = &state.db_pool;
    let repo = AccountRepository::new(db_pool);

    // Parse the UUID
    let account_id = match Uuid::parse_str(&parent_id) {
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid UUID format: {}", e)),
    };

    match repo.find_children(account_id).await {
        Ok(accounts) => Ok(accounts.into_iter().map(AccountViewModel::from).collect()),
        Err(err) => Err(ErrorResponse::from(Error::Database(err)).into()),
    }
}
