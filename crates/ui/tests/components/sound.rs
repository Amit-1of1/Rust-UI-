use super::render;
use dioxus::prelude::*;
use std::cell::RefCell;
use ui::prelude::*;

thread_local! {
    static SEEN: RefCell<Vec<(bool, bool, bool, bool)>> = const { RefCell::new(Vec::new()) };
}

#[component]
fn Probe() -> Element {
    let s = use_sound();
    SEEN.with(|v| {
        v.borrow_mut().push((
            s.is_available(),
            s.is_on(),
            s.would_play(SoundEvent::Success),
            s.would_play(SoundEvent::Tap),
        ))
    });
    rsx! {}
}

fn seen() -> (bool, bool, bool, bool) {
    SEEN.with(|v| v.borrow_mut().pop().expect("probe rendered"))
}

#[test]
fn sound_is_off_until_the_person_turns_it_on() {
    render(|| rsx! { SoundProvider { taps: true, Probe {} } });
    assert_eq!(seen(), (true, false, false, false));
}

#[test]
fn sound_is_silent_without_a_provider() {
    render(|| rsx! { Probe {} });
    assert_eq!(seen(), (false, false, false, false));
}

#[test]
fn sound_toggle_is_an_unchecked_labelled_switch() {
    let html = render(|| rsx! { SoundProvider { SoundToggle {} } });
    assert!(html.contains("role=\"switch\""), "{html}");
    assert!(html.contains("aria-checked=false"), "{html}");
    assert!(html.contains(">Sounds</span>"), "{html}");
    let label_id = html
        .split("aria-labelledby=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("labelled");
    assert!(html.contains(&format!("id=\"{label_id}\"")), "{html}");
}

#[test]
fn sound_toggle_renders_nothing_without_a_provider() {
    let html = render(|| rsx! { SoundToggle {} });
    assert!(!html.contains("switch"), "{html}");
}

#[test]
fn sound_settings_list_each_sound_only_when_on() {
    let html = render(|| rsx! { SoundProvider { taps: true, SoundSettings {} } });
    assert!(html.contains("role=\"switch\""), "{html}");
    // Off: no per-sound rows yet.
    assert!(!html.contains("<select"), "{html}");
}

#[test]
fn sound_styles_have_names_that_round_trip() {
    for style in SoundStyle::ALL {
        assert_eq!(SoundStyle::from_name(style.name()), Some(style));
    }
    assert_eq!(SoundStyle::default(), SoundStyle::Ripple);
}

#[test]
fn sound_notes_stay_short_and_quiet() {
    for style in SoundStyle::ALL {
        for event in SoundEvent::ALL {
            let notes = event.notes(style);
            assert!(!notes.is_empty(), "{style:?} {event:?}");
            let end = notes.iter().map(|n| n.at + n.length).fold(0.0, f64::max);
            // Minor sounds follow every press: keep them short so they never sit on top of a
            // screen reader for long. The others stay under a second.
            let limit = if event.is_minor() { 0.35 } else { 1.0 };
            assert!(end <= limit, "{style:?} {event:?} lasts {end}s");
            for n in &notes {
                assert!(n.gain > 0.0 && n.gain <= 0.25, "{style:?} {event:?} gain {}", n.gain);
                assert!((100.0..=14_000.0).contains(&n.from_hz), "{style:?} {event:?}");
                assert!((100.0..=14_000.0).contains(&n.to_hz), "{style:?} {event:?}");
                assert!(n.glide <= n.length, "{style:?} {event:?}");
            }
        }
    }
}
