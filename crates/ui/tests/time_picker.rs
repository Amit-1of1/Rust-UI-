//! Keyboard and value logic of the time picker (`TimeState`) and the ISO helpers the form
//! fields use. The browser side is tests/e2e/time_picker.mjs.

use proptest::prelude::*;
use ui::components::date_time_picker::{format_iso_date_time, parse_iso_date_time};
use ui::components::time_picker::{
    format_iso_time, format_time, parse_iso_time, HourCycle, Period, Time, TimeKey, TimeOutcome,
    TimeSegment, TimeState,
};
use ui::components::ui_strings::UiStrings;

fn hm(h: u8, m: u8) -> Time {
    Time::from_hms(h, m, 0).unwrap()
}

fn typed(state: &mut TimeState, keys: &str) {
    for c in keys.chars() {
        let key = match c {
            'a' => TimeKey::SetPeriod(Period::Am),
            'p' => TimeKey::SetPeriod(Period::Pm),
            d => TimeKey::Digit(d),
        };
        state.apply(key);
    }
}

#[test]
fn typing_fills_segments_and_moves_on() {
    let mut s = TimeState::new(HourCycle::H24, false, 1);
    typed(&mut s, "1");
    assert_eq!((s.hour, s.focus), (Some(1), 0), "1 could still become 1x");
    typed(&mut s, "4");
    assert_eq!((s.hour, s.focus), (Some(14), 1), "two digits move on");
    typed(&mut s, "7");
    assert_eq!(
        (s.minute, s.focus),
        (Some(7), 1),
        "7 can't start a minute > 59, stays on the last segment"
    );
    assert_eq!(s.time(), Some(hm(14, 7)));
}

#[test]
fn a_digit_that_cannot_lead_moves_on_at_once() {
    let mut s = TimeState::new(HourCycle::H24, false, 1);
    typed(&mut s, "3");
    assert_eq!((s.hour, s.focus), (Some(3), 1), "30+ is not an hour");
    let mut s = TimeState::new(HourCycle::H12, false, 1);
    typed(&mut s, "2");
    assert_eq!((s.hour, s.focus), (Some(2), 1), "20+ is not a 12-hour hour");
}

#[test]
fn leading_zero_waits_for_the_second_digit() {
    let mut s = TimeState::new(HourCycle::H24, false, 1);
    typed(&mut s, "0");
    assert_eq!((s.hour, s.focus), (Some(0), 0));
    typed(&mut s, "9");
    assert_eq!(
        (s.hour, s.focus),
        (Some(9), 1),
        "09 moves on (booking GAPS #18)"
    );
}

#[test]
fn an_overflowing_second_digit_starts_again() {
    let mut s = TimeState::new(HourCycle::H24, false, 1);
    s.apply(TimeKey::Focus(1));
    typed(&mut s, "5");
    assert_eq!((s.minute, s.focus), (Some(5), 1));
    s.apply(TimeKey::Focus(0));
    typed(&mut s, "2");
    typed(&mut s, "7");
    assert_eq!(
        s.hour,
        Some(7),
        "27 is not an hour: the 7 starts a new entry"
    );
}

#[test]
fn twelve_hour_needs_am_or_pm() {
    let mut s = TimeState::new(HourCycle::H12, false, 1);
    typed(&mut s, "1230");
    assert_eq!(s.focus, 2, "on the AM/PM segment");
    assert_eq!(s.time(), None);
    typed(&mut s, "a");
    assert_eq!(s.time(), Some(hm(0, 30)), "12:30 AM is just after midnight");
    typed(&mut s, "p");
    assert_eq!(s.time(), Some(hm(12, 30)), "12:30 PM is just after noon");
    s.set_time(Some(hm(17, 5)));
    assert_eq!((s.hour, s.period), (Some(5), Some(Period::Pm)));
    s.set_time(Some(hm(0, 0)));
    assert_eq!((s.hour, s.period), (Some(12), Some(Period::Am)));
}

#[test]
fn arrows_wrap_and_respect_the_minute_step() {
    let mut s = TimeState::new(HourCycle::H24, false, 15);
    assert_eq!(s.apply(TimeKey::Up), TimeOutcome::Changed);
    assert_eq!(
        s.hour,
        Some(0),
        "Up on an empty segment starts at the lowest value"
    );
    s.apply(TimeKey::Down);
    assert_eq!(s.hour, Some(23), "wraps");
    s.apply(TimeKey::Focus(1));
    s.apply(TimeKey::Down);
    assert_eq!(
        s.minute,
        Some(45),
        "Down on empty starts at the highest step"
    );
    s.apply(TimeKey::Up);
    assert_eq!(s.minute, Some(0), "wraps past 59");
    typed(&mut s, "07");
    s.apply(TimeKey::Up);
    assert_eq!(s.minute, Some(15), "snaps up to the next step");
    typed(&mut s, "07");
    s.apply(TimeKey::Down);
    assert_eq!(s.minute, Some(0), "snaps down to the step below");
    s.apply(TimeKey::End);
    assert_eq!(s.minute, Some(45));
    s.apply(TimeKey::Home);
    assert_eq!(s.minute, Some(0));
}

#[test]
fn page_keys_take_bigger_steps_on_minutes() {
    let mut s = TimeState::new(HourCycle::H24, true, 1);
    s.apply(TimeKey::Focus(2));
    s.apply(TimeKey::PageUp);
    s.apply(TimeKey::PageUp);
    assert_eq!(s.second, Some(10));
    s.apply(TimeKey::PageDown);
    s.apply(TimeKey::PageDown);
    assert_eq!(s.second, Some(50), "wraps to the top step");
}

#[test]
fn backspace_and_delete_clear_and_go_back() {
    let mut s = TimeState::new(HourCycle::H24, false, 1);
    s.set_time(Some(hm(8, 45)));
    s.apply(TimeKey::Focus(1));
    s.apply(TimeKey::Delete);
    assert_eq!((s.minute, s.time()), (None, None));
    assert_eq!(
        s.apply(TimeKey::Backspace),
        TimeOutcome::Moved,
        "empty segment: go back"
    );
    assert_eq!(s.focus, 0);
    typed(&mut s, "1");
    s.apply(TimeKey::Backspace);
    assert_eq!(s.hour, None, "Backspace undoes the digit just typed");
}

#[test]
fn left_and_right_move_within_the_segments() {
    let mut s = TimeState::new(HourCycle::H12, true, 1);
    assert_eq!(s.segments().len(), 4);
    assert_eq!(s.apply(TimeKey::Left), TimeOutcome::Ignored);
    for i in 1..4 {
        assert_eq!(s.apply(TimeKey::Right), TimeOutcome::Moved);
        assert_eq!(s.focus, i);
    }
    assert_eq!(s.apply(TimeKey::Right), TimeOutcome::Ignored);
    assert_eq!(s.segments()[3], TimeSegment::Period);
    assert_eq!(
        s.apply(TimeKey::Digit('5')),
        TimeOutcome::Ignored,
        "no digits in AM/PM"
    );
}

#[test]
fn iso_time_round_trips() {
    assert_eq!(format_iso_time(hm(9, 5), false), "09:05");
    assert_eq!(
        format_iso_time(Time::from_hms(23, 59, 7).unwrap(), true),
        "23:59:07"
    );
    assert_eq!(parse_iso_time("09:05"), Some(hm(9, 5)));
    assert_eq!(parse_iso_time("23:59:07"), Time::from_hms(23, 59, 7).ok());
    for bad in [
        "",
        "9:05",
        "24:00",
        "12:60",
        "12:00:00:00",
        "ab:cd",
        "12-00",
    ] {
        assert_eq!(parse_iso_time(bad), None, "{bad}");
    }
}

#[test]
fn iso_date_time_round_trips() {
    let v = parse_iso_date_time("2026-10-15T14:05").unwrap();
    assert_eq!(format_iso_date_time(v, false), "2026-10-15T14:05");
    assert_eq!(
        parse_iso_date_time("2026-10-15 14:05:09").map(|v| format_iso_date_time(v, true)),
        Some("2026-10-15T14:05:09".into())
    );
    for bad in [
        "",
        "2026-10-15",
        "2026-02-30T10:00",
        "2026-1-15T10:00",
        "2026-10-15T25:00",
    ] {
        assert_eq!(parse_iso_date_time(bad), None, "{bad}");
    }
}

#[test]
fn display_format_follows_the_hour_cycle() {
    let t = UiStrings::default();
    assert_eq!(format_time(hm(14, 5), HourCycle::H24, false, &t), "14:05");
    assert_eq!(format_time(hm(14, 5), HourCycle::H12, false, &t), "2:05 PM");
    assert_eq!(
        format_time(hm(0, 0), HourCycle::H12, true, &t),
        "12:00:00 AM"
    );
}

fn key() -> impl Strategy<Value = TimeKey> {
    prop_oneof![
        (0usize..5).prop_map(TimeKey::Focus),
        prop::char::range('0', '9').prop_map(TimeKey::Digit),
        Just(TimeKey::SetPeriod(Period::Am)),
        Just(TimeKey::SetPeriod(Period::Pm)),
        Just(TimeKey::Up),
        Just(TimeKey::Down),
        Just(TimeKey::PageUp),
        Just(TimeKey::PageDown),
        Just(TimeKey::Home),
        Just(TimeKey::End),
        Just(TimeKey::Backspace),
        Just(TimeKey::Delete),
        Just(TimeKey::Left),
        Just(TimeKey::Right),
    ]
}

proptest! {
    /// Whatever is pressed, every segment stays in its range, focus stays on a segment, and
    /// the time (once complete) matches the segments.
    #[test]
    fn any_key_sequence_keeps_segments_valid(
        twelve in any::<bool>(),
        seconds in any::<bool>(),
        step in 1u8..=30,
        keys in prop::collection::vec(key(), 0..60),
    ) {
        let cycle = if twelve { HourCycle::H12 } else { HourCycle::H24 };
        let mut s = TimeState::new(cycle, seconds, step);
        for k in keys {
            s.apply(k);
            prop_assert!(s.focus < s.segments().len());
            for seg in s.segments() {
                let (min, max) = s.range(seg);
                if let Some(v) = s.value(seg) {
                    prop_assert!(v >= min && v <= max, "{seg:?} = {v}");
                }
            }
            if let Some(time) = s.time() {
                let mut again = TimeState::new(cycle, seconds, step);
                again.set_time(Some(time));
                prop_assert_eq!(again.time(), Some(time));
                prop_assert_eq!(again.minute, s.minute);
            }
        }
    }
}
