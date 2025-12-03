use dioxus::prelude::*;

/// The Start page component that will be rendered when the current route is `[Route::Start]`
#[component]
pub fn Start() -> Element {
    rsx! {
        div {
            id: "start",
            form {
                label { "Url" }
                input { type: "text", placeholder: "Url" }
                input { type: "submit", value: "Submit" }
            }
        }
    }
}
