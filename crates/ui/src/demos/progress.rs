use crate::components::button::*;
use crate::components::progress::*;
use dioxus::prelude::*;

/// Driven by buttons rather than a JavaScript timer, so it also works on the native renderer.
#[component]
pub fn Demo() -> Element {
    let mut progress = use_signal(|| 30_u32);

    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 20rem;",
            Progress { label: "Uploading photos", show_value: true, value: progress() as f64 }
            Progress { aria_label: "Syncing", size: ProgressSize::Sm, value: None }
            div { style: "display: flex; gap: 0.5rem;",
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| progress.set((progress() + 10).min(100)),
                    "Add 10%"
                }
                Button { variant: ButtonVariant::Ghost, onclick: move |_| progress.set(0), "Reset" }
            }
        }
    }
}
