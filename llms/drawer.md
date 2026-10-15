# Drawer

- Copy into your app: `rust-ui add drawer` or `dx components add drawer --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::drawer::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A panel that slides up from the bottom and can be dragged down to dismiss.

```rust
Drawer { open: open(), on_open_change: move |v| open.set(v),
    DrawerTitle { "Move goal" }
    DrawerDescription { "Set your daily goal." }
    DrawerClose { "Close" }
}
```

It is built on the dialog primitive plus the motion layer's vertical `SwipeDismiss` (`vertical` and `one_way`): pull down far enough, or flick, and it springs away; release early and it springs back.

## Accessibility

- Role and name: a modal `dialog` named by `DrawerTitle` (always include one) and described by `DrawerDescription`; the handle is decorative (`aria-hidden`).
- Keyboard: focus is trapped inside; Escape closes; the Close button always works, so dragging is never required.
- Reduced motion: the slide-in is removed and the drag release becomes a near-instant tween.

## Source

Original to this repo, on `dioxus-primitives` dialog and `crates/motion`. Rust/UI's `drawer.rs` was only used to see the feature set.

## API

```rust
/// A panel that slides up from the bottom of the screen and can be dragged down to dismiss.
///
/// It is a modal dialog underneath (focus is trapped, Escape closes, the page behind is inert),
/// so it needs a `DrawerTitle`. Dragging is an extra: the Close button and Escape always work.
#[component]
pub fn Drawer(props: DialogRootProps) -> Element

#[component]
pub fn DrawerTitle(props: DialogTitleProps) -> Element

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
        Drawer { open: open(), on_open_change: move |v| open.set(v),
            DrawerTitle { "Move goal" }
            DrawerDescription { "Set your daily activity goal. Drag the handle down, or press Escape, to close." }
            p { style: "margin: 0.5rem 0;", "Pick a number that feels realistic for this week." }
            DrawerClose { "Close" }
        }
    }
}
```
