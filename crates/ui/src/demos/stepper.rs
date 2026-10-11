use crate::components::button::{Button, ButtonVariant};
use crate::components::stepper::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1.25rem; max-width: 36rem;",
            Stepper { total: 3, default_step: 1, "aria-label": "Checkout progress", footer: rsx! { StepperControls {} },
                StepperItem { step: 0, title: "Account", description: "Your details" }
                StepperItem { step: 1, title: "Shipping", description: "Where to send it" }
                StepperItem { step: 2, title: "Payment" }
            }
        }
    }
}

/// Back/Next buttons driven by the stepper context.
#[component]
fn StepperControls() -> Element {
    let ctx = use_stepper();
    rsx! {
        div { style: "display: flex; gap: 0.5rem;",
            Button {
                variant: ButtonVariant::Outline,
                disabled: !ctx.can_prev(),
                onclick: move |_| ctx.prev(),
                "Back"
            }
            Button { disabled: !ctx.can_next(), onclick: move |_| ctx.next(), "Next" }
        }
    }
}
