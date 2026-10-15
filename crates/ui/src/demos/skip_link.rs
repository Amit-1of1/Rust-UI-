use crate::components::skip_link::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "position: relative; display: flex; flex-direction: column; gap: 0.5rem; padding-top: 2.75rem;",
            SkipLink { target: "skip-link-demo-main" }
            p { style: "margin: 0;", "Tab into this demo: the skip link appears above, and Enter jumps past the links below." }
            nav { "aria-label": "Skip link demo",
                a { href: "#skip-link", "One" }
                " · "
                a { href: "#skip-link", "Two" }
            }
            // A page has one `MainContent` (`<main>`); the gallery already has its own, so the
            // demo skips to a plain focusable block.
            div { id: "skip-link-demo-main", tabindex: "-1", style: "outline: none;",
                p { style: "margin: 0;", "The main content." }
            }
        }
    }
}
