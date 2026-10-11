use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut on = use_signal(|| false);
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Switch {
                checked: on(),
                aria_label: "Airplane mode",
                on_checked_change: move |v| on.set(v),
            }
            span { style: "font-size: 0.875rem;", if on() { "On" } else { "Off" } }
        }
    }
}
