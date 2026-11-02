use crate::components::layout::*;
use dioxus::prelude::*;

const DOT: &str = "width: 0.75rem; height: 0.75rem; border-radius: var(--radius-full); background: var(--color-background-primary);";
const CELL: &str = "padding: var(--unit-x3); border-radius: var(--radius-medium); background: var(--color-background-neutral-faded); box-shadow: inset 0 0 0 1px var(--color-border-neutral); font-size: var(--font-size-caption-1); text-align: center;";

#[component]
pub fn Demo() -> Element {
    rsx! {
        Stack { gap: 6,
            Stack { gap: 1,
                Text { variant: TextVariant::Display, as_: TextAs::Div, "Display" }
                Text { variant: TextVariant::Title1, as_: TextAs::Div, "Title 1" }
                Text { variant: TextVariant::Title2, as_: TextAs::Div, "Title 2" }
                Text { variant: TextVariant::Title3, as_: TextAs::Div, "Title 3" }
                Text { variant: TextVariant::Title4, as_: TextAs::Div, "Title 4" }
                Text { variant: TextVariant::Body1, as_: TextAs::P, "Body 1: 16 px on a 24 px line, for reading." }
                Text { as_: TextAs::P, "Body 2: 14 px on a 20 px line, the kit's default." }
                Text { variant: TextVariant::Caption, color: TextColor::Faded, "Caption: 12 px on 16 px, for hints and metadata." }
            }
            Grid { min_item: "10rem", gap: 3,
                for (name, step) in [("x1", 1u8), ("x2", 2), ("x4", 4), ("x6", 6), ("x8", 8)] {
                    Stack { key: "{name}", direction: StackDirection::Row, gap: step, align: StackAlign::Center,
                        span { style: DOT }
                        span { style: DOT }
                        Text { variant: TextVariant::Caption, color: TextColor::Faded, "gap {name}" }
                    }
                }
            }
            Grid {
                GridItem { span: 4, span_m: 4, span_l: 8,
                    div { style: CELL, "span 4 / 4 / 8" }
                }
                GridItem { span: 4, span_m: 4, span_l: 4,
                    div { style: CELL, "span 4 / 4 / 4" }
                }
                for i in 0..4u8 {
                    GridItem { key: "{i}", span: 2, span_m: 2, span_l: 3,
                        div { style: CELL, "2 / 2 / 3" }
                    }
                }
            }
        }
    }
}
