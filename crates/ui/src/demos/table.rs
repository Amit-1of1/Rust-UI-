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
                        style: "text-align: right;",
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
                        TableCell { style: "text-align: right;", "{money(row.cents)}" }
                    }
                }
            }
            TableFooter {
                TableRow {
                    TableCell { colspan: "3", "Total" }
                    TableCell { style: "text-align: right;", "{money(total)}" }
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
