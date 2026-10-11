use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 24rem;",
            InputGroup {
                InputGroupAddon { "https://" }
                InputGroupInput { "aria-label": "Website", placeholder: "example.com" }
            }
            InputGroup {
                InputGroupInput { "aria-label": "Amount", r#type: "number", placeholder: "0.00" }
                InputGroupAddon { end: true, "USD" }
            }
            InputGroup { invalid: true,
                InputGroupInput { "aria-label": "Username", value: "taken_name", "aria-invalid": "true" }
            }
        }
    }
}
