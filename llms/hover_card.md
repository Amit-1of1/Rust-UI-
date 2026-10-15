# Hover card

- Copy into your app: `rust-ui add hover_card` or `dx components add hover_card --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::hover_card::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The HoverCard component can be used to display additional information when a user hovers over an element. It is useful for showing tooltips, additional details, or any other content that should be revealed on hover.

## Component Structure

```rust
// The HoverCard component wraps the trigger element and the content that will be displayed on hover.
HoverCard {
    HoverCardTrigger {
        // Anything inside the trigger (icon, text, rich markup) acts as the hover target.
        {children}
    }
    HoverCardContent {
        side: ContentSide::Bottom,
        align: ContentAlign::Start,
        {children}
    }
}
```

## Don't put controls in the trigger

`HoverCardTrigger` is itself focusable (`role="button"`, `tabindex="0"`), so keyboard users can open the card. A button or link inside it is a control inside a control (axe `nested-interactive`), and screen readers announce it badly. Put text, an avatar or an icon directly inside the trigger, and put links and buttons inside `HoverCardContent` (or next to the trigger).

## API

```rust
#[component]
pub fn HoverCard(props: HoverCardProps) -> Element

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element
```

## Example

```rust
use crate::components::hover_card::*;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "padding: 50px; display: flex; flex-direction: row; flex-wrap: wrap; gap: 40px; justify-content: center; align-items: center;",
            HoverCard {
                HoverCardTrigger {
                    i { "Dioxus" }
                }
                HoverCardContent { side: ContentSide::Bottom,
                    div { padding: "1rem",
                        "Dioxus is"
                        i { " the " }
                        "Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time."
                    }
                }
            }
        }
    }
}
```
