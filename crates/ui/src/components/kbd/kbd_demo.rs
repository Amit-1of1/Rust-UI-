use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; gap: 1rem; align-items: center;",
            KbdGroup {
                Kbd { "Ctrl" }
                "+"
                Kbd { "K" }
            }
            Kbd { "Esc" }
        }
    }
}
