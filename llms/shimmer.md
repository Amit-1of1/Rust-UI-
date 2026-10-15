# Shimmer

- Copy into your app: `rust-ui add shimmer` or `dx components add shimmer --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::shimmer::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

Wraps loading content and sweeps a highlight across it.

## Accessibility

- Role and name: `aria-busy="true"` while loading.
- Keyboard: none
- Reduced motion: the sweep becomes a static tint

## Source

Structure and API adapted from Rust/UI (`shimmer.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.

## API

```rust
/// Wraps content that is still loading and sweeps a highlight across it. While `loading` is
/// true the wrapper is marked `aria-busy`. Under reduced motion the sweep becomes a static tint.
#[component]
pub fn Shimmer(
    #[props(default = true)] loading: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::shimmer::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut loading = use_signal(|| true);
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 24rem;",
            Shimmer { loading: loading(),
                div { style: "padding: 1rem;",
                    strong { "Quarterly report" }
                    p { style: "margin: 0.25rem 0 0;", "Revenue is up 12% on the previous quarter." }
                }
            }
            label {
                input {
                    r#type: "checkbox",
                    checked: loading(),
                    onchange: move |e| loading.set(e.checked()),
                }
                " Loading"
            }
        }
    }
}
```
