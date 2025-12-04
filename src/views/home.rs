use dioxus::prelude::*;

pub struct KanjiPoint {
    pub chapter: usize,
    pub furi: String,
    pub meaning: String,
    pub kanji: String,
    pub yomi: String,
}

pub struct ReadingPoint {
    pub index: usize,
    pub kanji_point: KanjiPoint,
}

impl ReadingPoint {
    pub fn new(index: usize, kanji_point: KanjiPoint) -> Self {
        Self { index, kanji_point }
    }
    pub fn meaning(&self) -> String {
        self.kanji_point.meaning.clone()
    }
    pub fn kanji(&self) -> String {
        self.kanji_point.kanji.clone()
    }
}

impl KanjiPoint {
    pub fn new(chapter: usize, furi: impl AsRef<str>, meaning: impl AsRef<str>) -> Self {
        let furi = furi.as_ref().to_string();
        let meaning = meaning.as_ref().to_string();
        let mut kanji_segments = Vec::new();
        let mut yomi_segments = Vec::new();
        for segment in furi.split("）") {
            let kanji_yomi = segment.split("（").collect::<Vec<&str>>();
            match kanji_yomi.len() {
                2 => {
                    kanji_segments.push(kanji_yomi[0].to_string());
                    yomi_segments.push(kanji_yomi[1].to_string());
                }
                1 => {
                    let kana = kanji_yomi[0].to_string();
                    kanji_segments.push(kana.clone());
                    yomi_segments.push(kana);
                }
                _ => panic!("Invalid furi segment: {}", segment),
            }
        }
        Self {
            chapter,
            furi,
            meaning,
            kanji: kanji_segments.join(""),
            yomi: yomi_segments.join(""),
        }
    }
}

#[component]
pub fn Home() -> Element {
    let points = vec![
        KanjiPoint::new(1, "始（はじ）まり", "the beginning"),
        KanjiPoint::new(
            1,
            "幸（こう）か（）不（ふ）幸（こう）か",
            "for better or worse, lucky or unlucky",
        ),
    ];
    let points = points
        .into_iter()
        .enumerate()
        .map(|(index, point)| ReadingPoint::new(index, point))
        .collect::<Vec<_>>();
    rsx! {
        div { class: "container",
            div { class: "section",
                h1 { class: "title", "Reading Worksheet" }
                h2 { class: "subtitle", "Chapter 1" }
                table { class: "table is-fullwidth",
                    thead {
                        tr {
                            th { "Number" }
                            th { "Yomi" }
                            th { "Meaning" }
                            th { "Kanji" }
                        }
                    }
                    tbody {
                        for point in points.iter() {
                            ReadingRow{index: point.index, kanji: point.kanji()}
                        }
                    }
                }
            }
            div { class: "section",
                h1 { class: "title", "Glossary" }
                h2 { class: "subtitle", "Chapter 1" }
                table { class: "table is-fullwidth",
                    thead {
                        tr {
                            th { class: "is-narrow", "Number" }
                            th { "Meaning" }
                        }
                    }
                    tbody {
                        for point in points.iter() {
                            GlossaryRow{index: point.index, meaning: point.meaning()}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ReadingRow(index: usize, kanji: String) -> Element {
    let number = index + 1;
    rsx! {
        tr {
            td { class: "is-narrow",
                label { class: "label is-large", "{number}" }
            }
            td {
                input { class: "input is-large", type: "text" }
            }
            td {
                input { class: "input is-large", type: "text" }
            }
            td {
                label { class: "label is-large", "{kanji}" }
            }
        }
    }
}

#[component]
fn GlossaryRow(index: usize, meaning: String) -> Element {
    let number = index + 1;
    rsx! {
        tr {
            class: "field",
            td { class: "is-narrow",
                p { class: "is-size-5", "{number}" }
            }
            td {
                p { class: "is-size-5", "{meaning}" }
            }
        }
    }
}
