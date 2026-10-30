//! TimePicker `min`/`max`: the arrow keys skip what is outside the range, a typed time outside
//! it is kept but reported, and a range can wrap past midnight.

use super::render;
use dioxus::prelude::*;
use ui::components::form::time_between;
use ui::prelude::*;

fn hm(h: u8, m: u8) -> Time {
    Time::from_hms(h, m, 0).unwrap()
}

fn office() -> TimeState {
    TimeState::new(HourCycle::H24, false, 15).with_range(Some(hm(9, 0)), Some(hm(17, 0)))
}

#[test]
fn hours_step_inside_the_range_and_wrap_within_it() {
    let mut s = office();
    s.apply(TimeKey::Up);
    assert_eq!(s.hour, Some(9), "Up on empty starts at the first allowed hour");
    s.apply(TimeKey::Down);
    assert_eq!(s.hour, Some(17), "Down from the first wraps to the last allowed hour");
    s.apply(TimeKey::Up);
    assert_eq!(s.hour, Some(9), "Up from the last wraps to the first");
    s.apply(TimeKey::End);
    assert_eq!(s.hour, Some(17));
    s.apply(TimeKey::Home);
    assert_eq!(s.hour, Some(9));
}

#[test]
fn minutes_follow_the_hour_already_chosen() {
    let mut s = office();
    s.set_time(Some(hm(17, 0)));
    s.apply(TimeKey::Focus(1));
    s.apply(TimeKey::Up);
    assert_eq!(s.minute, Some(0), "17:15 is past the end: only :00 is left");
    s.apply(TimeKey::End);
    assert_eq!(s.minute, Some(0));
    let mut s = TimeState::new(HourCycle::H24, false, 15)
        .with_range(Some(hm(9, 30)), Some(hm(17, 0)));
    s.set_time(Some(hm(9, 45)));
    s.apply(TimeKey::Focus(1));
    s.apply(TimeKey::Down);
    assert_eq!(s.minute, Some(30), "9:15 is before the start");
    s.apply(TimeKey::Down);
    assert_eq!(s.minute, Some(45), "wraps to the last allowed quarter");
    s.apply(TimeKey::Home);
    assert_eq!(s.minute, Some(30));
}

#[test]
fn typing_is_free_and_out_of_range_is_reported() {
    let mut s = office();
    for c in "1830".chars() {
        s.apply(TimeKey::Digit(c));
    }
    assert_eq!(s.time(), Some(hm(18, 30)), "typing isn't blocked");
    assert!(s.out_of_range());
    s.set_time(Some(hm(17, 0)));
    assert!(!s.out_of_range(), "max is inclusive");
    s.set_time(None);
    assert!(!s.out_of_range(), "an empty picker is not out of range");
}

#[test]
fn a_range_can_run_past_midnight() {
    let mut s = TimeState::new(HourCycle::H24, false, 1)
        .with_range(Some(hm(22, 0)), Some(hm(2, 0)));
    assert!(s.in_range(hm(23, 30)) && s.in_range(hm(1, 59)) && s.in_range(hm(2, 0)));
    assert!(!s.in_range(hm(12, 0)) && !s.in_range(hm(2, 1)));
    s.apply(TimeKey::Up);
    assert_eq!(s.hour, Some(0), "midnight is the lowest allowed hour");
    s.apply(TimeKey::Up);
    s.apply(TimeKey::Up);
    assert_eq!(s.hour, Some(2));
    s.apply(TimeKey::Up);
    assert_eq!(s.hour, Some(22), "jumps over the day");
}

#[test]
fn twelve_hour_hours_and_periods_respect_the_range() {
    let mut s = TimeState::new(HourCycle::H12, false, 1)
        .with_range(Some(hm(13, 0)), Some(hm(15, 0)));
    s.apply(TimeKey::Up);
    assert_eq!(s.hour, Some(1), "1 PM can be reached; 12 can't (AM or PM)");
    s.apply(TimeKey::Focus(2));
    s.apply(TimeKey::Up);
    assert_eq!(s.period, Some(Period::Pm), "AM has no allowed time");
    s.apply(TimeKey::Focus(0));
    s.apply(TimeKey::End);
    assert_eq!(s.hour, Some(3), "with PM set, 3 PM is the last hour");
}

#[test]
fn no_reachable_value_falls_back_to_plain_steps() {
    // 10:05 to 10:10 holds no quarter-hour: minutes step as if there were no range.
    let mut s = TimeState::new(HourCycle::H24, false, 15)
        .with_range(Some(hm(10, 5)), Some(hm(10, 10)));
    s.set_time(Some(hm(10, 0)));
    s.apply(TimeKey::Focus(1));
    s.apply(TimeKey::Up);
    assert_eq!(s.minute, Some(15));
}

#[test]
fn the_time_between_rule_checks_iso_values() {
    let rule = time_between(Some(hm(9, 0)), Some(hm(17, 0)), "Office hours only.");
    assert_eq!(rule("09:00"), Ok(()));
    assert_eq!(rule("17:00"), Ok(()));
    assert_eq!(rule("18:00"), Err("Office hours only.".to_string()));
    assert_eq!(rule(""), Ok(()), "empty is for required()");
    let night = time_between(Some(hm(22, 0)), Some(hm(2, 0)), "Night only.");
    assert_eq!(night("23:15"), Ok(()));
    assert!(night("12:00").is_err());
}

#[test]
fn an_out_of_range_time_renders_invalid() {
    let html = render(|| {
        rsx! {
            TimePicker {
                value: Time::from_hms(18, 0, 0).ok(),
                min: Time::from_hms(9, 0, 0).ok(),
                max: Time::from_hms(17, 0, 0).ok(),
            }
        }
    });
    assert!(html.contains("data-invalid=\"true\""), "{html}");
    assert!(html.contains("aria-invalid=\"true\""), "{html}");
    let html = render(|| {
        rsx! {
            TimePicker {
                value: Time::from_hms(10, 0, 0).ok(),
                min: Time::from_hms(9, 0, 0).ok(),
                max: Time::from_hms(17, 0, 0).ok(),
            }
        }
    });
    assert!(!html.contains("aria-invalid"), "{html}");
}

#[test]
fn time_field_passes_the_range_to_its_picker() {
    fn app() -> Element {
        let form = use_form();
        let field = form.field_with("18:30", vec![]);
        rsx! {
            TimeField {
                field,
                label: "Start",
                min: Time::from_hms(9, 0, 0).ok(),
                max: Time::from_hms(17, 0, 0).ok(),
            }
        }
    }
    let html = render(app);
    assert!(html.contains("aria-invalid=\"true\""), "18:30 shows invalid: {html}");
}
