# Stepper

- Copy into your app: `rust-ui add stepper` or `dx components add stepper --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::stepper::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A multi-step progress indicator with Back/Next control via `use_stepper()`.

To move on only when something allows it (a step's form validates), own the step: pass `current: step` (a `Signal<usize>`) and set it yourself; the step panels can then live outside the stepper. Disable the items the user hasn't reached so they can't skip ahead:

```rust
let step = use_signal(|| 0usize);
rsx! {
    Stepper { total: 3, current: step,
        StepperItem { step: 0usize, title: "Trip" }
        StepperItem { step: 1usize, title: "Details", disabled: reached() < 1 }
        StepperItem { step: 2usize, title: "Payment", disabled: reached() < 2 }
    }
    match step() { /* the current step's form */ }
}
```

## Accessibility

- Role and name: An ordered list; the current step has `aria-current="step"`; completed steps add hidden "(completed)" text.
- Keyboard: each step is a button (Tab/Enter/Space); `disabled` steps are skipped
- Reduced motion: none

## Source

Structure and API adapted from Rust/UI (`stepper.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.

## API

```rust
/// Shared state of a [`Stepper`]. Get it with [`use_stepper`] inside the stepper to build
/// Back/Next buttons.
pub struct StepperContext {
    pub current: Signal<usize>,
    pub total: usize,
}

/// The stepper context. Call inside a [`Stepper`].
pub fn use_stepper() -> StepperContext

/// A multi-step progress indicator. Steps are an ordered list; the current one carries
/// `aria-current="step"`. Put [`StepperItem`]s inside, one per step.
#[component]
pub fn Stepper(
    /// Number of steps.
    total: usize,
    #[props(default = 0)] default_step: usize,
    /// Drive the step from outside (for example, move on only after a form validates). When
    /// given, `default_step` is ignored and the stepper reads and writes this signal.
    #[props(default)]
    current: Option<Signal<usize>>,
    #[props(default)] vertical: bool,
    /// Called with the new index whenever the step changes.
    #[props(default)]
    on_change: Callback<usize>,
    /// Rendered after the list, inside the stepper, so it can call [`use_stepper`]
    /// (for example Back/Next buttons).
    #[props(default)]
    footer: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// One step. Clicking its button jumps there unless `disabled`.
#[component]
pub fn StepperItem(
    /// Zero-based index of this step.
    step: usize,
    #[props(into)] title: String,
    #[props(into, default)] description: Option<String>,
    #[props(default)] disabled: bool,
) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::stepper::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 1.25rem; max-width: 36rem;",
            Stepper { total: 3, default_step: 1, "aria-label": "Checkout progress", footer: rsx! { StepperControls {} },
                StepperItem { step: 0, title: "Account", description: "Your details" }
                StepperItem { step: 1, title: "Shipping", description: "Where to send it" }
                StepperItem { step: 2, title: "Payment" }
            }
        }
    }
}

/// Back/Next buttons driven by the stepper context.
#[component]
fn StepperControls() -> Element {
    let ctx = use_stepper();
    rsx! {
        div { style: "display: flex; gap: 0.5rem;",
            Button {
                variant: ButtonVariant::Outline,
                disabled: !ctx.can_prev(),
                onclick: move |_| ctx.prev(),
                "Back"
            }
            Button { disabled: !ctx.can_next(), onclick: move |_| ctx.next(), "Next" }
        }
    }
}
```
