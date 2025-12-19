use crate::Route;
use dioxus::prelude::*;

const LOGO_IMG: Asset = asset!("/assets/PlaygroundImage4.jpg");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar", role: "navigation", aria_label: "main navigation",
            div { class: "navbar-brand",
                Link{ class: "navbar-item", to: Route::Home{}, img { src: LOGO_IMG, }}
                Link{ class: "navbar-item", to: Route::Lesson{}, "Lesson"}
            }
            div { class: "navbar-menu",
                div { class: "navbar-start" }
                div { class: "navbar-end",
                    Link{ class: "navbar-item", to: Route::Start {}, "Settings"}
                }
            }
        }
        Outlet::<Route> {}
    }
}
