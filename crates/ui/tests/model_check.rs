//! Exhaustive model checking of the kit's keyboard state machines.
//!
//! `keyboard_state.rs` and `time_picker.rs` throw random key sequences at the pure reducers
//! (proptest). This file does what sampling can't: it walks *every* state a reducer can reach
//! from its start state, presses *every* key in each one, and checks the invariants on every
//! transition. When a check fails, the report is the shortest key sequence that breaks it
//! (breadth-first search keeps a parent pointer per state). A state machine that passes here
//! has been proven, not tested, for that configuration: there is no state left to try.
//!
//! On top of the per-transition invariants it checks three keyboard guarantees that a random
//! walk can only hint at:
//!
//! - **Completeness**: every value the widget can hold (every time of day, every code, every
//!   chart mark, every palette row) is reachable by keyboard from the empty start.
//! - **No trap**: from every reachable state a fixed, short key sequence gets back to empty.
//! - **Cost**: the worst-case number of keystrokes to reach any value, per configuration.
//!
//! The counts each test prints (`--nocapture`) are the numbers quoted in
//! docs/notes/model-checking.md. Run: `cargo test -p atoll-ui --test model_check -- --nocapture`.

use dioxus::prelude::{Key, NamedKey};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::panic::{catch_unwind, AssertUnwindSafe};
use ui::components::chart::{ChartCursor, ChartKey};
use ui::components::command::command_nav;
use ui::components::input_otp::{OtpEvent, OtpOutcome, OtpState};
use ui::components::time_picker::{
    HourCycle, Period, Time, TimeKey, TimeOutcome, TimeSegment, TimeState,
};

// ----- the checker -------------------------------------------------------------------------

/// The reachable state graph of one reducer, explored breadth-first.
struct Explored<S> {
    states: Vec<S>,
    /// Breadth-first depth of each state: the fewest keys that reach it from the start.
    depth: Vec<u32>,
    transitions: u64,
}

/// Explore every state reachable from `init` with `actions`. `step` applies one action and
/// returns the next state, or `Err(reason)` when an invariant fails on that transition. A
/// panic inside `step` counts as a failure too. The first failure aborts with the shortest
/// key sequence that leads to it.
fn explore<S, A>(
    name: &str,
    init: S,
    actions: &[A],
    step: impl Fn(&S, &A) -> Result<S, String>,
) -> Explored<S>
where
    S: Clone + Eq + Hash + Debug,
    A: Debug,
{
    let mut index: HashMap<S, u32> = HashMap::new();
    let mut states = vec![init.clone()];
    let mut parent: Vec<(u32, u16)> = vec![(u32::MAX, 0)];
    let mut depth = vec![0u32];
    index.insert(init, 0);
    let mut queue = VecDeque::from([0u32]);
    let mut transitions = 0u64;

    let trace = |parent: &[(u32, u16)], mut at: u32| -> Vec<&A> {
        let mut keys = Vec::new();
        while parent[at as usize].0 != u32::MAX {
            let (from, action) = parent[at as usize];
            keys.push(&actions[action as usize]);
            at = from;
        }
        keys.reverse();
        keys
    };

    // Panics are failures with a trace; keep the default hook from printing each one.
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut failure = None;
    'search: while let Some(at) = queue.pop_front() {
        let from = states[at as usize].clone();
        for (a, action) in actions.iter().enumerate() {
            transitions += 1;
            let result = catch_unwind(AssertUnwindSafe(|| step(&from, action)))
                .unwrap_or_else(|_| Err("panicked".to_string()));
            let next = match result {
                Ok(next) => next,
                Err(why) => {
                    failure = Some((at, action, why));
                    break 'search;
                }
            };
            if !index.contains_key(&next) {
                let id = states.len() as u32;
                index.insert(next.clone(), id);
                states.push(next);
                parent.push((at, a as u16));
                depth.push(depth[at as usize] + 1);
                queue.push_back(id);
            }
        }
    }
    std::panic::set_hook(hook);
    if let Some((at, action, why)) = failure {
        panic!(
            "{name}: {why}\n  from {:?}\n  after keys {:?}\n  then {action:?}",
            states[at as usize],
            trace(&parent, at)
        );
    }
    Explored {
        states,
        depth,
        transitions,
    }
}

fn report<S>(name: &str, explored: &Explored<S>, extra: &str) {
    println!(
        "{name}: {} states, {} transitions, max depth {}{extra}",
        explored.states.len(),
        explored.transitions,
        explored.depth.iter().max().copied().unwrap_or(0)
    );
}

// ----- time picker -------------------------------------------------------------------------

fn time_keys(segments: usize) -> Vec<TimeKey> {
    let mut keys: Vec<TimeKey> = (0..segments).map(TimeKey::Focus).collect();
    keys.extend(('0'..='9').map(TimeKey::Digit));
    keys.extend([
        TimeKey::Digit('x'),
        TimeKey::SetPeriod(Period::Am),
        TimeKey::SetPeriod(Period::Pm),
        TimeKey::Up,
        TimeKey::Down,
        TimeKey::PageUp,
        TimeKey::PageDown,
        TimeKey::Home,
        TimeKey::End,
        TimeKey::Backspace,
        TimeKey::Delete,
        TimeKey::Left,
        TimeKey::Right,
    ]);
    keys
}

fn is_arrow(key: TimeKey) -> bool {
    matches!(
        key,
        TimeKey::Up
            | TimeKey::Down
            | TimeKey::PageUp
            | TimeKey::PageDown
            | TimeKey::Home
            | TimeKey::End
    )
}

fn values(s: &TimeState) -> [Option<u8>; 4] {
    [
        s.value(TimeSegment::Hour),
        s.value(TimeSegment::Minute),
        s.value(TimeSegment::Second),
        s.value(TimeSegment::Period),
    ]
}

/// The invariants of one `TimeState::apply` transition.
fn time_step(from: &TimeState, key: &TimeKey) -> Result<TimeState, String> {
    let mut next = from.clone();
    let outcome = next.apply(*key);
    let segments = next.segments();
    if next.focus >= segments.len() {
        return Err(format!(
            "focus {} of {} segments",
            next.focus,
            segments.len()
        ));
    }
    for &segment in &segments {
        let (min, max) = next.range(segment);
        if let Some(v) = next.value(segment) {
            if v < min || v > max {
                return Err(format!("{segment:?} = {v}, outside {min}..={max}"));
            }
        }
    }
    let filled = segments.iter().all(|&s| next.value(s).is_some());
    if next.time().is_some() != filled {
        return Err(format!("time() = {:?} with filled = {filled}", next.time()));
    }
    match outcome {
        TimeOutcome::Ignored if next != *from => return Err("Ignored but changed".into()),
        TimeOutcome::Moved if values(&next) != values(from) => {
            return Err("Moved but a value changed".into())
        }
        _ => {}
    }
    if is_arrow(*key) {
        let segment = segments[from.focus.min(segments.len() - 1)];
        if next.value(segment).is_none() {
            return Err(format!("{key:?} left {segment:?} empty"));
        }
        // The arrow keys keep to the range: from an allowed complete time they never
        // step to a time outside it.
        if from.min.is_some() || from.max.is_some() {
            if let (Some(before), Some(after)) = (from.time(), next.time()) {
                if from.in_range(before) && !next.in_range(after) {
                    return Err(format!("{key:?} stepped out of range: {before} -> {after}"));
                }
            }
        }
    }
    // Up then Down is a no-op on the step grid (no range, so nothing is skipped).
    if from.min.is_none() && from.max.is_none() && matches!(key, TimeKey::Up | TimeKey::Down) {
        let segment = segments[from.focus.min(segments.len() - 1)];
        if let Some(v) = from.value(segment) {
            let (min, _) = from.range(segment);
            let step = if segment == TimeSegment::Minute {
                from.minute_step
            } else {
                1
            };
            if (v - min) % step == 0 {
                let mut back = next.clone();
                back.apply(if *key == TimeKey::Up {
                    TimeKey::Down
                } else {
                    TimeKey::Up
                });
                if back.value(segment) != Some(v) {
                    return Err(format!(
                        "{key:?} then back moved {segment:?} {v} -> {:?}",
                        back.value(segment)
                    ));
                }
            }
        }
    }
    Ok(next)
}

/// Clear every segment from any state: focus each one, Delete.
fn time_reset(state: &TimeState) -> TimeState {
    let mut s = state.clone();
    for i in 0..s.segments().len() {
        s.apply(TimeKey::Focus(i));
        s.apply(TimeKey::Delete);
    }
    s.apply(TimeKey::Focus(0));
    s
}

fn seconds_of(t: Time) -> u32 {
    let (h, m, s) = t.as_hms();
    h as u32 * 3600 + m as u32 * 60 + s as u32
}

struct TimeConfig {
    cycle: HourCycle,
    seconds: bool,
    step: u8,
    range: Option<(Time, Time)>,
}

fn hm(h: u8, m: u8) -> Time {
    Time::from_hms(h, m, 0).unwrap()
}

fn check_time(c: &TimeConfig) -> (usize, u64, u32) {
    let mut init = TimeState::new(c.cycle, c.seconds, c.step);
    if let Some((min, max)) = c.range {
        init = init.with_range(Some(min), Some(max));
    }
    let name = format!(
        "time {:?}{} step {}{}",
        c.cycle,
        if c.seconds { " +seconds" } else { "" },
        c.step,
        c.range
            .map(|(a, b)| format!(" range {a}..{b}"))
            .unwrap_or_default()
    );
    let keys = time_keys(init.segments().len());
    let explored = explore(&name, init.clone(), &keys, time_step);

    // No trap: the reset sequence gets every reachable state back to empty.
    for s in &explored.states {
        let reset = time_reset(s);
        assert_eq!(
            values(&reset),
            [None; 4],
            "{name}: reset left values in {s:?}"
        );
    }

    // Completeness: every time of day (to the second when shown) is reachable, and the
    // fewest keys any of them needs.
    let mut cost: HashMap<u32, u32> = HashMap::new();
    for (s, &d) in explored.states.iter().zip(&explored.depth) {
        if let Some(t) = s.time() {
            let e = cost.entry(seconds_of(t)).or_insert(d);
            *e = (*e).min(d);
        }
    }
    let expected = if c.seconds { 86_400 } else { 1_440 };
    assert_eq!(cost.len(), expected, "{name}: some times unreachable");
    let worst = cost.values().copied().max().unwrap_or(0);

    // With a range: every allowed time on the arrow-key grid is reachable with arrow keys
    // and focus moves alone, no typing.
    if let Some((min, max)) = c.range {
        let arrows: Vec<TimeKey> = keys
            .iter()
            .copied()
            .filter(|&k| is_arrow(k) || matches!(k, TimeKey::Focus(_)))
            .collect();
        let by_arrows = explore(&format!("{name} arrows"), init.clone(), &arrows, time_step);
        let reached: HashSet<u32> = by_arrows
            .states
            .iter()
            .filter_map(|s| s.time())
            .filter(|&t| init.in_range(t))
            .map(seconds_of)
            .collect();
        let (lo, hi) = (seconds_of(min), seconds_of(max));
        for t in (0..86_400u32).step_by(60) {
            let allowed = if lo <= hi {
                lo <= t && t <= hi
            } else {
                t >= lo || t <= hi
            };
            let on_grid = (t / 60 % 60) % c.step as u32 == 0;
            if allowed && on_grid {
                assert!(
                    reached.contains(&t),
                    "{name}: {:02}:{:02} allowed but not reachable by arrow keys",
                    t / 3600,
                    t / 60 % 60
                );
            }
        }
    }
    report(
        &name,
        &explored,
        &format!(", worst-case {worst} keys to any time"),
    );
    (explored.states.len(), explored.transitions, worst)
}

#[test]
fn time_picker_every_state_every_key() {
    let configs = [
        TimeConfig {
            cycle: HourCycle::H24,
            seconds: false,
            step: 1,
            range: None,
        },
        TimeConfig {
            cycle: HourCycle::H12,
            seconds: false,
            step: 1,
            range: None,
        },
        TimeConfig {
            cycle: HourCycle::H24,
            seconds: false,
            step: 15,
            range: None,
        },
        TimeConfig {
            cycle: HourCycle::H12,
            seconds: false,
            step: 7,
            range: None,
        },
        TimeConfig {
            cycle: HourCycle::H24,
            seconds: false,
            step: 5,
            range: Some((hm(9, 0), hm(17, 30))),
        },
        TimeConfig {
            cycle: HourCycle::H12,
            seconds: false,
            step: 15,
            range: Some((hm(9, 0), hm(17, 30))),
        },
        // Past midnight.
        TimeConfig {
            cycle: HourCycle::H24,
            seconds: false,
            step: 1,
            range: Some((hm(22, 0), hm(2, 0))),
        },
        TimeConfig {
            cycle: HourCycle::H12,
            seconds: false,
            step: 10,
            range: Some((hm(22, 0), hm(2, 0))),
        },
        TimeConfig {
            cycle: HourCycle::H12,
            seconds: false,
            step: 1,
            range: Some((hm(11, 45), hm(12, 15))),
        },
    ];
    let (mut states, mut transitions) = (0, 0);
    for c in &configs {
        let (s, t, _) = check_time(c);
        states += s;
        transitions += t;
    }
    println!(
        "time picker total: {} configurations, {states} states, {transitions} transitions",
        configs.len()
    );
}

/// Seconds multiply the state space by 61; one configuration per hour cycle.
#[test]
fn time_picker_with_seconds_every_state_every_key() {
    for cycle in [HourCycle::H24, HourCycle::H12] {
        check_time(&TimeConfig {
            cycle,
            seconds: true,
            step: 1,
            range: None,
        });
    }
}

// ----- one-time code -----------------------------------------------------------------------

#[derive(Clone, Debug)]
enum OtpKey {
    Event(OtpEvent),
    /// What the browser does for Backspace: a filled box loses its character (an `input`
    /// event with empty text), an empty box sends the key to the reducer.
    BrowserBackspace,
}

fn otp_keys(length: usize) -> Vec<OtpKey> {
    // Two accepted characters and one rejected one stand for the alphabet; the pairs cover
    // typing into a filled box (old + new character) and pastes, the long one overflows.
    let texts = ["", "1", "2", "a", "11", "12", "21", "1a2", "1212121212121"];
    let mut keys = Vec::new();
    for slot in 0..=length {
        for text in texts {
            keys.push(OtpKey::Event(OtpEvent::Input {
                slot,
                text: text.to_string(),
            }));
        }
        keys.push(OtpKey::Event(OtpEvent::Backspace(slot)));
        keys.push(OtpKey::Event(OtpEvent::Left(slot)));
        keys.push(OtpKey::Event(OtpEvent::Right(slot)));
    }
    keys.push(OtpKey::Event(OtpEvent::Home));
    keys.push(OtpKey::Event(OtpEvent::End));
    keys.push(OtpKey::BrowserBackspace);
    keys
}

fn otp_step(from: &OtpState, key: &OtpKey) -> Result<OtpState, String> {
    let mut next = from.clone();
    let event = match key {
        OtpKey::Event(e) => e.clone(),
        OtpKey::BrowserBackspace if from.chars[from.focus].is_some() => OtpEvent::Input {
            slot: from.focus,
            text: String::new(),
        },
        OtpKey::BrowserBackspace => OtpEvent::Backspace(from.focus),
    };
    let outcome = next.apply(event.clone());
    let len = next.chars.len();
    if len != from.chars.len() {
        return Err("length changed".into());
    }
    if next.focus >= len {
        return Err(format!("focus {} of {len}", next.focus));
    }
    if next.chars.iter().flatten().any(|&c| !next.accepts(c)) {
        return Err(format!("holds a rejected character: {:?}", next.chars));
    }
    if next.is_complete() != (next.code().chars().count() == len) {
        return Err("is_complete disagrees with code()".into());
    }
    match outcome {
        OtpOutcome::Ignored if next != *from => return Err("Ignored but changed".into()),
        OtpOutcome::Moved if next.chars != from.chars => {
            return Err("Moved but the code changed".into())
        }
        _ => {}
    }
    // A single accepted character typed into an empty box lands in that box.
    if let OtpEvent::Input { slot, text } = &event {
        let mut chars = text.chars();
        if let (Some(c), None) = (chars.next(), chars.next()) {
            if *slot < len
                && from.chars[*slot].is_none()
                && next.accepts(c)
                && next.chars[*slot] != Some(c)
            {
                return Err(format!("typed {c:?} into box {slot}, got {:?}", next.chars));
            }
        }
    }
    Ok(next)
}

#[test]
fn otp_every_state_every_key() {
    let (mut states, mut transitions) = (0, 0);
    for numeric in [true, false] {
        for length in 1..=6 {
            let init = OtpState::new(length, numeric);
            let keys = otp_keys(length);
            let name = format!(
                "otp length {length}{}",
                if numeric { "" } else { " alphanumeric" }
            );
            let explored = explore(&name, init.clone(), &keys, otp_step);

            // Completeness: every code over the alphabet, gaps included, is reachable.
            let alphabet: &[Option<char>] = if numeric {
                &[None, Some('1'), Some('2')]
            } else {
                &[None, Some('1'), Some('2'), Some('a')]
            };
            let codes: HashSet<&Vec<Option<char>>> =
                explored.states.iter().map(|s| &s.chars).collect();
            assert_eq!(
                codes.len(),
                alphabet.len().pow(length as u32),
                "{name}: some codes unreachable"
            );

            // No trap: End, then holding Backspace clears every box within length + 1 presses.
            for s in &explored.states {
                let mut t = s.clone();
                t.apply(OtpEvent::End);
                for _ in 0..=length {
                    t = otp_step(&t, &OtpKey::BrowserBackspace).unwrap();
                }
                assert!(
                    t.chars.iter().all(Option::is_none),
                    "{name}: Backspace did not clear {s:?}"
                );
                assert_eq!(t.focus, 0, "{name}: focus not back on the first box");
            }
            report(&name, &explored, "");
            states += explored.states.len();
            transitions += explored.transitions;
        }
    }
    println!("otp total: 12 configurations, {states} states, {transitions} transitions");
}

// ----- command palette ---------------------------------------------------------------------

#[test]
fn command_palette_every_row_every_key() {
    let keys = [
        Key::Named(NamedKey::ArrowDown),
        Key::Named(NamedKey::ArrowUp),
        Key::Named(NamedKey::Home),
        Key::Named(NamedKey::End),
        Key::Named(NamedKey::Tab),
        Key::Named(NamedKey::Enter),
        Key::Named(NamedKey::Escape),
    ];
    let (mut states, mut transitions, mut worst_overall) = (0, 0, 0);
    for len in 0..=64usize {
        let explored = explore(&format!("palette {len} rows"), 0usize, &keys, |&at, key| {
            let next = command_nav(key, at, len).unwrap_or(at);
            let moves = matches!(
                key,
                Key::Named(NamedKey::ArrowDown)
                    | Key::Named(NamedKey::ArrowUp)
                    | Key::Named(NamedKey::Home)
                    | Key::Named(NamedKey::End)
            );
            if moves != command_nav(key, at, len).is_some() {
                return Err(format!("{key:?} moved = {}", !moves));
            }
            if (len == 0 && next != 0) || (len > 0 && next >= len) {
                return Err(format!("highlight {next} of {len}"));
            }
            Ok(next)
        });
        assert_eq!(
            explored.states.len(),
            len.max(1),
            "{len} rows: unreachable rows"
        );
        let worst = explored.depth.iter().copied().max().unwrap_or(0) as usize;
        // Wrapping arrows plus Home and End: no row is more than half the list away.
        assert!(worst <= len / 2, "{len} rows: worst case {worst} keys");
        worst_overall = worst_overall.max(worst);
        states += explored.states.len();
        transitions += explored.transitions;
    }
    println!(
        "palette total: 65 list lengths, {states} states, {transitions} transitions, \
         worst-case {worst_overall} keys (64 rows)"
    );
}

// ----- chart keyboard cursor ---------------------------------------------------------------

#[test]
fn chart_cursor_every_mark_every_key() {
    let keys = [
        ChartKey::Next,
        ChartKey::Previous,
        ChartKey::NextSeries,
        ChartKey::PreviousSeries,
        ChartKey::First,
        ChartKey::Last,
    ];
    let (mut states, mut transitions) = (0, 0);
    for categories in 0..=24usize {
        for series in 0..=6usize {
            let name = format!("chart {categories}x{series}");
            let explored = explore(&name, ChartCursor::default(), &keys, |&c, &key| {
                let next = c.apply(key, categories, series);
                if categories == 0 || series == 0 {
                    return (next == ChartCursor::default())
                        .then_some(next)
                        .ok_or_else(|| "moved in an empty chart".to_string());
                }
                if next.category >= categories || next.series >= series {
                    return Err(format!("cursor {next:?} outside {categories}x{series}"));
                }
                // Moves stop at the ends: Next on the last point stays there.
                if key == ChartKey::Next && c.category == categories - 1 && next != c {
                    return Err(format!("Next at the end moved {c:?} -> {next:?}"));
                }
                Ok(next)
            });
            let marks = (categories * series).max(1);
            assert_eq!(explored.states.len(), marks, "{name}: unreachable marks");
            states += explored.states.len();
            transitions += explored.transitions;
        }
    }
    println!("chart total: 175 chart shapes, {states} states, {transitions} transitions");
}
