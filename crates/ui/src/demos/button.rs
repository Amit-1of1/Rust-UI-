use crate::components::button::*;
use crate::components::icons::{Plus, Settings, Trash};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Button { "Primary" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
            Button { variant: ButtonVariant::Destructive, "Delete" }
            Button { variant: ButtonVariant::Link, "Link" }
        }
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; margin-top: 0.75rem;",
            Button { size: ButtonSize::Xs, "Extra small" }
            Button { size: ButtonSize::Sm, "Small" }
            Button { "Default" }
            Button { size: ButtonSize::Lg, "Large" }
            Button { disabled: true, "Disabled" }
            LoadingDemo {}
        }
        // Icon-only buttons need a `label` (their accessible name); leaving it out doesn't compile.
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; margin-top: 0.75rem;",
            IconButton { label: "Add", size: IconButtonSize::Xs, Plus { size: "0.75rem", "aria-hidden": "true" } }
            IconButton { label: "Settings", variant: ButtonVariant::Outline, size: IconButtonSize::Sm,
                Settings { size: "1rem", "aria-hidden": "true" }
            }
            IconButton { label: "Settings", variant: ButtonVariant::Secondary, Settings { size: "1rem", "aria-hidden": "true" } }
            IconButton { label: "Delete", variant: ButtonVariant::Destructive, size: IconButtonSize::Lg,
                Trash { size: "1.25rem", "aria-hidden": "true" }
            }
        }
    }
}

/// Press to see the loading state for two seconds; the button keeps its size.
#[component]
fn LoadingDemo() -> Element {
    let mut loading = use_signal(|| false);
    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            loading: loading(),
            onclick: move |_| {
                loading.set(true);
                spawn(async move {
                    dioxus_sdk_time::sleep(std::time::Duration::from_secs(2)).await;
                    loading.set(false);
                });
            },
            "Save changes"
        }
        Button { loading: true, "Loading" }
    }
}
