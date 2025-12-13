use crate::core::data::{import_csv, lesson_view, ImportCsvForm, PhraseView};
use dioxus::fullstack::Form;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

#[component]
pub fn Lesson() -> Element {
    let importing = use_signal(|| false);
    let loader = use_loader(move || async move { lesson_view().await })?;
    let lesson_view = loader.read();
    rsx! {
        section { class: "section",
            h1{ class: "title", "Lesson" }
            h2{ class: "subtitle", {lesson_view.title.clone()} }
        }
        Phrases{ phrases: lesson_view.phrases.clone(), importing: importing.clone() }
        footer { class: "footer" }
        if *importing.read() {
            ImportDialog{ importing: importing.clone(), lesson_id: lesson_view.lesson_id }
        }
    }
}

#[component]
fn Phrases(phrases: Vec<PhraseView>, importing: WriteSignal<bool>) -> Element {
    let no_phrases = phrases.is_empty();
    rsx! {
        section { class: "section",
            h5 { class: "title is-5", "Phrases" }
            div { class: "container",
                div { class: "block",
                    if no_phrases {
                         "No phrases yet"
                    } else {
                        PhraseTable{ phrases }
                    }
                }
                if no_phrases {
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
}

#[component]
fn ImportDialog(importing: WriteSignal<bool>, lesson_id: i64) -> Element {
    const SAMPLE_CSV_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQjXD1Z1nrpTS60VhvlyI3Gha7bS-XP1r_nv3ITYbw4JBL-FA8SB6irRsVHhlEje5ZZT_H8uwFuRGgw/pub?gid=0&single=true&output=csv";
    rsx! {
        div { id: "import-dialog", class: "modal is-active",
            div { class: "modal-background" }
            form { class: "modal-card",
                onsubmit: move |evt| async move {
                    evt.prevent_default();
                    *importing.write() = false;
                    let data = evt.values().into_iter().collect::<HashMap<_, _>>();
                    let FormValue::Text(csv_url) = data.get("csv_url").unwrap() else { unreachable!() };
                    let FormValue::Text(lesson_id) = data.get("lesson_id").unwrap() else { unreachable!() };
                    let lesson_id = i64::from_str(lesson_id).unwrap();
                    let values: ImportCsvForm = ImportCsvForm { lesson_id, csv_url: csv_url.to_string() };
                    import_csv(Form(values)).await.expect("Failed to import CSV");
                    info!("Loader restarted")
                },
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "Import CSV" }
                    button { class: "delete", aria_label: "close", onclick: move |_| *importing.write() = false }
                }
                section { class: "modal-card-body",
                    input { type: "hidden", name: "lesson_id", value: lesson_id }
                    div { class: "field",
                        label { class: "label", "URL" }
                        div { class: "control is-expanded",
                            input {class: "input",type: "url",name: "csv_url",value: SAMPLE_CSV_URL}
                        }
                    }
                }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button { class: "button is-success", type: "submit",
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
