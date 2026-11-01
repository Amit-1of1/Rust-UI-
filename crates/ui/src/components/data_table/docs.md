# DataTable

A table with sorting, a filter box, pagination, optional row selection, column hiding and resizing, CSV export and virtualised rows for long lists, built on `Table`.

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

## Columns: hide, resize

`column_menu: true` adds a "Columns" menu to the toolbar with a checkbox per column; `DataColumn::hidden()` starts a column hidden and `.always_visible()` keeps it out of the menu (the last shown column can't be hidden either). Hidden columns are not searched by the filter box and not exported. `on_columns_change` gets every column's visibility, to save it.

`resizable_columns: true` gives every header a handle on its end edge: drag it, or Tab to it and use the arrow keys (Shift for bigger steps, Home for the narrowest). The first resize fixes every column at the width it had, and the table becomes exactly as wide as its columns (it scrolls sideways when wider than its box); long cell text is then cut with "…". `DataColumn::width(px)` and `.min_width(px)` set a start width and a limit; `on_column_resize` gets `(column, width)` when a resize ends.

## Export to CSV

`exportable: true` adds an "Export CSV" button. It saves the selected rows, or every row that matches the filter when none are selected (all pages, not just the shown one), in the current sort order and with the shown columns. In the browser it downloads `<caption>.csv` (`export_name` changes it). On desktop and native there's no browser download: pass `on_export: move |csv: String| { ... }` and save it yourself (with a file dialog). `table_csv(&columns, &rows, &order)` is the pure function behind it: RFC 4180 quoting, and cells that would run as spreadsheet formulas (`=`, `+`, `-`, `@` at the start) get a `'` in front, except numbers in `numeric` columns. In server mode the button exports the page you have; export everything on the server with `table_csv` there.

## Long lists: virtualised rows

`virtual_height: Some(480.0)` puts the rows in a scrolling box that tall (CSS pixels) and renders only the rows in view plus a few either side, so 100,000 rows cost what 30 do; the header stays at the top. Paging is off. Every row is `row_height` pixels tall (40 by default; long text is cut with "…"), which is what lets the table know where each row is without measuring it. The table keeps `aria-rowcount` and each row `aria-rowindex`, so screen readers still say "row 5,001 of 10,000". `virtual_window(...)` is the windowing maths, for your own layouts. Past a few thousand rows coming from a server, use server mode instead: virtualising saves the rendering, not the download.

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
- Column menu: native checkboxes named after their column in a popover. Resize handles are focusable `role="separator"` elements named "Resize <column>", with `aria-valuenow` (the width in pixels) and `aria-valuemin`.
- Virtualised: the scroll box is a focusable `role="region"` named by the caption, so keyboard users can scroll it; the table has `aria-rowcount` and rows `aria-rowindex`.
- Status: "Showing 1–5 of 14 rows" and the selected count are in a `role="status"` region, so filter results are announced. While `loading` it reads "Loading" and the table has `aria-busy`.
- Reduced motion: no animation.

## Source

Original to this repo (sorting headers come from our `Table`). The feature set follows Rust/UI's `data_table` / `data_grid`, which were not copied.

## Stability

Experimental. The column API, the `toolbar` slot, column visibility, resizing, export and virtual rows are new and may change in any release; see `docs/STABILITY.md` in the kit repository.
