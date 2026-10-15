# DateTimePicker

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
