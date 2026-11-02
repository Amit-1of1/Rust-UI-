use crate::components::spinner::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; gap: 1rem; align-items: center;",
            Spinner { size: SpinnerSize::Sm }
            Spinner {}
            Spinner { size: SpinnerSize::Lg }
            Spinner { size: SpinnerSize::Xl, label: "Loading invoices" }
            // The ring follows the text colour around it.
            span { style: "display: inline-flex; gap: 0.5rem; align-items: center; color: var(--error-text-color);",
                Spinner { label: "Retrying payment" }
                "Retrying payment"
            }
        }
    }
}
