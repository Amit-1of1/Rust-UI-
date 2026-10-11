use crate::components::button::*;
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
        }
    }
}
