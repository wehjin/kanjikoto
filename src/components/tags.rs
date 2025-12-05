use dioxus::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tags(Vec<String>);

impl Tags {
    pub fn new(tags: Vec<String>) -> Self {
        Self(tags)
    }
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }
}

#[component]
pub fn TagsCell(tags: Tags) -> Element {
    rsx! {
        div { class: "tags",
            for tag in tags.iter() {
                span { class: "tag is-info is-light", "{tag}" }
            }
        }
    }
}
