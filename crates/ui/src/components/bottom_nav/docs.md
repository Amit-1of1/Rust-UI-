# BottomNav

A mobile-style tab bar of links or buttons.

## Accessibility

- Role and name: a `nav` landmark named by `label`; the active item has `aria-current="page"`; icons are `aria-hidden`.
- Keyboard: Tab through items; links or buttons
- Reduced motion: none

## Source

Structure and API adapted from Rust/UI (`bottom_nav.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.
