use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    rsx! {
        div { class: "container",
            h1 { class: "title", "Home"}
            button {
                class: "button is-primary",
                onclick: move |_| { nav.push(Route::Lesson {}); },
                "Lesson"
            }
        }
    }
}
