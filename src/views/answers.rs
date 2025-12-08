use crate::components::hint::{get_hints, Hint, HintsCell};
use crate::core::api;
use crate::core::drill_point::DrillPoint;
use dioxus::prelude::*;
use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

#[component]
pub fn Answers(seed: usize) -> Element {
    let mut answers = use_signal(|| Vec::<Answer>::new());
    let drills = use_resource(move || async move {
        let mut drills = api::get_drills().await;
        if seed != 0 {
            let mut rng = ChaChaRng::seed_from_u64(seed as u64);
            drills.shuffle(&mut rng);
        }
        drills
    });
    use_effect(move || {
        let answers_vec = drills
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(index, point)| Answer::new(index, point))
            .collect::<Vec<_>>();
        answers.set(answers_vec);
    });

    rsx! {
        div { class: "container",
            div { class: "section",
                h1 { class: "title", "Worksheet"}
                h2 { class: "subtitle", "Write the yomi and meaning for each kanji phrase"}
            }
            table { class: "table",
                thead {
                    tr {
                        th { "Lesson" }
                        th { "Kanji" }
                        th { "Yomi and Meaning"}
                        th {}
                        th { "Check Answer" }
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
    pub kanji: String,
    pub hints: Vec<Hint>,
}

impl Answer {
    pub fn new(index: usize, drill_point: DrillPoint) -> Self {
        let kanji = drill_point.kanji.clone();
        let hints = get_hints(&drill_point);
        Self {
            index,
            kanji,
            hints,
        }
    }
}

#[component]
fn AnswerRow(state: Answer) -> Element {
    let number = state.index + 1;
    rsx! {
        tr {
            td { class: "is-narrow",
                label { class: "label is-large has-text-centered", "{number}" }
            }
            td { class: "is-narrow",
                 label { class: "label is-large", "{state.kanji}" }
            }
            td {
                input { class: "input is-large", type: "text", readonly: true }
            }
            td { class: "is-narrow",
                span { class: "is-size-1", "|" }
            }
            td { class: "is-narrow",
                HintsCell { hints: state.hints.clone() }
            }
        }
    }
}
