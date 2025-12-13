use crate::core::data::{lesson_view, PhraseView};
use dioxus::prelude::*;

#[component]
pub fn Lesson() -> Element {
    let lesson_view =
        use_resource(|| async move { lesson_view().await.expect("Failed to fetch lesson") });

    let lesson_view = lesson_view.cloned();
    rsx! {
            match lesson_view {
                Some(lesson_view) => rsx! {
                    section { class: "section",
                        h1{ class: "title", "Lesson" }
                        h2{ class: "subtitle", {lesson_view.title} }
                    }
                    Phrases{ phrases: lesson_view.phrases }
                    footer { class: "footer" }
                },
                None => rsx! {
                    section { class: "section",
                        h1{ class: "title has-skeleton", "Loading..." }
                    }
                },
            }

    }
}

#[component]
fn Phrases(phrases: Vec<PhraseView>) -> Element {
    rsx! {
        section { class: "section",
            h5 { class: "title is-5", "Phrases" }
            div { class: "container",
                div { class: "block",
                    if phrases.is_empty() {
                         "No phrases yet"
                    } else {
                        table { class: "table is-striped is-hoverable is-fullwidth",
                            thead {
                                tr {
                                    th { "Prompt" }
                                    th { "Reading" }
                                    th { "Meaning" }
                                }
                            }
                            tbody {
                                for phrase in phrases {
                                    tr {
                                        td { {phrase.prompt} }
                                        td { {phrase.reading} }
                                        td { {phrase.meaning} }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "block",
                    button { class: "button", "Add phrase" }
                }
            }
        }
    }
}
