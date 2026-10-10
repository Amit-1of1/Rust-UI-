# Status

A coloured dot plus required text label (idle, online, degraded, down). Optional `pulse`.

## Accessibility

- Role and name: The dot is `aria-hidden`; the label carries the meaning, never colour alone.
- Keyboard: none
- Reduced motion: the pulse ring is removed

## Source

Structure and API adapted from Rust/UI (`status.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.
