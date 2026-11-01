//! Data table v2: column visibility, column resize, CSV export and virtualised rows.

use super::render;
use dioxus::prelude::*;
use std::time::Instant;
use ui::components::data_table::*;
use ui::components::table::SortDirection;

fn people() -> (Vec<DataColumn>, Vec<Vec<String>>) {
    let columns = vec![
        DataColumn::new("Name").sortable().always_visible(),
        DataColumn::new("Notes"),
        DataColumn::new("Balance").numeric(),
    ];
    let rows = [
        ("Ada", "likes \"maths\", tea", "-12.50"),
        ("Grace", "=HYPERLINK(\"x\")", "$1,200"),
        ("Alan", "line one\nline two", "+3"),
        ("Linus", "@admin", "n/a"),
    ]
    .iter()
    .map(|(a, b, c)| vec![a.to_string(), b.to_string(), c.to_string()])
    .collect();
    (columns, rows)
}

#[test]
fn csv_quotes_fields_and_ends_lines_with_crlf() {
    let (columns, rows) = people();
    let csv = table_csv(&columns, &rows, &[0, 2]);
    assert_eq!(
        csv,
        "Name,Notes,Balance\r\nAda,\"likes \"\"maths\"\", tea\",-12.50\r\nAlan,\"line one\nline two\",+3\r\n"
    );
}

#[test]
fn csv_defuses_formulas_but_keeps_signed_numbers() {
    let (columns, rows) = people();
    let csv = table_csv(&columns, &rows, &[1, 3]);
    let lines: Vec<&str> = csv.split("\r\n").collect();
    assert_eq!(lines[1], r#"Grace,"'=HYPERLINK(""x"")","$1,200""#);
    assert_eq!(lines[2], "Linus,'@admin,n/a");
    // A text column that happens to hold "-5" is still defused: it isn't a number column.
    let text = vec![DataColumn::new("Code")];
    assert_eq!(table_csv(&text, &[vec!["-5".into()]], &[0]), "Code\r\n'-5\r\n");
}

#[test]
fn csv_leaves_out_hidden_columns_and_follows_the_given_order() {
    let (mut columns, rows) = people();
    columns[1].hidden = true;
    let csv = table_csv(&columns, &rows, &[3, 0]);
    assert_eq!(csv, "Name,Balance\r\nLinus,n/a\r\nAda,-12.50\r\n");
}

#[test]
fn the_filter_does_not_search_hidden_columns() {
    let (columns, rows) = people();
    assert_eq!(table_view(&columns, &rows, "admin", None, 0, 0).rows, [3]);
    let hidden: Vec<DataColumn> = columns
        .iter()
        .cloned()
        .map(|c| if c.header == "Notes" { c.hidden() } else { c })
        .collect();
    assert_eq!(table_view(&hidden, &rows, "admin", None, 0, 0).matched, 0);
    // Sorting by a hidden column still works.
    let v = table_view(&hidden, &rows, "", Some((0, SortDirection::Ascending)), 0, 0);
    assert_eq!(v.rows, [0, 2, 1, 3]);
}

#[test]
fn the_virtual_window_covers_the_viewport_plus_overscan() {
    // 400 px of 40 px rows at the top: rows 0..11 (10 in view, one partly) and 5 more.
    assert_eq!(virtual_window(0.0, 400.0, 40.0, 1000, 5), 0..16);
    // Scrolled to row 100: five above, the view, five below.
    assert_eq!(virtual_window(4000.0, 400.0, 40.0, 1000, 5), 95..116);
    // Near the end it stops at the last row.
    assert_eq!(virtual_window(39_800.0, 400.0, 40.0, 1000, 5), 990..1000);
    // Scrolled past the end (rows were filtered away): an empty or short window, no panic.
    assert_eq!(virtual_window(99_999.0, 400.0, 40.0, 10, 5), 10..10);
    assert_eq!(virtual_window(0.0, 400.0, 40.0, 0, 5), 0..0);
    assert_eq!(virtual_window(0.0, 400.0, 0.0, 10, 5), 0..0);
}

#[test]
fn hidden_columns_are_left_out_and_the_menu_lists_hideable_ones() {
    let html = render(|| {
        let (columns, rows) = people();
        let columns = columns
            .into_iter()
            .map(|c| if c.header == "Notes" { c.hidden() } else { c })
            .collect::<Vec<_>>();
        rsx! {
            DataTable { columns, rows, caption: "People", column_menu: true, exportable: true }
        }
    });
    assert!(!html.contains(">Notes</th>"), "{html}");
    assert!(!html.contains("@admin"), "hidden cells are not rendered: {html}");
    assert!(html.contains(">Columns<"), "{html}");
    assert!(html.contains(">Export CSV<"), "{html}");
}

#[test]
fn resize_handles_are_named_separators() {
    let html = render(|| {
        let (columns, rows) = people();
        let columns: Vec<DataColumn> = columns.into_iter().map(|c| c.width(120.0)).collect();
        rsx! {
            DataTable { columns, rows, caption: "People", resizable_columns: true, filterable: false }
        }
    });
    assert!(html.contains("role=\"separator\""), "{html}");
    assert!(html.contains("aria-label=\"Resize Name\""), "{html}");
    assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
    assert!(html.contains("aria-valuenow=\"120\""), "{html}");
    assert!(html.contains("aria-valuemin=\"48\""), "{html}");
    // Every column has a width, so the layout is fixed at their sum.
    assert!(html.contains("table-layout: fixed; width: 360px;"), "{html}");
    assert!(html.contains("<col style=\"width: 120px;\""), "{html}");
}

const ROWS: usize = 20_000;

fn many_rows() -> (Vec<DataColumn>, Vec<Vec<String>>) {
    let columns = vec![
        DataColumn::new("Order").sortable(),
        DataColumn::new("Customer"),
        DataColumn::new("Total").numeric(),
    ];
    let rows = (0..ROWS)
        .map(|i| {
            vec![
                format!("#{:05}", i + 1),
                format!("Customer {}", i % 977),
                format!("${}.{:02}", (i * 37) % 5000, i % 100),
            ]
        })
        .collect();
    (columns, rows)
}

#[test]
fn a_virtual_table_renders_only_the_rows_in_view() {
    let html = render(|| {
        let (columns, rows) = many_rows();
        rsx! {
            DataTable { columns, rows, caption: "Orders", virtual_height: 400.0, row_height: 40.0 }
        }
    });
    let body_rows = html.matches("aria-rowindex=").count() - 1; // minus the header row
    assert!((10..=30).contains(&body_rows), "{body_rows} rows rendered");
    assert!(html.contains("aria-rowcount=\"20001\""), "the table says how many rows it has");
    assert!(html.contains("role=\"region\""), "the scroll box is a named, focusable region");
    assert!(html.contains("tabindex=\"0\""));
    assert!(html.contains("max-height: 400px;"));
    // The spacer below stands in for the rest: 20,000 rows of 40 px, minus those rendered.
    let below = (ROWS - body_rows) * 40;
    assert!(html.contains(&format!("height: {below}px;")), "{below}");
    assert!(html.contains("20000 rows"), "the status counts every row");
}

/// The perf check: a 20,000-row table, rendered whole and virtualised. Virtualised it renders
/// a few dozen rows, so it must be many times faster. Timings vary between machines, so the
/// bar is a ratio and is loose; the numbers are printed for `--nocapture`.
#[test]
fn virtualising_a_long_list_is_much_faster_than_rendering_it_all() {
    fn time(app: fn() -> Element) -> (f64, usize) {
        let start = Instant::now();
        let html = render(app);
        (start.elapsed().as_secs_f64() * 1000.0, html.matches("<tr").count())
    }
    let (full_ms, full_rows) = time(|| {
        let (columns, rows) = many_rows();
        rsx! { DataTable { columns, rows, caption: "Orders", page_size: 0 } }
    });
    let (virtual_ms, virtual_rows) = time(|| {
        let (columns, rows) = many_rows();
        rsx! { DataTable { columns, rows, caption: "Orders", virtual_height: 400.0 } }
    });
    println!(
        "{ROWS} rows: all {full_ms:.0} ms ({full_rows} <tr>), virtual {virtual_ms:.0} ms ({virtual_rows} <tr>)"
    );
    assert!(full_rows > ROWS);
    assert!(virtual_rows < 40);
    assert!(
        virtual_ms * 4.0 < full_ms,
        "virtual {virtual_ms:.0} ms vs all {full_ms:.0} ms"
    );
}
