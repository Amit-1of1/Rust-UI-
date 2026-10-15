# Label

- Copy into your app: `rust-ui add label` or `dx components add label --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::label::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Label component is used to provide a label for form elements, enhancing accessibility and usability. It is typically used in conjunction with form controls like inputs, selects, and textareas.

## Component Structure

```rust
Label {
    html_for: "id", // The ID of the form control this label is associated with
}
button {
    id: "id", // The ID of the labeled element
}
```

## API

```rust
#[component]
pub fn Label(props: LabelProps) -> Element
```

## Example

```rust
use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 20rem;",
            Label { html_for: "label-demo-name", "Your name" }
            Input { id: "label-demo-name", placeholder: "Ada Lovelace" }
        }
    }
}
```
