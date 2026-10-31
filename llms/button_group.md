# ButtonGroup

- Copy into your app: `atoll add button_group` or `dx components add button_group --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::button_group::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

Joins buttons or inputs into a segmented control, horizontal or vertical.

## Accessibility

- Role and name: `role="group"`; pass an `aria-label`.
- Keyboard: normal Tab order; focused item is raised above its neighbours
- Reduced motion: none

## Source

Structure and API adapted from Rust/UI (`button_group.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.

## API

```rust
/// Joins `Button`s (and inputs) into one segmented control. Give it an `aria-label` so the
/// group is announced; it carries `role="group"`.
#[component]
pub fn ButtonGroup(
    #[props(default)] vertical: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// A non-interactive label segment inside a `ButtonGroup`.
#[component]
pub fn ButtonGroupText(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
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
```
