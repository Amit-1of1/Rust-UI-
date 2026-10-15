use crate::components::form::{required, use_form};
use crate::components::number_field::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut guests = use_signal(|| 2.0);
    let form = use_form();
    let nights = form.field_with("3", vec![required()]);
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 2rem; align-items: flex-start;",
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                span { id: "number-demo-guests", style: "font-size: 0.875rem; font-weight: 500;", "Guests" }
                NumberInput {
                    label: "Guests",
                    "aria-labelledby": "number-demo-guests",
                    value: guests(),
                    min: 1.0,
                    max: 8.0,
                    on_value_change: move |v| guests.set(v),
                }
            }
            NumberField { field: nights, label: "Nights", description: "Between 1 and 30.", min: 1.0, max: 30.0 }
        }
    }
}
