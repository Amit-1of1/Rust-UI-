//! The ISO date text a date picker keeps in a form: every date there is reads back as itself,
//! and anything read back is written exactly as it was typed (nothing lenient, such as signs).

use proptest::prelude::*;
use time::{Date, Duration};
use ui::components::date_picker::{format_iso_date, parse_iso_date};

/// Every day from 0000-01-01 to 9999-12-31 (3,652,425 dates).
#[test]
fn every_date_round_trips() {
    let mut d = Date::from_calendar_date(0, time::Month::January, 1).unwrap();
    let last = Date::from_calendar_date(9999, time::Month::December, 31).unwrap();
    let mut n = 0u32;
    loop {
        let text = format_iso_date(d);
        assert_eq!(parse_iso_date(&text), Some(d), "{text}");
        n += 1;
        if d == last {
            break;
        }
        d += Duration::days(1);
    }
    assert_eq!(n, 3_652_425);
}

#[test]
fn signs_and_other_near_misses_are_refused() {
    for text in ["2026-+1-+5", "+202-10-15", "2026-1-05", "2026-01-5", "2026-02-30", "2026-13-01", "2026-00-10", " 2026-01-05 x", "2026-01-05-"] {
        assert_eq!(parse_iso_date(text), None, "{text}");
    }
    assert!(parse_iso_date(" 2026-01-05 ").is_some());
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 20_000, ..ProptestConfig::default() })]

    /// Whatever text comes in, if it is read as a date, it was that date's canonical form.
    #[test]
    fn whatever_is_read_was_canonical(text in "[0-9+ -]{8,12}") {
        if let Some(d) = parse_iso_date(&text) {
            prop_assert_eq!(format_iso_date(d), text.trim());
        }
    }
}
