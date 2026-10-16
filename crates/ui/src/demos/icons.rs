use crate::components::button::{Button, ButtonVariant, IconButton};
use crate::components::icons::{
    lucide, Bell, ChevronRight, Copy, Download, Heart, Icon, IconSize, Search, Settings, Star,
    Trash,
};
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
            // Theme sizes (`--icon-size-*`), and an icon chosen at runtime from its data.
            div { style: "display: flex; gap: 0.75rem; align-items: end;",
                for size in [IconSize::Xs, IconSize::Sm, IconSize::Md, IconSize::Lg, IconSize::Xl] {
                    Icon { icon: lucide::SPARKLES, size }
                }
            }
            // A labelled icon is an image with a name; nothing else says "Verified" here.
            p { style: "display: flex; gap: 0.5rem; align-items: center; margin: 0;",
                "Ada Lovelace"
                Icon { icon: lucide::BADGE_CHECK, size: IconSize::Sm, label: "Verified" }
            }
            // Directional icons mirror in right-to-left text.
            div { style: "display: flex; gap: 1.5rem;",
                span { dir: "ltr", style: "display: inline-flex; gap: 0.25rem; align-items: center;",
                    "Next"
                    ChevronRight { size: IconSize::Sm }
                }
                span {
                    dir: "rtl",
                    lang: "ar",
                    style: "display: inline-flex; gap: 0.25rem; align-items: center;",
                    "التالي"
                    ChevronRight { size: IconSize::Sm }
                }
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
                // Icon-only: `IconButton`'s required `label` carries the name.
                IconButton { label: "Search", variant: ButtonVariant::Ghost,
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
