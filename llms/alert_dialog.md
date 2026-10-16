# Alert dialog

- Copy into your app: `rust-ui add alert_dialog` or `dx components add alert_dialog --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::alert_dialog::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

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

## Keyboard and accessibility

Follows the WAI-ARIA [Alert and Message Dialogs](https://www.w3.org/WAI/ARIA/apg/patterns/alertdialog/) pattern. Roles: `alertdialog`. Works on native (Blitz): partly.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | On the button that opens it | Opens the alert dialog and moves focus into it. | yes |
| Tab | Inside the alert dialog | Moves between its buttons; focus stays inside. | yes |
| Escape | Inside the alert dialog | Closes it as Cancel would and returns focus to the button that opened it. | not yet |

States:

- `open`: The alert dialog is shown over the page.
- `closed`: Only the trigger is shown.

Native: Closing works, but focus is left on the removed content instead of going back to the trigger.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// The props for [`AlertDialog`].
///
/// `title` is required: it names the alert dialog for screen readers. Unlike a
/// plain `Dialog` it can't be hidden, since an interruption
/// should say what it is about.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { AlertDialog { open: true, title: "Rename file", p { "…" } } }
/// # }
/// ```
///
/// Leaving `title` out is a compile error ("Missing required field title"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { AlertDialog { open: true, p { "…" } } }
/// # }
/// ```
pub struct AlertDialogProps {
    /// The heading, and the dialog's accessible name (`aria-labelledby`).
    #[props(into)]
    pub title: String,

    /// The message read after the title (`aria-describedby`), such as what will be lost. For
    /// richer text, put an [`AlertDialogDescription`] in the children instead (not both).
    #[props(default, into)]
    pub description: Option<String>,

    /// The id of the alert dialog root element. If not provided, a unique id will be generated.
    pub id: ReadSignal<Option<String>>,

    /// Whether the alert dialog should be open by default. Only used if `open` is not provided.
    #[props(default)]
    pub default_open: bool,

    /// The controlled open state of the alert dialog.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// Called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Attributes for the backdrop element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The content below the title: usually [`AlertDialogActions`].
    pub children: Element,
}

#[component]
pub fn AlertDialog(props: AlertDialogProps) -> Element

/// A longer message for an [`AlertDialog`] (links, formatting). For plain text, use the
/// alert dialog's `description` prop instead.
#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element
```

## Example

```rust
use crate::components::button::Button;

use crate::components::alert_dialog::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut confirmed = use_signal(|| false);

    rsx! {
        Button {
            r#type: "button",
            "data-style": "outline",
            style: "margin-bottom: 1.5rem;",
            onclick: move |_| open.set(true),
            "Show Alert Dialog"
        }
        AlertDialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Delete item",
            description: "Are you sure you want to delete this item? This action cannot be undone.",
            AlertDialogActions {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction { on_click: move |_| confirmed.set(true), "Delete" }
            }
        }
        if confirmed() {
            p { style: "color: var(--contrast-error-color); margin-top: 16px; font-weight: 600;",
                "Item deleted!"
            }
        }
    }
}
```
