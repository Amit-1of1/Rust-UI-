use crate::components::chips::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut done = use_signal(|| false);
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            Chips { label: "Filter by status",
                Chip { default_checked: true, "Open" }
                Chip { "In review" }
                Chip { checked: done(), on_change: move |v| done.set(v), "Done" }
                Chip { disabled: true, "Archived" }
            }
            p { style: "margin: 0; font-size: 0.875rem; color: var(--secondary-color-5);",
                if done() { "Showing done tasks." } else { "Hiding done tasks." }
            }
        }
    }
}
