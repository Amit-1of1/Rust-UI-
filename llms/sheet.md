# Sheet

- Copy into your app: `rust-ui add sheet` or `dx components add sheet --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`, `icons`, `ui_strings`).
- Import: `use crate::components::sheet::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The sheet component is a panel that slides in from the edge of the screen. It can be used to display additional content, forms, or navigation menus without leaving the current page.

## Component Structure

```rust
Sheet {
    open: open(),
    // Which edge to slide in from. Available sides: Top, Right (default), Bottom, Left.
    "data-side": SheetSide::Right.as_str(),
    // Required: shown in the sheet's header and used as its accessible name.
    // Leaving it out is a compile error.
    title: "Edit Profile",
    description: "Make changes to your profile here.",
    SheetContentClose {}
    SheetFooter {
        SheetClose { "Close" }
    }
}
```

The sheet renders its own header (`title`, then `description`) above the children. `hide_title: true` hides that header visually but keeps it for screen readers, as the mobile sidebar does. For a description with links or formatting, put a `SheetDescription` in the children instead of `description`.

## SheetClose with `as` prop

The `as` prop allows you to render a custom element while preserving the close behavior, similar to shadcn/ui's `asChild` pattern.

```rust
// Default: renders as <button>
SheetClose { "Close" }

// Custom element: attributes include the preset onclick handler
SheetClose {
    as: |attributes| rsx! {
        a { href: "#", ..attributes, "Go back" }
    }
}
```

## API

```rust
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

/// The props for [`Sheet`].
///
/// `title` is required: a sheet is a dialog, and one without a name is announced as just
/// "dialog". Hide it with `hide_title` when the content makes the purpose clear to sighted users.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Sheet { open: true, title: "Rename file", p { "…" } } }
/// # }
/// ```
///
/// Leaving `title` out is a compile error ("Missing required field title"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Sheet { open: true, p { "…" } } }
/// # }
/// ```
pub struct SheetProps {
    /// The sheet's heading, and its accessible name (`aria-labelledby`). Shown in the header.
    #[props(into)]
    pub title: String,

    /// One line read after the title (`aria-describedby`), shown under it. For richer text, put
    /// a [`SheetDescription`] in the children instead (not both).
    #[props(default, into)]
    pub description: Option<String>,

    /// Hide the header (title and `description`) visually; screen readers still read it.
    #[props(default)]
    pub hide_title: bool,

    /// The ID of the dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the sheet is modal. If true, it will trap focus within the sheet when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the sheet.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the sheet if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// Called when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Attributes for the sheet panel (`"data-side"` picks the edge).
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The sheet's content, shown below the header.
    pub children: Element,
}

#[component]
pub fn Sheet(props: SheetProps) -> Element

#[component]
pub fn SheetContentClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element

#[component]
pub fn SheetHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn SheetFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// A longer description for a [`Sheet`] (links, formatting). For one line of text, use the
/// sheet's `description` prop instead.
#[component]
pub fn SheetDescription(props: DialogDescriptionProps) -> Element

#[component]
pub fn SheetClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::sheet::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut side = use_signal(|| SheetSide::Right);
    let mut show = move |s: SheetSide| {
        side.set(s);
        open.set(true);
    };
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem;",
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Left), "Left" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Right), "Right" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Top), "Top" }
            Button { variant: ButtonVariant::Outline, onclick: move |_| show(SheetSide::Bottom), "Bottom" }
        }
        Sheet {
            open: open(),
            on_open_change: move |v| open.set(v),
            "data-side": side().as_str(),
            title: "Edit profile",
            description: "Make changes to your profile here.",
            div { style: "display: grid; gap: 0.5rem; max-width: 20rem; padding: 0 1rem;",
                Label { html_for: "sheet-name", "Name" }
                Input { id: "sheet-name", initial_value: "Ferris" }
            }
            SheetFooter {
                Button { onclick: move |_| open.set(false), "Save changes" }
            }
            SheetContentClose {}
        }
    }
}
```
