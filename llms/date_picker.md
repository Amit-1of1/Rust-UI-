# Date picker

- Copy into your app: `atoll add date_picker` or `dx components add date_picker --git https://github.com/Amit-1of1/Rust-UI-` (also copies `calendar`, `form`, `icons`, `popover`, `ui_strings`).
- Import: `use crate::components::date_picker::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

The DatePicker component is used to display a date input and a Calendar popover, allowing users to enter or select a date value.

## Component Structure

```rust
DatePicker {
    // The currently selected date in the date picker (if any).
    selected_date,
    on_value_change: move |v: Option<Date>| {
        // This callback is triggered when a date is selected in the
        // calendar or the user entered it from the keyboard.
        // The date parameter contains the selected date.
    },
    // Optional number of pre-composed calendar months to show in the popover.
    month_count: 1,
    // Optional placeholder formatters for the input fields.
    on_format_day_placeholder: || "D",
    on_format_month_placeholder: || "M",
    on_format_year_placeholder: || "Y",
}
```

The styled `DatePicker` and `DateRangePicker` render the input, trigger, popover content, and calendar by default.

## Naming

Name a picker from its visible label: `DateRangePicker { aria_labelledby: "dates-label", .. }` (or `aria_label`). The picker's group then carries that name; the English "Date" / "Date Range" is only used when you pass none. The segments are named "year", "month" and "day" inside it, and a segment moves on to the next once it holds all its digits ("01" for January).

## In a form

`DateRangeField` (in the form component) binds a `DateRangePicker` to a `use_form` field, labelled, described and validated like the other fields. Its value is `2026-10-15/2026-10-20`; `format_iso_date_range` and `parse_iso_date_range` write and read that, and `format_iso_date` / `parse_iso_date` do the same for one date (`2026-10-15`, the `<input type="date">` format).

## API

```rust
pub use dioxus_primitives::calendar::DateRange;

pub use time::{Date, Month};

/// `"2026-10-15"`: the format of `<input type="date">`.
pub fn format_iso_date(date: Date) -> String

/// Reads `"2026-10-15"`; anything else (including a date that doesn't exist) is `None`.
pub fn parse_iso_date(text: &str) -> Option<Date>

/// `"2026-10-15/2026-10-20"`: an ISO 8601 interval of two dates, the format
/// [`DateRangeField`](crate::components::form::DateRangeField) keeps in a form field.
pub fn format_iso_date_range(range: DateRange) -> String

/// Reads `"2026-10-15/2026-10-20"`. The dates may come in either order; the range is always
/// start to end.
pub fn parse_iso_date_range(text: &str) -> Option<DateRange>

pub struct DatePickerProps {
    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<Option<Date>>,

    /// The selected date
    #[props(default)]
    pub selected_date: ReadSignal<Option<Date>>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the date picker is enable user input
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub read_only: ReadSignal<bool>,

    /// Lower limit of the range of available dates
    #[props(default = fixed_date(1925, Month::January, 1))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = fixed_date(2050, Month::December, 31))]
    pub max_date: Date,

    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// Specify how many months are visible at once
    #[props(default = 1)]
    pub month_count: u8,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

pub struct DateRangePickerProps {
    /// Callback when value changes
    #[props(default)]
    pub on_range_change: Callback<Option<DateRange>>,

    /// The selected date
    #[props(default)]
    pub selected_range: ReadSignal<Option<DateRange>>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the date picker is enable user input
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub read_only: ReadSignal<bool>,

    /// Lower limit of the range of available dates
    #[props(default = fixed_date(1925, Month::January, 1))]
    pub min_date: Date,

    /// Upper limit of the range of available dates
    #[props(default = fixed_date(2050, Month::December, 31))]
    pub max_date: Date,

    /// Callback when display day placeholder
    #[props(default = Callback::new(|_| "D".to_string()))]
    pub on_format_day_placeholder: Callback<(), String>,

    /// Callback when display month placeholder
    #[props(default = Callback::new(|_| "M".to_string()))]
    pub on_format_month_placeholder: Callback<(), String>,

    /// Callback when display year placeholder
    #[props(default = Callback::new(|_| "Y".to_string()))]
    pub on_format_year_placeholder: Callback<(), String>,

    /// Specify how many months are visible at once
    #[props(default = 1)]
    pub month_count: u8,

    /// Unavailable dates
    #[props(default)]
    pub disabled_ranges: ReadSignal<Vec<DateRange>>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element

#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element
```

## Example

```rust
use crate::components::button::Button;
use crate::components::date_picker::*;
use crate::components::form::*;
use dioxus::prelude::*;

use time::Date;

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut selected_range = use_signal(|| None::<DateRange>);
    let form = use_form();
    let stay = form.field(vec![
        custom(|v| {
            if v.is_empty() {
                Err("Choose your dates.".to_string())
            } else {
                Ok(())
            }
        }),
        days_between(Some(2), Some(14), "Stays are 2 to 14 nights."),
    ]);
    let mut booked = use_signal(|| None::<String>);

    rsx! {
        div { style: "display: grid; gap: 1rem; justify-items: start;",
            DatePicker {
                selected_date: selected_date(),
                on_value_change: move |v| {
                    tracing::info!("Selected date changed: {:?}", v);
                    selected_date.set(v);
                },
            }
            DateRangePicker {
                selected_range: selected_range(),
                on_range_change: move |r| selected_range.set(r),
            }
            Form {
                state: form,
                on_valid: move |_| booked.set(Some(stay.value())),
                DateRangeField {
                    field: stay,
                    label: "Stay",
                    description: "Check-in and check-out, 2 to 14 nights.",
                }
                Button { r#type: "submit", "Check availability" }
            }
            if let Some(range) = booked() {
                p { role: "status", style: "margin: 0;", "Looking for {range}" }
            }
        }
    }
}
```
