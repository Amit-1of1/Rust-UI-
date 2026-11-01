# Dialog

- Copy into your app: `atoll add dialog` or `dx components add dialog --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `icons`, `ui_strings`).
- Import: `use crate::components::dialog::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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

## Sizes, footer and close button

- `size: DialogSize::Sm | Default | Lg | Xl | Full` sets the panel's width; `Full` fills the screen.
- `DialogFooter` lines up the action buttons at the end (stacked on phones).
- `DialogClose` is a button that closes the dialog (give it a `class`, or render your own element with `as`); `DialogContentClose` is the corner X, named "Close" in the current locale.

## Keyboard and accessibility

Follows the WAI-ARIA [Dialog (Modal)](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/) pattern. Roles: `dialog`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | On the button that opens it | Opens the dialog and moves focus into it. | yes |
| Space | On the button that opens it | Opens the dialog and moves focus into it. | yes |
| Tab | Inside the dialog | Moves to the next control; from the last one it wraps to the first, so focus stays in the dialog. | yes |
| Shift + Tab | Inside the dialog | Moves to the previous control, wrapping from the first to the last. | yes |
| Escape | Inside the dialog | Closes the dialog and returns focus to the button that opened it. | yes |

States:

- `open`: The dialog is shown over the page.
- `closed`: Only the trigger is shown.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

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

    /// How wide the dialog may grow: `Sm` for a confirmation, `Lg`/`Xl` for forms and tables,
    /// `Full` for most of the screen. On phones every size fills the width.
    #[props(default)]
    pub size: DialogSize,

    /// Adds a close button (an X, named "Close") in the top corner.
    #[props(default)]
    pub show_close: bool,

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

/// How wide a [`Dialog`] may grow.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum DialogSize {
    /// 24rem: a short confirmation.
    Sm,
    /// 32rem.
    #[default]
    Default,
    /// 42rem: a form.
    Lg,
    /// 56rem: a table or a two-column form.
    Xl,
    /// Nearly the whole screen.
    Full,
}

/// The row of buttons at the bottom of a [`Dialog`] ("Cancel", "Save"), aligned to the end and
/// stacked on narrow screens.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DialogFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// A button that closes the [`Dialog`] it is in ("Cancel", "Done"). Style it by passing a
/// `class`, or render your own element with `as`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DialogClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element

/// The X in a [`Dialog`]'s corner, named "Close" (translatable). `show_close: true` adds it.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DialogContentClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut wide = use_signal(|| false);
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 0.75rem;",
            Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(true), "Open dialog" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| wide.set(true), "Open a large dialog" }
        }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Are you sure?",
            description: "This action cannot be undone.",
            size: DialogSize::Sm,
            DialogFooter {
                Button { variant: ButtonVariant::Outline, onclick: move |_| open.set(false), "Cancel" }
                Button { variant: ButtonVariant::Destructive, onclick: move |_| open.set(false), "Delete" }
            }
        }
        Dialog {
            open: wide(),
            on_open_change: move |v| wide.set(v),
            title: "Invite people",
            description: "They get an email with a link to join the workspace.",
            size: DialogSize::Lg,
            show_close: true,
            p { style: "margin: 0;", "Paste one email address per line, or import a CSV file." }
            DialogFooter {
                Button { onclick: move |_| wide.set(false), "Send invites" }
            }
        }
    }
}
```
