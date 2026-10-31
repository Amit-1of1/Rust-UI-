# Callout

- Copy into your app: `atoll add callout` or `dx components add callout --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::callout::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

An inline note (info, success, warning) for docs and forms. Static: use `Alert` for messages that must interrupt.

## Accessibility

- Role and name: `role="note"`; the optional title is a paragraph.
- Keyboard: none (not interactive)
- Reduced motion: none

## Source

Structure and API adapted from Rust/UI (`callout.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.

## API

```rust
pub enum CalloutVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
}

/// An inline note for docs and forms. Unlike `Alert` it is not announced when it appears, so use
/// it for static content; use `Alert` for messages that must interrupt.
#[component]
pub fn Callout(
    #[props(default)] variant: CalloutVariant,
    /// Optional bold heading.
    #[props(into, default)]
    title: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::callout::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 32rem;",
            Callout { title: "Heads up", "Callouts are static notes. Use " code { "Alert" } " to interrupt." }
            Callout { variant: CalloutVariant::Info, title: "Tip", "Press / to open the command palette." }
            Callout { variant: CalloutVariant::Success, "Saved. Your changes are live." }
            Callout { variant: CalloutVariant::Warning, title: "Careful", "This replaces the existing file." }
        }
    }
}
```
