# Navbar

- Copy into your app: `atoll add navbar` or `dx components add navbar --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::navbar::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Navbar component can be used to display a navigation bar with collapsible sections.

## Component Structure

```rust
// The Navbar component wraps the entire menu bar and contains the individual menus in the order of their index.
Navbar {
    // The NavbarNav contains the individual menus that can be opened.
    NavbarNav {
        // The index of the menu, used to determine the order in which menus are displayed.
        index: 0,
        // The menubar trigger is the element that will display the menu when activated.
        NavbarTrigger {
            // The content of the trigger button
            {children}
        }
        // The menubar content contains all the items that will be displayed in the menu when it is opened.
        NavbarContent {
            // Each menubar item represents an individual items in the menu.
            NavbarItem {
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
pub fn Navbar(props: NavbarProps) -> Element

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element
```
