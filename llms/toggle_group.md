# Toggle group

- Copy into your app: `atoll add toggle_group` or `dx components add toggle_group --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::toggle_group::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Toggle Group component is used to create a group of toggle buttons that allows the user to select one or more options from a set. It is useful for creating a set of options.

## Component Structure

```rust
// The ToggleGroup component wraps all toggle items in the group.
ToggleGroup {
    // The orientation of the toggle group, true for horizontal and false for vertical.
    horizontal: true,
    // The toggle item represents each individual toggle button in the group.
    ToggleItem {
        // The index of the toggle item, used to determine the order in which items are focused.
        index: 0,
        // The contents of the toggle item button
        {children}
    }
}
```

## Keyboard and accessibility

Follows the WAI-ARIA [Toolbar (group of toggle buttons)](https://www.w3.org/WAI/ARIA/apg/patterns/toolbar/) pattern. Roles: `group`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Space | On a toggle | Turns it on or off (aria-pressed). | yes |
| Enter | On a toggle | Turns it on or off (aria-pressed). | yes |
| ArrowRight | On a toggle | Moves focus to the next toggle. | yes |
| ArrowLeft | On a toggle | Moves focus to the previous toggle. | yes |
| Home | On a toggle | Moves focus to the first toggle. | yes |
| End | On a toggle | Moves focus to the last toggle. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element

#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element
```

## Example

```rust
use crate::components::toggle_group::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {

        ToggleGroup { horizontal: true, allow_multiple_pressed: true, role: "group", aria_label: "Text style",
            ToggleItem { index: 0usize, aria_label: "Bold",
                b { "B" }
            }
            ToggleItem { index: 1usize, aria_label: "Italic",
                i { "I" }
            }
            ToggleItem { index: 2usize, aria_label: "Underline",
                u { "U" }
            }
        }
    }
}
```
