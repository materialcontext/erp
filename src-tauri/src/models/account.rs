// src-tauri/models/account.rs

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use sqlx::Type;
use std::fmt;
use uuid::Uuid;

/// AccountType represents the different types of accounts in the chart of accounts
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "UPPERCASE")]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl PgHasArrayType for AccountType {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_varchar")
    }
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountType::Asset => write!(f, "ASSET"),
            AccountType::Liability => write!(f, "LIABILITY"),
            AccountType::Equity => write!(f, "EQUITY"),
            AccountType::Revenue => write!(f, "REVENUE"),
            AccountType::Expense => write!(f, "EXPENSE"),
        }
    }
}

impl AccountType {
    /// Convert a string to AccountType
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "ASSET" => Some(Self::Asset),
            "LIABILITY" => Some(Self::Liability),
            "EQUITY" => Some(Self::Equity),
            "REVENUE" => Some(Self::Revenue),
            "EXPENSE" => Some(Self::Expense),
            _ => None,
        }
    }

    /// Check if the account type is debit-normal
    pub fn is_debit_normal(&self) -> bool {
        matches!(self, Self::Asset | Self::Expense)
    }

    /// Check if the account type is credit-normal
    pub fn is_credit_normal(&self) -> bool {
        matches!(self, Self::Liability | Self::Equity | Self::Revenue)
    }
}

/// AccountCategory provides primary categorization of accounts
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "UPPERCASE")]
pub enum AccountCategory {
    // Asset categories
    CurrentAsset,
    FixedAsset,
    OtherAsset,

    // Liability categories
    CurrentLiability,
    LongTermLiability,
    OtherLiability,

    // Equity categories
    OwnerEquity,
    RetainedEarnings,

    // Revenue categories
    OperatingRevenue,
    NonOperatingRevenue,

    // Expense categories
    OperatingExpense,
    NonOperatingExpense,
}

impl PgHasArrayType for AccountCategory {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_varchar")
    }
}

impl fmt::Display for AccountCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentAsset => write!(f, "CURRENT_ASSET"),
            Self::FixedAsset => write!(f, "FIXED_ASSET"),
            Self::OtherAsset => write!(f, "OTHER_ASSET"),
            Self::CurrentLiability => write!(f, "CURRENT_LIABILITY"),
            Self::LongTermLiability => write!(f, "LONG_TERM_LIABILITY"),
            Self::OtherLiability => write!(f, "OTHER_LIABILITY"),
            Self::OwnerEquity => write!(f, "OWNER_EQUITY"),
            Self::RetainedEarnings => write!(f, "RETAINED_EARNINGS"),
            Self::OperatingRevenue => write!(f, "OPERATING_REVENUE"),
            Self::NonOperatingRevenue => write!(f, "NON_OPERATING_REVENUE"),
            Self::OperatingExpense => write!(f, "OPERATING_EXPENSE"),
            Self::NonOperatingExpense => write!(f, "NON_OPERATING_EXPENSE"),
        }
    }
}

impl AccountCategory {
    /// Convert a string to AccountCategory
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "CURRENT_ASSET" => Some(Self::CurrentAsset),
            "FIXED_ASSET" => Some(Self::FixedAsset),
            "OTHER_ASSET" => Some(Self::OtherAsset),
            "CURRENT_LIABILITY" => Some(Self::CurrentLiability),
            "LONG_TERM_LIABILITY" => Some(Self::LongTermLiability),
            "OTHER_LIABILITY" => Some(Self::OtherLiability),
            "OWNER_EQUITY" => Some(Self::OwnerEquity),
            "RETAINED_EARNINGS" => Some(Self::RetainedEarnings),
            "OPERATING_REVENUE" => Some(Self::OperatingRevenue),
            "NON_OPERATING_REVENUE" => Some(Self::NonOperatingRevenue),
            "OPERATING_EXPENSE" => Some(Self::OperatingExpense),
            "NON_OPERATING_EXPENSE" => Some(Self::NonOperatingExpense),
            _ => None,
        }
    }

    /// Get appropriate categories for a given account type
    pub fn for_account_type(account_type: AccountType) -> Vec<Self> {
        match account_type {
            AccountType::Asset => vec![Self::CurrentAsset, Self::FixedAsset, Self::OtherAsset],
            AccountType::Liability => vec![
                Self::CurrentLiability,
                Self::LongTermLiability,
                Self::OtherLiability,
            ],
            AccountType::Equity => vec![Self::OwnerEquity, Self::RetainedEarnings],
            AccountType::Revenue => vec![Self::OperatingRevenue, Self::NonOperatingRevenue],
            AccountType::Expense => vec![Self::OperatingExpense, Self::NonOperatingExpense],
        }
    }
}

/// Domain model for an Account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: AccountType,
    pub category: AccountCategory,
    pub subcategory: Option<String>,
    pub is_active: bool,
    pub parent_id: Option<Uuid>,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Data transfer object for account from database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub is_active: bool,
    pub parent_id: Option<Uuid>,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Struct for creating a new account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAccount {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: AccountType,
    pub category: AccountCategory,
    pub subcategory: Option<String>,
    pub parent_id: Option<Uuid>,
}

impl Account {
    /// Creates a new Account with default values
    pub fn new(new_account: NewAccount) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            code: new_account.code,
            name: new_account.name,
            description: new_account.description,
            account_type: new_account.account_type,
            category: new_account.category,
            subcategory: new_account.subcategory,
            is_active: true,
            parent_id: new_account.parent_id,
            balance: Decimal::ZERO,
            created_at: now,
            updated_at: now,
        }
    }

    /// Checks if the account is a debit-normal account
    pub fn is_debit_normal(&self) -> bool {
        self.account_type.is_debit_normal()
    }

    /// Checks if the account is a credit-normal account
    pub fn is_credit_normal(&self) -> bool {
        self.account_type.is_credit_normal()
    }

    /// Updates the account balance
    pub fn update_balance(&mut self, amount: Decimal) {
        self.balance += amount;
        self.updated_at = Utc::now();
    }
}

impl From<AccountDto> for Account {
    fn from(dto: AccountDto) -> Self {
        Self {
            id: dto.id,
            code: dto.code,
            name: dto.name,
            description: dto.description,
            account_type: AccountType::from_str(&dto.account_type).unwrap_or(AccountType::Asset),
            category: AccountCategory::from_str(&dto.category)
                .unwrap_or(AccountCategory::CurrentAsset),
            subcategory: dto.subcategory,
            is_active: dto.is_active,
            parent_id: dto.parent_id,
            balance: dto.balance,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

impl From<Account> for AccountDto {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            code: account.code,
            name: account.name,
            description: account.description,
            account_type: account.account_type.to_string(),
            category: account.category.to_string(),
            subcategory: account.subcategory,
            is_active: account.is_active,
            parent_id: account.parent_id,
            balance: account.balance,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}
