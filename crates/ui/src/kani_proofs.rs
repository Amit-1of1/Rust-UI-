//! Kani proofs (bounded model checking of the real Rust code): `cargo kani -p atoll-ui`, run by
//! `scripts/kani.sh` and the "Kani" workflow. Unlike the tests, which try examples, Kani checks
//! every value of each `kani::any()` input: for the keyboard reducers here that means every
//! `usize`, where `tests/model_check.rs` enumerates sizes up to 24 × 6. Arithmetic overflow and
//! panics are checked everywhere along the way. docs/notes/kani.md says how to add one.
//!
//! This file is not in any component folder, so components copied into an app never carry it.

use crate::components::chart::{ChartCursor, ChartKey};
use crate::components::command::{command_nav, step_active};
use crate::components::pagination::page_items;
use dioxus::prelude::Key;

fn any_chart_key() -> ChartKey {
    match kani::any::<u8>() % 6 {
        0 => ChartKey::Next,
        1 => ChartKey::Previous,
        2 => ChartKey::NextSeries,
        3 => ChartKey::PreviousSeries,
        4 => ChartKey::First,
        _ => ChartKey::Last,
    }
}

/// From any cursor (even one left over from a bigger chart), any key keeps the chart's cursor on
/// a real mark, moves at most one step, and Home/End reach the ends. For every chart size.
#[kani::proof]
fn chart_cursor_stays_on_a_mark() {
    let from = ChartCursor {
        category: kani::any(),
        series: kani::any(),
    };
    let (categories, series): (usize, usize) = (kani::any(), kani::any());
    let key = any_chart_key();
    let to = from.apply(key, categories, series);
    if categories == 0 || series == 0 {
        assert!(to == ChartCursor::default());
        return;
    }
    assert!(to.category < categories && to.series < series);
    let c = from.category.min(categories - 1);
    let s = from.series.min(series - 1);
    assert!(to.category.abs_diff(c) <= 1 || matches!(key, ChartKey::First | ChartKey::Last));
    assert!(to.series.abs_diff(s) <= 1);
    match key {
        ChartKey::First => assert!(to.category == 0 && to.series == s),
        ChartKey::Last => assert!(to.category == categories - 1 && to.series == s),
        _ => {}
    }
}

/// The command palette's arrow keys wrap around a list of any length a `Vec` can have.
#[kani::proof]
fn command_palette_wraps_for_every_length() {
    let len: usize = kani::any();
    let current: usize = kani::any();
    // A Vec never holds more than isize::MAX items, and the palette keeps `current` in range.
    kani::assume(len <= isize::MAX as usize && current < len);
    let down = step_active(current, 1, len);
    let up = step_active(current, -1, len);
    assert!(down < len && up < len);
    assert!(down == if current + 1 == len { 0 } else { current + 1 });
    assert!(up == if current == 0 { len - 1 } else { current - 1 });
    // Home and End, and nothing else moves.
    if len > 0 {
        assert!(command_nav(&Key::Home, current, len) == Some(0));
        assert!(command_nav(&Key::End, current, len) == Some(len - 1));
    }
    assert!(command_nav(&Key::Enter, current, len).is_none());
}

/// `siblings` is a prop, so it can be anything: `page + siblings` used to overflow (a panic in
/// debug builds) for a huge value. Any page and any `siblings` now work. (The shape of the list
/// is proved exhaustively in tests/components/pagination.rs: walking the list symbolically makes
/// CBMC run out of memory.)
#[kani::proof]
#[kani::unwind(5)]
fn pagination_survives_any_siblings() {
    let pages: usize = kani::any();
    kani::assume(pages <= 3);
    let items = page_items(kani::any(), pages, kani::any());
    let siblings_huge = page_items(1, pages, usize::MAX);
    assert!(items.len() <= 3 && siblings_huge.len() == pages.max(1));
}
