# Toast

- Copy into your app: `atoll add toast` or `dx components add toast --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`).
- Import: `use crate::components::toast::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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

## API

```rust
/// A button in a toast that answers it, such as "Undo" after a delete.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct ToastAction {
    /// The button's text ("Undo").
    pub label: String,
    /// Runs when the button is pressed; the toast then closes.
    pub on_action: Callback<()>,
}

#[component]
pub fn ToastProvider(
    #[props(default = ReadSignal::new(Signal::new(Some(Duration::from_secs(5)))))]
    default_duration: ReadSignal<Option<Duration>>,
    #[props(default = ReadSignal::new(Signal::new(10)))] max_toasts: ReadSignal<usize>,
    #[props(default)] render_toast: Option<Callback<toast::ToastPropsWithOwner, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// A toast's title and optional description. Built from `"title"`, a `String`, or a
/// `(title, description)` pair.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct ToastMessage {
    pub title: String,
    pub description: Option<String>,
}
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::toast::{ToastProvider, ToastsExt};
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastType};

/// Self-contained: the demo brings its own provider, so it also works on a docs page.
#[component]
pub fn Demo() -> Element {
    rsx! {
        ToastProvider { Buttons {} }
    }
}

#[component]
fn Buttons() -> Element {
    let toasts = use_toast();
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.notify_success(("Saved", "Your changes are stored.")),
                "Success toast"
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.notify_error(("Failed", "Something went wrong.")),
                "Error toast"
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| toasts.notify_info("Copied"),
                "Info toast"
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    toasts.notify_action(ToastType::Info, ("Invoice deleted", "INV-1042 moved to the bin."), "Undo",
                        move |_| toasts.notify_success("Invoice restored"))
                },
                "Toast with an action"
            }
        }
    }
}
```
