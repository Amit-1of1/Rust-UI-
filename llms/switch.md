# Switch

- Copy into your app: `atoll add switch` or `dx components add switch --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::switch::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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

## Keyboard and accessibility

Follows the WAI-ARIA [Switch](https://www.w3.org/WAI/ARIA/apg/patterns/switch/) pattern. Roles: `switch`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Space | On the switch | Turns it on or off. | yes |

Known gaps:

- Enter does not toggle it (optional in the APG); Space does.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

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
