//! `table_view` sorts with precomputed keys; this checks it against the plain definition
//! (lower-case text order, numbers numerically with unparsable ones last, stable) on random
//! tables.

use proptest::prelude::*;
use std::cmp::Ordering;
use ui::components::data_table::*;
use ui::components::table::SortDirection;

fn number(cell: &str) -> Option<f64> {
    let cleaned: String = cell
        .chars()
        .filter(|c| !matches!(c, '$' | '€' | '£' | ',' | '%' | ' '))
        .collect();
    cleaned.parse().ok()
}

fn reference(rows: &[Vec<String>], numeric: bool, direction: SortDirection) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..rows.len()).collect();
    let cell = |i: usize| rows[i][0].as_str();
    indices.sort_by(|&a, &b| {
        let ord = if numeric {
            match (number(cell(a)), number(cell(b))) {
                (Some(x), Some(y)) => x.total_cmp(&y),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
        } else {
            cell(a).to_lowercase().cmp(&cell(b).to_lowercase())
        };
        if numeric && (number(cell(a)).is_none() || number(cell(b)).is_none()) {
            ord
        } else if direction == SortDirection::Descending {
            ord.reverse()
        } else {
            ord
        }
    });
    indices
}

fn cell() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-cA-C]{0,3}",
        (-50i32..50).prop_map(|n| n.to_string()),
        (0u32..5000).prop_map(|n| format!("${n}")),
        Just("n/a".to_string()),
        Just("1.5".to_string()),
    ]
}

proptest! {
    #[test]
    fn sorting_matches_the_definition(
        cells in prop::collection::vec(cell(), 0..40),
        numeric in any::<bool>(),
        descending in any::<bool>(),
    ) {
        let rows: Vec<Vec<String>> = cells.into_iter().map(|c| vec![c]).collect();
        let column = DataColumn::new("A").sortable();
        let columns = vec![if numeric { column.numeric() } else { column }];
        let direction = if descending { SortDirection::Descending } else { SortDirection::Ascending };
        let view = table_view(&columns, &rows, "", Some((0, direction)), 0, 0);
        prop_assert_eq!(view.rows, reference(&rows, numeric, direction));
    }
}
