mod app;
mod components;
mod services;

use crate::components::AppLayout;
use crate::components::Home;
use app::App;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

#[derive(Routable, Clone, Debug, PartialEq)]
enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/accounting")]
    Accounting {},
    #[route("/accounting/ledger")]
    Ledger {},
    #[route("/accounting/journal")]
    Journal {},
    #[route("/settings")]
    Settings {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn Login() -> Element {
    todo!()
}

#[component]
fn Dashboard() -> Element {
    todo!()
}

#[component]
fn Accounting() -> Element {
    todo!()
}

#[component]
fn Ledger() -> Element {
    todo!()
}

#[component]
fn Journal() -> Element {
    todo!()
}

#[component]
fn Settings() -> Element {
    todo!()
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
