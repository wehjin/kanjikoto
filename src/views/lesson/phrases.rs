use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::core::data::PhraseView;

#[component]
pub fn PhraseTable(phrases: ReadSignal<Vec<PhraseView>>) -> Element {
    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    struct PhraseRow {
        number: String,
        phrase: PhraseView,
    }
    let rows = phrases
        .iter()
        .enumerate()
        .map(|(index, phrase)| PhraseRow {
            number: (index + 1).to_string(),
            phrase: phrase.cloned(),
        })
        .collect::<Vec<_>>();
    rsx! {
        table { class: "table is-striped is-hoverable is-fullwidth",
            thead {
                tr {
                    th { "#"}
                    th { "Prompt" }
                    th { "Reading" }
                    th { "Meaning" }
                }
            }
            tbody {
                for row in rows.iter() {
                    tr {
                        td { { row.number.clone() } }
                        td { { row.phrase.prompt.clone()} }
                        td { { row.phrase.reading.clone()} }
                        td { { row.phrase.meaning.clone()} }
                    }
                }
            }
        }
    }
}