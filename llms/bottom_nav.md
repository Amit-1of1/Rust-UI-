# BottomNav

- Copy into your app: `atoll add bottom_nav` or `dx components add bottom_nav --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::bottom_nav::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A mobile-style tab bar of links or buttons.

## Accessibility

- Role and name: a `nav` landmark named by `label`; the active item has `aria-current="page"`; icons are `aria-hidden`.
- Keyboard: Tab through items; links or buttons
- Reduced motion: none

## Source

Structure and API adapted from Rust/UI (`bottom_nav.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.

## API

```rust
/// A mobile-style tab bar fixed to the bottom of its container. Wrap [`BottomNavItem`]s;
/// `label` names the landmark (for example "Main").
#[component]
pub fn BottomNav(
    #[props(into)] label: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// One destination. With `href` it renders a link, otherwise a button. `current` marks the
/// active page (`aria-current="page"`).
#[component]
pub fn BottomNavItem(
    #[props(into)] label: String,
    #[props(into, default)] href: Option<String>,
    #[props(default)] current: bool,
    #[props(default)] onclick: Callback<MouseEvent>,
    /// Decorative icon; hidden from assistive tech.
    #[props(default)]
    icon: Option<Element>,
) -> Element
```

## Example

```rust
use crate::components::bottom_nav::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut current = use_signal(|| 0_usize);
    rsx! {
        div { style: "max-width: 24rem; border: 1px solid var(--primary-color-6); border-radius: var(--radius); overflow: hidden;",
            BottomNav { label: "Demo tabs",
                for (index , name) in ["Home", "Search", "Inbox"].into_iter().enumerate() {
                    BottomNavItem {
                        label: name,
                        current: current() == index,
                        onclick: move |_| current.set(index),
                        icon: rsx! {
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                circle { cx: "12", cy: "12", r: "8" }
                            }
                        },
                    }
                }
            }
        }
    }
}
```
