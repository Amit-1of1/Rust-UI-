# DateTimePicker

- Copy into your app: `atoll add date_time_picker` or `dx components add date_time_picker --git https://github.com/Amit-1of1/Rust-UI-` (also copies `date_picker`, `form`, `time_picker`, `ui_strings`).
- Import: `use crate::components::date_time_picker::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

A `DatePicker` and a `TimePicker` that edit one value, for bookings, reminders and anything scheduled.

```rust
let mut when = use_signal(|| None::<PrimitiveDateTime>);
rsx! {
    DateTimePicker {
        label: "Departure",
        value: when(),
        on_value_change: move |v| when.set(v),
        default_time: Time::from_hms(9, 0, 0).ok(), // filled in when a date is picked first
        hour_cycle: HourCycle::H12,
        min_date: today,
    }
}
```

`value` is `Option<PrimitiveDateTime>` (`time`, re-exported): `Some` once both a date and a time are entered. The date half takes `min_date`, `max_date` and `disabled_ranges` like `DatePicker`; the time half takes `hour_cycle`, `seconds` and `minute_step` like `TimePicker`. `name` submits it with a native `<form>` as `2026-10-15T14:05`; `format_iso_date_time` and `parse_iso_date_time` write and read that format (`<input type="datetime-local">`'s).

In a `use_form` form, use `DateTimeField { field, label: "Departure" }`: the field holds `2026-10-15T14:05`, empty until both halves are filled.

## Accessibility

- Role and name: a `role="group"` named by `label` (default "Date and time") or by a visible label through `labelled_by`, with the description and error in `aria-describedby`. Inside are the date picker's group and the time picker's group; see their docs.
- Keyboard: as `DatePicker` and `TimePicker`; Tab moves from the date segments to the calendar button to the time segments.
- `DateTimeField` validates when focus leaves the whole picker, calendar included, and a failed submit focuses the hours (the date segments come from dioxus-primitives and cannot be focused from outside).
- Reduced motion: the calendar popover's fade follows `DatePicker`.

## Source

Original to this repo; composes the kit's `DatePicker` and `TimePicker`.

## Stability

Experimental. New on 2026-10-15; see `docs/STABILITY.md` in the kit repository.

## API

```rust
pub use time::PrimitiveDateTime;

/// `"2026-10-15T14:05"` (with `:09` seconds when asked): the format of
/// `<input type="datetime-local">`, used by
/// [`DateTimeField`](crate::components::form::DateTimeField) to keep a value in a form field.
pub fn format_iso_date_time(value: PrimitiveDateTime, seconds: bool) -> String

/// Reads `"2026-10-15T14:05"` or `"2026-10-15T14:05:09"` (a space works in place of `T`).
pub fn parse_iso_date_time(text: &str) -> Option<PrimitiveDateTime>

/// A [`DatePicker`] and a [`TimePicker`] that edit one value. The value is `Some` once both a
/// date and a time are entered.
///
/// ```rust,ignore
/// let mut when = use_signal(|| None::<PrimitiveDateTime>);
/// DateTimePicker { value: when(), on_value_change: move |v| when.set(v), hour_cycle: HourCycle::H12 }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DateTimePicker(
    #[props(default)] value: ReadSignal<Option<PrimitiveDateTime>>,
    #[props(default)] on_value_change: Callback<Option<PrimitiveDateTime>>,
    #[props(default)] hour_cycle: HourCycle,
    #[props(default)] seconds: bool,
    #[props(default = 1)] minute_step: u8,
    /// Filled in when a date is picked while the time is still empty, for example 09:00.
    #[props(default)]
    default_time: Option<Time>,
    #[props(default = Date::from_calendar_date(1925, Month::January, 1).expect("valid date"))]
    min_date: Date,
    #[props(default = Date::from_calendar_date(2050, Month::December, 31).expect("valid date"))]
    max_date: Date,
    /// Dates that can't be picked.
    #[props(default)]
    disabled_ranges: ReadSignal<Vec<DateRange>>,
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] read_only: ReadSignal<bool>,
    /// Accessible name of the group when no visible label is linked with `labelled_by`.
    /// Defaults to "Date and time" from `UiStrings`.
    #[props(into, default)]
    label: Option<String>,
    /// Id of a visible label for the whole picker.
    #[props(into, default)]
    labelled_by: Option<String>,
    /// Id for the time's first segment, so a `<label for>` can point at it.
    #[props(into, default)]
    time_segment_id: Option<String>,
    #[props(default)] invalid: bool,
    /// Ids of a description and error message for the group.
    #[props(into, default)]
    described_by: Option<String>,
    /// Submit the value with a native `<form>` under this name, as `YYYY-MM-DDTHH:MM`.
    #[props(into, default)]
    name: Option<String>,
    /// Called when focus leaves the picker, its calendar included.
    #[props(default)]
    on_blur: Callback<()>,
    /// The time's first segment, once mounted.
    #[props(default)]
    on_mounted: Callback<MountedEvent>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
use crate::components::button::Button;
use crate::components::date_time_picker::*;
use crate::components::form::*;
use crate::components::time_picker::{HourCycle, Time};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let form = use_form();
    let departure = form.field(vec![custom(|v| {
        if v.is_empty() {
            Err("Pick a date and a time.".to_string())
        } else {
            Ok(())
        }
    })]);
    let mut saved = use_signal(|| None::<String>);
    rsx! {
        div { style: "display: grid; gap: 1rem; justify-items: start;",
            Form {
                state: form,
                on_valid: move |_| saved.set(Some(departure.value())),
                DateTimeField {
                    field: departure,
                    label: "Departure",
                    description: "Picking a date fills in 09:00.",
                    default_time: Time::from_hms(9, 0, 0).ok(),
                    minute_step: 5,
                }
                Button { r#type: "submit", "Book" }
            }
            if let Some(value) = saved() {
                p { role: "status", style: "margin: 0;", "Departure: {value}" }
            }
            DateTimePicker { label: "Return", hour_cycle: HourCycle::H12 }
        }
    }
}
```
