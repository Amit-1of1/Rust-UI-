# DataTable

A table with sorting, a filter box, pagination and optional row selection, built on `Table`.

```rust
DataTable {
    columns: vec![DataColumn::new("Name").sortable(), DataColumn::new("Salary").sortable().numeric()],
    rows: vec![vec!["Ada".into(), "$142,000".into()]],
    caption: "Team directory",
    page_size: 10,
    selectable: true,
    on_select: move |indices| { /* original row indices */ },
}
```

Cells are text. For custom content (a badge, a link, a row menu) pass `render_cell: move |(row, col)| rsx! { ... }`; `row` is the index into `rows`, and the text in `rows` is still what filtering and sorting use. For a different layout altogether, compose `Table` yourself and call `table_view(...)` for the filter, sort and page logic; it is a pure function.

Paging and selection are kept as row indices, so both reset when `rows` changes (an outside filter, an insert or a delete).

## Accessibility

- Role and name: a native `<table>` with a `<caption>`; sortable headers are buttons inside `<th scope="col" aria-sort>`; the filter has an `aria-label`; each row checkbox is named after the row's first cell, and the header checkbox is "Select all rows on this page".
- Keyboard: everything is a native control (Tab, Space, Enter).
- Status: "Showing 1–5 of 14 rows" and the selected count are in a `role="status"` region, so filter results are announced.
- Reduced motion: no animation.

## Source

Original to this repo (sorting headers come from our `Table`). The feature set follows Rust/UI's `data_table` / `data_grid`, which were not copied.
