use crate::core::data::card::Card;
use crate::core::data::lesson_status::LessonStatus;
use crate::core::data::{query_lesson_status, update_practice_cards};
use crate::views::practice::PracticeSessionSection;
use dioxus::core::Element;
use dioxus::core_macro::component;
use dioxus::fullstack::{use_loader, Loader};
use dioxus::prelude::*;

#[component]
pub fn TodaySection(lesson_id: i64) -> Element {
    let mut practicing = use_signal(|| false);
    let mut lesson_status =
        use_loader(move || async move { query_lesson_status(lesson_id).await })?;

    let mut record_practice = use_action(move |_| async move {
        *practicing.write() = false;
        lesson_status.restart();
        Ok(()) as Result<()>
    });

    let mut record_pass =
        use_action(move |card: Card| async move { update_practice_cards(vec![card]).await });

    rsx! {
        TodayLessonStatus{ lesson_status, practicing: practicing.clone() }
        if practicing() {
            div { class: "modal is-active",
                div { class: "modal-background" }
                div { class: "modal-card",
                    header { class: "modal-card-head",
                        div { class: "modal-card-title",
                            p { class: "title", "Reading Practice"}
                            p { class: "subtitle", "Read today's cards" }
                        }
                        button { class: "delete", aria_label: "close", onclick: move |_| *practicing.write() = false }
                    }
                    footer { class: "modal-card-foot",
                        div { class: "container",
                            PracticeSessionSection {
                                onsave: move |cards| record_practice.call(cards),
                                onpass: move |card| record_pass.call(card),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TodayLessonStatus(
    lesson_status: Loader<LessonStatus>,
    practicing: WriteSignal<bool>,
) -> Element {
    let (ready, learned) = lesson_status().to_ready_learned();
    rsx! {
        div { class: "columns",
            div { class: "column m-3 is-flex is-flex-direction-column",
                StatusCard { title: "Ready", style: "is-primary",
                    div { class: "buttons",
                        for _ in 0..ready {
                            ReadyButton{ onclick: move |_| *practicing.write() = true}
                        }
                    }
                }
            }
            div { class: "column m-3 is-flex is-flex-direction-column",
                StatusCard { title: "Learned", style: "is-warning",
                    if learned == 0 {
                        p { "None today" }
                    } else {
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
fn ReadyButton(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "button is-large is-primary is-outlined",
            onclick,
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
