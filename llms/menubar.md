# Menubar

- Copy into your app: `rust-ui add menubar` or `dx components add menubar --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::menubar::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Menubar component can be used to display a menu bar with collapsible menus.

## Component Structure

```rust
// The Menubar component wraps the entire menu bar and contains the individual menus in the order of their index.
Menubar {
    // The MenubarMenu contains the individual menus that can be opened.
    MenubarMenu {
        // The index of the menu, used to determine the order in which menus are displayed.
        index: 0,
        // The menubar trigger is the element that will display the menu when activated.
        MenubarTrigger {
            // The content of the trigger button
            {children}
        }
        // The menubar content contains all the items that will be displayed in the menu when it is opened.
        MenubarContent {
            // Each menubar item represents an individual items in the menu.
            MenubarItem {
                // The value of the item which will be passed to the on_select callback when the item is selected.
                value: "",
                on_select: |value: String| {
                    // This callback is triggered when the item is selected.
                    // The value parameter contains the value of the selected item.
                },
            }
        }
    }
}
```

## API

```rust
#[component]
pub fn Menubar(props: MenubarProps) -> Element

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element
```

## Example

```rust
use crate::components::menubar::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            Menubar {
                MenubarMenu { index: 0usize,
                    MenubarTrigger { "File" }
                    MenubarContent {
                        MenubarItem {
                            index: 0usize,
                            value: "new".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "New"
                        }
                        MenubarItem {
                            index: 1usize,
                            value: "open".to_string(),
                            disabled: true,
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Open"
                        }
                        MenubarItem {
                            index: 2usize,
                            value: "save".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Save"
                        }
                    }
                }
                MenubarMenu { index: 1usize,
                    MenubarTrigger { "Edit" }
                    MenubarContent {
                        MenubarItem {
                            index: 0usize,
                            value: "cut".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Cut"
                        }
                        MenubarItem {
                            index: 1usize,
                            value: "copy".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Copy"
                        }
                        MenubarItem {
                            index: 2usize,
                            value: "paste".to_string(),
                            on_select: move |value| {
                                tracing::info!("Selected value: {}", value);
                            },
                            "Paste"
                        }
                    }
                }
            }
        }
    }
}
```
