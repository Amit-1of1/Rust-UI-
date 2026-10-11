use super::component::*;
use crate::components::button::{Button, ButtonVariant};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "max-width: 32rem;",
            Empty {
                EmptyHeader {
                    EmptyMedia {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
                        }
                    }
                    EmptyTitle { "No projects yet" }
                    EmptyDescription { "Create your first project to see it here." }
                }
                EmptyContent {
                    Button { "Create project" }
                    Button { variant: ButtonVariant::Outline, "Import" }
                }
            }
        }
    }
}
