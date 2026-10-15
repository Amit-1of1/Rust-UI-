# Progress

- Copy into your app: `rust-ui add progress` or `dx components add progress --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::progress::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The Progress component is used to display the progress of a task or operation. It can be used to indicate loading states, file uploads, or any other process that takes time to complete.

## Component Structure

```rust
Progress {
    // The current progress value (0 to max
    value: 0.5,
    // The maximum value of the progress (default is 100.0)
    max: 1.0,
    // Elements that will be displayed inside the progress bar
    {children}
}
```

## API

```rust
#[component]
pub fn Progress(props: ProgressProps) -> Element
```

## Example

```rust
use crate::components::button::*;
use crate::components::progress::*;
use dioxus::prelude::*;

/// Driven by buttons rather than a JavaScript timer, so it also works on the native renderer.
#[component]
pub fn Demo() -> Element {
    let mut progress = use_signal(|| 30_u32);

    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 20rem;",
            Progress { aria_label: "Upload progress", value: progress() as f64 }
            div { style: "display: flex; gap: 0.5rem;",
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| progress.set((progress() + 10).min(100)),
                    "Add 10%"
                }
                Button { variant: ButtonVariant::Ghost, onclick: move |_| progress.set(0), "Reset" }
            }
        }
    }
}
```
