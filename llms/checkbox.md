# Checkbox

- Copy into your app: `rust-ui add checkbox` or `dx components add checkbox --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::checkbox::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Checkbox component is used to create an accessible checkbox input.

## Component Structure

```rust
// The checkbox component creates a checkbox button
Checkbox {
    // The checkbox indicator is a child component which will only be visible when the checkbox is checked.
    CheckboxIndicator {
        // The content of the checkbox indicator, typically an icon or image.
        {children}
    }
}
```

## API

```rust
pub use dioxus_primitives::checkbox::CheckboxState;

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element
```

## Example

```rust
use crate::components::checkbox::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Checkbox { id: "terms", name: "terms" }
            Label { html_for: "terms", "Accept terms" }
        }
    }
}
```
