use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
