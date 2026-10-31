# Separator

- Copy into your app: `atoll add separator` or `dx components add separator --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::separator::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Separator component is a simple horizontal or vertical line that can be used to visually separate content in your application.

## Component Structure

```rust
// The Separator component can be oriented horizontally or vertically.
Separator {
    // The orientation of the separator line. true for horizontal, false for vertical.
    horizontal: true,
    // The decorative property controls if the separator is decorative and should not be visible to screen readers.
    decorative: false,
}
```

## API

```rust
#[component]
pub fn Separator(props: SeparatorProps) -> Element
```

## Example

```rust
use crate::components::separator::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "max-width: 20rem;",
            strong { "Atoll kit" }
            p { style: "margin: 0.25rem 0; font-size: 0.875rem;", "A personal component library." }
            Separator { horizontal: true, decorative: true, style: "margin: 1rem 0;" }
            div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; font-size: 0.875rem;", "Docs" Separator { horizontal: false, decorative: true, style: "height: 1rem;" } "Source" }
        }
    }
}
```
