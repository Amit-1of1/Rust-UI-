# Stepper

A multi-step progress indicator with Back/Next control via `use_stepper()`.

## Accessibility

- Role and name: An ordered list; the current step has `aria-current="step"`; completed steps add hidden "(completed)" text.
- Keyboard: each step is a button (Tab/Enter/Space); `disabled` steps are skipped
- Reduced motion: none

## Source

Structure and API adapted from Rust/UI (`stepper.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.
