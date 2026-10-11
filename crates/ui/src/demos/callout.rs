use crate::components::callout::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 32rem;",
            Callout { title: "Heads up", "Callouts are static notes. Use " code { "Alert" } " to interrupt." }
            Callout { variant: CalloutVariant::Info, title: "Tip", "Press / to open the command palette." }
            Callout { variant: CalloutVariant::Success, "Saved. Your changes are live." }
            Callout { variant: CalloutVariant::Warning, title: "Careful", "This replaces the existing file." }
        }
    }
}
