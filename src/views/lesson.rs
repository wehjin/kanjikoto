use crate::core::data::{import_csv, lesson_view, ImportDetails, PhraseView};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum LessonTab {
    Today,
    Phrases,
}

#[component]
fn LessonTabs(current_tab: Signal<LessonTab>) -> Element {
    #[component]
    fn TabItem(tab: LessonTab, current_tab: Signal<LessonTab>) -> Element {
        let is_active = tab == current_tab.read().clone();
        let title = match tab {
            LessonTab::Today => "Today",
            LessonTab::Phrases => "Phrases",
        };
        rsx! {
            li { class: if is_active {"is-active"},
                a {
                    onclick: move |_| *current_tab.write() = tab,
                    {title}
                }
            }
        }
    }
    rsx! {
        div { class: "tabs is-medium",
            ul {
                TabItem { tab: LessonTab::Today, current_tab: current_tab.clone() }
                TabItem { tab: LessonTab::Phrases, current_tab: current_tab.clone() }
            }
        }
    }
}

#[component]
pub fn Lesson() -> Element {
    let current_tab = use_signal(|| LessonTab::Today);
    let mut current_lesson = use_loader(move || async move { lesson_view().await })?;
    let current_lesson_id = use_memo(move || current_lesson().map(|it| it.lesson_id));
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
                    LessonTabs{ current_tab }
                    div { class: "container",
                        match current_tab() {
                            LessonTab::Today => rsx! {
                                TodaySection { lesson_id: current_lesson_id }
                            },
                            LessonTab::Phrases => rsx! {
                                if lesson.phrases.is_empty() {
                                    "No phrases yet"
                                } else {
                                    PhraseTable{ phrases: lesson.phrases }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TodaySection(lesson_id: ReadSignal<Option<i64>>) -> Element {
    match lesson_id() {
        None => rsx! {
            div { class: "skeleton-block" }
        },
        Some(_lesson_id) => {
            let ready = 5usize;
            let learned = 10usize;
            rsx! {
                div { class: "columns",
                    div { class: "column m-3 is-flex is-flex-direction-column",
                        StatusCard { title: "Ready", style: "is-primary",
                            div { class: "buttons",
                                for _ in 0..ready {
                                    ReadyButton{}
                                }
                            }
                        }
                    }
                    div { class: "column m-3 is-flex is-flex-direction-column",
                        StatusCard { title: "Learned", style: "is-warning",
                            p { class: "buttons",
                                for _ in 0..learned {
                                    LearnedIcon{}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatusCard(title: String, style: String, children: Element) -> Element {
    rsx! {
        article { class: "panel {style} is-flex-grow-1",
            p { class: "panel-heading is-small", {title} }
            p { class: "panel-tabs",
                div { class: "p-4",
                    {children}
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
