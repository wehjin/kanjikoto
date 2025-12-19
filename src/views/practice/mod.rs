use crate::core::data::card::Card;
use crate::core::data::{lesson_view, query_practice_cards};
use deck::Deck;
use dioxus::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub mod deck;

#[derive(Debug, Clone, Eq, PartialEq)]
enum SessionState {
    Start,
    Prompt { deck: Deck },
    Learn { deck: Deck },
    Check { deck: Deck },
    Done { deck: Deck },
}

#[component]
pub fn PracticeSessionSection(
    onsave: EventHandler<Vec<Card>>,
    onpass: EventHandler<Card>,
) -> Element {
    let mut session = use_signal(|| SessionState::Start);

    let mut start_action = use_action(move |_input: ()| async move {
        let lesson = lesson_view().await.unwrap().unwrap();
        let cards = query_practice_cards(lesson.lesson_id).await.unwrap();
        assert!(cards.len() > 0);
        let rng = StdRng::seed_from_u64(rand::random());
        let deck = Deck::from_cards(cards, rng);
        *session.write() = SessionState::Prompt { deck };
        Ok(()) as Result<()>
    });

    match session() {
        SessionState::Start => rsx! {
            button {
                class: "button is-primary",
                onclick: move |_| {
                    start_action.call(());
                },
                "Start"
            }
        },
        SessionState::Done { deck } => rsx! {
            div { class: "block",
                div { class: "title is-5", "Stats" }
                div { class: "container",
                    div { class: "field is-grouped is-grouped-multiline",
                        div { class: "control",
                            div { class: "tags has-addons",
                                span { class: "tag is-dark", "Passes"}
                                span { class: "tag is-success", "{deck.stats.passed}"}
                            }
                        }
                        div { class: "control",
                            div { class: "tags has-addons",
                                span { class: "tag is-dark", "Fails"}
                                span { class: "tag is-warning", "{deck.stats.failed}"}
                            }
                        }
                        div { class: "control",
                            div { class: "tags has-addons",
                                span { class: "tag is-dark", "Repeats"}
                                span { class: "tag is-link", "{deck.stats.repeated}"}
                            }
                        }
                        div { class: "control",
                            div { class: "tags has-addons",
                                span { class: "tag is-dark", "Learns"}
                                span { class: "tag is-info", "{deck.stats.learned}"}
                            }
                        }
                    }
                }
            }
            div { class: "block",
                button {
                    class: "button is-primary",
                    onclick: move |_| {
                        *session.write() = SessionState::Start;
                        onsave.call(deck.clone().into_cards());
                    },
                    "Done"
                }
            }
        },
        SessionState::Prompt { deck } => rsx! {
            PromptSection { deck, session }
        },
        SessionState::Learn { deck } => rsx! {
            LearnSection { deck, session }
        },
        SessionState::Check { deck } => rsx! {
            CheckSection { deck, session, onpass }
        },
    }
}

#[component]
fn PromptSection(deck: Deck, session: WriteSignal<SessionState>) -> Element {
    let title = deck.top.front.kanji.clone();
    let turns = deck.turns_remaining();
    rsx! {
        div { class: "card",
            div { class: "card-content",
                nav { class: "level is-mobile",
                    div { class: "level-left",
                        div { class: "level-item",
                            span { class: "heading", "Read and translate"}
                        }
                    }
                    div { class: "level-right",
                        div { class: "level-item",
                            span { class: "tag is-light", "{turns}"}
                        }
                    }
                }
                section { class: "section has-text-centered",
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
    let turns = deck.turns_remaining();
    rsx! {
        div { class: "card",
            div { class: "card-content", BackContent{ card, turns } }
            footer { class: "card-footer",
                a { class: "card-footer-item",
                    href: "#",
                    onclick: move |_| {
                        let deck = deck.clone().learn();
                        *session.write() = SessionState::Prompt { deck };
                    },
                    "Next" }
            }
        }
    }
}

#[component]
fn CheckSection(
    deck: Deck,
    session: WriteSignal<SessionState>,
    onpass: EventHandler<Card>,
) -> Element {
    let card = deck.top.clone();
    let turns = deck.turns_remaining();
    rsx! {
        div { class: "card",
            div { class: "card-content", BackContent{ card, turns } }
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
                            let deck = deck.clone().repeat();
                            *session.write() = SessionState::Prompt { deck};

                        }
                    },
                    "Repeat"
                }
                a { class: "card-footer-item",
                    href: "#",
                    onclick: {
                        let deck = deck.clone();
                        move |_| {
                            onpass.call(deck.top.clone());
                            let deck = deck.clone().pass();
                            *session.write() = if deck.is_all_passed() {
                                SessionState::Done { deck }
                            } else {
                                SessionState::Prompt { deck}
                            };
                        }
                    },
                    "Pass"
                }
            }
        }
    }
}

#[component]
fn BackContent(card: Card, turns: usize) -> Element {
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
        nav { class: "level is-mobile",
            div { class: "level-left",
                div { class: "level-item",
                    span { class: "tag is-info is-large", "{tag}"}
                }
            }
        }
        div { class: "container",
            section { class: "section",
                h1 { class: "title has-text-centered", {title} }
                div { class: "tags are-medium is-centered",
                    for subtitle in subtitles {
                        span { class: "tag is-warning is-light", "{subtitle}"}
                    }
                }
            }
        }
    }
}
