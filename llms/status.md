# Status

- Copy into your app: `atoll add status` or `dx components add status --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::status::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A coloured dot plus required text label (idle, online, degraded, down). Optional `pulse`.

## Accessibility

- Role and name: The dot is `aria-hidden`; the label carries the meaning, never colour alone.
- Keyboard: none
- Reduced motion: the pulse ring is removed

## Source

Structure and API adapted from Rust/UI (`status.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.

## API

```rust
pub enum StatusKind {
    #[default]
    Neutral,
    Active,
    Warning,
    Error,
    Info,
}

/// A coloured dot with a text label. The label is required: colour alone never carries the
/// meaning. `pulse` adds a soft ring animation (off under reduced motion).
#[component]
pub fn Status(
    #[props(default)] kind: StatusKind,
    #[props(default)] pulse: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::status::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 1.25rem;",
            Status { "Idle" }
            Status { kind: StatusKind::Active, pulse: true, "Online" }
            Status { kind: StatusKind::Warning, "Degraded" }
            Status { kind: StatusKind::Error, "Down" }
            Status { kind: StatusKind::Info, "Deploying" }
        }
    }
}
```
