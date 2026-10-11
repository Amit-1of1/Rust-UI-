use crate::components::button::{Button, ButtonVariant};
use crate::components::popover::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
            PopoverTrigger { "Open popover" }
            PopoverContent { gap: "0.5rem",
                strong { "Dimensions" }
                span { style: "font-size: 0.875rem;", "Set the dimensions for the layer." }
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Done" }
            }
        }
    }
}
