# Dropdown menu

- Copy into your app: `rust-ui add dropdown_menu` or `dx components add dropdown_menu --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::dropdown_menu::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The DropdownMenu component is used to create a dropdown menu that can be triggered by a button click. It allows users to select an option from a list of items.

## Component Structure

```rust
// The dropdown menu component must wrap all dropdown items.
DropdownMenu {
    // The dropdown menu trigger is the button that will display the dropdown menu when clicked.
    DropdownMenuTrigger {
        // The content of the trigger to display inside the button.
        {children}
    }
    // The dropdown menu content contains all the items that will be displayed in the dropdown menu.
    DropdownMenuContent {
        // Each dropdown menu item represents an individual option in the dropdown menu. Items are displayed in order based on the order of the index property.
        DropdownMenuItem {
            // The index of the item, used to determine the order in which items are displayed.
            index: 0,
            // The value of the item which will be passed to the on_select callback when the item is selected.
            value: "",
            on_select: |value: String| {
                // This callback is triggered when the item is selected.
                // The value parameter contains the value of the selected item.
            },
        }
    }
}
```

## Keyboard and accessibility

Follows the WAI-ARIA [Menu Button](https://www.w3.org/WAI/ARIA/apg/patterns/menu-button/) pattern. Roles: `listbox`, `option`. Works on native (Blitz): partly.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | On the trigger | Opens the menu; focus stays on the trigger until an arrow key moves it in (the APG moves it to the first item at once). | yes |
| Space | On the trigger | Opens the menu, as Enter does. | yes |
| ArrowDown | In the open menu | Moves focus to the first item, then to the next one. | yes |
| ArrowUp | In the open menu | Moves focus to the previous item, wrapping from the first to the last. | yes |
| Enter | On an item | Chooses the item, closes the menu and returns focus to the trigger. | not yet |
| Escape | In the open menu | Closes the menu and returns focus to the trigger. | yes |

States:

- `open`: The list of actions is shown.
- `closed`: Only the trigger is shown.

Known gaps:

- The items are a `listbox` of `option`s, not a `menu` of `menuitem`s (upstream markup).
- Opening does not move focus to the first item; the first arrow key does.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element

#[component]
pub fn DropdownMenuItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuItemProps<T>,
) -> Element
```

## Example

```rust
use crate::components::dropdown_menu::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut picked = use_signal(|| None::<&'static str>);
    let actions = ["Edit", "Duplicate", "Archive", "Delete"];
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            DropdownMenu { default_open: false,
                DropdownMenuTrigger { "Actions" }
                DropdownMenuContent {
                    for (i, a) in actions.into_iter().enumerate() {
                        DropdownMenuItem::<&'static str> {
                            value: a,
                            index: i,
                            on_select: move |v| picked.set(Some(v)),
                            {a}
                        }
                    }
                }
            }
            if let Some(a) = picked() {
                span { style: "font-size: 0.875rem;", "Selected: {a}" }
            }
        }
    }
}
```
