# Progress

- Copy into your app: `atoll add progress` or `dx components add progress --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`).
- Import: `use crate::components::progress::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The Progress component is used to display the progress of a task or operation. It can be used to indicate loading states, file uploads, or any other process that takes time to complete.

## Component Structure

```rust
Progress {
    // The current progress value (0 to max). `None` is indeterminate.
    value: 0.5,
    // The maximum value of the progress (default is 100.0)
    max: 1.0,
}
```

## Label, value and size

`label` puts a visible label above the bar and names it for screen readers; without one, give the bar an `aria-label`. `show_value` adds the percentage. `size` is `Sm`, `Default` or `Lg`.

```rust
Progress { label: "Uploading photos", show_value: true, value: uploaded() }
```

When the length of the wait is unknown, pass `value: None`: the bar moves until the work finishes or fails (and stops moving when the system asks for reduced motion). When you know how much is left, pass the value so the bar and the percentage show it.

## API

```rust
/// A [`Progress`] bar's thickness.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum ProgressSize {
    Sm,
    #[default]
    Default,
    Lg,
}

/// The props for [`Progress`].
pub struct ProgressProps {
    /// The current value, between 0 and `max`. `None` is indeterminate: the bar moves until
    /// the work finishes or fails, for waits whose length is unknown.
    pub value: ReadSignal<Option<f64>>,

    /// The maximum value. Defaults to 100.
    #[props(default = ReadSignal::new(Signal::new(100.0)))]
    pub max: ReadSignal<f64>,

    /// A visible label above the bar ("Uploading photos"), which also names it. Without one,
    /// give the bar an `aria-label`.
    #[props(default, into)]
    pub label: Option<String>,

    /// Shows the percentage next to the label (or above the bar).
    #[props(default)]
    pub show_value: bool,

    #[props(default)]
    pub size: ProgressSize,

    /// Additional attributes for the bar (`role="progressbar"`), such as `aria-label`.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Unused; kept so code that passes children still builds.
    #[props(default)]
    pub children: Element,
}

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
            Progress { label: "Uploading photos", show_value: true, value: progress() as f64 }
            Progress { aria_label: "Syncing", size: ProgressSize::Sm, value: None }
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
