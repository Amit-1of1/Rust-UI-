# Item

- Copy into your app: `rust-ui add item` or `dx components add item --git https://github.com/Amit-1of1/Rust-UI-` (also copies `separator`).
- Import: `use crate::components::item::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A versatile component for displaying content with media, title, description, and actions.

`ItemGroup` is a list (`role="list"`), and every `Item` inside it is a list item (`role="listitem"`); an `Item` on its own has no role.

## Component Structure

```rust
ItemGroup {
    Item {
        // Available variants: Default, Outline, Muted
        variant: ItemVariant::Outline,

        // Available sizes: Default, Sm
        size: ItemSize::Default,

        ItemHeader {
            "Optional header"
        }

        ItemMedia {
            // Media variants: Default, Icon, Image
            variant: ItemMediaVariant::Image,
            img { src: "/path/to/image.png", alt: "Description" }
        }

        ItemContent {
            ItemTitle { "Item title" }
            ItemDescription { "Detailed description that can span multiple lines." }
        }

        ItemActions {
            button { "Primary action" }
        }

        ItemFooter {
            "Optional footer"
        }
    }

    ItemSeparator {}

    Item {
        // ... next item in the group
    }
}
```

## API

```rust
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

#[component]
pub fn ItemGroup(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn ItemSeparator(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
) -> Element

#[component]
pub fn Item(
    #[props(default)] variant: ItemVariant,
    #[props(default)] size: ItemSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element

#[component]
pub fn ItemMedia(
    #[props(default)] variant: ItemMediaVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn ItemContent(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn ItemTitle(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn ItemDescription(
    #[props(extends=GlobalAttributes)]
    #[props(extends=p)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn ItemActions(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn ItemHeader(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn ItemFooter(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
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
```
