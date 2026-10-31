# Alert

- Copy into your app: `atoll add alert` or `dx components add alert --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::alert::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A message that interrupts: it is announced as soon as it appears. Use `Callout` for a quiet inline note.

## Accessibility

- Role and name: `role="alert"`
- Keyboard: none (not interactive)
- Reduced motion: none

## API

```rust
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

/// A callout. Announced to screen readers (`role="alert"`) when it appears.
#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn AlertTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn AlertDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::alert::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 28rem;",
            Alert {
                AlertTitle { "Heads up" }
                AlertDescription { "You can add components to your app with the registry." }
            }
            Alert { variant: AlertVariant::Destructive,
                AlertTitle { "Something went wrong" }
                AlertDescription { "Your session has expired. Sign in again to continue." }
            }
        }
    }
}
```
