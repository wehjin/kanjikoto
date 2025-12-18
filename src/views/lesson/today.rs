use crate::core::data::query_lesson_status;
use dioxus::core::Element;
use dioxus::core_macro::component;
use dioxus::fullstack::use_loader;
use dioxus::prelude::*;

#[component]
pub fn TodaySection(lesson_id: ReadSignal<Option<i64>>) -> Element {
    match lesson_id() {
        None => rsx! { div { class: "skeleton-block" } },
        Some(lesson_id) => rsx! { TodayLessonStatus{ lesson_id } },
    }
}

#[component]
fn TodayLessonStatus(lesson_id: i64) -> Element {
    let status = use_loader(move || async move { query_lesson_status(lesson_id).await })?();
    let (ready, learned) = status.to_ready_learned();
    rsx! {
        div { class: "columns",
            div { class: "column m-3 is-flex is-flex-direction-column",
                StatusCard { title: "Ready", style: "is-primary",
                    div { class: "buttons",
                        for _ in 0..ready {
                            ReadyButton{}
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
fn ReadyButton() -> Element {
    rsx! {
        button { class: "button is-large is-primary is-outlined",
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
