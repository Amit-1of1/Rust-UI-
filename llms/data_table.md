# DataTable

- Copy into your app: `atoll add data_table` or `dx components add data_table --git https://github.com/Amit-1of1/Rust-UI-` (also copies `button`, `input`, `popover`, `table`, `ui_strings`).
- Import: `use crate::components::data_table::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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

## API

```rust
/// One column: its header and how it behaves.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct DataColumn {
    pub header: String,
    pub sortable: bool,
    /// Right-aligned, and sorted as numbers (`$1,200`, `45%` and `3.5` all parse).
    pub numeric: bool,
    /// Not shown, not searched by the filter box and not exported. With `column_menu` the
    /// reader can show it again.
    pub hidden: bool,
    /// Listed in the column menu, so the reader can hide it (default `true`).
    pub hideable: bool,
    /// Width in CSS pixels. `None` sizes the column by its content until someone resizes it.
    pub width: Option<f64>,
    /// The narrowest a resize can make the column, in CSS pixels (default 48).
    pub min_width: f64,
}

/// What a [`DataTable`] in server mode asks for: the filter text, the sort and the page.
/// Hand it to your server function; [`table_view`] applies the same rules to rows in memory.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct TableQuery {
    /// What was typed in the filter box.
    pub query: String,
    /// The sorted column and direction, if any.
    pub sort: Option<(usize, SortDirection)>,
    /// Zero-based page.
    pub page: usize,
    pub page_size: usize,
}

/// What the table shows for a given query, sort and page. Pure, so it is easy to test.
pub struct TableView {
    /// Original row indices on the current page, in display order.
    pub rows: Vec<usize>,
    /// Rows that match the filter, across all pages.
    pub matched: usize,
    pub pages: usize,
    /// The current page, clamped into range (zero-based).
    pub page: usize,
}

/// Filter (every whitespace-separated term must appear in some cell of a column that is not
/// `hidden`, ignoring case), sort (stable; unparsable numbers last) and paginate (`page_size`
/// 0 shows everything).
pub fn table_view(
    columns: &[DataColumn],
    rows: &[Vec<String>],
    query: &str,
    sort: Option<(usize, SortDirection)>,
    page: usize,
    page_size: usize,
) -> TableView

/// The rows in `order` (original indices, as in [`TableView::rows`]) as CSV: a header line,
/// then one line per row, with only the columns that are not `hidden`. Follows RFC 4180
/// (comma separated, CRLF line ends, fields with commas, quotes or line breaks quoted).
///
/// A text cell that starts with `=`, `+`, `-`, `@`, a tab or a carriage return gets a `'`
/// in front, so a spreadsheet opens it as text instead of running it as a formula (CSV
/// injection). Numbers in `numeric` columns, such as `-12.5`, are left alone.
pub fn table_csv(columns: &[DataColumn], rows: &[Vec<String>], order: &[usize]) -> String

/// Which rows a virtualised table renders: the ones inside a `viewport` px tall window
/// scrolled `scroll_top` px down a list of `total` rows of `row_height` px each, plus
/// `overscan` rows on each side so a fast scroll doesn't show blank space. Positions are in
/// display order (indices into [`TableView::rows`]).
pub fn virtual_window(
    scroll_top: f64,
    viewport: f64,
    row_height: f64,
    total: usize,
    overscan: usize,
) -> std::ops::Range<usize>

/// A table with sorting, a filter box, pagination and optional row selection. Cells are text
/// unless `render_cell` is given; `toolbar` adds controls beside the filter box. It can also
/// let the reader hide columns (`column_menu`), resize them (`resizable_columns`) and save
/// what they see as CSV (`exportable`), and it renders only the visible rows of a long list
/// (`virtual_height`). For a different layout compose `Table` yourself and reuse
/// [`table_view`], [`table_csv`] and [`virtual_window`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DataTable(
    columns: Vec<DataColumn>,
    rows: Vec<Vec<String>>,
    /// Describes the table to assistive tech and is shown below it.
    #[props(into)]
    caption: String,
    /// Rows per page; `0` shows all rows. Ignored when `virtual_height` is set.
    #[props(default = 10)]
    page_size: usize,
    #[props(default = true)] filterable: bool,
    #[props(default)] selectable: bool,
    /// Called with the original indices of the selected rows whenever the selection changes.
    #[props(default)]
    on_select: Callback<Vec<usize>>,
    /// Custom cell content: called with `(row, column)` (original row index) and rendered
    /// instead of the text. The text in `rows` is still what filtering and sorting use.
    #[props(default)]
    render_cell: Option<Callback<(usize, usize), Element>>,
    /// Extra controls (outside filters, actions) shown in the toolbar beside the filter box.
    #[props(default)]
    toolbar: Option<Element>,
    /// Server mode: the number of rows that match on the server. `rows` is then only the
    /// current page, already filtered and sorted there; the table does no filtering, sorting
    /// or paging itself and reports every change through `on_query_change`.
    #[props(default)]
    total_rows: Option<usize>,
    /// Server mode: called when the filter text, the sort or the page changes (not on the
    /// first render, which shows `TableQuery::new(page_size)`). Fetch that page and pass it
    /// back as `rows` and `total_rows`.
    #[props(default)]
    on_query_change: Callback<TableQuery>,
    /// A page is being fetched: the table is marked busy and the status says so.
    #[props(default)]
    loading: bool,
    /// Show a "Columns" menu in the toolbar with a checkbox for every `hideable` column.
    #[props(default)]
    column_menu: bool,
    /// Called with every column's visibility (`true` = shown) when the reader changes it.
    #[props(default)]
    on_columns_change: Callback<Vec<bool>>,
    /// Give every header a resize handle: drag it, or focus it and use the arrow keys.
    #[props(default)]
    resizable_columns: bool,
    /// Called with `(column, width in CSS pixels)` when a resize ends, to save the layout.
    #[props(default)]
    on_column_resize: Callback<(usize, f64)>,
    /// Show an "Export CSV" button in the toolbar. It exports the selected rows, or every row
    /// that matches the filter when none are selected, in the current sort order and with
    /// the visible columns (see [`table_csv`]).
    #[props(default)]
    exportable: bool,
    /// Receives the CSV instead of the browser download. Needed on desktop and native, where
    /// there is no browser to download it (save it with a file dialog there).
    #[props(default)]
    on_export: Option<Callback<String>>,
    /// The downloaded file's name. Defaults to the caption, as `team-directory.csv`.
    #[props(default, into)]
    export_name: Option<String>,
    /// Virtualise: show the rows in a scrolling box this many CSS pixels tall and render only
    /// the rows in view, so 100,000 rows cost what 30 do. Paging is off; the header stays put.
    #[props(default)]
    virtual_height: Option<f64>,
    /// Every row's height in CSS pixels when virtualised. Long cell text is cut with "…".
    #[props(default = 40.0)]
    row_height: f64,
) -> Element
```

## Example

```rust
use crate::components::data_table::*;
use crate::components::toggle::Toggle;
use dioxus::prelude::*;

const PEOPLE: [(&str, &str, &str, &str); 14] = [
    ("Ada Lovelace", "Engineering", "London", "$142,000"),
    ("Grace Hopper", "Engineering", "New York", "$151,500"),
    ("Alan Turing", "Research", "Manchester", "$138,250"),
    ("Katherine Johnson", "Research", "Hampton", "$129,900"),
    ("Margaret Hamilton", "Engineering", "Boston", "$147,300"),
    ("Linus Torvalds", "Platform", "Portland", "$160,000"),
    ("Barbara Liskov", "Research", "Cambridge", "$155,750"),
    ("Dennis Ritchie", "Platform", "Murray Hill", "$133,400"),
    ("Hedy Lamarr", "Design", "Vienna", "$98,000"),
    ("Tim Berners-Lee", "Platform", "Geneva", "$149,000"),
    ("Radia Perlman", "Platform", "Redmond", "$144,800"),
    ("Donald Knuth", "Research", "Stanford", "$170,000"),
    ("Frances Allen", "Engineering", "Yorktown", "$140,100"),
    ("Edsger Dijkstra", "Research", "Eindhoven", "$137,600"),
];

#[component]
pub fn Demo() -> Element {
    // An outside filter, shown in the table's toolbar beside its own filter box.
    let mut research_only = use_signal(|| false);
    let rows: Vec<Vec<String>> = PEOPLE
        .iter()
        .filter(|(_, team, _, _)| !research_only() || *team == "Research")
        .map(|(a, b, c, d)| vec![a.to_string(), b.to_string(), c.to_string(), d.to_string()])
        .collect();
    let columns = vec![
        DataColumn::new("Name").sortable().always_visible(),
        DataColumn::new("Team").sortable(),
        DataColumn::new("Office"),
        DataColumn::new("Salary").sortable().numeric(),
    ];
    let mut picked = use_signal(Vec::<usize>::new);
    let orders = use_hook(orders);
    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            DataTable {
                columns,
                rows,
                caption: "Team directory",
                page_size: 5,
                selectable: true,
                on_select: move |s| picked.set(s),
                column_menu: true,
                resizable_columns: true,
                exportable: true,
                toolbar: rsx! {
                    Toggle {
                        pressed: research_only(),
                        on_pressed_change: move |on| research_only.set(on),
                        "Research only"
                    }
                },
            }
            p { style: "margin: 0; font-size: 0.875rem;", "Selected rows: {picked().len()}" }
            // A long list: only the rows in view are rendered, so 10,000 cost what 20 do.
            DataTable {
                columns: vec![
                    DataColumn::new("Order").sortable(),
                    DataColumn::new("Customer").sortable(),
                    DataColumn::new("Status"),
                    DataColumn::new("Total").sortable().numeric(),
                ],
                rows: orders.clone(),
                caption: "10,000 orders",
                virtual_height: 360.0,
                exportable: true,
            }
        }
    }
}

/// 10,000 made-up orders for the virtualised table, the same on every run.
fn orders() -> Vec<Vec<String>> {
    const CUSTOMERS: [&str; 6] = ["Acme", "Globex", "Initech", "Umbrella", "Hooli", "Vandelay"];
    const STATUS: [&str; 4] = ["Paid", "Pending", "Refunded", "Shipped"];
    (0..10_000_usize)
        .map(|i| {
            vec![
                format!("#{:05}", i + 1),
                CUSTOMERS[(i * 7) % CUSTOMERS.len()].to_string(),
                STATUS[(i * 3) % STATUS.len()].to_string(),
                format!("${}.{:02}", (i * 37) % 5000 + 5, (i * 13) % 100),
            ]
        })
        .collect()
}
```
