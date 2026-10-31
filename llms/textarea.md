# Textarea

- Copy into your app: `atoll add textarea` or `dx components add textarea --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::textarea::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The textarea element is used to allow users to enter multi-line text input in a user interface.

## Component Structure

```rust
textarea {
    // Global html attributes
    class: "dx-textarea",
    "data-style": "default",
    // Children
    {children}
}
```

## API

```rust
pub enum TextareaVariant {
    #[default]
    Default,
    Fade,
    Outline,
    Ghost,
}

#[component]
pub fn Textarea(
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
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(default)] variant: TextareaVariant,
    /// Grow with the text instead of scrolling, up to `--textarea-max-height` (CSS
    /// `field-sizing: content`, no JavaScript). Browsers without it keep a fixed height.
    #[props(default)]
    autosize: bool,
    #[props(extends=GlobalAttributes)]
    #[props(extends=textarea)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::label::Label;
use crate::components::textarea::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut description = use_signal(|| "sample text".to_string());
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1.5rem",

            p { id: "textarea-message", "Description here: {description}" }

            div {
                display: "flex",
                flex_direction: "column",
                gap: ".5rem",
                justify_content: "center",

                Label { html_for: "default", "Default" }
                Textarea {
                    id: "default",
                    variant: TextareaVariant::Default,
                    placeholder: "Enter your description",
                    value: description,
                    oninput: move |e: FormEvent| description.set(e.value()),
                }
            }

            div {
                display: "flex",
                flex_direction: "column",
                gap: ".5rem",

                Label { html_for: "autosize", "Grows as you type" }
                Textarea {
                    id: "autosize",
                    autosize: true,
                    placeholder: "Write a few lines",
                }
            }
        }
    }
}
```
