use crate::components::icons::Clock;
use crate::components::ui_strings::{use_ui_strings, UiStrings};
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
/// Re-exported so apps can name the picker's value without depending on `time` themselves.
pub use time::Time;

#[css_module("/src/components/time_picker/style.css")]
struct Styles;

/// 24-hour ("14:05") or 12-hour ("2:05 PM") entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum HourCycle {
    #[default]
    H24,
    H12,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Period {
    Am,
    Pm,
}

/// One editable part of a [`TimePicker`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeSegment {
    Hour,
    Minute,
    Second,
    Period,
}

/// A key (or a focus change) sent to one segment of a [`TimePicker`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeKey {
    /// The segment at this index received focus: typing starts afresh there.
    Focus(usize),
    Digit(char),
    /// A letter that picks AM or PM (the component maps the localised first letters).
    SetPeriod(Period),
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Backspace,
    Delete,
    Left,
    Right,
}

/// What [`TimeState::apply`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeOutcome {
    /// Not ours; let the browser handle the key.
    Ignored,
    /// Only focus moved.
    Moved,
    /// A segment's value changed (and focus may have moved).
    Changed,
}

/// The segments of a [`TimePicker`] and which one has focus. Each part may be empty while
/// the user types. Pure, so the keyboard behaviour is tested without a browser
/// (`crates/ui/tests/time_picker.rs`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TimeState {
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub period: Option<Period>,
    pub cycle: HourCycle,
    pub seconds: bool,
    /// Arrow keys move minutes by this much (1 to 30).
    pub minute_step: u8,
    /// Earliest allowed time. With [`max`](Self::max) earlier than this, the range runs past
    /// midnight (22:00 to 02:00).
    pub min: Option<Time>,
    /// Latest allowed time.
    pub max: Option<Time>,
    /// Index into [`segments`](Self::segments).
    pub focus: usize,
    /// Digits typed into the focused segment since it got focus.
    typed: String,
}

impl TimeState {
    pub fn new(cycle: HourCycle, seconds: bool, minute_step: u8) -> Self {
        Self {
            hour: None,
            minute: None,
            second: None,
            period: None,
            cycle,
            seconds,
            minute_step: minute_step.clamp(1, 30),
            min: None,
            max: None,
            focus: 0,
            typed: String::new(),
        }
    }

    /// Limit the arrow keys to times from `min` to `max` (either may be open). Typing still takes
    /// any time; [`out_of_range`](Self::out_of_range) then reports it.
    pub fn with_range(mut self, min: Option<Time>, max: Option<Time>) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Whether `time` is inside `min..=max`, wrapping past midnight when `max < min`.
    pub fn in_range(&self, time: Time) -> bool {
        let t = seconds_of_day(time);
        self.allowed().iter().any(|&(a, b)| a <= t && t <= b)
    }

    /// A complete time that is outside `min..=max`.
    pub fn out_of_range(&self) -> bool {
        self.time().is_some_and(|t| !self.in_range(t))
    }

    /// The allowed range as inclusive spans of seconds since midnight.
    fn allowed(&self) -> Vec<(u32, u32)> {
        let min = self.min.map_or(0, seconds_of_day);
        let max = self.max.map_or(DAY - 1, seconds_of_day);
        if min <= max {
            vec![(min, max)]
        } else {
            vec![(min, DAY - 1), (0, max)]
        }
    }

    fn limited(&self) -> bool {
        self.min.is_some() || self.max.is_some()
    }

    /// The 24-hour hours a 12-hour `hour` can mean: one once AM/PM is known, both before.
    fn hours_for(&self, hour: u8, period: Option<Period>) -> Vec<u8> {
        match self.cycle {
            HourCycle::H24 => vec![hour],
            HourCycle::H12 => {
                let base = hour % 12;
                match period {
                    Some(Period::Am) => vec![base],
                    Some(Period::Pm) => vec![base + 12],
                    None => vec![base, base + 12],
                }
            }
        }
    }

    /// Whether `value` in `segment` can still lead to an allowed time, given the segments
    /// above it (hours, then minutes, then seconds; AM/PM goes with the hours). Segments below
    /// are not looked at, so the hours can be picked first.
    fn can_reach(&self, segment: TimeSegment, value: u8) -> bool {
        if !self.limited() {
            return true;
        }
        let allowed = self.allowed();
        let hits = |from: u32, to: u32| allowed.iter().any(|&(a, b)| from <= b && to >= a);
        let hour_span = |h: u8| (h as u32 * 3600, h as u32 * 3600 + 3599);
        match segment {
            TimeSegment::Hour => self
                .hours_for(value, self.period)
                .into_iter()
                .any(|h| hits(hour_span(h).0, hour_span(h).1)),
            TimeSegment::Period => {
                let period = if value == 0 { Period::Am } else { Period::Pm };
                match self.hour {
                    Some(hour) => self
                        .hours_for(hour, Some(period))
                        .into_iter()
                        .any(|h| hits(hour_span(h).0, hour_span(h).1)),
                    None => {
                        let start = value as u32 * 12 * 3600;
                        hits(start, start + 12 * 3600 - 1)
                    }
                }
            }
            TimeSegment::Minute => {
                let Some(hour) = self.hour else { return true };
                self.hours_for(hour, self.period).into_iter().any(|h| {
                    let from = h as u32 * 3600 + value as u32 * 60;
                    hits(from, from + 59)
                })
            }
            TimeSegment::Second => {
                let (Some(hour), Some(minute)) = (self.hour, self.minute) else {
                    return true;
                };
                self.hours_for(hour, self.period).into_iter().any(|h| {
                    let at = h as u32 * 3600 + minute as u32 * 60 + value as u32;
                    hits(at, at)
                })
            }
        }
    }

    pub fn segments(&self) -> Vec<TimeSegment> {
        let mut segments = vec![TimeSegment::Hour, TimeSegment::Minute];
        if self.seconds {
            segments.push(TimeSegment::Second);
        }
        if self.cycle == HourCycle::H12 {
            segments.push(TimeSegment::Period);
        }
        segments
    }

    /// Lowest and highest value of a segment (AM is 0 and PM is 1).
    pub fn range(&self, segment: TimeSegment) -> (u8, u8) {
        match segment {
            TimeSegment::Hour if self.cycle == HourCycle::H12 => (1, 12),
            TimeSegment::Hour => (0, 23),
            TimeSegment::Minute | TimeSegment::Second => (0, 59),
            TimeSegment::Period => (0, 1),
        }
    }

    pub fn value(&self, segment: TimeSegment) -> Option<u8> {
        match segment {
            TimeSegment::Hour => self.hour,
            TimeSegment::Minute => self.minute,
            TimeSegment::Second => self.second,
            TimeSegment::Period => self.period.map(|p| (p == Period::Pm) as u8),
        }
    }

    fn set(&mut self, segment: TimeSegment, value: Option<u8>) {
        match segment {
            TimeSegment::Hour => self.hour = value,
            TimeSegment::Minute => self.minute = value,
            TimeSegment::Second => self.second = value,
            TimeSegment::Period => {
                self.period = value.map(|v| if v == 0 { Period::Am } else { Period::Pm })
            }
        }
    }

    fn step(&self, segment: TimeSegment) -> u8 {
        match segment {
            TimeSegment::Minute => self.minute_step,
            _ => 1,
        }
    }

    /// The time, once every segment has a value.
    pub fn time(&self) -> Option<Time> {
        let second = if self.seconds { self.second? } else { 0 };
        let hour = match self.cycle {
            HourCycle::H24 => self.hour?,
            HourCycle::H12 => self.hour? % 12 + if self.period? == Period::Pm { 12 } else { 0 },
        };
        Time::from_hms(hour, self.minute?, second).ok()
    }

    /// Fill every segment from `time`, or empty them all.
    pub fn set_time(&mut self, time: Option<Time>) {
        self.typed.clear();
        let Some(time) = time else {
            (self.hour, self.minute, self.second, self.period) = (None, None, None, None);
            return;
        };
        let (h, m, s) = time.as_hms();
        self.minute = Some(m);
        self.second = Some(s);
        match self.cycle {
            HourCycle::H24 => self.hour = Some(h),
            HourCycle::H12 => {
                self.hour = Some(if h % 12 == 0 { 12 } else { h % 12 });
                self.period = Some(if h >= 12 { Period::Pm } else { Period::Am });
            }
        }
    }

    /// The segment's digits ("09"), or `None` while it is empty. AM/PM is left to the caller.
    pub fn text(&self, segment: TimeSegment) -> Option<String> {
        self.value(segment).map(|v| format!("{v:02}"))
    }

    pub fn apply(&mut self, key: TimeKey) -> TimeOutcome {
        let segments = self.segments();
        let last = segments.len() - 1;
        if let TimeKey::Focus(index) = key {
            self.focus = index.min(last);
            self.typed.clear();
            return TimeOutcome::Moved;
        }
        let segment = segments[self.focus.min(last)];
        let (min, max) = self.range(segment);
        let step = self.step(segment);
        // The highest value reachable from `min` in whole steps.
        let top = min + (max - min) / step * step;
        let current = self.value(segment);
        let new_value = match key {
            TimeKey::Focus(_) => unreachable!(),
            TimeKey::Left => return self.move_to(self.focus.checked_sub(1)),
            TimeKey::Right => {
                return self.move_to(Some(self.focus + 1).filter(|&i| i <= last));
            }
            TimeKey::Digit(c) if segment != TimeSegment::Period => {
                let Some(digit) = c.to_digit(10) else {
                    return TimeOutcome::Ignored;
                };
                self.typed.push(c);
                let mut typed: u32 = self.typed.parse().unwrap_or(digit);
                if typed > max as u32 {
                    // "7" after "3" in minutes is not 37 here: start again with the new digit.
                    self.typed = c.to_string();
                    typed = digit;
                }
                let complete = self.typed.len() >= 2 || typed * 10 > max as u32;
                let value = (typed >= min as u32).then_some(typed as u8).or(current);
                self.set(segment, value);
                if complete {
                    self.typed.clear();
                    if self.focus < last {
                        self.focus += 1;
                    }
                }
                return TimeOutcome::Changed;
            }
            TimeKey::SetPeriod(period) if segment == TimeSegment::Period => {
                self.period = Some(period);
                return TimeOutcome::Changed;
            }
            TimeKey::Up | TimeKey::PageUp => {
                let by = if key == TimeKey::PageUp && segment != TimeSegment::Hour {
                    10u8.max(step)
                } else {
                    step
                };
                let up = |current: Option<u8>| match current {
                    None => min,
                    Some(v) => {
                        let next = (v - min) / by * by + by + min;
                        if next > max {
                            min
                        } else {
                            next
                        }
                    }
                };
                Some(self.first_reachable(segment, current, up))
            }
            TimeKey::Down | TimeKey::PageDown => {
                let by = if key == TimeKey::PageDown && segment != TimeSegment::Hour {
                    10u8.max(step)
                } else {
                    step
                };
                // Wrap to the highest value on this key's grid (50 for Page Down on minutes).
                let top = min + (max - min) / by * by;
                let down = |current: Option<u8>| match current {
                    None => top,
                    Some(v) if (v - min) % by != 0 => (v - min) / by * by + min,
                    Some(v) if v >= min + by => v - by,
                    Some(_) => top,
                };
                Some(self.first_reachable(segment, current, down))
            }
            TimeKey::Home => Some(self.first_reachable(segment, None, |v| match v {
                None => min,
                Some(v) => (v + step).min(top),
            })),
            TimeKey::End => Some(self.first_reachable(segment, None, |v| match v {
                None => top,
                Some(v) => v.saturating_sub(step).max(min),
            })),
            TimeKey::Backspace if !self.typed.is_empty() => {
                self.typed.pop();
                self.typed.parse::<u8>().ok().filter(|v| *v >= min)
            }
            TimeKey::Backspace if current.is_none() => {
                return self.move_to(self.focus.checked_sub(1));
            }
            TimeKey::Backspace | TimeKey::Delete => None,
            TimeKey::Digit(_) | TimeKey::SetPeriod(_) => return TimeOutcome::Ignored,
        };
        if !matches!(key, TimeKey::Backspace) {
            self.typed.clear();
        }
        self.set(segment, new_value);
        if !matches!(key, TimeKey::Backspace | TimeKey::Delete) {
            self.keep_in_range(segment);
        }
        TimeOutcome::Changed
    }

    /// After an arrow key on `segment`, pull the segments below it to the nearest allowed time
    /// when the step left a complete time outside the range: Down on the hour of 9:55 in a
    /// 9:00 to 17:30 range gives 17:30, not 17:55. Found by the exhaustive model check
    /// (`crates/ui/tests/model_check.rs`).
    fn keep_in_range(&mut self, segment: TimeSegment) {
        let Some(now) = self.time() else { return };
        if !self.limited() || self.in_range(now) {
            return;
        }
        let (h, m, s) = now.as_hms();
        let current = seconds_of_day(now) as i64;
        let minutes: Vec<u8> = match segment {
            TimeSegment::Hour | TimeSegment::Period => {
                (0..60).step_by(self.minute_step as usize).collect()
            }
            TimeSegment::Minute => vec![m],
            TimeSegment::Second => return,
        };
        let seconds: Vec<u8> = if self.seconds { (0..60).collect() } else { vec![s] };
        let best = minutes
            .iter()
            .flat_map(|&m| seconds.iter().map(move |&s| (m, s)))
            .filter_map(|(m, s)| Time::from_hms(h, m, s).ok())
            .filter(|&t| self.in_range(t))
            .min_by_key(|&t| (seconds_of_day(t) as i64 - current).abs());
        // The range boundaries themselves may sit off the minute grid (a 9:07 minimum).
        let edge = [self.min, self.max]
            .into_iter()
            .flatten()
            .filter(|t| t.hour() == h && (segment != TimeSegment::Minute || t.minute() == m))
            .min_by_key(|&t| (seconds_of_day(t) as i64 - current).abs());
        if let Some(t) = best.or(edge) {
            self.minute = Some(t.minute());
            if self.seconds {
                self.second = Some(t.second());
            }
        }
    }

    /// Step with `next` from `current` until a value can still reach an allowed time (see
    /// `can_reach`). When none can, the current value stays if it is allowed; otherwise the
    /// first step is taken anyway, as without a range.
    fn first_reachable(
        &self,
        segment: TimeSegment,
        current: Option<u8>,
        next: impl Fn(Option<u8>) -> u8,
    ) -> u8 {
        let first = next(current);
        let mut value = first;
        // A segment has at most 60 values; a step can't visit more.
        for _ in 0..61 {
            if self.can_reach(segment, value) {
                return value;
            }
            value = next(Some(value));
        }
        // No step lands on an allowed value. Keep the current one when it is allowed (PM in a
        // 9:00 to 17:30 range at 2:06 PM: AM would leave the range); otherwise the range is
        // unreachable from here and the key steps as it would without one.
        match current {
            Some(v) if self.can_reach(segment, v) => v,
            _ => first,
        }
    }

    fn move_to(&mut self, index: Option<usize>) -> TimeOutcome {
        match index {
            Some(index) => {
                self.focus = index;
                self.typed.clear();
                TimeOutcome::Moved
            }
            None => TimeOutcome::Ignored,
        }
    }
}

const DAY: u32 = 86_400;

fn seconds_of_day(time: Time) -> u32 {
    let (h, m, s) = time.as_hms();
    h as u32 * 3600 + m as u32 * 60 + s as u32
}

/// `"14:05"`, or `"14:05:09"` with seconds: the format of `<input type="time">`, used by
/// [`TimeField`](crate::components::form::TimeField) to keep a time in a form field.
pub fn format_iso_time(time: Time, seconds: bool) -> String {
    let (h, m, s) = time.as_hms();
    if seconds {
        format!("{h:02}:{m:02}:{s:02}")
    } else {
        format!("{h:02}:{m:02}")
    }
}

/// Reads `"14:05"` or `"14:05:09"`; anything else is `None`.
pub fn parse_iso_time(text: &str) -> Option<Time> {
    let mut parts = text.trim().split(':');
    let mut next = |required: bool| -> Option<Option<u8>> {
        match parts.next() {
            Some(p) if p.len() == 2 && p.bytes().all(|b| b.is_ascii_digit()) => {
                Some(p.parse().ok())
            }
            None if !required => Some(None),
            _ => None,
        }
    };
    let h = next(true)??;
    let m = next(true)??;
    let s = next(false)?.unwrap_or(0);
    if parts.next().is_some() {
        return None;
    }
    Time::from_hms(h, m, s).ok()
}

/// How a time reads in the picker's own format: `"14:05"` or `"2:05 PM"`.
pub fn format_time(time: Time, cycle: HourCycle, seconds: bool, strings: &UiStrings) -> String {
    let mut state = TimeState::new(cycle, seconds, 1);
    state.set_time(Some(time));
    let (_, m, s) = time.as_hms();
    let hour = state.hour.unwrap_or_default();
    let mut out = match cycle {
        HourCycle::H24 => format!("{hour:02}:{m:02}"),
        HourCycle::H12 => format!("{hour}:{m:02}"),
    };
    if seconds {
        out.push_str(&format!(":{s:02}"));
    }
    if cycle == HourCycle::H12 {
        let period = if state.period == Some(Period::Pm) {
            &strings.pm
        } else {
            &strings.am
        };
        out.push(' ');
        out.push_str(period);
    }
    out
}

/// Focus a segment through its element handle, which works on every renderer.
fn focus_segment(segments: &[Option<Rc<MountedData>>], index: usize) {
    if let Some(Some(segment)) = segments.get(index) {
        let segment = segment.clone();
        spawn(async move {
            _ = segment.set_focus(true).await;
        });
    }
}

/// Calls `on_blur` when focus leaves everything inside the returned handlers' element.
/// Focus moving between two children fires `focusout` then `focusin` back to back, before
/// effects run, so only a real exit is seen.
pub(crate) fn use_focus_exit(on_blur: Callback<()>) -> Signal<bool> {
    let within = use_signal(|| false);
    let was = use_hook(|| Rc::new(Cell::new(false)));
    use_effect(move || {
        let now = within();
        if was.replace(now) && !now {
            on_blur.call(());
        }
    });
    within
}

/// A time input: one field per part (hours, minutes, optional seconds and AM/PM), each a
/// spin button. Type digits (focus moves on by itself), or use the arrow keys.
///
/// ```rust,ignore
/// let mut time = use_signal(|| None::<Time>);
/// TimePicker { value: time(), on_value_change: move |t| time.set(t), minute_step: 5 }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn TimePicker(
    /// The time; `None` while empty or only partly entered.
    #[props(default)]
    value: ReadSignal<Option<Time>>,
    /// Called with the new time once every segment is filled, and with `None` when one is
    /// cleared.
    #[props(default)]
    on_value_change: Callback<Option<Time>>,
    #[props(default)] hour_cycle: HourCycle,
    /// Add a seconds segment.
    #[props(default)]
    seconds: bool,
    /// Up and Down move minutes by this much; typing still takes any minute.
    #[props(default = 1)]
    minute_step: u8,
    /// Earliest allowed time. The arrow keys skip what is outside `min`..`max`; a typed time
    /// outside it is kept but marked invalid. A `max` earlier than `min` wraps past midnight.
    #[props(default)]
    min: Option<Time>,
    /// Latest allowed time (inclusive).
    #[props(default)]
    max: Option<Time>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] read_only: ReadSignal<bool>,
    /// Accessible name of the group when no visible label is linked with `labelled_by`.
    /// Defaults to "Time" from `UiStrings`.
    #[props(into, default)]
    label: Option<String>,
    /// Id of a visible label. The group and every segment are named by it ("Hours Start time").
    #[props(into, default)]
    labelled_by: Option<String>,
    /// Id for the first segment, so a `<label for>` can point at it.
    #[props(into, default)]
    first_segment_id: Option<String>,
    /// Marks every segment `aria-invalid`.
    #[props(default)]
    invalid: bool,
    /// Ids of a description and error message, read with every segment.
    #[props(into, default)]
    described_by: Option<String>,
    /// Submit the time with a native `<form>` under this name, as `HH:MM` or `HH:MM:SS`.
    #[props(into, default)]
    name: Option<String>,
    /// Called when focus leaves the picker.
    #[props(default)]
    on_blur: Callback<()>,
    /// The first segment's element, once mounted (for focus on a failed submit).
    #[props(default)]
    on_mounted: Callback<MountedEvent>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let t = use_ui_strings();
    let base = use_hook(|| {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        format!("dx-time-{}", NEXT.fetch_add(1, Ordering::Relaxed))
    });
    // Start from the value, so the first frame (and server rendering) shows it.
    let mut emitted = use_signal(|| *value.peek());
    let mut state = use_signal(move || {
        let mut state = TimeState::new(hour_cycle, seconds, minute_step).with_range(min, max);
        state.set_time(*value.peek());
        state
    });
    let mut elements = use_signal(Vec::<Option<Rc<MountedData>>>::new);
    // Follow the controlled value, but not our own echo: while the user is half-way through
    // (state partly filled, `None` emitted) a `None` from the parent must not clear it.
    use_effect(move || {
        let incoming = value();
        if incoming != *emitted.peek() {
            emitted.set(incoming);
            state.write().set_time(incoming);
        }
    });
    let mut within = use_focus_exit(on_blur);

    let mut apply = move |key: TimeKey| -> TimeOutcome {
        let before = state.peek().focus;
        let outcome = {
            // The range props may have changed since the state was made.
            let mut state = state.write();
            (state.min, state.max) = (min, max);
            state.apply(key)
        };
        let now = state.peek().clone();
        if outcome == TimeOutcome::Changed {
            let time = now.time();
            if time != *emitted.peek() {
                emitted.set(time);
                on_value_change.call(time);
            }
        }
        if now.focus != before && !matches!(key, TimeKey::Focus(_)) {
            focus_segment(&elements.peek(), now.focus);
        }
        outcome
    };

    let current = state.read().clone().with_range(min, max);
    let invalid = invalid || current.out_of_range();
    let segments = current.segments();
    let is_disabled = disabled();
    let is_read_only = read_only();
    let group_label = labelled_by
        .is_none()
        .then(|| label.clone().unwrap_or_else(|| t.time.clone()));
    let hidden_value = current
        .time()
        .map(|time| format_iso_time(time, seconds))
        .unwrap_or_default();
    let am_key = t.am.chars().next().map(|c| c.to_lowercase().to_string());
    let pm_key = t.pm.chars().next().map(|c| c.to_lowercase().to_string());

    rsx! {
        div {
            role: "group",
            "aria-label": group_label,
            "aria-labelledby": labelled_by.clone(),
            "data-disabled": is_disabled.then_some("true"),
            "data-invalid": invalid.then_some("true"),
            onfocusin: move |_| within.set(true),
            onfocusout: move |_| within.set(false),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_time_picker }), attributes]),
            for (i , segment) in segments.iter().copied().enumerate() {
                Fragment { key: "{i}",
                    if i > 0 && segment != TimeSegment::Period {
                        span { class: Styles::dx_time_separator, "aria-hidden": "true", ":" }
                    }
                    {
                        let (min, max) = current.range(segment);
                        let id = match (i, &first_segment_id) {
                            (0, Some(id)) => id.clone(),
                            _ => format!("{base}-{i}"),
                        };
                        let (name, placeholder, width) = match segment {
                            TimeSegment::Hour => (&t.hours, "––", 2),
                            TimeSegment::Minute => (&t.minutes, "––", 2),
                            TimeSegment::Second => (&t.seconds, "––", 2),
                            TimeSegment::Period => (&t.day_period, "––", t.am.chars().count().max(t.pm.chars().count()).max(2)),
                        };
                        let text = match segment {
                            TimeSegment::Period => current.period.map(|p| if p == Period::Pm { t.pm.clone() } else { t.am.clone() }),
                            _ => current.text(segment),
                        };
                        let value_text = text.clone().unwrap_or_else(|| t.empty_value.clone());
                        let labelled = labelled_by.as_ref().map(|l| format!("{id} {l}"));
                        let (am_key, pm_key) = (am_key.clone(), pm_key.clone());
                        rsx! {
                            input {
                                id: "{id}",
                                class: Styles::dx_time_segment,
                                r#type: "text",
                                role: "spinbutton",
                                inputmode: if segment == TimeSegment::Period { "text" } else { "numeric" },
                                autocomplete: "off",
                                spellcheck: "false",
                                size: "{width}",
                                style: "--segment-width: {width}ch",
                                placeholder,
                                value: text.unwrap_or_default(),
                                "aria-label": "{name}",
                                "aria-labelledby": labelled,
                                "aria-valuemin": "{min}",
                                "aria-valuemax": "{max}",
                                "aria-valuenow": current.value(segment).map(|v| v.to_string()),
                                "aria-valuetext": "{value_text}",
                                "aria-invalid": invalid.then_some("true"),
                                "aria-describedby": described_by.clone(),
                                disabled: is_disabled,
                                readonly: is_read_only,
                                onmounted: move |e: MountedEvent| {
                                    let mut list = elements.write();
                                    if list.len() <= i {
                                        list.resize(i + 1, None);
                                    }
                                    list[i] = Some(e.data());
                                    if i == 0 {
                                        on_mounted.call(e);
                                    }
                                },
                                onfocus: move |_| {
                                    apply(TimeKey::Focus(i));
                                },
                                onkeydown: {
                                    let (am_key, pm_key) = (am_key.clone(), pm_key.clone());
                                    move |e: KeyboardEvent| {
                                        let m = e.modifiers();
                                        if m.ctrl() || m.meta() || m.alt() {
                                            return;
                                        }
                                        let editing = !is_read_only;
                                        let key = match e.key() {
                                            Key::Named(NamedKey::ArrowLeft) => TimeKey::Left,
                                            Key::Named(NamedKey::ArrowRight) => TimeKey::Right,
                                            Key::Named(NamedKey::ArrowUp) if editing => TimeKey::Up,
                                            Key::Named(NamedKey::ArrowDown) if editing => TimeKey::Down,
                                            Key::Named(NamedKey::PageUp) if editing => TimeKey::PageUp,
                                            Key::Named(NamedKey::PageDown) if editing => TimeKey::PageDown,
                                            Key::Named(NamedKey::Home) if editing => TimeKey::Home,
                                            Key::Named(NamedKey::End) if editing => TimeKey::End,
                                            Key::Named(NamedKey::Backspace) if editing => TimeKey::Backspace,
                                            Key::Named(NamedKey::Delete) if editing => TimeKey::Delete,
                                            Key::Character(c) if editing => {
                                                let lower = c.to_lowercase();
                                                if Some(&lower) == am_key.as_ref() {
                                                    TimeKey::SetPeriod(Period::Am)
                                                } else if Some(&lower) == pm_key.as_ref() {
                                                    TimeKey::SetPeriod(Period::Pm)
                                                } else if let Some(d) = c.chars().next().filter(|d| d.is_ascii_digit()) {
                                                    TimeKey::Digit(d)
                                                } else {
                                                    // Other printable keys would only edit the text.
                                                    e.prevent_default();
                                                    return;
                                                }
                                            }
                                            _ => return,
                                        };
                                        if apply(key) != TimeOutcome::Ignored {
                                            e.prevent_default();
                                        }
                                    }
                                },
                                // Touch keyboards often send no usable key code, so the text
                                // arrives here instead: use the last character typed.
                                oninput: move |e: FormEvent| {
                                    let typed = e.value();
                                    let key = match typed.chars().last() {
                                        None => TimeKey::Delete,
                                        Some(c) if c.is_ascii_digit() => TimeKey::Digit(c),
                                        Some(c) => {
                                            let lower = c.to_lowercase().to_string();
                                            if Some(&lower) == am_key.as_ref() {
                                                TimeKey::SetPeriod(Period::Am)
                                            } else if Some(&lower) == pm_key.as_ref() {
                                                TimeKey::SetPeriod(Period::Pm)
                                            } else {
                                                return;
                                            }
                                        }
                                    };
                                    apply(key);
                                },
                            }
                        }
                    }
                }
            }
            Clock { class: Styles::dx_time_icon, size: "16px", "aria-hidden": "true" }
            if let Some(name) = name {
                input { r#type: "hidden", name, value: hidden_value }
            }
        }
    }
}
