# Marquee

A looping auto-scrolling strip (logos, quotes).

## Accessibility

- Role and name: `role="group"` with `label`; the repeated copy is `aria-hidden` and `inert`.
- Keyboard: a Pause/Play button (`aria-pressed`) and focus-within pauses; WCAG 2.2.2 needs a pause control for motion over 5 s
- Reduced motion: scrolling stops and the row becomes horizontally scrollable

## Source

Structure and API adapted from Rust/UI (`marquee.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.
