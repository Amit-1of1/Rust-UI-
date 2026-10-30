# TimePicker

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

## Earliest and latest time

`min` and `max` limit the picker to a range, inclusive: `min: Time::from_hms(9, 0, 0).ok(), max: Time::from_hms(17, 0, 0).ok()`. Up, Down, Home and End only stop on hours and minutes that can still make an allowed time (17 is the last hour, and at 17 the only minute is :00). Typing is not blocked: a typed 18:30 stays on screen, is reported in `on_value_change` and is marked invalid, the way React Aria's TimeField handles it. A `max` earlier than `min` means the range runs past midnight (22:00 to 02:00, a night shift). `TimeField` takes the same props and adds a range rule itself, with the message from `UiStrings::time_out_of_range` ("Choose a time from 9:00 AM to 5:00 PM."); `time_between(min, max, message)` is that rule for your own fields.

In a `use_form` form, use `TimeField { field, label: "Start" }`: the field holds `14:05` (empty until complete, so `required()` works), is labelled, marked invalid with its error linked, and gets focus on a failed submit. For a time with a date, see `DateTimePicker`. When a plain native control is enough (mobile-first forms), `TextField { r#type: "time" }` gives the browser's own time input.

## Accessibility

- Role and name: a `role="group"` named by `label` (default "Time") or by a visible label through `labelled_by`. Each segment is an `<input role="spinbutton">` named "Hours", "Minutes", "Seconds" or "AM/PM" (from `UiStrings`), plus the visible label when `labelled_by` is set, with `aria-valuenow`, `aria-valuemin`, `aria-valuemax` and `aria-valuetext` ("Empty" while empty). `first_segment_id` lets a `<label for>` reach the hours.
- Typing: digits fill the focused segment and move on after two digits, or at once when no second digit could follow ("3" in hours). A or P (the first letters of `UiStrings::am`/`pm`) set AM/PM. Tab moves through the segments and out. The other keys are in the table below.
- Out of range: a time outside `min`/`max` sets `aria-invalid` on every segment; `TimeField` also shows the range message.
- Touch: segments are real inputs with `inputmode="numeric"`, so phones show a number pad.
- Reduced motion: no animation.

## Source

Original to this repo. The segment behaviour follows the WAI-ARIA spin button pattern and React Aria's TimeField (behaviour only, nothing copied). dioxus-primitives' date segments are private and `contenteditable`, so they could not be reused.

## Stability

Experimental. New on 2026-10-15, `min`/`max` on 2026-10-30; the props may change once a real app uses it. See `docs/STABILITY.md` in the kit repository.
