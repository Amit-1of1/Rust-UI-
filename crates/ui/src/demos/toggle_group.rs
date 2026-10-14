use crate::components::toggle_group::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {

        ToggleGroup { horizontal: true, allow_multiple_pressed: true, role: "group", aria_label: "Text style",
            ToggleItem { index: 0usize, aria_label: "Bold",
                b { "B" }
            }
            ToggleItem { index: 1usize, aria_label: "Italic",
                i { "I" }
            }
            ToggleItem { index: 2usize, aria_label: "Underline",
                u { "U" }
            }
        }
    }
}
