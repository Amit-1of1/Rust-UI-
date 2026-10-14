# Chips

A row of on/off chips for filters and multi-choice tags. Each chip is a native checkbox inside its label; a check mark appears when it is on.

```rust
Chips { label: "Filter by status",
    Chip { default_checked: true, "Open" }
    Chip { "In review" }
    Chip { checked: done(), on_change: move |v| done.set(v), "Done" }
}
```

For one choice out of several, use `ToggleGroup` or `RadioGroup` instead.

## Accessibility

- Role and name: `role="group"` named by `label`; each chip is a native checkbox named by its text
- Keyboard: Tab moves between chips, Space toggles
- State: announced as checked or not checked, and shown with a check mark as well as colour; forced-colors mode outlines the selected chip
- Reduced motion: the check mark and colour change without transition

## Source

Idea from Rust/UI `chips.rs` (rust-ui/ui@43ddc65, MIT). Rewritten: upstream hides the checkbox with `display: none` (unreachable by keyboard and screen readers) and styles with Tailwind; here the checkbox stays focusable and the styles use theme tokens.
