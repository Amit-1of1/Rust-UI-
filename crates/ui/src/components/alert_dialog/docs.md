The AlertDialog primitive provides an accessible, composable modal dialog for critical user confirmations (such as destructive actions).

## Component Structure

```rust
let mut open = use_signal(|| false);
rsx! {
    button { onclick: move |_| open.set(true), type: "button", "Show Alert Dialog" }
    AlertDialog {
        open: open(),
        on_open_change: move |v| open.set(v),
        // Required: leaving it out is a compile error.
        title: "Delete item?",
        description: "This can't be undone.",
        AlertDialogActions {
            AlertDialogCancel { "Cancel" }
            AlertDialogAction { on_click: move |_| { /* destructive action */ }, "Confirm" }
        }
    }
}
```

### Components
- **AlertDialog**: Provides context, manages open state, renders the backdrop and the content panel. `title` (required) is the heading and the dialog's accessible name; `description` is the message read after it.
- **AlertDialogDescription**: A longer message with formatting or links, as a child, instead of `description`.
- **AlertDialogActions**: Container for action buttons.
- **AlertDialogAction**: Main action button (e.g., confirm/delete). Closes the dialog, *then* calls the optional `on_click`. So if `on_open_change(false)` clears state (such as "which item to delete"), `on_click` sees it already cleared: keep that state separate from the open flag.
- **AlertDialogCancel**: Cancel/close button. Closes dialog and calls optional `on_click`.

## Accessibility

- `title` is a required prop, so an alert dialog without a name doesn't compile (the error names the missing field: "Missing required field title"). It can't be hidden.
