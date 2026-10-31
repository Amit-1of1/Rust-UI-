//! What a production app needs beyond the demos: a server-paged `DataTable` and form fields
//! added while the form is open (line items).

mod common;

use common::render;
use dioxus::prelude::*;
use ui::prelude::*;

fn columns() -> Vec<DataColumn> {
    vec![
        DataColumn::new("Name").sortable(),
        DataColumn::new("Balance").sortable().numeric(),
    ]
}

#[test]
fn server_mode_shows_the_page_it_was_given_and_counts_the_server_total() {
    fn app() -> Element {
        // Page 1 of 20,000 customers, as the server sent it: deliberately not sorted, to
        // show the table does no local sorting or filtering in server mode.
        let rows: Vec<Vec<String>> = (0..25)
            .rev()
            .map(|i| vec![format!("Customer {i}"), format!("{i}.00")])
            .collect();
        rsx! {
            DataTable {
                columns: columns(),
                rows,
                caption: "Customers",
                page_size: 25,
                total_rows: Some(20_000),
                on_query_change: |_q: TableQuery| {},
            }
        }
    }
    let html = render(app);
    assert!(html.contains("Showing 1–25 of 20000 rows"), "{html}");
    assert!(html.contains("Page 1 of 800"), "{html}");
    assert_eq!(
        html.matches("<tr").count(),
        26,
        "header row + the 25 rows given"
    );
    let first = html.find("Customer 24").expect("first row");
    let last = html.find("Customer 0<").expect("last row");
    assert!(first < last, "rows keep the server's order");
}

#[test]
fn a_loading_table_is_busy_and_says_so() {
    fn app() -> Element {
        rsx! {
            DataTable {
                columns: columns(),
                rows: vec![],
                caption: "Customers",
                total_rows: Some(0),
                loading: true,
            }
        }
    }
    let html = render(app);
    assert!(html.contains("aria-busy=\"true\""), "{html}");
    assert!(html.contains(">Loading<"), "{html}");
}

#[test]
fn table_query_offsets() {
    let q = TableQuery {
        page: 3,
        ..TableQuery::new(25)
    };
    assert_eq!(q.offset(), 75);
    assert_eq!(TableQuery::new(10).sort, None);
}

#[test]
fn added_fields_are_validated_with_the_form_and_can_be_removed() {
    fn app() -> Element {
        let form = use_form();
        let client = form.field_with("Acme", vec![required()]);
        let lines = use_hook(|| {
            vec![
                form.add_field("Design", vec![required()]),
                form.add_field("", vec![required()]),
            ]
        });
        let invalid_with_empty_line = !form.validate_all();
        form.remove_field(lines[1]);
        let valid_without_it = form.validate_all();
        let distinct = lines[0].ids().control != lines[1].ids().control
            && lines[0].ids().control != client.ids().control;
        rsx! {
            p { "{invalid_with_empty_line} {valid_without_it} {distinct}" }
            for line in lines {
                TextField { key: "{line.ids().control}", field: line, label: "Item" }
            }
        }
    }
    let html = render(app);
    assert!(html.contains("true true true"), "{html}");
    assert!(html.contains("for=\"dx-field-2\""), "{html}");
}
