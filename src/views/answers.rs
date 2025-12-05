use crate::components::tags::{Tags, TagsCell};
use crate::core::api;
use dioxus::prelude::*;

#[component]
pub fn Answers() -> Element {
    let mut answers = use_signal(|| Vec::<Answer>::new());
    let drills = use_resource(|| async move { api::get_drills().await });
    use_effect(move || {
        let answers_vec = drills
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(index, point)| Answer::new(index, point.yomi.clone(), point.to_meanings()))
            .collect::<Vec<_>>();
        answers.set(answers_vec);
    });
    rsx! {
        div { class: "container",
            div { class: "section",
                h1 { class: "title", "Yomi"}
                h2 { class: "subtitle", "Check your answers"}
            }
            table { class: "table",
                thead {
                    tr {
                        th { "Lesson" }
                        th { "Yomi" }
                        th { "Meaning" }
                    }
                }
                tbody {
                    for answer in answers.iter() {
                        AnswerRow { state: answer.clone() }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Answer {
    pub index: usize,
    pub yomi: String,
    pub meanings: Vec<String>,
}

impl Answer {
    pub fn new(index: usize, yomi: String, meanings: Vec<String>) -> Self {
        Self {
            index,
            yomi,
            meanings,
        }
    }
}

#[component]
fn AnswerRow(state: Answer) -> Element {
    let number = state.index + 1;
    let tags = Tags::new(state.meanings);
    rsx! {
        tr {
            td { class: "is-narrow",
                label { class: "label is-large has-text-centered", "{number}" }
            }
            td {
                 label { class: "label is-large", "{state.yomi}" }
            }
            td { TagsCell { tags } }
        }
    }
}
