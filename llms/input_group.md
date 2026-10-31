# InputGroup

- Copy into your app: `atoll add input_group` or `dx components add input_group --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::input_group::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

An input with text, icons or buttons attached inside one bordered box.

```rust
InputGroup {
    InputGroupAddon { "https://" }
    InputGroupInput { "aria-label": "Website" }
    InputGroupAddon { end: true, Button { "Go" } }
}
```

The group draws the border and the focus ring (`:focus-within`), so the control inside has none. `invalid` turns the border to the error colour.

## Accessibility

- Role and name: `role="group"`; the control needs its own label (`<label for>` or `aria-label`), because addon text is not a label. Add `aria-invalid` on the control when invalid and show an error message linked with `aria-describedby`.
- Keyboard: Tab moves between the control and any buttons in addons.
- Reduced motion: no animation.

## Source

Structure adapted from Rust/UI's `input_group.rs` (MIT, rust-ui/ui commit 43ddc65); styling and API rewritten.

## API

```rust
/// An input with text, icons or buttons attached inside one bordered box ("https://", "USD",
/// a search button). Use `InputGroupInput` for the control and `InputGroupAddon` for the rest.
/// Give the control a visible `<label>` or `aria-label`.
#[component]
pub fn InputGroup(
    #[props(default)] invalid: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Static content or a button before (default) or after (`end`) the control.
#[component]
pub fn InputGroupAddon(
    #[props(default)] end: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The text control. It has no border of its own; the group draws it.
#[component]
pub fn InputGroupInput(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    oninput: Option<EventHandler<FormEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
) -> Element
```

## Example

```rust
use crate::components::input_group::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 24rem;",
            InputGroup {
                InputGroupAddon { "https://" }
                InputGroupInput { "aria-label": "Website", placeholder: "example.com" }
            }
            InputGroup {
                InputGroupInput { "aria-label": "Amount", r#type: "number", placeholder: "0.00" }
                InputGroupAddon { end: true, "USD" }
            }
            InputGroup { invalid: true,
                InputGroupInput { "aria-label": "Username", value: "taken_name", "aria-invalid": "true" }
            }
        }
    }
}
```
