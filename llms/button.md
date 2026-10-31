# Button

- Copy into your app: `atoll add button` or `dx components add button --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`).
- Import: `use crate::components::button::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The button element is used to trigger actions or events in a user interface.

For a link that should look like a button (the router's `Link`, which renders its own `<a>`), spread `button_look(variant, size)` onto it: `Link { to: Route::Trips {}, attributes: button_look(ButtonVariant::Outline, ButtonSize::Default), "My trips" }`. Don't also pass `class`. A link keeps link semantics: it navigates on Enter, not Space.

## Icon-only buttons

A button that shows only an icon is an `IconButton`. Its `label` is required (it becomes the `aria-label`), so an icon button with no name doesn't compile; `ButtonSize` has no icon sizes, so a plain `Button` can't be squared into one either. Mark the icon decorative.

```rust
IconButton { label: "Search", variant: ButtonVariant::Ghost, size: IconButtonSize::Sm,
    Search { "aria-hidden": "true" }
}
```

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

/// A [`Button`]'s size. A button that shows only an icon is an [`IconButton`], which has its own
/// [`IconButtonSize`] and a required `label`.
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
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

/// An [`IconButton`]'s size: a square button sized for one icon.
pub enum IconButtonSize {
    Xs,
    Sm,
    #[default]
    Default,
    Lg,
}

/// A button that shows only an icon, such as a close or search button.
///
/// `label` is required: with no text inside, it is the button's only accessible name
/// (`aria-label`), so "Close" or "Search" instead of nothing. Mark the icon itself decorative.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # use ui::icons::Search;
/// # fn app() -> Element {
/// rsx! { IconButton { label: "Search", variant: ButtonVariant::Ghost, Search { "aria-hidden": "true" } } }
/// # }
/// ```
///
/// Leaving `label` out is a compile error ("Missing required field label"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # use ui::icons::Search;
/// # fn app() -> Element {
/// rsx! { IconButton { variant: ButtonVariant::Ghost, Search { "aria-hidden": "true" } } }
/// # }
/// ```
///
/// and so is squaring a plain [`Button`] with an icon size, which no longer exists:
///
/// ```compile_fail,E0599
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Button { size: ButtonSize::Icon, "+" } }
/// # }
/// ```
#[component]
pub fn IconButton(
    /// What the button does, read by screen readers ("Close", "Search").
    #[props(into)]
    label: String,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: IconButtonSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// The icon.
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::button::*;
use crate::components::icons::{Plus, Settings, Trash};
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
        // Icon-only buttons need a `label` (their accessible name); leaving it out doesn't compile.
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; margin-top: 0.75rem;",
            IconButton { label: "Add", size: IconButtonSize::Xs, Plus { size: "0.75rem", "aria-hidden": "true" } }
            IconButton { label: "Settings", variant: ButtonVariant::Outline, size: IconButtonSize::Sm,
                Settings { size: "1rem", "aria-hidden": "true" }
            }
            IconButton { label: "Settings", variant: ButtonVariant::Secondary, Settings { size: "1rem", "aria-hidden": "true" } }
            IconButton { label: "Delete", variant: ButtonVariant::Destructive, size: IconButtonSize::Lg,
                Trash { size: "1.25rem", "aria-hidden": "true" }
            }
        }
    }
}
```
