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
