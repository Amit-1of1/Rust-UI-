use crate::components::carousel::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "max-width: 32rem;",
            Carousel { label: "Featured plans",
                for (i , name) in ["Starter", "Team", "Business", "Enterprise"].into_iter().enumerate() {
                    CarouselItem { label: "{i + 1} of 4", strong { "{name}" } }
                }
            }
        }
    }
}
