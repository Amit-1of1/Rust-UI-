use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

#[component]
pub fn Demo() -> Element {
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
