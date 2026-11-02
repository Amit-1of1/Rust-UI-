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
