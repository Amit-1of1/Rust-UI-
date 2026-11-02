# Context menu

- Copy into your app: `atoll add context_menu` or `dx components add context_menu --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
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
## Submenus

Put a `ContextMenuSub` where an item would go. Its `index` is the trigger's place among the menu's items; the items inside `ContextMenuSubContent` count from 0 again. Submenus can nest.

```rust
ContextMenuSub { index: 2usize,
    ContextMenuSubTrigger { "Share" }
    ContextMenuSubContent {
        ContextMenuItem { index: 0usize, value: "a".to_string(), "Copy link" }
    }
}
```

ArrowRight, Enter or Space on the trigger opens the submenu and focuses its first item; ArrowLeft or Escape closes only the submenu and puts focus back on the trigger; choosing an item closes the whole menu. With a mouse the submenu opens after the pointer rests on its trigger for 100 ms, and stays open for 300 ms after the pointer leaves, so it can cross other items on the way. The submenu opens to the inline end and does not flip at screen edges, and the arrow keys are not mirrored in right-to-left text yet.

## Stability

The `ContextMenuSub` parts are experimental (new on 2026-11-01, docs/STABILITY.md); the rest of the component is stable.

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

/// A nested menu inside [`ContextMenuContent`] (or another submenu). Its trigger is an item of
/// the surrounding menu at `index`; the content holds ordinary [`ContextMenuItem`]s.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ContextMenuSub(props: ContextMenuSubProps) -> Element

/// The item that opens a [`ContextMenuSub`], with a chevron after its label.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ContextMenuSubTrigger(props: ContextMenuSubTriggerProps) -> Element

/// The menu of a [`ContextMenuSub`]: the context menu's own look, beside its trigger.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ContextMenuSubContent(props: ContextMenuSubContentProps) -> Element
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
                // A submenu: its trigger is item 3 of this menu, its items count from 0.
                ContextMenuSub { index: 3usize,
                    ContextMenuSubTrigger { "Share" }
                    ContextMenuSubContent {
                        ContextMenuItem {
                            value: "copy-link".to_string(),
                            index: 0usize,
                            on_select: move |value| selected_item.set(Some(value)),
                            "Copy link"
                        }
                        ContextMenuItem {
                            value: "email".to_string(),
                            index: 1usize,
                            on_select: move |value| selected_item.set(Some(value)),
                            "Email"
                        }
                    }
                }
                ContextMenuItem {
                    value: "delete".to_string(),
                    index: 4usize,
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
