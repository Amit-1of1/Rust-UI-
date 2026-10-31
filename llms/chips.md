# Chips

- Copy into your app: `atoll add chips` or `dx components add chips --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::chips::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A row of on/off chips for filters and multi-choice tags. Each chip is a native checkbox inside its label; a check mark appears when it is on.

```rust
Chips { label: "Filter by status",
    Chip { default_checked: true, "Open" }
    Chip { "In review" }
    Chip { checked: done(), on_change: move |v| done.set(v), "Done" }
}
```

For one choice out of several, use `ToggleGroup` or `RadioGroup` instead.

## Accessibility

- Role and name: `role="group"` named by `label`; each chip is a native checkbox named by its text
- Keyboard: Tab moves between chips, Space toggles
- State: announced as checked or not checked, and shown with a check mark as well as colour; forced-colors mode outlines the selected chip
- Reduced motion: the check mark and colour change without transition

## Source

Idea from Rust/UI `chips.rs` (rust-ui/ui@43ddc65, MIT). Rewritten: upstream hides the checkbox with `display: none` (unreachable by keyboard and screen readers) and styles with Tailwind; here the checkbox stays focusable and the styles use theme tokens.

## API

```rust
/// A labelled group of [`Chip`]s, for filters and multi-choice tags.
#[component]
pub fn Chips(
    /// Names the group for screen readers, e.g. "Filter by status".
    #[props(into)]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// One on/off chip: a native checkbox inside its label, so Tab reaches it, Space toggles it
/// and screen readers announce "checked". A check mark shows the state as well as the colour.
///
/// Uncontrolled with `default_checked`, or controlled with `checked` plus `on_change`.
#[component]
pub fn Chip(
    /// Controlled state. Leave unset to let the chip keep its own.
    checked: Option<bool>,
    /// Starting state when uncontrolled.
    #[props(default)]
    default_checked: bool,
    #[props(default)] disabled: bool,
    /// Form field name and value, when the chips sit in a `<form>`.
    #[props(into)]
    name: Option<String>,
    #[props(into)] value: Option<String>,
    #[props(default)] on_change: EventHandler<bool>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::chips::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut done = use_signal(|| false);
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            Chips { label: "Filter by status",
                Chip { default_checked: true, "Open" }
                Chip { "In review" }
                Chip { checked: done(), on_change: move |v| done.set(v), "Done" }
                Chip { disabled: true, "Archived" }
            }
            p { style: "margin: 0; font-size: 0.875rem; color: var(--secondary-color-5);",
                if done() { "Showing done tasks." } else { "Hiding done tasks." }
            }
        }
    }
}
```
