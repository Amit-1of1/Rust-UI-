# Toggle

- Copy into your app: `rust-ui add toggle` or `dx components add toggle --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::toggle::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Toggle component is used to create a toggle button that can be pressed or unpressed. It is useful for creating a binary option, such as enabling or disabling a feature.

## Component Structure

```rust
// The Toggle component is used to create a toggle button that can be pressed or unpressed.
Toggle {
    // The contents of the toggle button.
    "Bold",
}
```

## Keyboard and accessibility

Follows the WAI-ARIA [Button (toggle)](https://www.w3.org/WAI/ARIA/apg/patterns/button/) pattern. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Space | On the toggle | Presses or releases it. | yes |
| Enter | On the toggle | Presses or releases it. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
#[component]
pub fn Toggle(props: ToggleProps) -> Element
```

## Example

```rust
use crate::components::toggle::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { width: "2rem", height: "2rem",
            em { "B" }
        }
    }
}
```
