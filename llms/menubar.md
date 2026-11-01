# Menubar

- Copy into your app: `atoll add menubar` or `dx components add menubar --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::menubar::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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

## Submenus

Put a `MenubarSub` where an item would go. Its `index` is the trigger's place among the menu's items; the items inside `MenubarSubContent` count from 0 again. Submenus can nest.

```rust
MenubarSub { index: 2usize,
    MenubarSubTrigger { "Open recent" }
    MenubarSubContent {
        MenubarItem { index: 0usize, value: "a".to_string(), "notes.md" }
    }
}
```

ArrowRight, Enter or Space on the trigger opens the submenu and focuses its first item; ArrowLeft or Escape closes only the submenu and puts focus back on the trigger; choosing an item closes the whole menu. With a mouse the submenu opens after the pointer rests on its trigger for 100 ms, and stays open for 300 ms after the pointer leaves, so it can cross other items on the way. The submenu opens to the inline end and does not flip at screen edges, and the arrow keys are not mirrored in right-to-left text yet.

## Stability

The `MenubarSub` parts are experimental (new on 2026-11-01, docs/STABILITY.md); the rest of the component is stable.

## Keyboard and accessibility

Follows the WAI-ARIA [Menu Bar](https://www.w3.org/WAI/ARIA/apg/patterns/menubar/) pattern. Roles: `menubar`, `menu`, `menuitem`. Works on native (Blitz): partly.

| Key | When | What it does | Native |
|---|---|---|---|
| ArrowRight | On a menu bar item | Moves focus to the next menu bar item. | yes |
| ArrowLeft | On a menu bar item | Moves focus to the previous menu bar item. | yes |
| ArrowDown | On a menu bar item | Opens its menu and moves focus to the first item. | yes |
| Enter | On a menu bar item | Opens or closes its menu. | yes |
| Escape | In an open menu | Closes the menu. | yes |
| Escape | After the menu closes | Focus goes back to the menubar item that opened it. | not yet |
| ArrowRight | On an item with a submenu | Opens the submenu and moves focus to its first item. | yes |
| ArrowLeft | In a submenu | Closes the submenu and returns focus to the item that opened it. | yes |
| Escape | In a submenu | Closes only the submenu; focus goes back to the item that opened it. | yes |

States:

- `file-open`: The first menu is open.
- `file-closed`: The first menu is closed.

Known gaps:

- Submenus open to the inline end and do not flip at screen edges; in right-to-left text ArrowRight still opens and ArrowLeft still closes.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

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

/// A nested menu inside [`MenubarContent`] (or another submenu). Its trigger is an item of the
/// surrounding menu at `index`; the content holds ordinary [`MenubarItem`]s.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn MenubarSub(props: MenubarSubProps) -> Element

/// The item that opens a [`MenubarSub`], with a chevron after its label.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn MenubarSubTrigger(props: MenubarSubTriggerProps) -> Element

/// The menu of a [`MenubarSub`]: the menubar's own look, beside its trigger.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn MenubarSubContent(props: MenubarSubContentProps) -> Element
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
                        // A submenu: its trigger is item 2 of this menu, its items count from 0.
                        MenubarSub { index: 2usize,
                            MenubarSubTrigger { "Open recent" }
                            MenubarSubContent {
                                MenubarItem {
                                    index: 0usize,
                                    value: "recent:notes.md".to_string(),
                                    on_select: move |value| {
                                        tracing::info!("Selected value: {}", value);
                                    },
                                    "notes.md"
                                }
                                MenubarItem {
                                    index: 1usize,
                                    value: "recent:budget.csv".to_string(),
                                    on_select: move |value| {
                                        tracing::info!("Selected value: {}", value);
                                    },
                                    "budget.csv"
                                }
                            }
                        }
                        MenubarItem {
                            index: 3usize,
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
