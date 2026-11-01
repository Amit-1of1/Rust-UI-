use crate::components::button::{Button, ButtonVariant, IconButton};
use crate::components::icons::{
    atoll, lucide, Bell, ChevronRight, Copy, Download, Heart, Icon, IconSize, Search, Settings,
    Star, Trash,
};
use crate::components::input::Input;
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
            AtollIcons {}
        }
    }
}

/// Atoll's own icons, searchable by name, default label and keyword, grouped by category.
#[component]
fn AtollIcons() -> Element {
    let mut query = use_signal(String::new);
    let q = query.read().trim().to_lowercase();
    let matches = |i: usize| {
        let m = &atoll::INDEX[i];
        q.is_empty()
            || m.name.contains(&q)
            || m.label.to_lowercase().contains(&q)
            || m.keywords.iter().any(|k| k.contains(&q))
    };
    let shown: Vec<usize> = (0..atoll::ALL.len()).filter(|&i| matches(i)).collect();
    let count = shown.len();
    rsx! {
        section { "aria-labelledby": "atoll-icons-title", style: "display: flex; flex-direction: column; gap: 0.75rem;",
            h3 { id: "atoll-icons-title", style: "margin: 0.5rem 0 0;", "Atoll icons" }
            p { style: "margin: 0; color: var(--secondary-color-5);",
                "Drawn for the kit on Lucide's 24px grid: ui::icons::atoll::Crate {{}}. Reserved icons mean one action everywhere."
            }
            label { r#for: "atoll-icon-search", style: "font-weight: 500;", "Search icons" }
            Input {
                id: "atoll-icon-search",
                r#type: "search",
                placeholder: "crate, agent, delete...",
                value: "{query}",
                oninput: move |e: FormEvent| query.set(e.value()),
            }
            p { role: "status", style: "margin: 0; font-size: 0.875rem; color: var(--secondary-color-5);",
                "{count} of {atoll::ALL.len()} icons"
            }
            for category in atoll::CATEGORIES {
                if shown.iter().any(|&i| atoll::INDEX[i].category == category) {
                    div { key: "{category}",
                        h4 { style: "margin: 0.5rem 0 0.25rem; font-size: 0.8125rem; text-transform: capitalize; color: var(--secondary-color-5);",
                            "{category}"
                        }
                        ul {
                            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(6.5rem, 1fr)); gap: 0.5rem; list-style: none; margin: 0; padding: 0;",
                            for i in shown.iter().copied().filter(|&i| atoll::INDEX[i].category == category) {
                                AtollTile { key: "{atoll::INDEX[i].name}", index: i }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AtollTile(index: usize) -> Element {
    let meta = &atoll::INDEX[index];
    let title = match meta.reserved {
        Some(r) => format!("{}: reserved for {r}. Keywords: {}", meta.label, meta.keywords.join(", ")),
        None => format!("{}. Keywords: {}", meta.label, meta.keywords.join(", ")),
    };
    rsx! {
        li {
            title,
            style: "display: flex; flex-direction: column; align-items: center; gap: 0.375rem; padding: 0.75rem 0.25rem 0.5rem; border: 1px solid var(--primary-color-6); border-radius: var(--radius, 0.5rem); font-size: 0.75rem; text-align: center;",
            Icon { icon: atoll::ALL[index], size: IconSize::Lg }
            span { "{meta.name}" }
            if meta.reserved.is_some() {
                span { style: "font-size: 0.6875rem; color: var(--secondary-color-5);", "reserved" }
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
