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
