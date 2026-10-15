# Popover

- Copy into your app: `rust-ui add popover` or `dx components add popover --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::popover::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Popover primitive provides an interactive modal that is positioned relative to the target element. It can be used to display additional options, or actions related to the trigger.

## Component Structure

```rust
// The PopoverRoot is the root component that contains the trigger and content.
PopoverRoot {
    // The PopoverTrigger contains the elements that will trigger the popover to display when clicked.
    PopoverTrigger {
        "Show Popover"
    }
    // The PopoverContent contains the content that will be displayed when the user clicks on the trigger.
    PopoverContent {
        side: ContentSide::Top,
        align: ContentAlign::Center,
        {children}
    }
}
```

## API

```rust
#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::popover::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        PopoverRoot { open: open(), on_open_change: move |v| open.set(v),
            PopoverTrigger { "Open popover" }
            PopoverContent { gap: "0.5rem",
                strong { "Dimensions" }
                span { style: "font-size: 0.875rem;", "Set the dimensions for the layer." }
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Done" }
            }
        }
    }
}
```
