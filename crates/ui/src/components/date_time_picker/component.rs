use crate::components::date_picker::*;
use crate::components::time_picker::*;
use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
pub use time::PrimitiveDateTime;

#[css_module("/src/components/date_time_picker/style.css")]
struct Styles;

/// `"2026-10-15T14:05"` (with `:09` seconds when asked): the format of
/// `<input type="datetime-local">`, used by
/// [`DateTimeField`](crate::components::form::DateTimeField) to keep a value in a form field.
pub fn format_iso_date_time(value: PrimitiveDateTime, seconds: bool) -> String {
    format!(
        "{}T{}",
        format_iso_date(value.date()),
        format_iso_time(value.time(), seconds)
    )
}

/// Reads `"2026-10-15T14:05"` or `"2026-10-15T14:05:09"` (a space works in place of `T`).
pub fn parse_iso_date_time(text: &str) -> Option<PrimitiveDateTime> {
    let text = text.trim();
    let (date, time) = text.split_once(['T', ' '])?;
    Some(PrimitiveDateTime::new(
        parse_iso_date(date)?,
        parse_iso_time(time)?,
    ))
}

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
) -> Element {
    let t = use_ui_strings();
    let mut date = use_signal(|| (*value.peek()).map(|v| v.date()));
    let mut time = use_signal(|| (*value.peek()).map(|v| v.time()));
    let mut emitted = use_signal(|| *value.peek());
    use_effect(move || {
        let incoming = value();
        if incoming != *emitted.peek() {
            emitted.set(incoming);
            if let Some(v) = incoming {
                date.set(Some(v.date()));
                time.set(Some(v.time()));
            } else {
                // A cleared value clears both halves; an unfinished entry stays.
                date.set(None);
                time.set(None);
            }
        }
    });
    let mut emit = move || {
        let combined = date
            .peek()
            .zip(*time.peek())
            .map(|(d, t)| PrimitiveDateTime::new(d, t));
        if combined != *emitted.peek() {
            emitted.set(combined);
            on_value_change.call(combined);
        }
    };
    let mut within = use_focus_exit(on_blur);
    let group_label = labelled_by
        .is_none()
        .then(|| label.clone().unwrap_or_else(|| t.date_and_time.clone()));
    let hidden_value = date()
        .zip(time())
        .map(|(d, t)| format_iso_date_time(PrimitiveDateTime::new(d, t), seconds))
        .unwrap_or_default();

    rsx! {
        div {
            role: "group",
            "aria-label": group_label,
            "aria-labelledby": labelled_by.clone(),
            "aria-describedby": described_by,
            "data-invalid": invalid.then_some("true"),
            onfocusin: move |_| within.set(true),
            onfocusout: move |_| within.set(false),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_date_time_picker }), attributes]),
            DatePicker {
                selected_date: date(),
                on_value_change: move |d: Option<Date>| {
                    date.set(d);
                    if d.is_some() && time.peek().is_none() {
                        if let Some(default_time) = default_time {
                            time.set(Some(default_time));
                        }
                    }
                    emit();
                },
                min_date,
                max_date,
                disabled_ranges,
                disabled,
                read_only,
            }
            TimePicker {
                value: time(),
                on_value_change: move |v: Option<Time>| {
                    time.set(v);
                    emit();
                },
                hour_cycle,
                seconds,
                minute_step,
                disabled,
                read_only,
                // Named "Time" inside the outer group, which carries the field's label.
                first_segment_id: time_segment_id,
                invalid,
                on_mounted,
            }
            if let Some(name) = name {
                input { r#type: "hidden", name, value: hidden_value }
            }
        }
    }
}
