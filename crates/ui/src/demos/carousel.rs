use crate::components::carousel::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "max-width: 32rem; display: grid; gap: 1.5rem;",
            Carousel { label: "Featured plans",
                for (i , name) in ["Starter", "Team", "Business", "Enterprise"].into_iter().enumerate() {
                    CarouselItem { label: "{i + 1} of 4", strong { "{name}" } }
                }
            }
            // Smaller slides that adapt: two and a bit in view on a wide screen, one on a phone.
            Carousel { label: "Recent invoices", item_size: "max(40%, 11rem)",
                for n in 1..=6 {
                    CarouselItem { key: "{n}", label: "{n} of 6", "Invoice #{n}" }
                }
            }
        }
    }
}
