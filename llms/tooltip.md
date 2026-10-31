# Tooltip

- Copy into your app: `atoll add tooltip` or `dx components add tooltip --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::tooltip::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Tooltip component is used to display additional information when a user hovers over an element.

## Component Structure

```rust
// The Tooltip component wraps the trigger element and the content that will be displayed on hover.
Tooltip {
    // The TooltipTrigger contains the elements that will trigger the tooltip to display when hovered over.
    TooltipTrigger {
        // The elements that will trigger the tooltip when hovered over.
        {children}
    }
    // The TooltipContent contains the content that will be displayed when the user hovers over the trigger.
    TooltipContent {
        // The side of the TooltipTrigger where the content will be displayed. Can be one of Top, Right, Bottom, or Left.
        side: ContentSide::Top,
        // The alignment of the TooltipContent relative to the TooltipTrigger. Can be one of Start, Center, or End.
        align: ContentAlign::Center,
        // The content of the tooltip, which can include text, images, or any other elements.
        {children}
    }
}
```

## API

```rust
#[component]
pub fn Tooltip(props: TooltipProps) -> Element

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element
```

## Example

```rust
use crate::components::tooltip::*;
use dioxus::prelude::*;
use dioxus_primitives::ContentSide;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { "Hover or focus me" }
            TooltipContent { side: ContentSide::Top, "Add to library" }
        }
    }
}
```
