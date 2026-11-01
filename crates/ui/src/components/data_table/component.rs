use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::popover::{PopoverContent, PopoverRoot, PopoverTrigger};
use crate::components::table::*;
use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use std::collections::BTreeSet;
use std::rc::Rc;

#[css_module("/src/components/data_table/style.css")]
struct Styles;

/// One column: its header and how it behaves.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, PartialEq, Debug)]
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

impl DataColumn {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            sortable: false,
            numeric: false,
            hidden: false,
            hideable: true,
            width: None,
            min_width: 48.0,
        }
    }

    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    /// Start hidden; the column menu can show it.
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Keep it out of the column menu, so it can't be hidden (an id or name column).
    pub fn always_visible(mut self) -> Self {
        self.hideable = false;
        self
    }

    /// Start at this width, in CSS pixels.
    pub fn width(mut self, px: f64) -> Self {
        self.width = Some(px);
        self
    }

    /// The narrowest a resize can make it, in CSS pixels.
    pub fn min_width(mut self, px: f64) -> Self {
        self.min_width = px;
        self
    }
}

/// What a [`DataTable`] in server mode asks for: the filter text, the sort and the page.
/// Hand it to your server function; [`table_view`] applies the same rules to rows in memory.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TableQuery {
    /// What was typed in the filter box.
    pub query: String,
    /// The sorted column and direction, if any.
    pub sort: Option<(usize, SortDirection)>,
    /// Zero-based page.
    pub page: usize,
    pub page_size: usize,
}

impl TableQuery {
    /// The first page, unfiltered and unsorted: what the table starts with.
    pub fn new(page_size: usize) -> Self {
        Self {
            page_size,
            ..Self::default()
        }
    }

    /// Rows to skip: `page * page_size` (SQL `OFFSET`).
    pub fn offset(&self) -> usize {
        self.page * self.page_size
    }
}

/// What the table shows for a given query, sort and page. Pure, so it is easy to test.
#[derive(Clone, PartialEq, Debug)]
pub struct TableView {
    /// Original row indices on the current page, in display order.
    pub rows: Vec<usize>,
    /// Rows that match the filter, across all pages.
    pub matched: usize,
    pub pages: usize,
    /// The current page, clamped into range (zero-based).
    pub page: usize,
}

fn number(cell: &str) -> Option<f64> {
    let cleaned: String = cell
        .chars()
        .filter(|c| !matches!(c, '$' | '€' | '£' | ',' | '%' | ' '))
        .collect();
    cleaned.parse().ok()
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
) -> TableView {
    // Lower-cased once per term and per cell, not once per comparison: this runs on every
    // render, and a selection click re-renders the whole table.
    let terms: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    let mut indices: Vec<usize> = if terms.is_empty() {
        (0..rows.len()).collect()
    } else {
        (0..rows.len())
            .filter(|&i| {
                let cells: Vec<String> = rows[i]
                    .iter()
                    .enumerate()
                    .filter(|(c, _)| !columns.get(*c).is_some_and(|col| col.hidden))
                    .map(|(_, cell)| cell.to_lowercase())
                    .collect();
                terms.iter().all(|t| cells.iter().any(|c| c.contains(t)))
            })
            .collect()
    };

    if let Some((col, direction)) = sort {
        if direction != SortDirection::None && col < columns.len() {
            let cell = |i: usize| rows[i].get(col).map(String::as_str).unwrap_or("");
            let descending = direction == SortDirection::Descending;
            if columns[col].numeric {
                // Unparsable numbers stay last in both directions.
                let mut keyed: Vec<(Option<f64>, usize)> =
                    indices.iter().map(|&i| (number(cell(i)), i)).collect();
                keyed.sort_by(|(x, _), (y, _)| match (x, y) {
                    (Some(x), Some(y)) if descending => y.total_cmp(x),
                    (Some(x), Some(y)) => x.total_cmp(y),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                });
                indices = keyed.into_iter().map(|(_, i)| i).collect();
            } else {
                let mut keyed: Vec<(String, usize)> =
                    indices.iter().map(|&i| (cell(i).to_lowercase(), i)).collect();
                if descending {
                    keyed.sort_by(|(a, _), (b, _)| b.cmp(a));
                } else {
                    keyed.sort_by(|(a, _), (b, _)| a.cmp(b));
                }
                indices = keyed.into_iter().map(|(_, i)| i).collect();
            }
        }
    }

    let matched = indices.len();
    if page_size == 0 {
        return TableView {
            rows: indices,
            matched,
            pages: 1,
            page: 0,
        };
    }
    let pages = matched.div_ceil(page_size).max(1);
    let page = page.min(pages - 1);
    let rows = indices
        .into_iter()
        .skip(page * page_size)
        .take(page_size)
        .collect();
    TableView {
        rows,
        matched,
        pages,
        page,
    }
}

/// The rows in `order` (original indices, as in [`TableView::rows`]) as CSV: a header line,
/// then one line per row, with only the columns that are not `hidden`. Follows RFC 4180
/// (comma separated, CRLF line ends, fields with commas, quotes or line breaks quoted).
///
/// A text cell that starts with `=`, `+`, `-`, `@`, a tab or a carriage return gets a `'`
/// in front, so a spreadsheet opens it as text instead of running it as a formula (CSV
/// injection). Numbers in `numeric` columns, such as `-12.5`, are left alone.
pub fn table_csv(columns: &[DataColumn], rows: &[Vec<String>], order: &[usize]) -> String {
    let shown: Vec<usize> = (0..columns.len()).filter(|&c| !columns[c].hidden).collect();
    let mut out = String::new();
    let mut line = |cells: &mut dyn Iterator<Item = String>| {
        let mut first = true;
        for cell in cells {
            if !first {
                out.push(',');
            }
            first = false;
            if cell.contains([',', '"', '\n', '\r']) || cell.starts_with(' ') || cell.ends_with(' ') {
                out.push('"');
                out.push_str(&cell.replace('"', "\"\""));
                out.push('"');
            } else {
                out.push_str(&cell);
            }
        }
        out.push_str("\r\n");
    };
    line(&mut shown.iter().map(|&c| csv_safe(&columns[c].header, false)));
    for &i in order {
        let Some(row) = rows.get(i) else { continue };
        line(&mut shown.iter().map(|&c| {
            let cell = row.get(c).map(String::as_str).unwrap_or("");
            csv_safe(cell, columns[c].numeric)
        }));
    }
    out
}

fn csv_safe(cell: &str, numeric: bool) -> String {
    let risky = cell.starts_with(['=', '+', '-', '@', '\t', '\r']);
    if risky && !(numeric && number(cell).is_some()) {
        format!("'{cell}")
    } else {
        cell.to_string()
    }
}

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
) -> std::ops::Range<usize> {
    if total == 0 || row_height <= 0.0 {
        return 0..0;
    }
    let first = (scroll_top.max(0.0) / row_height).floor() as usize;
    let visible = (viewport.max(0.0) / row_height).ceil() as usize + 1;
    let start = first.saturating_sub(overscan).min(total);
    let end = (first + visible + overscan).min(total);
    start.min(end)..end
}

/// The view of a server-paged table: `rows_on_page` rows (the page the server sent) out of
/// `total` matching rows.
fn server_view(rows_on_page: usize, total: usize, page: usize, page_size: usize) -> TableView {
    let pages = if page_size == 0 {
        1
    } else {
        total.div_ceil(page_size).max(1)
    };
    TableView {
        rows: (0..rows_on_page).collect(),
        matched: total,
        pages,
        page: page.min(pages - 1),
    }
}

/// Where a column resize drag started.
#[derive(Clone, Copy, PartialEq, Debug)]
struct ResizeDrag {
    column: usize,
    start_x: f64,
    start_width: f64,
}

/// Rows rendered above and below the visible ones in a virtualised table.
const OVERSCAN: usize = 8;
/// Arrow key step for a focused resize handle, in CSS pixels (Shift: four times).
const RESIZE_STEP: f64 = 16.0;
/// Width given to a column shown from the menu after the others were resized.
const SHOWN_COLUMN_WIDTH: f64 = 160.0;

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
) -> Element {
    let server = total_rows.is_some();
    let page_size = if virtual_height.is_some() {
        0
    } else {
        page_size
    };
    let mut query = use_signal(String::new);
    let mut sort = use_signal(|| None::<(usize, SortDirection)>);
    let mut page = use_signal(|| 0_usize);
    let mut selected = use_signal(BTreeSet::<usize>::new);
    // Column state is kept per original column index; columns added later fall back to
    // what their `DataColumn` says.
    let mut visible = use_signal(|| columns.iter().map(|c| !c.hidden).collect::<Vec<_>>());
    let mut widths = use_signal(|| columns.iter().map(|c| c.width).collect::<Vec<_>>());
    let mut select_width = use_signal(|| None::<f64>);
    let mut boxes = use_signal(Vec::<Option<Rc<MountedData>>>::new);
    let mut select_box = use_signal(|| None::<Rc<MountedData>>);
    let mut rtl = use_signal(|| false);
    let mut drag = use_signal(|| None::<ResizeDrag>);
    let mut first_row = use_signal(|| 0_usize);

    // Selection is kept as row indices, so it means nothing once the rows change (an outside
    // filter, an insert or a delete). Drop it then, and go back to the first page.
    use_effect(use_reactive!(|rows| {
        let _ = rows;
        // In server mode new rows are the page that was asked for: keep it.
        if !server && *page.peek() != 0 {
            page.set(0);
        }
        if !selected.peek().is_empty() {
            selected.set(BTreeSet::new());
            on_select.call(Vec::new());
        }
    }));

    let is_shown = move |c: usize, column: &DataColumn| -> bool {
        visible.read().get(c).copied().unwrap_or(!column.hidden)
    };
    // The columns as the reader set them up: hidden ones aren't searched or exported.
    let effective: Vec<DataColumn> = columns
        .iter()
        .enumerate()
        .map(|(c, column)| DataColumn {
            hidden: !is_shown(c, column),
            ..column.clone()
        })
        .collect();
    let shown: Vec<usize> = (0..effective.len())
        .filter(|&c| !effective[c].hidden)
        .collect();
    let current: Vec<bool> = effective.iter().map(|c| !c.hidden).collect();
    let mins: Vec<f64> = columns.iter().map(|c| c.min_width).collect();
    let width_of = move |c: usize| -> Option<f64> {
        widths.read().get(c).copied().flatten()
    };

    // Filtering and sorting only rerun when the rows, the query, the sort or the page
    // change, not on a selection click or a parent's unrelated re-render.
    let view = use_memo(use_reactive(
        (&effective, &rows, &page_size, &total_rows),
        move |(columns, rows, page_size, total_rows)| match total_rows {
            Some(total) => server_view(rows.len(), total, page(), page_size),
            None => table_view(&columns, &rows, &query(), sort(), page(), page_size),
        },
    ));
    // Server mode: report changes (skipping the first run, which is the initial query).
    let mut reported = use_signal(|| false);
    use_effect(move || {
        let next = TableQuery {
            query: query(),
            sort: sort(),
            page: page(),
            page_size,
        };
        if !*reported.peek() {
            reported.set(true);
        } else if server {
            on_query_change.call(next);
        }
    });
    let view = view();
    let page_rows = view.rows.clone();
    let all_on_page =
        !page_rows.is_empty() && page_rows.iter().all(|i| selected.read().contains(i));
    let shown_count = if page_size == 0 {
        view.matched
    } else {
        page_rows.len()
    };
    let first = if shown_count == 0 {
        0
    } else {
        view.page * page_size + 1
    };
    let last = if page_size == 0 {
        view.matched
    } else {
        view.page * page_size + shown_count
    };
    let t = use_ui_strings();
    let total = view.matched.to_string();
    let status = if loading {
        t.loading.clone()
    } else if view.matched == 0 {
        t.no_rows.clone()
    } else if page_size == 0 {
        fill(&t.row_count, &[("count", &total)])
    } else {
        fill(
            &t.showing_rows,
            &[
                ("first", &first.to_string()),
                ("last", &last.to_string()),
                ("total", &total),
            ],
        )
    };
    let selected_note = fill(
        &t.selected_count,
        &[("count", &selected.read().len().to_string())],
    );
    let page_of = fill(
        &t.page_of,
        &[
            ("page", &(view.page + 1).to_string()),
            ("pages", &view.pages.to_string()),
        ],
    );
    let selected_count = selected.read().len();

    // Virtualised: only the rows in the scrolled window are rendered, between two spacers.
    let window = match virtual_height {
        Some(height) => virtual_window(
            first_row() as f64 * row_height,
            height,
            row_height,
            page_rows.len(),
            OVERSCAN,
        ),
        None => 0..page_rows.len(),
    };
    let above = window.start as f64 * row_height;
    let below = page_rows.len().saturating_sub(window.end) as f64 * row_height;
    let span = shown.len() + usize::from(selectable);

    // Resized columns switch the table to a fixed layout of exactly those widths.
    let fixed = !shown.is_empty() && shown.iter().all(|&c| width_of(c).is_some());
    let table_style = fixed.then(|| {
        let sum: f64 = shown.iter().filter_map(|&c| width_of(c)).sum();
        match (selectable, select_width()) {
            (true, Some(w)) => format!("table-layout: fixed; width: {}px;", sum + w),
            (true, None) => format!("table-layout: fixed; width: calc({sum}px + 2.75rem);"),
            _ => format!("table-layout: fixed; width: {sum}px;"),
        }
    });
    let any_width = shown.iter().any(|&c| width_of(c).is_some());

    // Measure every shown column once, the first time one is resized, so the others keep
    // the width they had when the layout turns fixed.
    let freeze = move || async move {
        let measured: Vec<(usize, Rc<MountedData>)> = boxes
            .peek()
            .iter()
            .enumerate()
            .filter_map(|(c, b)| b.clone().map(|b| (c, b)))
            .collect();
        let mut lefts = Vec::new();
        for (c, mounted) in measured {
            let Ok(rect) = mounted.get_client_rect().await else { continue };
            lefts.push(rect.origin.x);
            let mut w = widths.write();
            if w.len() <= c {
                w.resize(c + 1, None);
            }
            if w[c].is_none() {
                w[c] = Some(rect.size.width.round());
            }
        }
        if lefts.len() > 1 {
            rtl.set(lefts[1] < lefts[0]);
        }
        let selection = select_box.peek().clone();
        if let Some(mounted) = selection {
            if select_width.peek().is_none() {
                if let Ok(rect) = mounted.get_client_rect().await {
                    select_width.set(Some(rect.size.width.round()));
                }
            }
        }
    };
    let mut set_width = move |c: usize, px: f64, min: f64| {
        let px = px.max(min).round();
        let mut w = widths.write();
        if w.len() <= c {
            w.resize(c + 1, None);
        }
        w[c] = Some(px);
    };

    let export = {
        let effective = effective.clone();
        let rows = rows.clone();
        let caption = caption.clone();
        let export_name = export_name.clone();
        move |_| {
            let picked = selected.peek().clone();
            let order: Vec<usize> = if server {
                (0..rows.len()).collect()
            } else {
                table_view(&effective, &rows, &query.peek(), *sort.peek(), 0, 0).rows
            };
            let order: Vec<usize> = if picked.is_empty() {
                order
            } else {
                order.into_iter().filter(|i| picked.contains(i)).collect()
            };
            let csv = table_csv(&effective, &rows, &order);
            if let Some(on_export) = on_export {
                on_export.call(csv);
            } else {
                let name = export_name.clone().unwrap_or_else(|| file_name(&caption));
                download_csv(&name, &csv);
            }
        }
    };

    rsx! {
        div {
            class: Styles::dx_data_table,
            "data-resizing": drag().is_some().then_some("true"),
            onpointermove: move |e| {
                let Some(d) = *drag.peek() else { return };
                let x = e.client_coordinates().x;
                let delta = if *rtl.peek() { d.start_x - x } else { x - d.start_x };
                let min = mins.get(d.column).copied().unwrap_or(48.0);
                set_width(d.column, d.start_width + delta, min);
            },
            onpointerup: move |_| end_drag(drag, widths, on_column_resize),
            onpointercancel: move |_| end_drag(drag, widths, on_column_resize),
            onpointerleave: move |_| end_drag(drag, widths, on_column_resize),
            if filterable || toolbar.is_some() || column_menu || exportable {
                div { class: Styles::dx_data_table_toolbar,
                    if filterable {
                        Input {
                            class: Styles::dx_data_table_filter,
                            r#type: "search",
                            placeholder: t.filter_rows.clone(),
                            "aria-label": t.filter_rows.clone(),
                            value: "{query()}",
                            oninput: move |e: FormEvent| {
                                query.set(e.value());
                                page.set(0);
                            },
                        }
                    }
                    {toolbar}
                    if column_menu || exportable {
                        div { class: Styles::dx_data_table_actions,
                            if column_menu {
                                PopoverRoot {
                                    PopoverTrigger { {t.columns.clone()} }
                                    PopoverContent {
                                        class: Styles::dx_data_table_columns.to_string(),
                                        side: ContentSide::Bottom,
                                        align: ContentAlign::End,
                                        for (c , column) in columns.iter().enumerate().filter(|(_, col)| col.hideable) {
                                            {
                                                let on = !effective[c].hidden;
                                                // The last shown column can't be hidden.
                                                let last_one = on && shown.len() == 1;
                                                let frozen = fixed;
                                                let start_width = column.width;
                                                let current = current.clone();
                                                rsx! {
                                                    label { key: "{c}", class: Styles::dx_data_table_column_option,
                                                        input {
                                                            r#type: "checkbox",
                                                            checked: on,
                                                            disabled: last_one,
                                                            onchange: move |e: FormEvent| {
                                                                let mut next = current.clone();
                                                                next[c] = e.checked();
                                                                if e.checked() && frozen && width_of(c).is_none() {
                                                                    set_width(c, start_width.unwrap_or(SHOWN_COLUMN_WIDTH), 0.0);
                                                                }
                                                                visible.set(next.clone());
                                                                on_columns_change.call(next);
                                                            },
                                                        }
                                                        "{column.header}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if exportable {
                                Button { variant: ButtonVariant::Outline, onclick: export, {t.export_csv.clone()} }
                            }
                        }
                    }
                }
            }
            div {
                class: if virtual_height.is_some() { Styles::dx_data_table_viewport.to_string() },
                style: virtual_height.map(|h| format!("max-height: {h}px;")),
                role: virtual_height.map(|_| "region"),
                "aria-label": virtual_height.map(|_| caption.clone()),
                tabindex: virtual_height.map(|_| "0"),
                onscroll: move |e| {
                    if virtual_height.is_some() && row_height > 0.0 {
                        let next = (e.scroll_top().max(0.0) / row_height) as usize;
                        if next != *first_row.peek() {
                            first_row.set(next);
                        }
                    }
                },
                Table {
                    "aria-busy": loading.then_some("true"),
                    "aria-rowcount": virtual_height.map(|_| (view.matched + 1).to_string()),
                    "data-layout": fixed.then_some("fixed"),
                    style: table_style,
                    TableCaption { "{caption}" }
                    if any_width {
                        colgroup {
                            if selectable {
                                col { style: select_width().map(|w| format!("width: {w}px;")).unwrap_or_else(|| "width: 2.75rem;".into()) }
                            }
                            for c in shown.iter().copied() {
                                col { key: "{c}", style: width_of(c).map(|w| format!("width: {w}px;")) }
                            }
                        }
                    }
                    TableHeader {
                        TableRow { "aria-rowindex": virtual_height.map(|_| "1"),
                            if selectable {
                                TableHead {
                                    trailing: rsx! {
                                        if resizable_columns {
                                            span {
                                                class: Styles::dx_data_table_measure,
                                                "aria-hidden": "true",
                                                onmounted: move |e| select_box.set(Some(e.data())),
                                            }
                                        }
                                    },
                                    input {
                                        r#type: "checkbox",
                                        "aria-label": t.select_all_rows.clone(),
                                        checked: all_on_page,
                                        onchange: {
                                            let page_rows = page_rows.clone();
                                            move |e: FormEvent| {
                                                selected
                                                    .with_mut(|set| {
                                                        for i in &page_rows {
                                                            if e.checked() {
                                                                set.insert(*i);
                                                            } else {
                                                                set.remove(i);
                                                            }
                                                        }
                                                    });
                                                on_select.call(selected.read().iter().copied().collect());
                                            }
                                        },
                                    }
                                }
                            }
                            for index in shown.iter().copied() {
                                {
                                    let column = &columns[index];
                                    let direction = match sort() {
                                        Some((c, d)) if c == index => d,
                                        _ => SortDirection::None,
                                    };
                                    let min = column.min_width;
                                    let resize_label = fill(&t.resize_column, &[("column", &column.header)]);
                                    let now = width_of(index);
                                    rsx! {
                                        TableHead {
                                            key: "{index}",
                                            sort: direction,
                                            on_sort: column
                                                .sortable
                                                .then(|| {
                                                    EventHandler::new(move |_| {
                                                        let next = if direction == SortDirection::None && sort().is_none_or(|(c, _)| c != index) {
                                                            SortDirection::Ascending
                                                        } else {
                                                            direction.cycle()
                                                        };
                                                        sort.set(Some((index, next)));
                                                        page.set(0);
                                                    })
                                                }),
                                            numeric: column.numeric,
                                            trailing: rsx! {
                                                if resizable_columns {
                                                    span {
                                                        class: Styles::dx_data_table_measure,
                                                        "aria-hidden": "true",
                                                        onmounted: move |e| {
                                                            let mut b = boxes.write();
                                                            if b.len() <= index {
                                                                b.resize(index + 1, None);
                                                            }
                                                            b[index] = Some(e.data());
                                                        },
                                                    }
                                                    div {
                                                        class: Styles::dx_data_table_resize,
                                                        role: "separator",
                                                        tabindex: "0",
                                                        "aria-orientation": "vertical",
                                                        "aria-label": resize_label,
                                                        "aria-valuenow": now.map(|w| format!("{w}")),
                                                        "aria-valuemin": format!("{min}"),
                                                        "data-dragging": drag().is_some_and(|d| d.column == index).then_some("true"),
                                                        onpointerdown: move |e: PointerEvent| {
                                                            e.prevent_default();
                                                            e.stop_propagation();
                                                            let start_x = e.client_coordinates().x;
                                                            spawn(async move {
                                                                freeze().await;
                                                                let start_width = width_of(index).unwrap_or(min);
                                                                drag.set(Some(ResizeDrag { column: index, start_x, start_width }));
                                                            });
                                                        },
                                                        onkeydown: move |e: KeyboardEvent| {
                                                            let step = if e.modifiers().shift() { RESIZE_STEP * 4.0 } else { RESIZE_STEP };
                                                            let key = e.key();
                                                            let change = match key {
                                                                Key::ArrowRight => Some(step),
                                                                Key::ArrowLeft => Some(-step),
                                                                Key::Home => Some(f64::NEG_INFINITY),
                                                                _ => None,
                                                            };
                                                            let Some(change) = change else { return };
                                                            e.prevent_default();
                                                            spawn(async move {
                                                                freeze().await;
                                                                // The handle moves the way the arrow points.
                                                                let change = if *rtl.peek() && change.is_finite() { -change } else { change };
                                                                let from = width_of(index).unwrap_or(min);
                                                                set_width(index, from + change, min);
                                                                if let Some(w) = width_of(index) {
                                                                    on_column_resize.call((index, w));
                                                                }
                                                            });
                                                        },
                                                    }
                                                }
                                            },
                                            "{column.header}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    TableBody {
                        if above > 0.0 {
                            tr { class: Styles::dx_data_table_spacer, "aria-hidden": "true",
                                td { colspan: "{span}", style: "height: {above}px;" }
                            }
                        }
                        for (position , i) in page_rows[window.clone()].iter().copied().enumerate() {
                            TableRow {
                                key: "{i}",
                                "data-selected": selected.read().contains(&i).then_some("true"),
                                "aria-rowindex": virtual_height.map(|_| (window.start + position + 2).to_string()),
                                style: virtual_height.map(|_| format!("height: {row_height}px;")),
                                if selectable {
                                    TableCell {
                                        input {
                                            r#type: "checkbox",
                                            "aria-label": fill(&t.select_row, &[("row", rows[i].first().map(String::as_str).unwrap_or_default())]),
                                            checked: selected.read().contains(&i),
                                            onchange: move |e: FormEvent| {
                                                selected
                                                    .with_mut(|set| {
                                                        if e.checked() {
                                                            set.insert(i);
                                                        } else {
                                                            set.remove(&i);
                                                        }
                                                    });
                                                on_select.call(selected.read().iter().copied().collect());
                                            },
                                        }
                                    }
                                }
                                for c in shown.iter().copied() {
                                    TableCell {
                                        key: "{c}",
                                        numeric: columns.get(c).is_some_and(|col| col.numeric),
                                        if let Some(render) = render_cell {
                                            {render.call((i, c))}
                                        } else {
                                            {rows[i].get(c).cloned().unwrap_or_default()}
                                        }
                                    }
                                }
                            }
                        }
                        if below > 0.0 {
                            tr { class: Styles::dx_data_table_spacer, "aria-hidden": "true",
                                td { colspan: "{span}", style: "height: {below}px;" }
                            }
                        }
                    }
                }
            }
            div { class: Styles::dx_data_table_footer,
                p { class: Styles::dx_data_table_status, role: "status",
                    "{status}"
                    if selectable && selected_count > 0 {
                        " · {selected_note}"
                    }
                }
                if page_size != 0 && view.pages > 1 {
                    nav { "aria-label": t.table_pages.clone(), class: Styles::dx_data_table_pages,
                        Button {
                            variant: ButtonVariant::Outline,
                            disabled: view.page == 0,
                            onclick: move |_| page.set(view.page.saturating_sub(1)),
                            {t.previous.clone()}
                        }
                        span { "{page_of}" }
                        Button {
                            variant: ButtonVariant::Outline,
                            disabled: view.page + 1 >= view.pages,
                            onclick: move |_| page.set(view.page + 1),
                            {t.next.clone()}
                        }
                    }
                }
            }
        }
    }
}

/// Ends a resize drag and reports the new width.
fn end_drag(
    mut drag: Signal<Option<ResizeDrag>>,
    widths: Signal<Vec<Option<f64>>>,
    on_column_resize: Callback<(usize, f64)>,
) {
    let Some(d) = *drag.peek() else { return };
    drag.set(None);
    if let Some(w) = widths.peek().get(d.column).copied().flatten() {
        on_column_resize.call((d.column, w));
    }
}

/// `"Team directory"` becomes `team-directory.csv`.
fn file_name(caption: &str) -> String {
    let mut name = String::new();
    for ch in caption.chars() {
        if ch.is_alphanumeric() {
            name.extend(ch.to_lowercase());
        } else if !name.is_empty() && !name.ends_with('-') {
            name.push('-');
        }
    }
    let name = name.trim_end_matches('-');
    if name.is_empty() {
        "table.csv".to_string()
    } else {
        format!("{name}.csv")
    }
}

/// Saves `csv` as a file through the browser (a Blob and a temporary link, all in Rust).
/// Elsewhere there is no browser to save through: apps pass `on_export` instead.
#[cfg(target_arch = "wasm32")]
fn download_csv(name: &str, csv: &str) {
    use wasm_bindgen::JsCast;
    let Some(document) = web_sys::window().and_then(|w| w.document()) else { return };
    // A byte order mark, so spreadsheets read the file as UTF-8.
    let parts = js_sys::Array::of1(&format!("\u{feff}{csv}").into());
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("text/csv;charset=utf-8");
    let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options) else {
        return;
    };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else { return };
    if let Ok(link) = document.create_element("a") {
        let _ = link.set_attribute("href", &url);
        let _ = link.set_attribute("download", name);
        if let Ok(link) = link.dyn_into::<web_sys::HtmlElement>() {
            link.click();
        }
    }
    // The download may still be reading the file, so the previous export's file is the one
    // let go: at most one is kept.
    thread_local! {
        static LAST: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
    }
    if let Some(previous) = LAST.with(|last| last.replace(Some(url))) {
        let _ = web_sys::Url::revoke_object_url(&previous);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn download_csv(_name: &str, _csv: &str) {}
