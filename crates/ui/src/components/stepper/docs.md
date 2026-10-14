# Stepper

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
