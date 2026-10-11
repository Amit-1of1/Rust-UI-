use crate::components::tooltip::*;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { "Hover or focus me" }
            TooltipContent { side: ContentSide::Top, "Add to library" }
        }
    }
}
