use crate::Route;
use async_std::task::sleep;
use dioxus::html::input::list;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Define types for our dynamic data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SystemStatus {
    database_connected: bool,
    version: String,
    last_backup: Option<String>,
    fiscal_year: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct RecentActivity {
    id: String,
    action: String,
    description: String,
    timestamp: String,
    user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct FinancialMetric {
    name: String,
    value: String,
    change: f64, // percentage change
    period: String,
}

#[component]
pub fn Home() -> Element {
    // State for our dynamic components
    let mut system_status = use_signal(|| Option::<SystemStatus>::None);
    let mut recent_activities = use_signal(Vec::<RecentActivity>::new);
    let mut financial_metrics = use_signal(Vec::<FinancialMetric>::new);

    // Individual loading states
    let mut status_loading = use_signal(|| true);
    let mut activities_loading = use_signal(|| true);
    let mut metrics_loading = use_signal(|| true);

    // Fetch system status
    use_effect(move || {
        spawn(async move {
            // For now, we'll simulate with a delay
            sleep(Duration::from_millis(800)).await;

            // Mock data
            system_status.set(Some(SystemStatus {
                database_connected: true,
                version: "1.0.0".to_string(),
                last_backup: Some("2025-03-04T06:00:00Z".to_string()),
                fiscal_year: "2025".to_string(),
            }));

            status_loading.set(false);
        });
    });

    // Fetch recent activities
    use_effect(move || {
        spawn(async move {
            // Simulate API call
            sleep(Duration::from_millis(1200)).await;

            // Mock data
            recent_activities.set(vec![
                RecentActivity {
                    id: "act1".to_string(),
                    action: "Journal Entry".to_string(),
                    description: "Created invoice payment #INV-2025-042".to_string(),
                    timestamp: "10 minutes ago".to_string(),
                    user: "John Doe".to_string(),
                },
                RecentActivity {
                    id: "act2".to_string(),
                    action: "Account Created".to_string(),
                    description: "Added new expense account 'Office Supplies'".to_string(),
                    timestamp: "2 hours ago".to_string(),
                    user: "Jane Smith".to_string(),
                },
                RecentActivity {
                    id: "act3".to_string(),
                    action: "Report Generated".to_string(),
                    description: "Monthly P&L statement for February 2025".to_string(),
                    timestamp: "Yesterday".to_string(),
                    user: "John Doe".to_string(),
                },
            ]);

            activities_loading.set(false);
        });
    });

    // Fetch financial metrics
    use_effect(move || {
        spawn(async move {
            // Simulate API call
            sleep(Duration::from_millis(1500)).await;

            // Mock data
            financial_metrics.set(vec![
                FinancialMetric {
                    name: "Revenue".to_string(),
                    value: "$125,430.00".to_string(),
                    change: 5.2,
                    period: "This Month".to_string(),
                },
                FinancialMetric {
                    name: "Expenses".to_string(),
                    value: "$78,230.00".to_string(),
                    change: -2.1,
                    period: "This Month".to_string(),
                },
                FinancialMetric {
                    name: "Net Profit".to_string(),
                    value: "$47,200.00".to_string(),
                    change: 12.5,
                    period: "This Month".to_string(),
                },
                FinancialMetric {
                    name: "Cash Balance".to_string(),
                    value: "$253,890.00".to_string(),
                    change: 3.7,
                    period: "Current".to_string(),
                },
            ]);

            metrics_loading.set(false);
        });
    });

    let mut show_all_activities = use_signal(|| false);

    // needs to live long enough to be used alter
    let activities = recent_activities.read();

    // list recent activities onclick
    let list_recent_activities = {
        let show_all = show_all_activities.read();
        
        // Determine how many activities to show
        let activities_to_show = if *show_all {
            activities.len()
        } else {
            activities.len().min(3) // Only show up to 3 activities when not expanded
        };
        
        activities.iter()
            .take(activities_to_show)
            .map(|activity| {
                // Extract the first character as a string
                let first_char = activity.action.chars().next()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "A".to_string());
                
                rsx! {
                    div { class: "py-3 flex items-start",
                        div { class: "flex-shrink-0 mr-3",
                            div { class: "h-8 w-8 rounded-full bg-indigo-100 flex items-center justify-center",
                                span { class: "text-indigo-600 text-sm font-medium", 
                                    {first_char}
                                }
                            }
                        }
                        div { class: "min-w-0 flex-1",
                            p { class: "text-sm font-medium text-gray-900",
                                "{activity.action}"
                            }
                            p { class: "text-sm text-gray-500",
                                "{activity.description}"
                            }
                            div { class: "mt-1 flex items-center text-xs text-gray-500",
                                span { "{activity.user} • {activity.timestamp}" }
                            }
                        }
                    }
                }
            })
    };

    // Render the component
    rsx! {
        div { class: "space-y-6",
            // Hero section with welcome message
            div { class: "bg-white p-6 rounded-lg shadow-md",
                h1 { class: "text-2xl font-bold text-gray-800 mb-2", "Welcome to Your ERP System" }
                p { class: "text-gray-600",
                    "Manage your business operations efficiently with our integrated platform."
                }
                div { class: "mt-4",
                    Link {
                        to: Route::Dashboard {},
                        class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                        "Go to Dashboard"
                    }
                }
            }

            // Financial metrics
            div { class: "bg-white p-6 rounded-lg shadow-md",
                h2 { class: "text-lg font-medium text-gray-900 mb-4", "Financial Overview" }

                {if *metrics_loading.read() {
                    rsx! {
                        div { class: "flex justify-center items-center h-24",
                            div { class: "animate-pulse text-gray-400", "Loading metrics..." }
                        }
                    }
                } else {
                    rsx! {
                        div { class: "grid grid-cols-1 md:grid-cols-4 gap-4",
                            {financial_metrics.read().iter().map(|metric| {
                                let change_color = if metric.change >= 0.0 { "text-green-600" } else { "text-red-600" };
                                let change_icon = if metric.change >= 0.0 { "↑" } else { "↓" };

                                rsx! {
                                    div { class: "border rounded-md p-4",
                                        p { class: "text-sm text-gray-500", "{metric.name}" }
                                        p { class: "text-xl font-semibold", "{metric.value}" }
                                        div { class: "flex items-center mt-1",
                                            span { class: "{change_color} text-sm font-medium",
                                                "{change_icon} {metric.change.abs()}%"
                                            }
                                            span { class: "text-xs text-gray-500 ml-2", "{metric.period}" }
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }}
            }

            // Quick access cards
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                // Accounting card
                div { class: "bg-white overflow-hidden shadow rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        h3 { class: "text-lg font-medium text-gray-900", "Accounting" }
                        p { class: "mt-1 text-sm text-gray-600",
                            "Manage your chart of accounts, journal entries, and financial reports."
                        }
                        div { class: "mt-4",
                            Link {
                                to: Route::Accounting {},
                                class: "text-sm font-medium text-indigo-600 hover:text-indigo-500",
                                "Access accounting module "
                                span { "→" }
                            }
                        }
                    }
                }

                // Ledger card
                div { class: "bg-white overflow-hidden shadow rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        h3 { class: "text-lg font-medium text-gray-900", "General Ledger" }
                        p { class: "mt-1 text-sm text-gray-600",
                            "View and manage your general ledger entries and account balances."
                        }
                        div { class: "mt-4",
                            Link {
                                to: Route::Ledger {},
                                class: "text-sm font-medium text-indigo-600 hover:text-indigo-500",
                                "Go to ledger "
                                span { "→" }
                            }
                        }
                    }
                }

                // Journal card
                div { class: "bg-white overflow-hidden shadow rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        h3 { class: "text-lg font-medium text-gray-900", "Journal Entries" }
                        p { class: "mt-1 text-sm text-gray-600",
                            "Create and manage journal entries for your financial transactions."
                        }
                        div { class: "mt-4",
                            Link {
                                to: Route::Journal {},
                                class: "text-sm font-medium text-indigo-600 hover:text-indigo-500",
                                "Manage journal entries "
                                span { "→" }
                            }
                        }
                    }
                }
            }

            // Recent activity section
            div { class: "bg-white p-6 rounded-lg shadow-md",
                h2 { class: "text-lg font-medium text-gray-900 mb-4", "Recent Activity" }

                {if *activities_loading.read() {
                    rsx! {
                        div { class: "flex justify-center items-center h-24",
                            div { class: "animate-pulse text-gray-400", "Loading activities..." }
                        }
                    }
                } else if recent_activities.read().is_empty() {
                    rsx! {
                        div { class: "text-center py-4 text-gray-500",
                            "No recent activities to display"
                        }
                    }
                } else {
                    rsx! {
                        div { class: "divide-y divide-gray-200",
                            // Only render the list_recent_activities if show_all_activities is true
                            {if *show_all_activities.read() {
                                rsx! { {list_recent_activities} }
                            } else {
                                rsx! {}  // Empty fragment when not showing activities
                            }}
                        }
                        div { class: "mt-4 text-center",
                            button {
                                class: "text-sm font-medium text-indigo-600 hover:text-indigo-500",
                                onclick: move |_| show_all_activities.set(!show_all_activities()),
                                if *show_all_activities.read() { "Show fewer" } else { "View all activity" }
                            }
                        }
                    }
                }}
            }

            // System status section
            div { class: "bg-white p-6 rounded-lg shadow-md",
                h2 { class: "text-lg font-medium text-gray-900 mb-4", "System Status" }

                {if *status_loading.read() {
                    rsx! {
                        div { class: "flex justify-center items-center h-24",
                            div { class: "animate-pulse text-gray-400", "Loading system status..." }
                        }
                    }
                } else if let Some(status) = system_status.read().as_ref() {
                    let db_status_color = if status.database_connected { "text-green-600" } else { "text-red-600" };
                    let db_status_text = if status.database_connected { "Connected" } else { "Disconnected" };

                    let formatted_backup = status.last_backup.as_ref()
                        .map(|date| {
                            // In a real app, you would parse and format this properly
                            "Today 06:00 AM".to_string()
                        })
                        .unwrap_or_else(|| "Never".to_string());

                    rsx! {
                        div { class: "grid grid-cols-1 md:grid-cols-4 gap-4",
                            div { class: "border rounded-md p-4 text-center",
                                p { class: "text-sm text-gray-500", "Database" }
                                p { class: "{db_status_color} text-lg font-semibold", "{db_status_text}" }
                            }
                            div { class: "border rounded-md p-4 text-center",
                                p { class: "text-sm text-gray-500", "Version" }
                                p { class: "text-lg font-semibold", "v{status.version}" }
                            }
                            div { class: "border rounded-md p-4 text-center",
                                p { class: "text-sm text-gray-500", "Last Backup" }
                                p { class: "text-lg font-semibold", "{formatted_backup}" }
                            }
                            div { class: "border rounded-md p-4 text-center",
                                p { class: "text-sm text-gray-500", "Fiscal Year" }
                                p { class: "text-lg font-semibold", "{status.fiscal_year}" }
                            }
                        }
                    }
                } else {
                    rsx! {
                        div { class: "text-center py-4 text-gray-500",
                            "Unable to retrieve system status"
                        }
                    }
                }}
            }
        }
    }
}
