//! Keyboard logic of the components we own, driven by random key sequences.
//!
//! The command palette and OTP input keep their keyboard behaviour in pure functions
//! (`command_nav`, `OtpState::apply`), so proptest can throw thousands of key sequences
//! at them and check the invariants after every step. The Zag.js idea: the state machine
//! is the component, the DOM is a view of it. The browser side is tests/e2e.

use dioxus::prelude::Key;
use proptest::prelude::*;
use ui::components::command::command_nav;
use ui::components::input_otp::{OtpEvent, OtpOutcome, OtpState};

// ----- command palette ---------------------------------------------------------------

#[derive(Clone, Debug)]
enum PaletteStep {
    Key(Key),
    /// Typing filters the list to `len` rows and resets the highlight, as the component does.
    Type(usize),
}

fn palette_step() -> impl Strategy<Value = PaletteStep> {
    prop_oneof![
        4 => prop_oneof![
            Just(Key::ArrowDown),
            Just(Key::ArrowUp),
            Just(Key::Home),
            Just(Key::End),
            Just(Key::Tab),
            Just(Key::Enter),
        ]
        .prop_map(PaletteStep::Key),
        1 => (0usize..8).prop_map(PaletteStep::Type),
    ]
}

proptest! {
    #[test]
    fn palette_highlight_stays_on_a_row(start_len in 0usize..8, steps in prop::collection::vec(palette_step(), 0..60)) {
        let (mut active, mut len) = (0usize, start_len);
        for step in steps {
            match step {
                PaletteStep::Type(n) => (active, len) = (0, n),
                PaletteStep::Key(key) => {
                    let before = active;
                    match command_nav(&key, active, len) {
                        Some(next) => active = next,
                        None => prop_assert!(!matches!(key, Key::ArrowDown | Key::ArrowUp | Key::Home | Key::End)),
                    }
                    if len > 0 {
                        prop_assert!(active < len, "{key:?}: {before} -> {active} of {len}");
                    } else {
                        prop_assert_eq!(active, 0);
                    }
                }
            }
        }
    }

    #[test]
    fn palette_down_then_up_returns(len in 1usize..10, start in 0usize..10) {
        let start = start % len;
        let down = command_nav(&Key::ArrowDown, start, len).unwrap();
        prop_assert_eq!(command_nav(&Key::ArrowUp, down, len), Some(start));
    }
}

#[test]
fn palette_wraps_and_jumps() {
    assert_eq!(command_nav(&Key::ArrowDown, 2, 3), Some(0));
    assert_eq!(command_nav(&Key::ArrowUp, 0, 3), Some(2));
    assert_eq!(command_nav(&Key::Home, 2, 3), Some(0));
    assert_eq!(command_nav(&Key::End, 0, 3), Some(2));
    assert_eq!(command_nav(&Key::Enter, 1, 3), None);
}

// ----- one-time code -----------------------------------------------------------------

fn otp_event(len: usize) -> impl Strategy<Value = OtpEvent> {
    // Slots past the end too: the reducer must ignore them, not panic.
    let slot = 0..len + 2;
    prop_oneof![
        4 => (slot.clone(), "[0-9a-z ]{0,8}").prop_map(|(slot, text)| OtpEvent::Input { slot, text }),
        2 => slot.clone().prop_map(OtpEvent::Backspace),
        1 => slot.clone().prop_map(OtpEvent::Left),
        1 => slot.prop_map(OtpEvent::Right),
        1 => Just(OtpEvent::Home),
        1 => Just(OtpEvent::End),
    ]
}

fn otp_run() -> impl Strategy<Value = (usize, bool, Vec<OtpEvent>)> {
    (1usize..9, any::<bool>()).prop_flat_map(|(len, numeric)| {
        (
            Just(len),
            Just(numeric),
            prop::collection::vec(otp_event(len), 0..40),
        )
    })
}

proptest! {
    #[test]
    fn otp_invariants_hold_after_every_event((len, numeric, events) in otp_run()) {
        let mut s = OtpState::new(len, numeric);
        for event in events {
            let before = s.clone();
            let outcome = s.apply(event.clone());
            // The number of boxes never changes and focus is always on one of them.
            prop_assert_eq!(s.chars.len(), len);
            prop_assert!(s.focus < len, "{event:?}: focus {}", s.focus);
            // Only accepted characters get in.
            prop_assert!(s.chars.iter().flatten().all(|&c| s.accepts(c)), "{:?}", s.chars);
            // The outcome tells the truth about what changed.
            match outcome {
                OtpOutcome::Ignored => prop_assert_eq!(&s, &before),
                OtpOutcome::Moved => prop_assert_eq!(&s.chars, &before.chars),
                OtpOutcome::Changed => {}
            }
            prop_assert_eq!(s.is_complete(), s.code().chars().count() == len);
        }
    }

    #[test]
    fn otp_paste_fills_from_the_box_onward(len in 1usize..9, slot in 0usize..9, digits in "[0-9]{1,12}") {
        let slot = slot % len;
        let mut s = OtpState::new(len, true);
        s.apply(OtpEvent::Input { slot, text: digits.clone() });
        let filled = digits.len().min(len - slot);
        let expect: String = digits.chars().take(filled).collect();
        prop_assert_eq!(s.code(), expect);
        prop_assert!(s.chars[..slot].iter().all(Option::is_none));
        prop_assert_eq!(s.focus, (slot + filled).min(len - 1));
    }

    #[test]
    fn otp_backspace_on_an_empty_box_clears_the_one_before(len in 2usize..9, slot in 1usize..9) {
        let slot = 1 + (slot - 1) % (len - 1);
        let mut s = OtpState::new(len, true);
        s.apply(OtpEvent::Input { slot: 0, text: "9".repeat(slot) });
        prop_assert!(s.chars[slot].is_none());
        prop_assert_eq!(s.apply(OtpEvent::Backspace(slot)), OtpOutcome::Changed);
        prop_assert!(s.chars[slot - 1].is_none());
        prop_assert_eq!(s.focus, slot - 1);
    }
}

#[test]
fn otp_typing_a_code_completes_it() {
    let mut s = OtpState::new(4, true);
    for (i, c) in "1234".chars().enumerate() {
        s.apply(OtpEvent::Input {
            slot: i,
            text: c.to_string(),
        });
    }
    assert!(s.is_complete());
    assert_eq!(s.code(), "1234");
    assert_eq!(s.focus, 3);
    // Letters are refused in numeric mode and leave the box empty.
    let mut s = OtpState::new(4, true);
    assert_eq!(
        s.apply(OtpEvent::Input {
            slot: 0,
            text: "a".into()
        }),
        OtpOutcome::Changed
    );
    assert_eq!(s.code(), "");
}

#[test]
fn otp_typing_over_a_filled_box_keeps_the_new_character() {
    // Where the box's text can't be selected on focus (native), the input reports the old
    // character next to the new one, on either side of the caret.
    let mut s = OtpState::new(4, true);
    s.apply(OtpEvent::Input {
        slot: 0,
        text: "1234".into(),
    });
    s.apply(OtpEvent::Input {
        slot: 1,
        text: "27".into(),
    });
    s.apply(OtpEvent::Input {
        slot: 2,
        text: "83".into(),
    });
    assert_eq!(s.code(), "1784");
    assert_eq!(s.focus, 3);
}
