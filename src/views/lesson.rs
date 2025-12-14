use crate::core::data::{
    import_csv, lesson_view, ImportDetails, LessonView, LessonViewStoreExt, PhraseView,
};
use dioxus::fullstack::Form;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

#[component]
pub fn Lesson() -> Element {
    let mut lesson = use_store(|| LessonView {
        lesson_id: i64::MAX,
        title: "".to_string(),
        phrases: vec![],
    });
    let mut loader = use_loader(move || async move { lesson_view().await })?;
    use_effect(move || {
        let next_lesson = loader.read().clone();
        lesson.set(next_lesson);
    });
    let importing = use_signal(|| false);
    let mut import_csv = use_action(move |details| async move {
        import_csv(Form(details))
            .await
            .expect("Failed to import CSV");
        loader.restart();
        Ok(()) as Result<()>
    });
    rsx! {
        if lesson.lesson_id() == i64::MAX {
            section { class: "section",
                h1{ class: "title", "Lesson" }
                div { class: "skeleton-block"}
            }
        } else {
            section { class: "section",
                h1{ class: "title", "Lesson" }
                h2{ class: "subtitle", {lesson.title()} }
            }
            Phrases{ phrases: lesson.phrases(), importing: importing.clone() }
            footer { class: "footer" }
            if importing.read().clone() {
                ImportDialog{
                    lesson_id: lesson.lesson_id().read().clone(),
                    importing: importing.clone(),
                    onimport: move |details| async move {
                        import_csv.call(details);
                    },
                }
            }
        }
    }
}

#[component]
fn Phrases(phrases: ReadSignal<Vec<PhraseView>>, importing: WriteSignal<bool>) -> Element {
    rsx! {
        section { class: "section",
            h5 { class: "title is-5", "Phrases" }
            div { class: "container",
                if phrases.is_empty() {
                    div { class: "block", "No phrases yet" }
                    div { class: "block",
                        button { class: "button",
                            onclick:  move |_| *importing.write() = true,
                            "Import CSV"
                        }
                    }
                } else {
                    div { class: "block", PhraseTable{ phrases } }
                }
            }
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
fn ImportDialog(
    lesson_id: i64,
    importing: WriteSignal<bool>,
    onimport: EventHandler<ImportDetails>,
) -> Element {
    const SAMPLE_CSV_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQjXD1Z1nrpTS60VhvlyI3Gha7bS-XP1r_nv3ITYbw4JBL-FA8SB6irRsVHhlEje5ZZT_H8uwFuRGgw/pub?gid=0&single=true&output=csv";
    rsx! {
        div { id: "import-dialog", class: "modal is-active",
            div { class: "modal-background" }
            form { class: "modal-card",
                onsubmit: move |evt| async move {
                    let data = evt.values().into_iter().collect::<HashMap<_, _>>();
                    let FormValue::Text(csv_url) = data.get("csv_url").unwrap() else { unreachable!() };
                    let FormValue::Text(lesson_id) = data.get("lesson_id").unwrap() else { unreachable!() };
                    let lesson_id = i64::from_str(lesson_id).unwrap();
                    let details = ImportDetails { lesson_id, csv_url: csv_url.to_string() };
                    onimport.call(details);
                    evt.prevent_default();
                    *importing.write() = false;
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
