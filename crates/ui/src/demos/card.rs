use crate::components::button::{Button, ButtonVariant};
use crate::components::card::*;
use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1.5rem; width: 100%;",
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Create project" }
                CardDescription { "Deploy your new project in one click." }
            }
            CardContent {
                div { style: "display: grid; gap: 0.5rem; max-width: 20rem;",
                    Label { html_for: "gallery-project", "Project name" }
                    Input { id: "gallery-project", placeholder: "my-app" }
                }
            }
            CardFooter { style: "justify-content: space-between;",
                Button { variant: ButtonVariant::Outline, "Cancel" }
                Button { "Create" }
            }
        }
        CardGroup { min_width: "12rem",
            Card {
                CardHeader {
                    CardTitle { CardLink { href: "#docs", "Read the docs" } }
                    CardDescription { "The whole card opens the guide." }
                }
            }
            Card { horizontal: true,
                CardHeader {
                    CardTitle { "Side by side" }
                    CardDescription { "On wide screens the parts sit in a row." }
                }
                CardFooter { Button { variant: ButtonVariant::Outline, "Open" } }
            }
        }
        }
    }
}
