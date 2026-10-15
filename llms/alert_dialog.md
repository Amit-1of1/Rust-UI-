# Alert dialog

- Copy into your app: `rust-ui add alert_dialog` or `dx components add alert_dialog --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::alert_dialog::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The AlertDialog primitive provides an accessible, composable modal dialog for critical user confirmations (such as destructive actions).

## Component Structure

```rust
let mut open = use_signal(|| false);
rsx! {
    button { onclick: move |_| open.set(true), type: "button", "Show Alert Dialog" }
    AlertDialog { open: open(), on_open_change: move |v| open.set(v),
        AlertDialogTitle { "Title" }
        AlertDialogDescription { "Description" }
        AlertDialogActions {
            AlertDialogCancel { "Cancel" }
            AlertDialogAction { on_click: move |_| { /* destructive action */ }, "Confirm" }
        }
    }
}
```

### Components
- **AlertDialog**: Provides context, manages open state, renders the backdrop and the content panel.
- **AlertDialogTitle**: The dialog's heading.
- **AlertDialogDescription**: Additional description for the dialog.
- **AlertDialogActions**: Container for action buttons.
- **AlertDialogAction**: Main action button (e.g., confirm/delete). Closes the dialog, *then* calls the optional `on_click`. So if `on_open_change(false)` clears state (such as "which item to delete"), `on_click` sees it already cleared: keep that state separate from the open flag.
- **AlertDialogCancel**: Cancel/close button. Closes dialog and calls optional `on_click`.

## API

```rust
#[component]
pub fn AlertDialog(props: AlertDialogRootProps) -> Element

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element

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
        AlertDialog { open: open(), on_open_change: move |v| open.set(v),
            AlertDialogTitle { "Delete item" }
            AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
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
