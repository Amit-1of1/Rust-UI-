use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "max-width: 20rem;",
            strong { "Rust UI kit" }
            p { style: "margin: 0.25rem 0; font-size: 0.875rem;", "A personal component library." }
            Separator { horizontal: true, decorative: true, style: "margin: 1rem 0;" }
            div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; font-size: 0.875rem;", "Docs" Separator { horizontal: false, decorative: true, style: "height: 1rem;" } "Source" }
        }
    }
}
