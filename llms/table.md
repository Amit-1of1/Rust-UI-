# Table

- Copy into your app: `atoll add table` or `dx components add table --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::table::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A styled table. Give a `TableHead` an `on_sort` to make it a sort button; see `DataTable` for filtering, paging and selection.

Mark number columns `numeric: true` on both the `TableHead` and its `TableCell`s: they align to the end with tabular figures, and a sortable numeric header puts its arrow before the text so the text lines up with the figures below.

## Accessibility

- Role and name: native table semantics; sortable headers are buttons with `aria-sort`
- Keyboard: sort buttons are tabbable and activate with Enter or Space
- Reduced motion: none

## API

```rust
/// A scrollable wrapper around a native `<table>`, so wide tables don't break the page.
#[component]
pub fn Table(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// A body cell. `numeric` aligns it to the end with tabular figures, to match a
/// `numeric` [`TableHead`].
#[component]
pub fn TableCell(
    #[props(default)] numeric: bool,
    #[props(extends=GlobalAttributes, extends=td)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// The direction a column is sorted in.
pub enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

/// A column header. Pass `on_sort` to make it a sort button (with `aria-sort`). `numeric`
/// aligns it to the end, with the sort arrow before the text so the text lines up with
/// the `numeric` cells below.
#[component]
pub fn TableHead(
    #[props(default)] sort: SortDirection,
    #[props(default)] on_sort: Option<EventHandler<()>>,
    #[props(default)] numeric: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element
```

## Example

```rust
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::table::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Invoice {
    id: &'static str,
    status: &'static str,
    method: &'static str,
    cents: u32,
}

const INVOICES: &[Invoice] = &[
    Invoice {
        id: "INV-001",
        status: "Paid",
        method: "Card",
        cents: 25_000,
    },
    Invoice {
        id: "INV-002",
        status: "Pending",
        method: "Bank",
        cents: 15_050,
    },
    Invoice {
        id: "INV-003",
        status: "Unpaid",
        method: "Card",
        cents: 35_000,
    },
    Invoice {
        id: "INV-004",
        status: "Paid",
        method: "Bank",
        cents: 45_075,
    },
];

#[derive(Copy, Clone, PartialEq)]
enum Column {
    Id,
    Status,
    Amount,
}

/// Sort `rows` by `column`. Stable, so equal rows keep their original order.
fn sorted(column: Column, direction: SortDirection) -> Vec<Invoice> {
    let mut rows = INVOICES.to_vec();
    match (column, direction) {
        (_, SortDirection::None) => {}
        (Column::Id, d) => rows.sort_by(|a, b| apply(d, a.id.cmp(b.id))),
        (Column::Status, d) => rows.sort_by(|a, b| apply(d, a.status.cmp(b.status))),
        (Column::Amount, d) => rows.sort_by(|a, b| apply(d, a.cents.cmp(&b.cents))),
    }
    rows
}

fn apply(direction: SortDirection, ordering: std::cmp::Ordering) -> std::cmp::Ordering {
    match direction {
        SortDirection::Descending => ordering.reverse(),
        _ => ordering,
    }
}

#[component]
pub fn Demo() -> Element {
    let sort = use_signal(|| (Column::Id, SortDirection::None));
    let (column, direction) = sort();
    let rows = sorted(column, direction);
    let total: u32 = rows.iter().map(|r| r.cents).sum();

    rsx! {
        Table {
            TableCaption { "A list of recent invoices." }
            TableHeader {
                TableRow {
                    SortableHead { label: "Invoice", column: Column::Id, sort }
                    SortableHead { label: "Status", column: Column::Status, sort }
                    TableHead { "Method" }
                    SortableHead {
                        label: "Amount",
                        column: Column::Amount,
                        sort,
                        numeric: true,
                    }
                }
            }
            TableBody {
                for row in rows {
                    TableRow { key: "{row.id}",
                        TableCell { style: "font-weight: 500;", "{row.id}" }
                        TableCell {
                            Badge {
                                variant: if row.status == "Paid" { BadgeVariant::Primary } else { BadgeVariant::Outline },
                                "{row.status}"
                            }
                        }
                        TableCell { "{row.method}" }
                        TableCell { numeric: true, "{money(row.cents)}" }
                    }
                }
            }
            TableFooter {
                TableRow {
                    TableCell { colspan: "3", "Total" }
                    TableCell { numeric: true, "{money(total)}" }
                }
            }
        }
    }
}

#[component]
fn SortableHead(
    label: &'static str,
    column: Column,
    sort: Signal<(Column, SortDirection)>,
    #[props(default)] numeric: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let (current, direction) = sort();
    let shown = if current == column {
        direction
    } else {
        SortDirection::None
    };
    let mut sort = sort;
    rsx! {
        TableHead {
            sort: shown,
            numeric,
            on_sort: move |_| {
                let next = if current == column { direction.cycle() } else { SortDirection::Ascending };
                sort.set((column, next));
            },
            attributes,
            "{label}"
        }
    }
}

fn money(cents: u32) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}
```
