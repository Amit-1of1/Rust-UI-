use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Open dialog" }
        Dialog { open: open(), on_open_change: move |v| open.set(v),
            DialogTitle { "Are you sure?" }
            DialogDescription { "This action cannot be undone." }
            div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; justify-content: flex-end; margin-top: 1rem;",
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Cancel" }
                Button { variant: ButtonVariant::Destructive, onclick: move |_| open.set(false), "Delete" }
            }
        }
    }
}
