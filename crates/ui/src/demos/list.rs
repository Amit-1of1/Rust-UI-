use crate::components::badge::*;
use crate::components::kbd::*;
use crate::components::list::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 2rem; align-items: flex-start;",
            List {
                ListItem { "Light and dark themes" }
                ListItem { "Keyboard first"
                    List { ListItem { "Tab, arrows, Escape" } }
                }
                ListItem { "No JavaScript of your own" }
            }
            List { marker: ListMarker::Number,
                ListItem { "Install the CLI" }
                ListItem { "Run " Kbd { "atoll create" } }
                ListItem { "Open the gallery" }
            }
            List { marker: ListMarker::None, spaced: true, aria_label: "Plans",
                ListItem { Badge { variant: BadgeVariant::Secondary, "Free" } " Every component" }
                ListItem { Badge { "Pro" } " Blocks and starters" }
            }
        }
    }
}
