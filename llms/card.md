# Card

- Copy into your app: `rust-ui add card` or `dx components add card --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::card::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The card component is a flexible container for grouping related content and actions. It provides a structured layout with optional header, content, and footer sections.

## Component Structure

```rust
// The Card component must wrap all card elements.
Card {
    // CardHeader contains the title, description, and optional action.
    CardHeader {
        // CardTitle displays the main heading.
        CardTitle { "Card Title" }
        // CardDescription provides supporting text.
        CardDescription { "Card description goes here." }
        // CardAction positions action elements (e.g., buttons) in the header.
        CardAction {
            Button { "Action" }
        }
    }
    // CardContent holds the main body content.
    CardContent {
        p { "Main content of the card." }
    }
    // CardFooter contains footer actions or information.
    CardFooter {
        Button { "Submit" }
    }
}
```

## Layout Notes

- When `CardAction` is present inside `CardHeader`, the header automatically switches to a two-column grid layout.

## API

```rust
#[component]
pub fn Card(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn CardHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn CardDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn CardAction(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn CardContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn CardFooter(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::*;
use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
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
    }
}
```
