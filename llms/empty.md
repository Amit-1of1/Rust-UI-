# Empty

- Copy into your app: `rust-ui add empty` or `dx components add empty --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::empty::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

Placeholder for a list, table or page with nothing in it, with optional icon, text and actions. Compose `Empty`, `EmptyHeader`, `EmptyMedia`, `EmptyTitle`, `EmptyDescription`, `EmptyContent`.

## Accessibility

- Role and name: `EmptyTitle` is `role="heading"` with a configurable `level`; the media is `aria-hidden`.
- Keyboard: actions are normal buttons
- Reduced motion: none

## Source

Structure and API adapted from Rust/UI (`empty.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.

## API

```rust
/// A placeholder for a list, table or page with nothing in it yet.
/// Compose with `EmptyHeader`, `EmptyMedia`, `EmptyTitle`, `EmptyDescription` and `EmptyContent`.
#[component]
pub fn Empty(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn EmptyHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Decorative media (an icon or illustration). Hidden from assistive tech.
#[component]
pub fn EmptyMedia(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The heading. `level` is the heading level (default 3); pick the one that fits the page outline.
#[component]
pub fn EmptyTitle(
    #[props(default = 3)] level: u8,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn EmptyDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Actions, usually one or two buttons.
#[component]
pub fn EmptyContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::empty::*;
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
```
