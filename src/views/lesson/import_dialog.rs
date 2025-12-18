use crate::core::data::ImportDetails;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn ImportDialog(
    importing: WriteSignal<bool>,
    onimport: EventHandler<ImportDetails>,
) -> Element {
    const SAMPLE_CSV_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQjXD1Z1nrpTS60VhvlyI3Gha7bS-XP1r_nv3ITYbw4JBL-FA8SB6irRsVHhlEje5ZZT_H8uwFuRGgw/pub?gid=0&single=true&output=csv";
    rsx! {
        div { id: "import-dialog", class: "modal is-active",
            div { class: "modal-background" }
            form { class: "modal-card",
                onsubmit: move |evt| async move {
                    let data = evt.values().into_iter().collect::<HashMap<_, _>>();
                    let FormValue::Text(csv_url) = data.get("csv_url").unwrap() else { unreachable!() };
                    let details = ImportDetails { csv_url: csv_url.to_string() };
                    onimport.call(details);
                    evt.prevent_default();
                    *importing.write() = false;
                },
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "Import CSV" }
                    button { class: "delete", aria_label: "close", onclick: move |_| *importing.write() = false }
                }
                section { class: "modal-card-body",
                    div { class: "field",
                        label { class: "label", "URL" }
                        div { class: "control is-expanded",
                            input {class: "input",type: "url",name: "csv_url",value: SAMPLE_CSV_URL}
                        }
                    }
                }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button { class: "button is-success", type: "submit",
                            "Import"
                        }
                        button { class: "button", onclick: move |_| *importing.write() = false,
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
