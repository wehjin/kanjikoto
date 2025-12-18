use crate::core::data::{import_csv, lesson_view};
use crate::views::lesson::import_dialog::ImportDialog;
use crate::views::lesson::phrases::PhraseTable;
use crate::views::lesson::tabs::LessonTabs;
use crate::views::lesson::today::TodaySection;
use dioxus::prelude::*;
use tabs::LessonTab;

pub mod import_dialog;
pub mod phrases;
pub mod tabs;
pub mod today;
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
        footer { class: "footer",
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
