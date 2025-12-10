use crate::core::api;
use crate::views::practice::card::Card;
use deck::Deck;
use dioxus::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub mod card;
pub mod deck;

#[component]
pub fn Practice() -> Element {
    rsx! {
        div { class: "container",
            div { class: "section",
                TitleSection {}
            }
            div { class: "section",
                SessionSection {}
            }
        }
    }
}

#[component]
fn TitleSection() -> Element {
    rsx! {
        h1 { class: "title", "Practice" }
        h2 { class: "subtitle", "Practice your reading skills" }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum SessionState {
    Wait,
    Start { deck: Deck },
    Prompt { deck: Deck },
    Learn { deck: Deck },
    Check { deck: Deck },
}

#[component]
fn SessionSection() -> Element {
    let mut session = use_signal(|| SessionState::Wait);
    use_resource(move || async move {
        let seed_value: u64 = rand::random();
        let rng = StdRng::seed_from_u64(seed_value);
        let drills = api::get_drills().await;
        let deck = Deck::new(drills, rng);
        *session.write() = SessionState::Start { deck };
    });
    rsx! {
        div { class: "columns",
            div { class: "column is-half-tablet is-one-third-desktop",
                match session.read().cloned() {
                    SessionState::Wait => rsx! {
                        progress { class: "progress is-small is-primary", max: "100", "15%" }
                    },
                    SessionState::Start{ deck } => rsx! {
                        StartSection { deck, session }
                    },
                    SessionState::Prompt{ deck } => rsx! {
                        PromptSection { deck, session }
                    },
                    SessionState::Learn{ deck } => rsx! {
                        LearnSection { deck, session }
                    },
                    SessionState::Check{ deck } => rsx! {
                        CheckSection { deck, session }
                    },
                }
            }
        }
    }
}

#[component]
fn StartSection(deck: Deck, session: WriteSignal<SessionState>) -> Element {
    rsx! {
        button {
            class: "button is-primary",
            onclick: move |_| {
                *session.write() = SessionState::Prompt { deck: deck.clone() }
            },
            "Start"
        }
    }
}

#[component]
fn PromptSection(deck: Deck, session: WriteSignal<SessionState>) -> Element {
    let title = deck.top.front.kanji.clone();
    rsx! {
        div { class: "card",
            div { class: "card-content",
                p { class: "subtitle is-6 has-text-grey-light", "Read and translate"}
                section { class: "section  has-text-centered",
                    h1 { class: "title", {title} }
                }
            }
            footer { class: "card-footer",
                a { class: "card-footer-item",
                    href: "#",
                    onclick: {
                        let deck = deck.clone();
                        move |_| {
                            *session.write() = SessionState::Learn { deck: deck.clone() };
                        }
                    },
                    "Learn"
                }
                a { class: "card-footer-item", href: "#",
                    onclick: {
                        let deck = deck.clone();
                        move |_| {
                            *session.write() = SessionState::Check { deck: deck.clone() };
                        }
                    },
                    "Check"
                }
            }
        }
    }
}

#[component]
fn LearnSection(deck: Deck, session: WriteSignal<SessionState>) -> Element {
    let card = deck.top.clone();
    rsx! {
        div { class: "card",
            div { class: "card-content", BackContent{ card } }
            footer { class: "card-footer",
                a { class: "card-footer-item",
                    href: "#",
                    onclick: move |_| {
                        let deck = deck.clone().next();
                        *session.write() = SessionState::Prompt { deck };
                    },
                    "Next" }
            }
        }
    }
}

#[component]
fn CheckSection(deck: Deck, session: WriteSignal<SessionState>) -> Element {
    let card = deck.top.clone();
    rsx! {
        div { class: "card",
            div { class: "card-content", BackContent{ card } }
            footer { class: "card-footer",
                a { class: "card-footer-item",
                    href: "#",
                    onclick: {
                        let deck = deck.clone();
                        move |_| {
                            let deck = deck.clone().fail();
                            *session.write() = SessionState::Prompt { deck };
                        }
                    },
                    "Fail"
                }
                a { class: "card-footer-item",
                    href: "#",
                    onclick: {
                        let deck = deck.clone();
                        move |_| {
                            let deck = deck.clone().pass();
                            *session.write() = SessionState::Prompt {  deck };
                        }
                    },
                    "Pass"
                }
            }
        }
    }
}

#[component]
fn BackContent(card: Card) -> Element {
    let tag = card.front.kanji.clone();
    let title = card.back.yomi.clone();
    let subtitles = card
        .back
        .meaning
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    rsx! {
        span { class: "tag is-info", "{tag}"}
            section { class: "section  has-text-centered",
                h1 { class: "title", {title} }
            p {
                for subtitle in subtitles {
                span { class: "tag is-light", "{subtitle}"}
                }
            }
        }
    }
}
