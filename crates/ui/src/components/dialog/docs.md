The dialog component can be used to display additional information or actions related to an item. It is a simple modal dialog that can be opened and closed by the user.

## Component Structure

```rust
// The dialog component must wrap all dialog elements.
Dialog {
    // The open prop determines if the dialog is currently open or closed.
    open: open(),
    // Required: the heading and the dialog's accessible name. Leaving it out is a compile error.
    title: "Item information",
    // Optional: one line read after the title.
    description: "Here is some additional information about the item.",
    // The content follows the title and description.
    p { "..." }
}
```

## Accessibility

- `title` is a required prop, so a dialog without a name doesn't compile (the error names the missing field: "Missing required field title"). Radix and shadcn only warn about this at runtime.
- `hide_title: true` hides the title and `description` visually but keeps them for screen readers, for dialogs whose content already makes the purpose clear (the command palette uses it).
- For a description with links or formatting, put a `DialogDescription` in the children instead of `description` (not both).
