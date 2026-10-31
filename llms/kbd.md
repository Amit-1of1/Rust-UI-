# Kbd

- Copy into your app: `atoll add kbd` or `dx components add kbd --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::kbd::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A keyboard key cap, for showing shortcuts.

## Accessibility

- Role and name: plain text in a `kbd` element
- Keyboard: none (not interactive)
- Reduced motion: none

## API

```rust
/// A keyboard key, e.g. `Kbd { "Ctrl" }`.
#[component]
pub fn Kbd(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// A row of keys that make up one shortcut.
#[component]
pub fn KbdGroup(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::kbd::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; gap: 1rem; align-items: center;",
            KbdGroup {
                Kbd { "Ctrl" }
                "+"
                Kbd { "K" }
            }
            Kbd { "Esc" }
        }
    }
}
```
