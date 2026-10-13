use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 20rem;",
            Label { html_for: "label-demo-name", "Your name" }
            Input { id: "label-demo-name", placeholder: "Ada Lovelace" }
        }
    }
}
