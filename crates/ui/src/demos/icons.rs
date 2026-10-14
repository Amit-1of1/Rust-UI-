use crate::components::button::{Button, ButtonVariant};
use crate::components::icons::{Bell, Copy, Download, Heart, Search, Settings, Star, Trash};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 1rem;",
            // Decorative icons in a row: the label below each one is the text.
            ul {
                "aria-label": "Sample icons",
                style: "display: flex; flex-wrap: wrap; gap: 1rem; list-style: none; margin: 0; padding: 0;",
                IconCell { label: "Bell", Bell { size: "1.25rem", "aria-hidden": "true" } }
                IconCell { label: "Heart", Heart { size: "1.25rem", "aria-hidden": "true" } }
                IconCell { label: "Settings", Settings { size: "1.25rem", "aria-hidden": "true" } }
                IconCell { label: "Star", Star { size: "1.25rem", "aria-hidden": "true" } }
                IconCell { label: "Download", Download { size: "1.25rem", "aria-hidden": "true" } }
            }
            div { style: "display: flex; gap: 0.5rem; align-items: center;",
                Button { variant: ButtonVariant::Outline,
                    Copy { size: "1rem", "aria-hidden": "true" }
                    "Copy"
                }
                Button { variant: ButtonVariant::Destructive,
                    Trash { size: "1rem", "aria-hidden": "true" }
                    "Delete"
                }
                // Icon-only: the button carries the name.
                Button { variant: ButtonVariant::Ghost, "aria-label": "Search",
                    Search { size: "1rem", "aria-hidden": "true" }
                }
                // Or with hidden text (the theme's `sr-only` class), which translators and
                // find-in-page also see.
                Button { variant: ButtonVariant::Ghost,
                    Settings { size: "1rem", "aria-hidden": "true" }
                    span { class: "sr-only", "Settings" }
                }
            }
        }
    }
}

#[component]
fn IconCell(label: &'static str, children: Element) -> Element {
    rsx! {
        li { style: "display: flex; flex-direction: column; align-items: center; gap: 0.25rem; font-size: 0.75rem; color: var(--secondary-color-5);",
            {children}
            span { "{label}" }
        }
    }
}
