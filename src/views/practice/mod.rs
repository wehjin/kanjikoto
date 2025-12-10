use crate::core::api;
use crate::core::drill_point::DrillPoint;
use card::Card;
use deck::Deck;
use dioxus::prelude::*;
use rand::prelude::SliceRandom;
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
    Start { rng: StdRng, deck: Deck },
}

#[component]
fn SessionSection() -> Element {
    let mut session = use_signal(|| SessionState::Wait);
    use_resource(move || async move {
        let seed_value: u64 = rand::random();
        let mut rng = StdRng::seed_from_u64(seed_value);
        let drills = api::get_drills().await;
        let deck = Deck::new(drills, &mut rng);
        *session.write() = SessionState::Start { rng, deck };
    });
    let session = session.read().cloned();
    rsx! {
        match session {
            SessionState::Wait => rsx! {
                progress { class: "progress is-small is-primary", max: "100", "15%" }
            },
            SessionState::Start{ rng, deck } => rsx! {
                button { class: "button is-primary", "Start" }
            }
        }
    }
}
