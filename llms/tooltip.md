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

## Timing

Hovering waits `delay_ms` (400 ms by default) before the tooltip opens, so moving the pointer across a toolbar doesn't flash every tooltip. Keyboard focus opens it at once. `Tooltip { delay_ms: 0 }` opens on hover straight away. Escape closes it.

## Keyboard and accessibility

Follows the WAI-ARIA [Tooltip](https://www.w3.org/WAI/ARIA/apg/patterns/tooltip/) pattern. Roles: `tooltip`. Works on native (Blitz): partly.

| Key | When | What it does | Native |
|---|---|---|---|
| Tab | On the trigger, with the tooltip shown (focus shows it) | Moves focus on and hides the tooltip. | not yet |
| Escape | While the tooltip is shown | Hides the tooltip; focus stays on the trigger. | yes |

States:

- `shown`: The tooltip text is shown next to the trigger.

Native: On native, Tab from the only focusable element in the demo leaves focus where it is, so the tooltip stays.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// The props for [`Tooltip`].
pub struct TooltipProps {
    /// Whether the tooltip is open (controlled). A controlled tooltip opens without delay.
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the tooltip is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// How long the pointer must rest on the trigger before the tooltip opens, in
    /// milliseconds, so tooltips don't flash open as the cursor crosses the screen. Keyboard
    /// focus opens it at once. Defaults to 400.
    #[props(default = 400)]
    pub delay_ms: u64,

    /// Additional attributes for the tooltip.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// A [`TooltipTrigger`] and a [`TooltipContent`].
    pub children: Element,
}

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
