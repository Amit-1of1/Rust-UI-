# Empty

Placeholder for a list, table or page with nothing in it, with optional icon, text and actions. Compose `Empty`, `EmptyHeader`, `EmptyMedia`, `EmptyTitle`, `EmptyDescription`, `EmptyContent`.

## Accessibility

- Role and name: `EmptyTitle` is `role="heading"` with a configurable `level`; the media is `aria-hidden`.
- Keyboard: actions are normal buttons
- Reduced motion: none

## Source

Structure and API adapted from Rust/UI (`empty.rs`, rust-ui/ui commit 43ddc65, MIT); styling rewritten in plain CSS with this kit's tokens. Behaviour and accessibility were reworked here.
