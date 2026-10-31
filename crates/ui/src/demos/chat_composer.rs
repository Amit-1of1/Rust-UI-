use crate::components::button::{ButtonVariant, IconButton, IconButtonSize};
use crate::components::chat_composer::*;
use crate::components::icons::Paperclip;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut sent = use_signal(Vec::<String>::new);
    rsx! {
        div { display: "flex", flex_direction: "column", gap: "0.75rem", max_width: "32rem",
            ChatComposer {
                label: "Message",
                placeholder: "Write a message…",
                onsend: move |text| sent.write().push(text),
                IconButton {
                    label: "Attach a file",
                    variant: ButtonVariant::Ghost,
                    size: IconButtonSize::Sm,
                    r#type: "button",
                    Paperclip { size: "1rem" }
                }
            }
            if sent().is_empty() {
                p { "data-slot": "sent", style: "margin: 0; color: var(--secondary-color-5); font-size: 0.875rem;",
                    "Nothing sent yet."
                }
            } else {
                ul { "data-slot": "sent", style: "margin: 0; padding-inline-start: 1.25rem; font-size: 0.875rem;",
                    for (i, text) in sent().into_iter().enumerate() {
                        li { key: "{i}", style: "white-space: pre-wrap;", "{text}" }
                    }
                }
            }
        }
    }
}
