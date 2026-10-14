# Table

A styled table. Give a `TableHead` an `on_sort` to make it a sort button; see `DataTable` for filtering, paging and selection.

Mark number columns `numeric: true` on both the `TableHead` and its `TableCell`s: they align to the end with tabular figures, and a sortable numeric header puts its arrow before the text so the text lines up with the figures below.

## Accessibility

- Role and name: native table semantics; sortable headers are buttons with `aria-sort`
- Keyboard: sort buttons are tabbable and activate with Enter or Space
- Reduced motion: none
