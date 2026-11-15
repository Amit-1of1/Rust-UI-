# Accordion

- Copy into your app: `atoll add accordion` or `dx components add accordion --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `icons`).
- Import: `use crate::components::accordion::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The accordion component is used to display collapsible content panels for presenting information in a limited amount of space. It allows users to expand and collapse sections to view more or less content.

## Component Structure

```rust
// The accordion component must wrap all accordion items.
Accordion {
    // Each accordion item contains both a trigger the expanded contents of the item.
    AccordionItem {
        // Each item must have an index starting from 0 to control where the item is placed.
        index: 0,
        // The trigger is used to expand or collapse the item.
        AccordionTrigger {}
        // The content that is shown when the item is expanded.
        AccordionContent {}
    }
}
```

## Keyboard and accessibility

Follows the WAI-ARIA [Accordion](https://www.w3.org/WAI/ARIA/apg/patterns/accordion/) pattern. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | On a section header | Opens or closes that section. | yes |
| Space | On a section header | Opens or closes that section. | yes |
| Tab | On a section header | Moves to the next header (or into the open section). | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
#[component]
pub fn Accordion(props: AccordionProps) -> Element

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element
```

## Example

```rust
use crate::components::accordion::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let items = [
        (
            "Is it accessible?",
            "Yes. Behaviour comes from dioxus-primitives, which follows WAI-ARIA.",
        ),
        (
            "Is it styled?",
            "Yes. It ships with a neutral, shadcn-like theme you own.",
        ),
        (
            "Is it animated?",
            "Open and close are animated in CSS; springs arrive in week 2.",
        ),
    ];
    rsx! {
        div { style: "max-width: 32rem;",
            Accordion { allow_multiple_open: false, horizontal: false,
                for (i, (q, a)) in items.into_iter().enumerate() {
                    AccordionItem { index: i,
                        AccordionTrigger { {q} }
                        AccordionContent {
                            p { style: "margin: 0; padding-bottom: 1rem;", {a} }
                        }
                    }
                }
            }
        }
    }
}
```
