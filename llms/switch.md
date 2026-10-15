# Switch

- Copy into your app: `rust-ui add switch` or `dx components add switch --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::switch::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Switch component allows users to toggle between two states, such as on and off.

## Component Structure

```rust
// The Switch component includes the switch thumb.
Switch {
    // The current state of the switch, true for on and false for off.
    checked: true,
    // Callback function triggered when the switch state changes.
    on_checked_change: |checked: bool| {
        // Handle the change in switch state.
    }
}
```

## API

```rust
#[component]
pub fn Switch(props: SwitchProps) -> Element
```

## Example

```rust
use crate::components::switch::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut on = use_signal(|| false);
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Switch {
                checked: on(),
                aria_label: "Airplane mode",
                on_checked_change: move |v| on.set(v),
            }
            span { style: "font-size: 0.875rem;", if on() { "On" } else { "Off" } }
        }
    }
}
```
