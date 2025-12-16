use crate::core::data::{import_csv, lesson_view, ImportDetails, PhraseView};
use dioxus::prelude::*;
use std::collections::HashMap;
#[component]
pub fn Lesson() -> Element {
    let mut current_lesson = use_loader(move || async move { lesson_view().await })?;
    let mut show_import_dialog = use_signal(|| false);
    let mut import_csv = use_action(move |details| async move {
        import_csv(details).await.expect("Failed to import CSV");
        current_lesson.restart();
        Ok(()) as Result<()>
    });
    rsx! {
        section { class: "section",
            match current_lesson.read().clone() {
                None => rsx! {
                    div { class: "block",
                        h1 { class: "title",
                            "Lesson"
                            button { class: "button ml-5",
                                onclick:  move |_| *show_import_dialog.write() = true,
                                "Import"
                            }
                        }
                    }
                    if show_import_dialog.read().clone() {
                        ImportDialog{
                            importing: show_import_dialog,
                            onimport: move |details| async move {
                                import_csv.call(details);
                            },
                        }
                    }
                },
                Some(lesson) => rsx! {
                    div { class: "block",
                        h1{ class: "title", {lesson.title} }
                    }
                    div { class: "block",
                        h5 { class: "title is-5", "Today" }
                        div { class: "container",
                            div { class: "columns",
                                div { class: "column m-3 is-flex is-flex-direction-column",
                                    div { class: "box-header",
                                        h6 { class: "title is-6 ml-3", "Ready"}
                                    }
                                    div { class: "box mt-3 is-flex-grow-1",
                                        p { class: "buttons",
                                            for _ in 0..12 {
                                                ReadyButton{}
                                            }
                                        }
                                    }
                                }
                                div { class: "column m-3 is-flex is-flex-direction-column",
                                    div { class: "box-header",
                                        h6 { class: "title is-6 ml-3", "Learned"}
                                    }
                                    div { class: "box mt-3 is-flex-grow-1",
                                        p { class: "buttons",
                                            for _ in 0..5 {
                                                LearnedIcon{}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "block",
                        h5 { class: "title is-5", "Phrases" }
                        div { class: "container",
                            if lesson.phrases.is_empty() {
                                div { class: "block", "No phrases yet" }
                            } else {
                                div { class: "block", PhraseTable{ phrases: lesson.phrases } }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ReadyButton() -> Element {
    rsx! {
        button { class: "button is-large is-primary is-outlined",
            span { class: "icon is-large",
                i { class: "fas fa-seedling fa-xl" }
            }
        }
    }
}

#[component]
fn LearnedIcon() -> Element {
    rsx! {
        span { class: "icon is-large has-text-warning",
            i { class: "fas fa-star fa-2xl" }
        }
    }
}

#[component]
fn PhraseTable(phrases: ReadSignal<Vec<PhraseView>>) -> Element {
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
                for phrase in phrases.iter() {
                    tr {
                        td { {phrase.prompt.clone()} }
                        td { {phrase.reading.clone()} }
                        td { {phrase.meaning.clone()} }
                    }
                }
            }
        }
    }
}

#[component]
fn ImportDialog(importing: WriteSignal<bool>, onimport: EventHandler<ImportDetails>) -> Element {
    const SAMPLE_CSV_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQjXD1Z1nrpTS60VhvlyI3Gha7bS-XP1r_nv3ITYbw4JBL-FA8SB6irRsVHhlEje5ZZT_H8uwFuRGgw/pub?gid=0&single=true&output=csv";
    rsx! {
        div { id: "import-dialog", class: "modal is-active",
            div { class: "modal-background" }
            form { class: "modal-card",
                onsubmit: move |evt| async move {
                    let data = evt.values().into_iter().collect::<HashMap<_, _>>();
                    let FormValue::Text(csv_url) = data.get("csv_url").unwrap() else { unreachable!() };
                    let details = ImportDetails { csv_url: csv_url.to_string() };
                    onimport.call(details);
                    evt.prevent_default();
                    *importing.write() = false;
                },
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "Import CSV" }
                    button { class: "delete", aria_label: "close", onclick: move |_| *importing.write() = false }
                }
                section { class: "modal-card-body",
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
