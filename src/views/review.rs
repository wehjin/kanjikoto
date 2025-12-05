use crate::core::api;
use crate::core::drill_point::DrillPoint;
use dioxus::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lesson {
    pub index: usize,
    pub prompt: String,
    pub hint: Hint,
}

impl Lesson {
    pub fn new(index: usize, drill: DrillPoint) -> Self {
        Self {
            index,
            prompt: drill.kanji,
            hint: Hint::new(drill.meaning, index),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hint {
    pub index: usize,
    pub tags: Vec<String>,
    pub visible: bool,
}

impl Hint {
    pub fn new(text: String, index: usize) -> Self {
        let tags = text
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        Self {
            index,
            tags,
            visible: false,
        }
    }
}

#[component]
pub fn Review() -> Element {
    let mut lessons = use_signal(|| Vec::<Lesson>::new());
    let drills = use_resource(|| async move { api::get_drills().await });
    use_effect(move || {
        let drill_lessons = drills
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(index, point)| Lesson::new(index, point))
            .collect::<Vec<_>>();
        lessons.set(drill_lessons);
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
                        th { "Kanji" }
                        th { "Hint" }
                        th { "Lesson" }
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
            td {
                label { class: "label is-large", "{state.prompt}" }
            }
            td { class: "is-narrow",
                HintCell {state: state.hint.clone(), lessons: lessons.clone()}
            }
            td { class: "is-narrow",
                label { class: "label is-large has-text-centered", "{number}" }
            }
        }
    }
}

#[component]
fn HintCell(state: Hint, lessons: WriteSignal<Vec<Lesson>>) -> Element {
    rsx! {
        if state.visible {
            div { class: "tags",
                for tag in state.tags.iter() {
                    span { class: "tag is-info is-light", "{tag}" }
                }
            }
        } else {
            button { class: "button is-link is-light is-small",
                onclick: move |_| lessons.write().get_mut(state.index).unwrap().hint.visible = true,
                "Show"}
        }
    }
}
