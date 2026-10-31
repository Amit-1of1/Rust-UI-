# Drawer

- Copy into your app: `rust-ui add drawer` or `dx components add drawer --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`).
- Import: `use crate::components::drawer::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A panel that slides up from the bottom and can be dragged down to dismiss.

```rust
Drawer {
    open: open(),
    on_open_change: move |v| open.set(v),
    title: "Move goal", // required
    description: "Set your daily goal.",
    DrawerClose { "Close" }
}
```

It is built on the dialog primitive plus the motion layer's vertical `SwipeDismiss` (`vertical` and `one_way`): pull down far enough, or flick, and it springs away; release early and it springs back.

## Accessibility

- Role and name: a modal `dialog` named by its `title` (a required prop: leaving it out is a compile error; `hide_title` hides it visually) and described by `description` or a `DrawerDescription` child; the handle is decorative (`aria-hidden`).
- Keyboard: focus is trapped inside; Escape closes; the Close button always works, so dragging is never required.
- Reduced motion: the slide-in is removed and the drag release becomes a near-instant tween.

## Source

Original to this repo, on `dioxus-primitives` dialog and `crates/motion`. Rust/UI's `drawer.rs` was only used to see the feature set.

## API

```rust
/// The props for [`Drawer`].
///
/// `title` is required: a drawer is a modal dialog, and one without a name is announced as just
/// "dialog". Hide it with `hide_title` when the content makes the purpose clear to sighted users.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Drawer { open: true, title: "Rename file", p { "…" } } }
/// # }
/// ```
///
/// Leaving `title` out is a compile error ("Missing required field title"):
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { Drawer { open: true, p { "…" } } }
/// # }
/// ```
pub struct DrawerProps {
    /// The drawer's heading, and its accessible name (`aria-labelledby`).
    #[props(into)]
    pub title: String,

    /// One line read after the title (`aria-describedby`), shown under it. For richer text, put
    /// a [`DrawerDescription`] in the children instead (not both).
    #[props(default, into)]
    pub description: Option<String>,

    /// Hide the title and `description` visually; screen readers still read them.
    #[props(default)]
    pub hide_title: bool,

    /// The ID of the dialog root element.
    pub id: ReadSignal<Option<String>>,

    /// Whether the drawer is modal. If true, it will trap focus within the drawer when open.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled `open` state of the drawer.
    pub open: ReadSignal<Option<bool>>,

    /// The default `open` state of the drawer if it is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// Called when the open state changes, including after a drag dismisses it.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Attributes for the drawer panel.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The drawer's content, shown below the title.
    pub children: Element,
}

/// A panel that slides up from the bottom of the screen and can be dragged down to dismiss.
///
/// It is a modal dialog underneath (focus is trapped, Escape closes, the page behind is inert),
/// named by its required `title`. Dragging is an extra: the Close button and Escape always work.
#[component]
pub fn Drawer(props: DrawerProps) -> Element

/// A longer description for a [`Drawer`] (links, formatting). For one line of text, use the
/// drawer's `description` prop instead.
#[component]
pub fn DrawerDescription(props: DialogDescriptionProps) -> Element

/// A button that closes the drawer.
#[component]
pub fn DrawerClose(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::button::Button;
use crate::components::drawer::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Open drawer" }
        Drawer {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Move goal",
            description: "Set your daily activity goal. Drag the handle down, or press Escape, to close.",
            p { style: "margin: 0.5rem 0;", "Pick a number that feels realistic for this week." }
            DrawerClose { "Close" }
        }
    }
}
```
