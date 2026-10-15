# Button

- Copy into your app: `rust-ui add button` or `dx components add button --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::button::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The button element is used to trigger actions or events in a user interface.

For a link that should look like a button (the router's `Link`, which renders its own `<a>`), spread `button_look(variant, size)` onto it: `Link { to: Route::Trips {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "My trips" }`. Don't also pass `class`. A link keeps link semantics: it navigates on Enter, not Space.

## Component Structure

```rust
button {
    // Global html attributes
    class: "dx-button",
    "data-style": "outline",
    "data-size": "default",
    // Children
    {children}
}
```

## API

```rust
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

/// A [`Button`]'s look for an element that is not a `<button>`, such as the router's `Link`
/// (which renders its own `<a>`):
/// `Link { to: Route::Trips {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "My trips" }`.
/// Don't also pass `class` to that element: it would replace the button class.
pub fn button_look(variant: ButtonVariant, size: ButtonSize) -> Vec<Attribute>

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::button::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Button { "Primary" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
            Button { variant: ButtonVariant::Destructive, "Delete" }
            Button { variant: ButtonVariant::Link, "Link" }
        }
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; margin-top: 0.75rem;",
            Button { size: ButtonSize::Xs, "Extra small" }
            Button { size: ButtonSize::Sm, "Small" }
            Button { "Default" }
            Button { size: ButtonSize::Lg, "Large" }
            Button { disabled: true, "Disabled" }
        }
    }
}
```
