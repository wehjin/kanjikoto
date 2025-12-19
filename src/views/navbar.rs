use crate::Route;
use dioxus::prelude::*;

const LOGO_IMG: Asset = asset!("/assets/PlaygroundImage4.jpg");

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
        section { class: "section is-fullheight-with-navbar",
            Outlet::<Route> {}
        }
    }
}
