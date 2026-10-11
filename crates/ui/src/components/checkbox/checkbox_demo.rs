use super::component::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Checkbox { id: "terms", name: "terms" }
            Label { html_for: "terms", "Accept terms" }
        }
    }
}
