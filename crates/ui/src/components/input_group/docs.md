# InputGroup

An input with text, icons or buttons attached inside one bordered box.

```rust
InputGroup {
    InputGroupAddon { "https://" }
    InputGroupInput { "aria-label": "Website" }
    InputGroupAddon { end: true, Button { "Go" } }
}
```

The group draws the border and the focus ring (`:focus-within`), so the control inside has none. `invalid` turns the border to the error colour.

## Accessibility

- Role and name: `role="group"`; the control needs its own label (`<label for>` or `aria-label`), because addon text is not a label. Add `aria-invalid` on the control when invalid and show an error message linked with `aria-describedby`.
- Keyboard: Tab moves between the control and any buttons in addons.
- Reduced motion: no animation.

## Source

Structure adapted from Rust/UI's `input_group.rs` (MIT, rust-ui/ui commit 43ddc65); styling and API rewritten.
