use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
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
                        tr {
                            td { class: "is-narrow",
                                label { class: "label is-large", "1" }
                            }
                            td {
                                input { class: "input is-large", type: "text" }
                            }
                            td {
                                input { class: "input is-large", type: "text" }
                            }
                            td {
                                label { class: "label is-large", "必要" }
                            }
                        }
                        tr {
                            td {
                                label { class: "label is-large", "2" }
                            }
                            td {
                                input { class: "input is-large", type: "text" }
                            }
                            td {
                                input { class: "input is-large", type: "text" }
                            }
                            td {
                                label { class: "label is-large", "便利" }
                            }
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
                        tr {
                            class: "field",
                            td { class: "is-narrow",
                                p { class: "is-size-5", "1" }
                            }
                            td {
                                p { class: "is-size-5", "necessary" }
                            }
                        }
                        tr {
                            class: "field",
                            td { class: "is-narrow",
                                p { class: "is-size-5", "2" }
                            }
                            td {
                                p { class: "is-size-5", "convenient" }
                            }
                        }
                    }
                }
            }
        }
    }
}
