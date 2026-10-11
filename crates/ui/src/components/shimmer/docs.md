# Shimmer

Wraps loading content and sweeps a highlight across it.

## Accessibility

- Role and name: `aria-busy="true"` while loading.
- Keyboard: none
- Reduced motion: the sweep becomes a static tint

## Source

Structure and API adapted from Rust/UI (`shimmer.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.
