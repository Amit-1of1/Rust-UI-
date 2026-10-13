use crate::components::button::{Button, ButtonVariant};
use crate::components::icons::{BadgeCheck, ChevronRight};
use crate::components::item::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1.5rem",
            width: "100%",
            max_width: "28rem",

            Item { variant: ItemVariant::Outline,
                ItemContent {
                    ItemTitle { "Basic Item" }
                    ItemDescription { "A simple item with title and description." }
                }
                ItemActions {
                    Button { variant: ButtonVariant::Outline, "Action" }
                }
            }

            Item {
                variant: ItemVariant::Outline,
                size: ItemSize::Sm,
                as: move |attrs: Vec<Attribute>| rsx! {
                    a { href: "#", ..attrs,
                        ItemMedia { variant: ItemMediaVariant::Icon,
                            BadgeCheckIcon {}
                        }
                        ItemContent {
                            ItemTitle { "Your profile has been verified." }
                        }
                        ItemActions {
                            ChevronRightIcon {}
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn BadgeCheckIcon() -> Element {
    rsx! {
        BadgeCheck { size: "20" }
    }
}

#[component]
fn ChevronRightIcon() -> Element {
    rsx! {
        ChevronRight { size: "16" }
    }
}
