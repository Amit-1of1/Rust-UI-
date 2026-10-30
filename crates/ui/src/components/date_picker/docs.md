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
