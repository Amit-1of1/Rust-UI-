# Sheet

- Copy into your app: `rust-ui add sheet` or `dx components add sheet --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`, `ui_strings`).
- Import: `use crate::components::sheet::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The sheet component is a panel that slides in from the edge of the screen. It can be used to display additional content, forms, or navigation menus without leaving the current page.

## Component Structure

```rust
Sheet {
    open: open(),
    // Which edge to slide in from. Available sides: Top, Right (default), Bottom, Left.
    "data-side": SheetSide::Right.as_str(),
    SheetContentClose {}
    SheetHeader {
        SheetTitle { "Edit Profile" }
        SheetDescription { "Make changes to your profile here." }
    }
    SheetFooter {
        SheetClose { "Close" }
    }
}
```

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

#[component]
pub fn Sheet(props: DialogRootProps) -> Element

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

#[component]
pub fn SheetTitle(props: DialogTitleProps) -> Element

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
        Sheet { open: open(), on_open_change: move |v| open.set(v), "data-side": side().as_str(),
            SheetHeader {
                SheetTitle { "Edit profile" }
                SheetDescription { "Make changes to your profile here." }
            }
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
