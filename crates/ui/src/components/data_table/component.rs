use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::table::*;
use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use std::collections::BTreeSet;

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
}

impl DataColumn {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            sortable: false,
            numeric: false,
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

/// Filter (every whitespace-separated term must appear in some cell, ignoring case), sort
/// (stable; unparsable numbers last) and paginate (`page_size` 0 shows everything).
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
                let cells: Vec<String> = rows[i].iter().map(|c| c.to_lowercase()).collect();
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
) -> Element {
    let mut query = use_signal(String::new);
    let mut sort = use_signal(|| None::<(usize, SortDirection)>);
    let mut page = use_signal(|| 0_usize);
    let mut selected = use_signal(BTreeSet::<usize>::new);

    // Selection is kept as row indices, so it means nothing once the rows change (an outside
    // filter, an insert or a delete). Drop it then, and go back to the first page.
    use_effect(use_reactive!(|rows| {
        let _ = rows;
        if *page.peek() != 0 {
            page.set(0);
        }
        if !selected.peek().is_empty() {
            selected.set(BTreeSet::new());
            on_select.call(Vec::new());
        }
    }));

    // Filtering and sorting only rerun when the rows, the query, the sort or the page
    // change, not on a selection click or a parent's unrelated re-render.
    let view = use_memo(use_reactive(
        (&columns, &rows, &page_size),
        move |(columns, rows, page_size)| {
            table_view(&columns, &rows, &query(), sort(), page(), page_size)
        },
    ));
    let view = view();
    let page_rows = view.rows.clone();
    let all_on_page =
        !page_rows.is_empty() && page_rows.iter().all(|i| selected.read().contains(i));
    let shown = if page_size == 0 {
        view.matched
    } else {
        page_rows.len()
    };
    let first = if shown == 0 {
        0
    } else {
        view.page * page_size + 1
    };
    let last = if page_size == 0 {
        view.matched
    } else {
        view.page * page_size + shown
    };
    let t = use_ui_strings();
    let total = view.matched.to_string();
    let status = if view.matched == 0 {
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

    rsx! {
        div { class: Styles::dx_data_table,
            if filterable || toolbar.is_some() {
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
                }
            }
            Table {
                TableCaption { "{caption}" }
                TableHeader {
                    TableRow {
                        if selectable {
                            TableHead {
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
                        for (index , column) in columns.iter().enumerate() {
                            {
                                let direction = match sort() {
                                    Some((c, d)) if c == index => d,
                                    _ => SortDirection::None,
                                };
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
                                        "{column.header}"
                                    }
                                }
                            }
                        }
                    }
                }
                TableBody {
                    for i in page_rows.iter().copied() {
                        TableRow { key: "{i}", "data-selected": selected.read().contains(&i).then_some("true"),
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
                            for (c , cell) in rows[i].iter().enumerate() {
                                TableCell {
                                    key: "{c}",
                                    numeric: columns.get(c).is_some_and(|col| col.numeric),
                                    if let Some(render) = render_cell {
                                        {render.call((i, c))}
                                    } else {
                                        "{cell}"
                                    }
                                }
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
