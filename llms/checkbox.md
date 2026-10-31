# Checkbox

- Copy into your app: `atoll add checkbox` or `dx components add checkbox --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::checkbox::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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

## Keyboard and accessibility

Follows the WAI-ARIA [Checkbox](https://www.w3.org/WAI/ARIA/apg/patterns/checkbox/) pattern. Roles: `checkbox`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Space | On the checkbox | Checks or unchecks it. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

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
