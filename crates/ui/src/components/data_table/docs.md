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

Outside filters or actions (a date range, a status select, an "Export" button) go in `toolbar: rsx! { ... }`; they sit in the same row as the filter box and wrap on narrow screens.

Paging and selection are kept as row indices, so both reset when `rows` changes (an outside filter, an insert or a delete).

## Server mode (large tables)

Past a few hundred rows, don't send every row to the browser: page on the server. Pass `total_rows: Some(total)` and only the current page as `rows`; the table then does no filtering, sorting or paging itself and calls `on_query_change` with a `TableQuery` (`query`, `sort`, `page`, `page_size`, `offset()`) whenever the filter, a sort header or the page changes. Fetch that page and hand it back. `loading: true` marks the table busy while you wait.

```rust
let mut q = use_signal(|| TableQuery::new(25));
let page = use_server_future(move || fetch_customers(q()))?; // your server function
let (rows, total) = /* the page's rows and the total match count */;
rsx! {
    DataTable { columns, rows, caption: "Customers", page_size: 25,
        total_rows: Some(total),
        on_query_change: move |next| q.set(next),
    }
}
```

On the server, `table_view(&columns, &all_rows, &q.query, q.sort, q.page, q.page_size)` applies the same filter and sort rules to rows in memory; with a database, turn `TableQuery` into `WHERE`/`ORDER BY`/`LIMIT q.page_size OFFSET q.offset()`. Selection is per page in server mode (it clears when a new page arrives). In the fullstack trial this took a 20,000-customer page from a 4.4 MB HTML response to 44 KB.

## Accessibility

- Role and name: a native `<table>` with a `<caption>`; sortable headers are buttons inside `<th scope="col" aria-sort>`; the filter has an `aria-label`; each row checkbox is named after the row's first cell, and the header checkbox is "Select all rows on this page".
- Keyboard: everything is a native control (Tab, Space, Enter).
- Status: "Showing 1–5 of 14 rows" and the selected count are in a `role="status"` region, so filter results are announced. While `loading` it reads "Loading" and the table has `aria-busy`.
- Reduced motion: no animation.

## Source

Original to this repo (sorting headers come from our `Table`). The feature set follows Rust/UI's `data_table` / `data_grid`, which were not copied.

## Stability

Experimental. The column API and the `toolbar` slot are new and may change in any release; see `docs/STABILITY.md` in the kit repository.
