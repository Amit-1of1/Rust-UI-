# Input

- Copy into your app: `atoll add input` or `dx components add input --git https://github.com/Amit-1of1/Rust-UI-` (also copies `field`).
- Import: `use crate::components::input::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The input element is used to accept a text input from the user. It can be styled and configured to fit various use cases, such as forms or search bars.

## Component Structure

```rust
input {
    // Global html attributes
    class: "dx-input",
    // Children
    {children}
}
```

## Sizes

`field_size: FieldSize::Sm | Default | Lg` changes height, padding and text size (`size` is already the HTML attribute).

## API

```rust
#[component]
pub fn Input(
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    oninvalid: Option<EventHandler<FormEvent>>,
    onselect: Option<EventHandler<SelectionEvent>>,
    onselectionchange: Option<EventHandler<SelectionEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    onwheel: Option<EventHandler<WheelEvent>>,
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    /// `Sm`, `Default` or `Lg` (`size` is taken by the HTML attribute).
    #[props(default)]
    field_size: crate::components::field::FieldSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::input::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut name = use_signal(String::new);
    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 20rem;",
            Label { html_for: "gallery-name", "Name" }
            Input {
                id: "gallery-name",
                placeholder: "Enter your name",
                value: name,
                oninput: move |e: FormEvent| name.set(e.value()),
            }
            if !name.read().is_empty() {
                p { style: "margin: 0; font-size: 0.875rem;", "Hello, {name}!" }
            }
            Label { html_for: "gallery-search-sm", "Small" }
            Input { id: "gallery-search-sm", field_size: crate::components::field::FieldSize::Sm, placeholder: "Filter rows" }
            Label { html_for: "gallery-email-lg", "Large" }
            Input { id: "gallery-email-lg", r#type: "email", field_size: crate::components::field::FieldSize::Lg, placeholder: "you@example.com" }
        }
    }
}
```
