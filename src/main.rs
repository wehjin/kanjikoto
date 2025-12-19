use dioxus::prelude::*;
use views::{Home, Lesson, Navbar, Start};

mod components;
mod core;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/lesson")]
    Lesson {},
    #[route("/start")]
    Start {},
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link{ rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css"}
        document::Link{ rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css"}
        Router::<Route> {}
    }
}
