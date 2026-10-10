use super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "max-width: 32rem;",
            Marquee { label: "Customer logos", duration: 15,
                for name in ["Acme", "Globex", "Initech", "Umbrella", "Hooli", "Stark"] {
                    span { style: "font-weight: 600; white-space: nowrap;", "{name}" }
                }
            }
        }
    }
}
