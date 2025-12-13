use crate::core::data::{lesson_view, PhraseView};
use dioxus::prelude::*;

#[component]
pub fn Lesson() -> Element {
    let importing = use_signal(|| false);
    let lesson_view =
        use_resource(|| async move { lesson_view().await.expect("Failed to fetch lesson") });
    rsx! {
            match lesson_view.cloned() {
                Some(lesson_view) => rsx! {
                    section { class: "section",
                        h1{ class: "title", "Lesson" }
                        h2{ class: "subtitle", {lesson_view.title} }
                    }
                    Phrases{ phrases: lesson_view.phrases, importing: importing.clone() }
                    footer { class: "footer" }
                    if importing.cloned() {
                        ImportDialog{ importing: importing.clone() }
                    }
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
fn Phrases(phrases: Vec<PhraseView>, importing: WriteSignal<bool>) -> Element {
    rsx! {
        section { class: "section",
            h5 { class: "title is-5", "Phrases" }
            div { class: "container",
                div { class: "block",
                    if phrases.is_empty() {
                         "No phrases yet"
                    } else {
                        PhraseTable{ phrases }
                    }
                }
                div { class: "block",
                    button { class: "button",
                        onclick:  move |_| *importing.write() = true,
                        "Import CSV"
                    }
                }
            }
        }
    }
}

#[component]
fn ImportDialog(importing: WriteSignal<bool>) -> Element {
    const CSV_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQjXD1Z1nrpTS60VhvlyI3Gha7bS-XP1r_nv3ITYbw4JBL-FA8SB6irRsVHhlEje5ZZT_H8uwFuRGgw/pub?gid=0&single=true&output=csv";
    rsx! {
        div { id: "import-dialog", class: "modal is-active",
            div { class: "modal-background" }
            form { class: "modal-card",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "Import CSV" }
                    button { class: "delete", aria_label: "close", onclick: move |_| *importing.write() = false }
                }
                section { class: "modal-card-body",
                    div { class: "field",
                        label { class: "label", "URL" }
                        div { class: "control is-expanded",
                            input { class: "input", type: "url", value: CSV_URL }
                        }
                    }
                }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button { class: "button is-success",
                            onclick: move |_| {
                                *importing.write() = false;
                            },
                            "Import"
                        }
                        button { class: "button", onclick: move |_| *importing.write() = false,
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PhraseTable(phrases: Vec<PhraseView>) -> Element {
    rsx! {
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
