use crate::core::api::DrillPoint;
use dioxus::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hint {
    pub style: HintStyle,
    pub text: String,
}

impl Hint {
    pub fn definition(text: String) -> Self {
        Self {
            style: HintStyle::Definition,
            text,
        }
    }
    pub fn reading(text: String) -> Self {
        Self {
            style: HintStyle::Reading,
            text,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HintStyle {
    Definition,
    Reading,
}

pub fn get_hints(drill: &DrillPoint) -> Vec<Hint> {
    let definitions = drill
        .to_meanings()
        .into_iter()
        .map(Hint::definition)
        .collect::<Vec<_>>();
    let yomi = Hint::reading(drill.yomi.clone());
    vec![yomi]
        .into_iter()
        .chain(definitions)
        .collect::<Vec<_>>()
}

#[component]
pub fn HintSpan(hint: Hint, light: bool) -> Element {
    let style = match hint.style {
        HintStyle::Definition => "is-info",
        HintStyle::Reading => "is-primary",
    };
    let light = if light { "is-light" } else { "" };
    let text = hint.text;
    rsx! {
        span { class: "tag {style} {light}", "{text}" }
    }
}

#[component]
pub fn HintsCell(hints: Vec<Hint>, light: bool) -> Element {
    rsx! {
        div {
            class: "tags",
            for hint in hints.iter() {
                HintSpan {hint: hint.clone(), light}
            }
        }
    }
}
