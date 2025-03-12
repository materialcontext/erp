#![allow(non_snake_case)]
use dioxus::events::{FormData, FormEvent};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

// Account model for the frontend
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewAccountModel {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub account_type: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub parent_id: Option<String>,
}

// API calls
async fn fetch_accounts() -> Result<Vec<AccountViewModel>, String> {
    let result =
        crate::services::tauri::invoke::<(), Vec<AccountViewModel>>("get_accounts", &()).await;

    match result {
        Ok(accounts) => Ok(accounts),
        Err(e) => Err(format!("Failed to fetch accounts: {}", e)),
    }
}

async fn create_account(new_account: NewAccountModel) -> Result<AccountViewModel, String> {
    let result = crate::services::tauri::invoke::<NewAccountModel, AccountViewModel>(
        "create_account",
        &new_account,
    )
    .await;

    match result {
        Ok(account) => Ok(account),
        Err(e) => Err(format!("Failed to create account: {}", e)),
    }
}

#[component]
pub fn AccountsComponent() -> Element {
    let mut accounts = use_signal(Vec::<AccountViewModel>::new);
    let mut error_message = use_signal(|| Option::<String>::None);
    let mut is_loading = use_signal(|| true);
    let mut show_form = use_signal(|| false);

    let mut new_account = use_signal(|| NewAccountModel {
        code: String::new(),
        name: String::new(),
        description: None,
        account_type: "ASSET".to_string(),
        category: "CURRENT_ASSET".to_string(),
        subcategory: None,
        parent_id: None,
    });

    // Load accounts on component mount
    use_effect(move || {
        is_loading.set(true);

        spawn(async move {
            match fetch_accounts().await {
                Ok(fetched_accounts) => {
                    accounts.set(fetched_accounts);
                    error_message.set(None);
                }
                Err(err) => {
                    error_message.set(Some(err.to_string()));
                }
            }
            is_loading.set(false);
        });
    });

    let account_types = vec!["ASSET", "LIABILITY", "EQUITY", "REVENUE", "EXPENSE"];

    let category_map: Rc<HashMap<&str, Vec<&str>>> = Rc::new({
        let mut map = HashMap::new();
        map.insert("ASSET", vec!["CURRENT_ASSET", "FIXED_ASSET", "OTHER_ASSET"]);
        map.insert(
            "LIABILITY",
            vec![
                "CURRENT_LIABILITY",
                "LONG_TERM_LIABILITY",
                "OTHER_LIABILITY",
            ],
        );
        map.insert("EQUITY", vec!["OWNER_EQUITY", "RETAINED_EARNINGS"]);
        map.insert(
            "REVENUE",
            vec!["OPERATING_REVENUE", "NON_OPERATING_REVENUE"],
        );
        map.insert(
            "EXPENSE",
            vec!["OPERATING_EXPENSE", "NON_OPERATING_EXPENSE"],
        );
        map
    });

    let categories = {
        let category_map = Rc::clone(&category_map);
        match category_map.get(new_account.read().account_type.as_str()) {
            Some(cats) => cats.clone(),
            None => vec![],
        }
    };

    let handle_submit = move |event: FormEvent| {
        event.prevent_default();

        is_loading.set(true);

        let new_account_clone = new_account.read().clone();

        spawn(async move {
            match create_account(new_account_clone).await {
                Ok(created_account) => {
                    accounts.set({
                        let mut updated_accounts = accounts().clone();
                        updated_accounts.push(created_account);
                        updated_accounts
                    });
                    show_form.set(false);
                    new_account.set(NewAccountModel {
                        code: String::new(),
                        name: String::new(),
                        description: None,
                        account_type: "ASSET".to_string(),
                        category: "CURRENT_ASSET".to_string(),
                        subcategory: None,
                        parent_id: None,
                    });
                    error_message.set(None);
                }
                Err(err) => {
                    error_message.set(Some(err.to_string()));
                }
            }
            is_loading.set(false);
        });
    };

    let toggle_form = move |_| {
        show_form.set(!show_form());
    };

    let category_map_clone = Rc::clone(&category_map);
    let account_type_options = account_types.iter().map(|acct_type| {
        rsx! {
            option { value: "{acct_type}", "{acct_type}" }
        }
    });

    let category_options = categories.iter().map(|category| {
        rsx! {
            option { value: "{category}", "{category}" }
        }
    });

    let account_row_read = accounts.read();
    let account_rows = account_row_read.iter().map(|account| {
        rsx! {
            tr { key: "{account.id}",
                td { class: "py-2 px-4 border-b", "{account.code}" }
                td { class: "py-2 px-4 border-b", "{account.name}" }
                td { class: "py-2 px-4 border-b", "{account.account_type}" }
                td { class: "py-2 px-4 border-b", "{account.category}" }
                td { class: "py-2 px-4 border-b text-right", "{account.balance}" }
                td { class: "py-2 px-4 border-b text-center",
                    span {
                        class: if account.is_active {
                            "inline-block px-2 py-1 text-xs font-semibold text-green-700 bg-green-100 rounded-full"
                        } else {
                            "inline-block px-2 py-1 text-xs font-semibold text-red-700 bg-red-100 rounded-full"
                        },
                        {if account.is_active { "Active" } else { "Inactive" }}
                    }
                }
                td { class: "py-2 px-4 border-b text-center",
                    button {
                        class: "text-blue-500 hover:text-blue-700 mr-2",
                        // onclick: move |_| view_account(account.id.clone()),
                        "View"
                    }
                    button {
                        class: "text-green-500 hover:text-green-700",
                        // onclick: move |_| edit_account(account.id.clone()),
                        "Edit"
                    }
                }
            }
        }
    });

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-2xl font-bold mb-4", "Chart of Accounts" }

            {match &*error_message.read() {

                Some(error) => rsx! {
                    div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                        span { class: "block sm:inline", "{error}" }
                    }
                },
                None => rsx! {}
            }}

            div { class: "mb-4 flex justify-between",
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: toggle_form,
                    {if *show_form.read() { "Cancel" } else { "Add New Account" }}
                }
            }

            {if *show_form.read() {
                rsx! {
                    form { class: "bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4", onsubmit: handle_submit,
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div { class: "mb-4",
                                label { class: "block text-gray-700 text-sm font-bold mb-2", r#for: "code", "Account Code" }
                                input {
                                    id: "code",
                                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                    r#type: "text",
                                    placeholder: "e.g., 1000",
                                    required: "true",
                                    value: "{new_account.read().code}",
                                    oninput: move |event: Event<FormData>| {
                                        let mut account = new_account().clone();
                                        account.code = event.value().clone();
                                        new_account.set(account);
                                    }
                                }
                            }
                            div { class: "mb-4",
                                label { class: "block text-gray-700 text-sm font-bold mb-2", r#for: "name", "Account Name" }
                                input {
                                    id: "name",
                                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                    r#type: "text",
                                    placeholder: "e.g., Cash",
                                    required: "true",
                                    value: "{new_account.read().name}",
                                    oninput: move |event: Event<FormData>| {
                                        let mut account = new_account().clone();
                                        account.name = event.value().clone();
                                        new_account.set(account);
                                    }
                                }
                            }
                            div { class: "mb-4",
                                label { class: "block text-gray-700 text-sm font-bold mb-2", r#for: "description", "Description" }
                                input {
                                    id: "description",
                                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                    r#type: "text",
                                    placeholder: "Optional description",
                                    value: "{new_account.read().description.clone().unwrap_or_default()}",
                                    oninput: move |event: Event<FormData>| {
                                        let mut account = new_account().clone();
                                        account.description = if event.value().is_empty() {
                                            None
                                        } else {
                                            Some(event.value().clone())
                                        };
                                        new_account.set(account);
                                    }
                                }
                            }
                            div { class: "mb-4",
                                label { class: "block text-gray-700 text-sm font-bold mb-2", r#for: "accountType", "Account Type" }
                                select {
                                    id: "accountType",
                                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                    required: "true",
                                    value: "{new_account.read().account_type}",
                                    onchange: move |event| {
                                         let account_type = event.value().clone();
                                         let default_category = match category_map_clone.get(account_type.as_str()) {
                                             Some(cats) if !cats.is_empty() => cats[0].to_string(),
                                             _ => String::new(),
                                         };

                                         let mut account_model = new_account().clone();
                                         account_model.account_type = account_type;
                                         account_model.category = default_category;
                                         new_account.set(account_model);
                                     },
                                    {account_type_options}
                                }
                            }
                            div { class: "mb-4",
                                label { class: "block text-gray-700 text-sm font-bold mb-2", r#for: "category", "Category" }
                                select {
                                    id: "category",
                                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                                    required: "true",
                                    value: "{new_account.read().category}",
                                    onchange: move |event: Event<FormData>| {
                                        let mut account = new_account().clone();
                                        account.category = event.value().clone();
                                        new_account.set(account);
                                    },
                                    {category_options}
                                }
                            }
                        }
                        div { class: "flex items-center justify-between mt-4",
                            button {
                                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                                r#type: "submit",
                                disabled: *is_loading.read(),
                                {if *is_loading.read() { "Saving..." } else { "Create Account" }}
                            }
                        }
                    }
                }
            } else {
                rsx! {}
            }}

            {if *is_loading.read() && accounts.read().is_empty() {
                rsx! {
                    div { class: "text-center p-4",
                        "Loading accounts..."
                    }
                }
            } else if accounts.read().is_empty() {
                rsx! {
                    div { class: "text-center p-4 bg-gray-100 rounded",
                        "No accounts found. Create your first account to get started."
                    }
                }
            } else {
                rsx! {
                    div { class: "overflow-x-auto",
                        table { class: "min-w-full bg-white",
                            thead { class: "bg-gray-100",
                                tr {
                                    th { class: "py-2 px-4 border-b text-left", "Code" }
                                    th { class: "py-2 px-4 border-b text-left", "Name" }
                                    th { class: "py-2 px-4 border-b text-left", "Type" }
                                    th { class: "py-2 px-4 border-b text-left", "Category" }
                                    th { class: "py-2 px-4 border-b text-right", "Balance" }
                                    th { class: "py-2 px-4 border-b text-center", "Status" }
                                    th { class: "py-2 px-4 border-b text-center", "Actions" }
                                }
                            }
                            tbody {
                                {account_rows}
                            }
                        }
                    }
                }
            }}
        }
    }
}
