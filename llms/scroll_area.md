# Scroll area

- Copy into your app: `atoll add scroll_area` or `dx components add scroll_area --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::scroll_area::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The ScrollArea component is used to create a scrollable container for its children. It can be used to enable scrolling for content that overflows the available space.

## Component Structure

```rust
// The ScrollArea component wraps all scrollable content.
ScrollArea {
    // The direction in which the scroll area can scroll. Can be one of Horizontal, Vertical, or Both.
    scroll_direction: ScrollDirection::Vertical,
    // The content of the scrollable area
    {children}
}
```

## API

```rust
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element
```

## Example

```rust
use crate::components::scroll_area::*;
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;

#[component]
pub fn Demo() -> Element {
    rsx! {

        ScrollArea {
            width: "10em",
            height: "10em",
            border: "1px solid var(--primary-color-6)",
            border_radius: "0.5em",
            padding: "0 1em 1em 1em",
            direction: ScrollDirection::Vertical,
            tabindex: "0",
            div {
                for i in 1..=20 {
                    p { "Scrollable content item {i}" }
                }
            }
        }
    }
}
```
