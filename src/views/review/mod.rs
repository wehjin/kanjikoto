use crate::components::hint::HintsCell;
use crate::core::api;
use dioxus::prelude::*;
use lesson::{Answer, Lesson};

mod lesson;

#[component]
pub fn Review() -> Element {
    let mut lessons = use_signal(|| Vec::<Lesson>::new());
    let drills = use_resource(|| async move { api::get_drills().await });
    use_effect(move || {
        let lessons_vec = drills
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(index, point)| Lesson::new(index, point))
            .collect::<Vec<_>>();
        lessons.set(lessons_vec);
    });
    rsx! {
        div { class: "container",
            div { class: "section",
                h1 { class: "title", "Kanji → Yomi"}
                h2 { class: "subtitle", "Write the yomi for each kanji phrase"}
            }
            table { class: "table",
                thead {
                    tr {
                        th { "Lesson" }
                        th { "Kanji" }
                        th { "Answer" }
                    }
                }
                tbody {
                    for lesson in lessons.iter() {
                        LessonRow {state: lesson.clone(), lessons: lessons.clone()}
                    }
                }
            }
        }
    }
}

#[component]
fn LessonRow(state: Lesson, lessons: WriteSignal<Vec<Lesson>>) -> Element {
    let number = state.index + 1;
    rsx! {
        tr {
            td { class: "is-narrow",
                label { class: "label is-large has-text-centered", "{number}" }
            }
            td { class: "is-narrow",
                label { class: "label is-large", "{state.prompt}" }
            }
            td { AnswerCell {answer: state.answer.clone(), lessons: lessons.clone()} }
        }
    }
}

#[component]
fn AnswerCell(answer: Answer, lessons: WriteSignal<Vec<Lesson>>) -> Element {
    rsx! {
        if answer.visible {
            HintsCell { hints: answer.hints.clone(), light: false }
        } else {
            button { class: "button is-info is-light is-small",
                onclick: move |_| lessons.write().get_mut(answer.lesson_index).unwrap().answer.visible = true,
                "Show"}
        }
    }
}
