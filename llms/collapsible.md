# Collapsible

- Copy into your app: `atoll add collapsible` or `dx components add collapsible --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::collapsible::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The `Collapsible` component allows users to create expandable sections that can be toggled open or closed. This is useful for displaying content in a compact manner, such as FAQs, menus, or any content that benefits from being hidden until needed.

## Component Structure

```rust
// The collapsible component must wrap all collapsible items.
Collapsible {
    // The trigger is used to expand or collapse the item.
    CollapsibleTrigger {}
    // The content that is shown when the item is expanded.
    CollapsibleContent {}
}
```

## Keyboard and accessibility

Follows the WAI-ARIA [Disclosure (Show/Hide)](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/) pattern. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | On the trigger | Shows or hides the content. | yes |
| Space | On the trigger | Shows or hides the content. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element

#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element

#[component]
pub fn CollapsibleItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn CollapsibleList(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::collapsible::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Collapsible {
            CollapsibleTrigger {
                b { "Recent Activity" }
            }
            CollapsibleList {
                CollapsibleItem { "Added a new feature to the collapsible component" }
                CollapsibleContent {
                    CollapsibleItem { "Fixed a bug in the collapsible component" }
                    CollapsibleItem { "Updated the documentation for the collapsible component" }
                }
            }
        }
    }
}
```
