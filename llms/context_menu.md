# Context menu

- Copy into your app: `atoll add context_menu` or `dx components add context_menu --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::context_menu::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The context menu component can be used to define a context menu that is displayed when the user right-clicks on an element. It can contain various menu items that the user can interact with.

## Component Structure

```rust
// The context menu component must wrap all context menu items.
ContextMenu {
    // The context menu trigger is the element that will display the context menu when right-clicked.
    ContextMenuTrigger {
        // The content of the trigger
        {children}
    }
    // The context menu content contains all the items that will be displayed in the context menu.
    ContextMenuContent {
        // Each context menu item represents an individual action in the context menu. Items are displayed in order based on the order of the index property.
        ContextMenuItem {
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
pub fn ContextMenu(props: ContextMenuProps) -> Element

#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element
```

## Example

```rust
use crate::components::context_menu::*;
use dioxus::prelude::*;
#[component]
pub fn Demo() -> Element {
    let mut selected_item = use_signal(|| None);

    rsx! {
        ContextMenu {
            ContextMenuTrigger { "right click here" }
            ContextMenuContent {
                ContextMenuItem {
                    value: "edit".to_string(),
                    index: 0usize,
                    on_select: move |value| {
                        selected_item.set(Some(value));
                    },
                    "Edit"
                }
                ContextMenuItem {
                    value: "undo".to_string(),
                    index: 1usize,
                    disabled: true,
                    on_select: move |value| {
                        selected_item.set(Some(value));
                    },
                    "Undo"
                }
                ContextMenuItem {
                    value: "duplicate".to_string(),
                    index: 2usize,
                    on_select: move |value| {
                        selected_item.set(Some(value));
                    },
                    "Duplicate"
                }
                ContextMenuItem {
                    value: "delete".to_string(),
                    index: 3usize,
                    on_select: move |value| {
                        selected_item.set(Some(value));
                    },
                    "Delete"
                }
            }
        }

        if let Some(item) = selected_item() {
            span { margin_left: "10px", "Selected: {item}" }
        }
    }
}
```
