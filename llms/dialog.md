# Dialog

- Copy into your app: `rust-ui add dialog` or `dx components add dialog --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::dialog::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The dialog component can be used to display additional information or actions related to an item. It is a simple modal dialog that can be opened and closed by the user.

## Component Structure

```rust
// The dialog component must wrap all dialog elements.
Dialog {
    // The open prop determines if the dialog is currently open or closed.
    open: open(),
    // The dialog title defines the heading of the dialog.
    DialogTitle {
        "Item information"
    }
    // The dialog description provides additional information about the dialog.
    DialogDescription {
        "Here is some additional information about the item."
    }
}
```

## API

```rust
#[component]
pub fn Dialog(props: DialogRootProps) -> Element

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Open dialog" }
        Dialog { open: open(), on_open_change: move |v| open.set(v),
            DialogTitle { "Are you sure?" }
            DialogDescription { "This action cannot be undone." }
            div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; justify-content: flex-end; margin-top: 1rem;",
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Cancel" }
                Button { variant: ButtonVariant::Destructive, onclick: move |_| open.set(false), "Delete" }
            }
        }
    }
}
```
