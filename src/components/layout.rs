use crate::Route;
use dioxus::prelude::*;

/// Main application layout that wraps all pages
#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100",
            // Navigation component
            NavBar {}

            // Main content
            main { class: "container mx-auto py-6 sm:px-6 lg:px-8",
                Outlet::<Route> {}
            }
        }
    }
}

/// Navigation bar component
#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "bg-white shadow-sm",
            div { class: "container mx-auto px-4",
                div { class: "flex justify-between h-16",
                    div { class: "flex items-center",
                        div { class: "flex-shrink-0 flex items-center",
                            h1 { class: "text-xl font-bold text-gray-800", "ERP System" }
                        }
                        div { class: "hidden md:ml-6 md:flex md:space-x-8",
                            Link {
                                to: Route::Home {},
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                "Home"
                            }
                            Link {
                                to: Route::Dashboard {},
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                "Dashboard"
                            }
                            Link {
                                to: Route::Accounting {},
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                "Accounting"
                            }
                            Link {
                                to: Route::Settings {},
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                "Settings"
                            }
                        }
                    }
                }
            }
        }
    }
}
