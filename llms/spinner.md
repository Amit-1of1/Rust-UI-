# Spinner

- Copy into your app: `rust-ui add spinner` or `dx components add spinner --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::spinner::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A loading indicator.

## Accessibility

- Role and name: `role="status"` with an `aria-label` (default "Loading")
- Keyboard: none (not interactive)
- Reduced motion: the spin slows to 3s instead of stopping, so it still shows that work is happening

## API

```rust
/// A loading indicator. `label` is read by screen readers.
#[component]
pub fn Spinner(
    /// Defaults to [`UiStrings::loading`](crate::components::ui_strings::UiStrings).
    #[props(into)]
    label: Option<String>,
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
            Spinner {}
            Spinner { style: "width: 1.5rem; height: 1.5rem;" }
            Spinner { style: "width: 2rem; height: 2rem; border-width: 3px;" }
        }
    }
}
```
