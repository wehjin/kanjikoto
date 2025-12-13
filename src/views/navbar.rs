use crate::Route;
use dioxus::prelude::*;

const LOGO_IMG: Asset = asset!("/assets/PlaygroundImage4.jpg");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
#[component]
pub fn Navbar() -> Element {
    let seed = 1;
    rsx! {
        nav { class: "navbar", role: "navigation", aria_label: "main navigation",
            div { class: "navbar-brand",
                Link{ class: "navbar-item", to: Route::Review{}, img { src: LOGO_IMG, }}
            }
            div { class: "navbar-menu is-active",
                div { class: "navbar-start",
                    Link{ class: "navbar-item", to: Route::Review{}, "Review"}
                    Link{ class: "navbar-item", to: Route::Answers{ seed }, "Worksheet"}
                    Link{ class: "navbar-item", to: Route::Practice{}, "Practice"}
                    Link{ class: "navbar-item", to: Route::Lesson{}, "Lesson"}
                }
                div { class: "navbar-end",
                    Link{ class: "navbar-item", to: Route::Start {}, "Settings"}
                }
            }
        }
        Outlet::<Route> {}
        footer { class: "footer has-background-white" }
    }
}
