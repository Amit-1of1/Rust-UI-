# NumberField

`NumberInput` is a number with - and + buttons, for counts and amounts (guests, quantity, nights). `NumberField` is the same control bound to a `use_form` field, with a label, a description and the field's error.

```rust
let mut guests = use_signal(|| 2.0);
rsx! {
    NumberInput { label: "Guests", value: guests(), min: 1.0, max: 8.0, on_value_change: move |v| guests.set(v) }
}

let form = use_form();
let nights = form.field_with("2", vec![required()]);
rsx! { NumberField { field: nights, label: "Nights", min: 1.0, max: 30.0 } }
```

`value` is a plain number (no signal needed); leave it out and use `default_value` for an uncontrolled one. `step` defaults to 1. Typed numbers outside `min`/`max` are pulled back into range when the box loses focus. `NumberField` keeps the number as text in the form field (`"2"`, `"2.5"`), so `field.value().parse()` reads it back.

## Accessibility

- Role and name: the box is `role="spinbutton"` with `aria-valuenow`, `aria-valuemin` and `aria-valuemax`; it is named by `label` (or by the visible label in `NumberField`). The buttons are named "Decrease {label}" and "Increase {label}" (`UiStrings::decrease` / `increase`)
- Keyboard: Up/Down change it by `step`, Page Up/Page Down by ten steps, Home/End go to `min`/`max`, Enter applies typed text. The buttons are left out of the Tab order, as in the WAI-ARIA spin button pattern, and disable themselves at the ends
- Forms: `NumberField` wires `aria-invalid` and `aria-describedby`, and a failed submit focuses the box
- Reduced motion: nothing animates

## Source

Original to this repo (booking app gap 12). Keyboard behaviour follows the WAI-ARIA APG spin button pattern.

## Stability

Experimental (docs/STABILITY.md): new on 2026-10-15; the `f64` value and the `empty` prop may change once forms need decimals or units.
