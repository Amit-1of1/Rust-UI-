# DataTable

- Copy into your app: `rust-ui add data_table` or `dx components add data_table --git https://github.com/Amit-1of1/Rust-UI-` (also copies `button`, `input`, `table`, `ui_strings`).
- Import: `use crate::components::data_table::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

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

/// Filter (every whitespace-separated term must appear in some cell, ignoring case), sort
/// (stable; unparsable numbers last) and paginate (`page_size` 0 shows everything).
pub fn table_view(
    columns: &[DataColumn],
    rows: &[Vec<String>],
    query: &str,
    sort: Option<(usize, SortDirection)>,
    page: usize,
    page_size: usize,
) -> TableView

/// A table with sorting, a filter box, pagination and optional row selection. Cells are text
/// unless `render_cell` is given; `toolbar` adds controls beside the filter box. For a different
/// layout compose `Table` yourself and reuse [`table_view`].
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
    /// Rows per page; `0` shows all rows.
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
        DataColumn::new("Name").sortable(),
        DataColumn::new("Team").sortable(),
        DataColumn::new("Office"),
        DataColumn::new("Salary").sortable().numeric(),
    ];
    let mut picked = use_signal(Vec::<usize>::new);
    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            DataTable {
                columns,
                rows,
                caption: "Team directory",
                page_size: 5,
                selectable: true,
                on_select: move |s| picked.set(s),
                toolbar: rsx! {
                    Toggle {
                        pressed: research_only(),
                        on_pressed_change: move |on| research_only.set(on),
                        "Research only"
                    }
                },
            }
            p { style: "margin: 0; font-size: 0.875rem;", "Selected rows: {picked().len()}" }
        }
    }
}
```
