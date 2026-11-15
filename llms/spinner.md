# Spinner

- Copy into your app: `atoll add spinner` or `dx components add spinner --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::spinner::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A loading indicator for waits of unknown length. For a known amount of work, use `Progress`, which can show how much is left.

`size` is `Sm`, `Default`, `Lg` or `Xl`. The ring takes the text colour of its parent, so it follows the colour of the button, alert or text it sits in. Give a spinner that stands alone a `label` that says what is loading ("Loading invoices"). Inside a button, use the button's `loading` instead.

## Accessibility

- Role and name: `role="status"` with an `aria-label` (default "Loading")
- Keyboard: none (not interactive)
- Reduced motion: the spin slows to 3s instead of stopping, so it still shows that work is happening

## API

```rust
/// A [`Spinner`]'s size.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum SpinnerSize {
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

/// A loading indicator. `label` is read by screen readers. The ring takes the text colour
/// of its parent, so inside a destructive button or an alert it follows that colour.
#[component]
pub fn Spinner(
    /// Defaults to [`UiStrings::loading`](crate::components::ui_strings::UiStrings). Say what
    /// is loading when the spinner stands alone ("Loading invoices").
    #[props(into)]
    label: Option<String>,
    #[props(default)] size: SpinnerSize,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
use crate::components::spinner::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; gap: 1rem; align-items: center;",
            Spinner { size: SpinnerSize::Sm }
            Spinner {}
            Spinner { size: SpinnerSize::Lg }
            Spinner { size: SpinnerSize::Xl, label: "Loading invoices" }
            // The ring follows the text colour around it.
            span { style: "display: inline-flex; gap: 0.5rem; align-items: center; color: var(--error-text-color);",
                Spinner { label: "Retrying payment" }
                "Retrying payment"
            }
        }
    }
}
```
