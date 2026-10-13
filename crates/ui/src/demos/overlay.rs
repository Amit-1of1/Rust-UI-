use crate::components::button::*;
use crate::components::overlay::*;
use dioxus::prelude::*;

/// A hand-made modal: `ModalPortal` is how a custom overlay gets the same treatment as the
/// kit's dialogs (window-sized on native, unchanged on the web).
#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Show banner" }
        if open() {
            ModalPortal {
                div {
                    role: "status",
                    style: "position: fixed; z-index: 1000; right: 0; bottom: 0; left: 0; display: flex; align-items: center; justify-content: space-between; gap: 1rem; padding: 1rem 1.5rem; border-top: 1px solid var(--primary-color-6); background: var(--primary-color-2); color: var(--secondary-color-4);",
                    span { "Pinned to the bottom of the window, wherever the trigger sits." }
                    Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Dismiss" }
                }
            }
        }
    }
}
