# Marquee

- Copy into your app: `rust-ui add marquee` or `dx components add marquee --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::marquee::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A looping auto-scrolling strip (logos, quotes).

## Accessibility

- Role and name: `role="group"` with `label`; the repeated copy is `aria-hidden` and `inert`.
- Keyboard: a Pause/Play button (`aria-pressed`) and focus-within pauses; WCAG 2.2.2 needs a pause control for motion over 5 s
- Reduced motion: scrolling stops and the row becomes horizontally scrollable

## Source

Structure and API adapted from Rust/UI (`marquee.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.

## API

```rust
/// A looping, auto-scrolling strip (logos, quotes). It has a pause button, pauses while
/// hovered or focused, and under reduced motion stops and becomes a scrollable row. The
/// repeated copy is hidden from assistive tech.
#[component]
pub fn Marquee(
    /// Seconds for one full loop.
    #[props(default = 20)]
    duration: u32,
    /// Accessible name for the strip. Defaults to "Scrolling content" ([`UiStrings`](crate::components::ui_strings::UiStrings)).
    #[props(into)]
    label: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::marquee::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "max-width: 32rem;",
            Marquee { label: "Customer logos", duration: 15,
                for name in ["Acme", "Globex", "Initech", "Umbrella", "Hooli", "Stark"] {
                    span { style: "font-weight: 600; white-space: nowrap;", "{name}" }
                }
            }
        }
    }
}
```
