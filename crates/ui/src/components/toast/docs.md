The Toast component is used to display brief messages to the user, typically for notifications or alerts. The toast messages can be focused with the keyboard with the `f6` key.

## Short form

For the common case use `ToastsExt` (in the prelude): it takes `&str`, `String` or a `(title, description)` pair, so no `.to_string()` or `ToastOptions::new()`:

```rust
let toasts = use_toast();
toasts.notify_success("Saved");
toasts.notify_error(("Failed", "Try again."));
```

`notify_warning` and `notify_info` work the same. Use the long form below for a duration or a permanent toast.

## Component Structure

```rust
// The Toast provider provides the toast context to its children and handler rendering any toasts that are sent.
ToastProvider {
    // Any child component can consume the toast context and send a toast to be rendered.
    button {
        onclick: |event: MouseEvent| {
            // Consume the toast context to send a toast.
            let toast_api = consume_toast();
            toast_api
                .error(
                    "Critical Error".to_string(),
                    ToastOptions::new()
                        .description("Some info you need")
                        .duration(Duration::from_secs(60))
                        .permanent(false),
                );
        },
        "Show Toast"
    }
}
```

## Icons, actions and pausing

Each kind of toast (success, error, warning, info) shows its own icon next to the title. A toast stops its countdown while the pointer is over it or focus is inside it, and starts again when they leave (`data-paused` is set meanwhile).

`notify_action` adds one button, such as Undo. Pressing it runs your callback and closes the toast:

```rust
let toasts = consume_toast();
toasts.notify_action(ToastType::Success, "Message archived", "Undo", move |_| restore());
```
