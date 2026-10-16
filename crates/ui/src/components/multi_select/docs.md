# MultiSelect

Pick several options from a list.

```rust
MultiSelect { label: "Teams", options: vec![MultiOption::new("design", "Design"), MultiOption::new("eng", "Engineering")],
              on_change: move |values| save(values) }
```

A button shows the selection ("Design, Engineering +2"). It opens a panel with a filter box and a list of native checkboxes. `summary()` and `filter_options()` are public, pure helpers.

## Accessibility

- Role and name: the button has `aria-expanded` and `aria-controls` and is named "label: current selection"; the panel is a `role="group"` of real checkboxes, so no custom listbox keyboard model is needed; the count or "No options match." is in a `role="status"` region.
- Focus: inside the open list, Tab moves through the filter, the checkboxes and the Clear/Done buttons. The keys are in the table below.
- Pointer: a click outside the panel closes it.
- Reduced motion: no animation.

## Source

Original to this repo (a checkbox-group disclosure, not an ARIA listbox). Rust/UI's `multi_select.rs` was only used to see the feature set.
