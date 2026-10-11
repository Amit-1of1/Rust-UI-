use crate::components::button::{Button, ButtonVariant};
use crate::components::button_group::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 1.5rem; align-items: flex-start;",
            ButtonGroup { "aria-label": "Text alignment",
                Button { variant: ButtonVariant::Outline, "Left" }
                Button { variant: ButtonVariant::Outline, "Center" }
                Button { variant: ButtonVariant::Outline, "Right" }
            }
            ButtonGroup { vertical: true, "aria-label": "Zoom",
                Button { variant: ButtonVariant::Outline, "Zoom in" }
                Button { variant: ButtonVariant::Outline, "Zoom out" }
            }
            ButtonGroup { "aria-label": "Page size",
                ButtonGroupText { "Rows" }
                Button { variant: ButtonVariant::Outline, "10" }
                Button { variant: ButtonVariant::Outline, "25" }
            }
        }
    }
}
