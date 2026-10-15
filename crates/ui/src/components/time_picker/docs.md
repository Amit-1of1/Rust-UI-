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
