use crate::components::input::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut name = use_signal(String::new);
    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 20rem;",
            Label { html_for: "gallery-name", "Name" }
            Input {
                id: "gallery-name",
                placeholder: "Enter your name",
                value: name,
                oninput: move |e: FormEvent| name.set(e.value()),
            }
            if !name.read().is_empty() {
                p { style: "margin: 0; font-size: 0.875rem;", "Hello, {name}!" }
            }
            Label { html_for: "gallery-search-sm", "Small" }
            Input { id: "gallery-search-sm", field_size: crate::components::field::FieldSize::Sm, placeholder: "Filter rows" }
            Label { html_for: "gallery-email-lg", "Large" }
            Input { id: "gallery-email-lg", r#type: "email", field_size: crate::components::field::FieldSize::Lg, placeholder: "you@example.com" }
        }
    }
}
