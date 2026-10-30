//! DateRangeField: a date range in a `use_form` field, as `"2026-10-15/2026-10-20"`.

use super::render;
use dioxus::prelude::*;
use ui::components::form::days_between;
use ui::prelude::*;

fn date(y: i32, m: u8, d: u8) -> Date {
    Date::from_calendar_date(y, Month::try_from(m).unwrap(), d).unwrap()
}

#[test]
fn iso_date_ranges_round_trip() {
    let range = DateRange::new(date(2026, 10, 15), date(2026, 10, 20));
    assert_eq!(format_iso_date_range(range), "2026-10-15/2026-10-20");
    assert_eq!(parse_iso_date_range("2026-10-15/2026-10-20"), Some(range));
    assert_eq!(
        parse_iso_date_range("2026-10-20/2026-10-15"),
        Some(range),
        "reversed dates still give start to end"
    );
    assert_eq!(format_iso_date(date(987, 1, 2)), "0987-01-02");
    for bad in [
        "",
        "2026-10-15",
        "2026-10-15/",
        "2026-02-30/2026-03-01",
        "2026-10-15/2026-10-20/2026-10-21",
        "2026-1-15/2026-10-20",
    ] {
        assert_eq!(parse_iso_date_range(bad), None, "{bad}");
    }
}

#[test]
fn days_between_counts_nights() {
    let rule = days_between(Some(2), Some(14), "2 to 14 nights.");
    assert_eq!(rule("2026-10-15/2026-10-17"), Ok(()));
    assert_eq!(rule("2026-10-15/2026-10-29"), Ok(()));
    assert!(rule("2026-10-15/2026-10-16").is_err());
    assert!(rule("2026-10-15/2026-10-30").is_err());
    assert_eq!(rule(""), Ok(()), "empty is for required()");
    let open_ended = days_between(Some(1), None, "At least a night.");
    assert!(open_ended("2026-10-15/2026-10-15").is_err());
    assert_eq!(open_ended("2026-01-01/2027-01-01"), Ok(()));
}

#[test]
fn the_group_is_labelled_and_described() {
    fn app() -> Element {
        let form = use_form();
        let field = form.field_with("2026-10-15/2026-10-20", vec![]);
        rsx! {
            DateRangeField { field, label: "Stay", description: "Two nights or more." }
        }
    }
    let html = render(app);
    let ids = ["-label\"", "aria-labelledby=\"dx-field-", "aria-describedby=\"dx-field-"];
    for id in ids {
        assert!(html.contains(id), "missing {id} in {html}");
    }
    assert!(html.contains("role=\"group\""), "{html}");
    assert!(html.contains("tabindex=\"-1\""), "focus target for a failed submit: {html}");
    assert!(!html.contains("aria-label=\"Date Range\""), "named by the label: {html}");
}
