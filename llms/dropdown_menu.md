# Dropdown menu

- Copy into your app: `atoll add dropdown_menu` or `dx components add dropdown_menu --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::dropdown_menu::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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
## Submenus

Put a `DropdownMenuSub` where an item would go. Its `index` is the trigger's place among the menu's items; the items inside `DropdownMenuSubContent` count from 0 again. Submenus can nest.

```rust
DropdownMenuSub { index: 2usize,
    DropdownMenuSubTrigger { "Move to" }
    DropdownMenuSubContent {
        DropdownMenuItem { index: 0usize, value: "a", "Inbox" }
    }
}
```

ArrowRight, Enter or Space on the trigger opens the submenu and focuses its first item; ArrowLeft or Escape closes only the submenu and puts focus back on the trigger; choosing an item closes the whole menu. With a mouse the submenu opens after the pointer rests on its trigger for 100 ms, and stays open for 300 ms after the pointer leaves, so it can cross other items on the way. The submenu opens to the inline end and does not flip at screen edges, and the arrow keys are not mirrored in right-to-left text yet.

## Stability

The `DropdownMenuSub` parts are experimental (new on 2026-11-01, docs/STABILITY.md); the rest of the component is stable.

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
| ArrowRight | On an item with a submenu | Opens the submenu and moves focus to its first item. | yes |
| ArrowDown | In a submenu | Moves focus to the next item of the submenu; the menu behind it stays put. | yes |
| ArrowLeft | In a submenu | Closes the submenu and returns focus to the item that opened it. | yes |
| Escape | In a submenu | Closes only the submenu; focus goes back to the item that opened it. | yes |
| Enter | On a submenu item | Chooses the item, closes the whole menu and returns focus to the trigger. | not yet |

States:

- `open`: The list of actions is shown.
- `closed`: Only the trigger is shown.

Known gaps:

- The items are a `listbox` of `option`s, not a `menu` of `menuitem`s (upstream markup).
- Opening does not move focus to the first item; the first arrow key does.
- A submenu's item is an `option`, which cannot carry `aria-haspopup` or `aria-expanded`; it names its open list with `aria-controls` instead, and that list is a `group` of options labelled by the item (a listbox cannot hold a listbox).
- Submenus open to the inline end and do not flip at screen edges; in right-to-left text ArrowRight still opens and ArrowLeft still closes.

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

/// A nested menu inside [`DropdownMenuContent`] (or another submenu). Its trigger is an item of
/// the surrounding menu at `index`; the content holds ordinary [`DropdownMenuItem`]s.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DropdownMenuSub(props: DropdownMenuSubProps) -> Element

/// The item that opens a [`DropdownMenuSub`], with a chevron after its label.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DropdownMenuSubTrigger(props: DropdownMenuSubTriggerProps) -> Element

/// The list of a [`DropdownMenuSub`]: the menu's own look, beside its trigger.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DropdownMenuSubContent(props: DropdownMenuSubContentProps) -> Element
```

## Example

```rust
use crate::components::dropdown_menu::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut picked = use_signal(|| None::<&'static str>);
    let pick = move |v| picked.set(Some(v));
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            DropdownMenu { default_open: false,
                DropdownMenuTrigger { "Actions" }
                DropdownMenuContent {
                    DropdownMenuItem::<&'static str> { value: "Edit", index: 0usize, on_select: pick, "Edit" }
                    DropdownMenuItem::<&'static str> { value: "Duplicate", index: 1usize, on_select: pick, "Duplicate" }
                    // A submenu: its trigger is item 2 of this menu, its items count from 0.
                    DropdownMenuSub { index: 2usize,
                        DropdownMenuSubTrigger { "Move to" }
                        DropdownMenuSubContent {
                            DropdownMenuItem::<&'static str> { value: "Inbox", index: 0usize, on_select: pick, "Inbox" }
                            DropdownMenuItem::<&'static str> { value: "Later", index: 1usize, on_select: pick, "Later" }
                            // Submenus nest.
                            DropdownMenuSub { index: 2usize,
                                DropdownMenuSubTrigger { "Project" }
                                DropdownMenuSubContent {
                                    DropdownMenuItem::<&'static str> { value: "Atoll", index: 0usize, on_select: pick, "Atoll" }
                                    DropdownMenuItem::<&'static str> { value: "Ledgerly", index: 1usize, on_select: pick, "Ledgerly" }
                                }
                            }
                        }
                    }
                    DropdownMenuItem::<&'static str> { value: "Archive", index: 3usize, on_select: pick, "Archive" }
                    DropdownMenuItem::<&'static str> { value: "Delete", index: 4usize, on_select: pick, "Delete" }
                }
            }
            if let Some(a) = picked() {
                span { style: "font-size: 0.875rem;", "Selected: {a}" }
            }
        }
    }
}
```
