use crate::components::button::{Button, ButtonVariant};
use crate::components::toast::{ToastProvider, ToastsExt};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastType};

/// Self-contained: the demo brings its own provider, so it also works on a docs page.
#[component]
pub fn Demo() -> Element {
    rsx! {
        ToastProvider { Buttons {} }
    }
}

#[component]
fn Buttons() -> Element {
    let toasts = use_toast();
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.notify_success(("Saved", "Your changes are stored.")),
                "Success toast"
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.notify_error(("Failed", "Something went wrong.")),
                "Error toast"
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.notify_info("Copied"),
                "Info toast"
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toasts.notify_action(ToastType::Info, ("Invoice deleted", "INV-1042 moved to the bin."), "Undo",
                        move |_| toasts.notify_success("Invoice restored"))
                },
                "Toast with an action"
            }
        }
    }
}
