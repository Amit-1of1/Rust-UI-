use crate::components::link::TextLink;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            p { style: "margin: 0;",
                "Read the "
                TextLink { href: "#theming", "theming guide" }
                " or browse the "
                TextLink { href: "https://lucide.dev/icons", external: true, "Lucide icons" }
                "."
            }
            nav { "aria-label": "Link demo",
                ul { style: "display: flex; gap: 1rem; list-style: none; margin: 0; padding: 0;",
                    li { TextLink { href: "#link", current: true, "Overview" } }
                    li { TextLink { href: "#chips", "Chips" } }
                    li { TextLink { href: "#icons", "Icons" } }
                }
            }
        }
    }
}
