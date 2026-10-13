//! Orders: the plain `DataTable`, fed from outside filters (date range, status toggles).

use crate::data::{self, money, ORDER_STATUSES};
use crate::overview::order_badge;
use crate::use_store;
use dioxus::prelude::*;
use std::collections::HashSet;
use ui::prelude::*;

#[component]
pub fn Orders() -> Element {
    let store = use_store();
    let all = use_memo(move || data::orders(&store.customers.read(), 120));
    let mut range = use_signal(|| None::<DateRange>);
    // Indices into ORDER_STATUSES that are shown. All on by default.
    let mut shown = use_signal(|| (0..ORDER_STATUSES.len()).collect::<HashSet<usize>>());

    let columns = vec![
        DataColumn::new("Order").sortable(),
        DataColumn::new("Customer").sortable(),
        DataColumn::new("Date").sortable(),
        DataColumn::new("Status").sortable(),
        DataColumn::new("Amount").sortable().numeric(),
    ];
    let rows: Vec<Vec<String>> = all
        .read()
        .iter()
        .filter(|o| {
            let status_ok = ORDER_STATUSES
                .iter()
                .position(|s| *s == o.status)
                .is_some_and(|i| shown.read().contains(&i));
            let date_ok = range().is_none_or(|r| r.contains(o.date));
            status_ok && date_ok
        })
        .map(|o| {
            vec![
                format!("#{}", o.id),
                o.customer.clone(),
                // ISO dates sort correctly as text.
                o.date.to_string(),
                o.status.clone(),
                money(o.amount),
            ]
        })
        .collect();
    let total: u32 = rows
        .iter()
        .filter_map(|r| {
            r[4].trim_start_matches('$')
                .replace(',', "")
                .parse::<u32>()
                .ok()
        })
        .sum();

    rsx! {
        div { class: "db-toolbar",
            div { class: "db-inline-field",
                span { id: "range-label", "Dates" }
                div { role: "group", "aria-labelledby": "range-label",
                    DateRangePicker {
                        selected_range: range(),
                        on_range_change: move |r| range.set(r),
                    }
                }
            }
            div { class: "db-inline-field",
                span { id: "status-toggle-label", "Status" }
                ToggleGroup {
                    "aria-labelledby": "status-toggle-label",
                    allow_multiple_pressed: true,
                    horizontal: true,
                    pressed: Some(shown()),
                    on_pressed_change: move |p| shown.set(p),
                    for (i , s) in ORDER_STATUSES.iter().enumerate() {
                        ToggleItem { index: i, "{s}" }
                    }
                }
            }
            if range().is_some() {
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| range.set(None),
                    "Clear dates"
                }
            }
        }
        p { class: "db-muted", "Total of matching orders: {money(total)}" }
        DataTable {
            columns,
            rows: rows.clone(),
            caption: "Orders",
            page_size: 15,
            selectable: true,
            // Column 3 is the status: show it as a badge. Sorting and filtering still use the text.
            render_cell: move |(row, col): (usize, usize)| {
                let text = &rows[row][col];
                if col == 3 {
                    rsx! { Badge { variant: order_badge(text), "{text}" } }
                } else {
                    rsx! { "{text}" }
                }
            },
        }
    }
}
