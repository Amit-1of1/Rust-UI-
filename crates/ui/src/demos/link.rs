use crate::components::link::{LinkVariant, TextLink};
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
            p { style: "margin: 0; max-width: 18rem;",
                "Links wrap with the text: "
                TextLink { href: "#start", "a long link that runs over more than one line of this paragraph" }
                "."
            }
            p { style: "margin: 0; display: flex; flex-wrap: wrap; gap: 1rem;",
                TextLink { href: "#privacy", variant: LinkVariant::Muted, "Privacy" }
                span { style: "color: var(--error-text-color);",
                    "Payment failed. "
                    TextLink { href: "#billing", variant: LinkVariant::Inherit, "Update your card" }
                }
                TextLink { href: "#danger", variant: LinkVariant::Destructive, "Delete account" }
                TextLink { href: "#export", disabled: true, "Export (Pro)" }
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
