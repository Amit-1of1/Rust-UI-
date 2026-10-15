# TimePicker

- Copy into your app: `rust-ui add time_picker` or `dx components add time_picker --git https://github.com/Amit-1of1/Rust-UI-` (also copies `form`, `icons`, `ui_strings`).
- Import: `use crate::components::time_picker::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A time input: one field per part (hours, minutes, optional seconds and AM/PM), each a spin button. Type digits and focus moves on by itself, or use the arrow keys. Written in Rust on plain inputs, with no JavaScript, so it also runs on the desktop webview and native.

```rust
let mut time = use_signal(|| Time::from_hms(9, 30, 0).ok());
rsx! {
    TimePicker {
        label: "Meeting",
        value: time(),
        on_value_change: move |t| time.set(t),
        minute_step: 15,                 // Up/Down move minutes by 15
        hour_cycle: HourCycle::H12,      // adds an AM/PM segment
        seconds: false,
    }
}
```

`value` is `Option<Time>` (`time::Time`, re-exported). It is `Some` once every segment is filled and `None` while any is empty; a half-typed entry stays on screen. `name` submits the time with a native `<form>` as `HH:MM`. `format_time` writes a time the way the picker shows it ("2:05 PM"); `format_iso_time` / `parse_iso_time` read and write `14:05`, the `<input type="time">` format.

In a `use_form` form, use `TimeField { field, label: "Start" }`: the field holds `14:05` (empty until complete, so `required()` works), is labelled, marked invalid with its error linked, and gets focus on a failed submit. For a time with a date, see `DateTimePicker`. When a plain native control is enough (mobile-first forms), `TextField { r#type: "time" }` gives the browser's own time input.

## Accessibility

- Role and name: a `role="group"` named by `label` (default "Time") or by a visible label through `labelled_by`. Each segment is an `<input role="spinbutton">` named "Hours", "Minutes", "Seconds" or "AM/PM" (from `UiStrings`), plus the visible label when `labelled_by` is set, with `aria-valuenow`, `aria-valuemin`, `aria-valuemax` and `aria-valuetext` ("Empty" while empty). `first_segment_id` lets a `<label for>` reach the hours.
- Keyboard: digits fill the focused segment and move on after two digits, or at once when no second digit could follow ("3" in hours). Up/Down step and wrap (minutes by `minute_step`), Page Up/Page Down move minutes and seconds by 10, Home/End jump to the lowest/highest value, Backspace undoes a digit, then clears, then goes back a segment; Delete clears. A or P (the first letters of `UiStrings::am`/`pm`) set AM/PM. Left/Right move between segments; Tab moves through them and out.
- Touch: segments are real inputs with `inputmode="numeric"`, so phones show a number pad.
- Reduced motion: no animation.

## Source

Original to this repo. The segment behaviour follows the WAI-ARIA spin button pattern and React Aria's TimeField (behaviour only, nothing copied). dioxus-primitives' date segments are private and `contenteditable`, so they could not be reused.

## Stability

Experimental. New on 2026-10-15; the props may change once a real app uses it (min/max times are not supported yet). See `docs/STABILITY.md` in the kit repository.

## API

```rust
pub use time::Time;

/// 24-hour ("14:05") or 12-hour ("2:05 PM") entry.
pub enum HourCycle {
    #[default]
    H24,
    H12,
}

pub enum Period {
    Am,
    Pm,
}

/// One editable part of a [`TimePicker`].
pub enum TimeSegment {
    Hour,
    Minute,
    Second,
    Period,
}

/// A key (or a focus change) sent to one segment of a [`TimePicker`].
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
pub struct TimeState {
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    pub period: Option<Period>,
    pub cycle: HourCycle,
    pub seconds: bool,
    /// Arrow keys move minutes by this much (1 to 30).
    pub minute_step: u8,
    /// Index into [`segments`](Self::segments).
    pub focus: usize,
    /// Digits typed into the focused segment since it got focus.
    typed: String,
}

/// `"14:05"`, or `"14:05:09"` with seconds: the format of `<input type="time">`, used by
/// [`TimeField`](crate::components::form::TimeField) to keep a time in a form field.
pub fn format_iso_time(time: Time, seconds: bool) -> String

/// Reads `"14:05"` or `"14:05:09"`; anything else is `None`.
pub fn parse_iso_time(text: &str) -> Option<Time>

/// How a time reads in the picker's own format: `"14:05"` or `"2:05 PM"`.
pub fn format_time(time: Time, cycle: HourCycle, seconds: bool, strings: &UiStrings) -> String

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
) -> Element
```

## Example

```rust
use crate::components::button::Button;
use crate::components::form::*;
use crate::components::time_picker::*;
use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let strings = use_ui_strings();
    let mut meeting = use_signal(|| Time::from_hms(9, 30, 0).ok());
    let form = use_form();
    let reminder = form.field(vec![custom(|v| {
        if v.is_empty() {
            Err("Enter a time.".to_string())
        } else {
            Ok(())
        }
    })]);
    let mut saved = use_signal(|| None::<String>);
    let meeting_text = meeting()
        .map(|t| format_time(t, HourCycle::H24, false, &strings))
        .unwrap_or_else(|| "not set".to_string());
    rsx! {
        div { style: "display: grid; gap: 1rem; justify-items: start;",
            TimePicker {
                label: "Meeting",
                value: meeting(),
                on_value_change: move |t| meeting.set(t),
                minute_step: 15,
            }
            p { style: "margin: 0;", "Meeting: {meeting_text}" }
            TimePicker { label: "Alarm", hour_cycle: HourCycle::H12, seconds: true }
            Form {
                state: form,
                on_valid: move |_| {
                    let text = parse_iso_time(&reminder.value())
                        .map(|t| format_time(t, HourCycle::H12, false, &strings));
                    saved.set(text);
                },
                TimeField {
                    field: reminder,
                    label: "Reminder",
                    hour_cycle: HourCycle::H12,
                    description: "Type the hour, minutes, then A or P.",
                }
                Button { r#type: "submit", "Set reminder" }
            }
            if let Some(text) = saved() {
                p { role: "status", style: "margin: 0;", "Reminder set for {text}" }
            }
        }
    }
}
```
