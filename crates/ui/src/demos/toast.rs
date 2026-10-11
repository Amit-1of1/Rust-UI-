use crate::components::button::{Button, ButtonVariant};
use crate::components::toast::ToastProvider;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

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
                onclick: move |_| toasts.success("Saved".to_string(), ToastOptions::new().description("Your changes are stored.")),
                "Success toast"
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.error("Failed".to_string(), ToastOptions::new().description("Something went wrong.")),
                "Error toast"
            }
        }
    }
}
