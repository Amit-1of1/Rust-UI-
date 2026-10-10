use ui::components::data_table::*;
use ui::components::table::SortDirection::{Ascending, Descending};

fn cols() -> Vec<DataColumn> {
    vec![
        DataColumn::new("Name").sortable(),
        DataColumn::new("Pay").sortable().numeric(),
    ]
}

fn rows() -> Vec<Vec<String>> {
    [
        ("bob", "$1,200"),
        ("Alice", "$300"),
        ("carol", "n/a"),
        ("dave", "45%"),
    ]
    .iter()
    .map(|(a, b)| vec![a.to_string(), b.to_string()])
    .collect()
}

#[test]
fn text_sort_ignores_case_and_numbers_sort_numerically() {
    let by_name = table_view(&cols(), &rows(), "", Some((0, Ascending)), 0, 0);
    assert_eq!(by_name.rows, [1, 0, 2, 3], "Alice, bob, carol, dave");
    let by_pay = table_view(&cols(), &rows(), "", Some((1, Ascending)), 0, 0);
    assert_eq!(by_pay.rows, [3, 1, 0, 2], "45, 300, 1200, then n/a last");
}

#[test]
fn descending_reverses_numbers_but_unparsable_stay_last() {
    let v = table_view(&cols(), &rows(), "", Some((1, Descending)), 0, 0);
    assert_eq!(v.rows, [0, 1, 3, 2]);
}

#[test]
fn filter_needs_every_term_in_some_cell_ignoring_case() {
    let v = table_view(&cols(), &rows(), "ALI 300", None, 0, 0);
    assert_eq!(v.rows, [1]);
    assert_eq!(table_view(&cols(), &rows(), "zzz", None, 0, 0).matched, 0);
    assert_eq!(table_view(&cols(), &rows(), "  ", None, 0, 0).matched, 4);
}

#[test]
fn pagination_clamps_and_reports_pages() {
    let v = table_view(&cols(), &rows(), "", None, 0, 3);
    assert_eq!((v.rows.len(), v.pages, v.page, v.matched), (3, 2, 0, 4));
    let last = table_view(&cols(), &rows(), "", None, 99, 3);
    assert_eq!(
        (last.rows, last.page),
        (vec![3], 1),
        "out-of-range page clamps to the last"
    );
    let empty = table_view(&cols(), &rows(), "zzz", None, 5, 3);
    assert_eq!((empty.pages, empty.page, empty.rows.len()), (1, 0, 0));
}

#[test]
fn sorting_is_stable_and_none_keeps_original_order() {
    let same: Vec<Vec<String>> = (0..4)
        .map(|i| vec!["x".to_string(), i.to_string()])
        .collect();
    let v = table_view(&cols(), &same, "", Some((0, Ascending)), 0, 0);
    assert_eq!(v.rows, [0, 1, 2, 3]);
    let none = table_view(
        &cols(),
        &rows(),
        "",
        Some((0, ui::components::table::SortDirection::None)),
        0,
        0,
    );
    assert_eq!(none.rows, [0, 1, 2, 3]);
}
