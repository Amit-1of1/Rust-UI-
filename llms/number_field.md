# NumberField

- Copy into your app: `rust-ui add number_field` or `dx components add number_field --git https://github.com/Amit-1of1/Rust-UI-` (also copies `button`, `field`, `form`, `icons`, `ui_strings`).
- Import: `use crate::components::number_field::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

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
- Buttons: left out of the Tab order, as in the WAI-ARIA spin button pattern, and disabled at the ends. Typed text is applied on Enter or when the field loses focus. The keys are in the table below
- Forms: `NumberField` wires `aria-invalid` and `aria-describedby`, and a failed submit focuses the box
- Reduced motion: nothing animates

## Source

Original to this repo (booking app gap 12). Keyboard behaviour follows the WAI-ARIA APG spin button pattern.

## Stability

Experimental (docs/STABILITY.md): new on 2026-10-15; the `f64` value and the `empty` prop may change once forms need decimals or units.

## Keyboard and accessibility

Follows the WAI-ARIA [Spinbutton](https://www.w3.org/WAI/ARIA/apg/patterns/spinbutton/) pattern. Roles: `spinbutton`. Works on native (Blitz): partly.

| Key | When | What it does | Native |
|---|---|---|---|
| ArrowUp | In the field | Increases the value by one step. | yes |
| ArrowDown | In the field | Decreases the value by one step. | yes |
| Home | In the field | Sets the minimum. | yes |
| End | In the field | Sets the maximum. | yes |
| PageUp | In the field | Increases the value by ten steps, up to the maximum. | yes |
| PageDown | In the field | Decreases the value by ten steps, down to the minimum. | yes |
| Enter | After typing a number | Applies the typed value. | not yet |

Native: Typing a new value into the field does not apply on Enter yet.

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// A number with - and + buttons: a `role="spinbutton"` text box that Up/Down change by
/// `step`, Page Up/Down by ten steps, and Home/End set to `min`/`max`. Typing works too;
/// the value is kept between `min` and `max` when the box loses focus.
///
/// Controlled with `value` + `on_value_change` (a plain number, no signal needed), or
/// uncontrolled with `default_value`. `label` names the buttons ("Increase Guests") and,
/// unless `aria-label`, `aria-labelledby` or an `id` for a `<label for>` is passed, the box
/// itself. For `use_form`, use [`NumberField`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn NumberInput(
    #[props(into)] label: String,
    #[props(default)] value: Option<f64>,
    #[props(default)] default_value: f64,
    #[props(default)] min: Option<f64>,
    #[props(default)] max: Option<f64>,
    #[props(default = 1.0)] step: f64,
    #[props(default)] disabled: bool,
    /// Shows an empty box: no number has been entered yet. Typing, a key or a button sets one
    /// (starting from `value`).
    #[props(default)]
    empty: bool,
    #[props(default)] on_value_change: Callback<f64>,
    #[props(default)] onblur: Option<EventHandler<FocusEvent>>,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    /// Goes on the text box: `id`, `name`, `aria-*`, `class`, ...
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
) -> Element

/// A [`NumberInput`] bound to a `use_form` field, with a visible label, an optional
/// description and the field's error. The field holds the number as text: start it with
/// `form.field_with("2", ..)`, or leave it empty (the box is empty and `required()` applies).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn NumberField(
    field: FormField,
    #[props(into)] label: String,
    #[props(into, default)] description: Option<String>,
    #[props(default)] min: Option<f64>,
    #[props(default)] max: Option<f64>,
    #[props(default = 1.0)] step: f64,
    #[props(default)] disabled: bool,
    /// Anything else for the text box (`name`, `class`, ...). The field's wiring wins.
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
use crate::components::form::{required, use_form};
use crate::components::number_field::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut guests = use_signal(|| 2.0);
    let form = use_form();
    let nights = form.field_with("3", vec![required()]);
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 2rem; align-items: flex-start;",
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                span { id: "number-demo-guests", style: "font-size: 0.875rem; font-weight: 500;", "Guests" }
                NumberInput {
                    label: "Guests",
                    "aria-labelledby": "number-demo-guests",
                    value: guests(),
                    min: 1.0,
                    max: 8.0,
                    on_value_change: move |v| guests.set(v),
                }
            }
            NumberField { field: nights, label: "Nights", description: "Between 1 and 30.", min: 1.0, max: 30.0 }
        }
    }
}
```
