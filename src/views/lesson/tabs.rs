use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LessonTab {
    Today,
    Phrases,
}

#[component]
pub fn LessonTabs(current_tab: Signal<LessonTab>) -> Element {
    #[component]
    fn TabItem(tab: LessonTab, current_tab: Signal<LessonTab>) -> Element {
        let is_active = tab == current_tab.read().clone();
        let title = match tab {
            LessonTab::Today => "Today",
            LessonTab::Phrases => "Phrases",
        };
        rsx! {
            li { class: if is_active {"is-active"},
                a {
                    onclick: move |_| *current_tab.write() = tab,
                    {title}
                }
            }
        }
    }
    rsx! {
        div { class: "tabs is-centered is-medium",
            ul {
                TabItem { tab: LessonTab::Today, current_tab: current_tab.clone() }
                TabItem { tab: LessonTab::Phrases, current_tab: current_tab.clone() }
            }
        }
    }
}