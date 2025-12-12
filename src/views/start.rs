use crate::core::data;
use dioxus::prelude::*;

/// The Start page component that will be rendered when the current route is `[Route::Start]`
#[component]
pub fn Start() -> Element {
    let users = use_resource(move || async move {
        let users = data::users().await.unwrap();
        users.into_iter().map(|user| user.id).collect::<Vec<_>>()
    });
    rsx! {
        div {
            id: "start",
            p {
                "Users"
                match users.cloned() {
                    None => rsx! {
                        "Loading..."
                    },
                    Some(users) => rsx! {
                        for user in users.iter() {
                            p {"{user}"}
                        }
                    }
                }
            }
            form {
                label { "Url" }
                input { type: "text", placeholder: "Url" }
                input { type: "submit", value: "Submit" }
            }
        }
    }
}
