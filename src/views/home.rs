use crate::core::api;
use crate::core::drill_point::DrillPoint;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let drills = use_resource(|| async move { api::get_drills().await });
    rsx! {
        div { class: "container",
            Worksheets{ drills: drills.cloned().unwrap_or_default() }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorksheetRow {
    pub index: usize,
    pub kanji_point: DrillPoint,
}

impl WorksheetRow {
    pub fn new(index: usize, kanji_point: DrillPoint) -> Self {
        Self { index, kanji_point }
    }
    pub fn yomi(&self) -> String {
        self.kanji_point.yomi.clone()
    }
}

#[component]
fn Worksheets(drills: Vec<DrillPoint>) -> Element {
    let rows = drills
        .into_iter()
        .enumerate()
        .map(|(index, point)| WorksheetRow::new(index, point))
        .collect::<Vec<_>>();
    rsx! {
        div { class: "section",
            style: "page-break-before: always; break-before: page;",
            h1 { class: "title", "Answers" }
            h2 { class: "subtitle", "Chapter 1" }
            table { class: "table is-fullwidth",
                thead {
                    tr {
                        th { class: "is-narrow", "Number" }
                        th { "Yomi" }
                    }
                }
                tbody {
                    for row in rows.iter() {
                        YomiRow{index: row.index, yomi: row.yomi()}
                    }
                }
            }
        }
    }
}

#[component]
fn YomiRow(index: usize, yomi: String) -> Element {
    let number = index + 1;
    rsx! {
        tr {
            class: "field",
            td { class: "is-narrow",
                p { class: "is-size-5", "{number}" }
            }
            td {
                p { class: "is-size-5", "{yomi}" }
            }
        }
    }
}
