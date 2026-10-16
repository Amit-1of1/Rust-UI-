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

## API

```rust
/// The props for [`Dialog`].
///
/// `title` is required: a dialog without a name is announced as just "dialog". Hide it with
/// `hide_title` when the content already makes the purpose clear to sighted users.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Dialog { open: true, title: "Rename file", p { "…" } } }
/// # }
/// ```
///
/// Leaving `title` out is a compile error ("Missing required field title"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Dialog { open: true, p { "…" } } }
/// # }
/// ```
pub struct DialogProps {
    /// The dialog's heading, and its accessible name (`aria-labelledby`).
    #[props(into)]
    pub title: String,

    /// One line read after the title (`aria-describedby`). For richer text, put a
    /// [`DialogDescription`] in the children instead (not both).
    #[props(default, into)]
    pub description: Option<String>,

    /// Hide the title and `description` visually; screen readers still read them.
    #[props(default)]
    pub hide_title: bool,

    /// The ID of the dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the dialog is modal. If true, it will trap focus within the dialog when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the dialog.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the dialog if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// Called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Attributes for the dialog panel.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The dialog's content, shown below the title.
    pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element

/// A longer description for a [`Dialog`] (links, formatting). For one line of text, use the
/// dialog's `description` prop instead.
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
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Are you sure?",
            description: "This action cannot be undone.",
            div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; justify-content: flex-end; margin-top: 1rem;",
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Cancel" }
                Button { variant: ButtonVariant::Destructive, onclick: move |_| open.set(false), "Delete" }
            }
        }
    }
}
```
