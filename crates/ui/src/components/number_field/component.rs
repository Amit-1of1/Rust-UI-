use crate::components::button::{ButtonVariant, IconButton, IconButtonSize};
use crate::components::field::{Field, FieldDescription, FieldError, FieldLabel};
use crate::components::form::FormField;
use crate::components::icons::{Minus, Plus};
use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/number_field/style.css")]
struct Styles;

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
) -> Element {
    let t = use_ui_strings();
    let mut own = use_signal(|| clamp(default_value, min, max));
    let current = value.unwrap_or(own());
    // What is in the box while someone types; `None` shows `current`.
    let mut draft = use_signal(|| None::<String>);

    let mut commit = move |v: f64| {
        let v = clamp(v, min, max);
        own.set(v);
        draft.set(None);
        if v != current || empty {
            on_value_change.call(v);
        }
    };

    let named = attributes
        .iter()
        .any(|a| matches!(a.name, "aria-label" | "aria-labelledby" | "id"));
    let wiring = attributes!(input {
        r#type: "text",
        role: "spinbutton",
        inputmode: if step.fract() == 0.0 && min.is_some_and(|m| m >= 0.0) {
            "numeric"
        } else {
            "decimal"
        },
        autocomplete: "off",
        aria_valuenow: (!empty).then(|| format_number(current)),
        aria_valuemin: min.map(format_number),
        aria_valuemax: max.map(format_number),
        aria_label: (!named).then(|| label.clone()),
        value: draft().unwrap_or_else(|| if empty {
            String::new()
        } else {
            format_number(current)
        }),
        disabled: disabled,
    });
    let at_min = min.is_some_and(|m| current <= m);
    let at_max = max.is_some_and(|m| current >= m);
    let decrease = fill(&t.decrease, &[("label", &label)]);
    let increase = fill(&t.increase, &[("label", &label)]);

    rsx! {
        div { class: Styles::dx_number_input, "data-disabled": disabled.then_some("true"),
            // The buttons are for pointers; keyboard users have the arrow keys, so they are
            // left out of the Tab order (WAI-ARIA spin button pattern).
            IconButton {
                label: decrease,
                variant: ButtonVariant::Outline,
                size: IconButtonSize::Sm,
                tabindex: "-1",
                disabled: disabled || at_min,
                onclick: move |_| commit(current - step),
                Minus { "aria-hidden": "true", size: "1rem" }
            }
            input {
                onkeydown: move |e: KeyboardEvent| {
                    let next = match e.key() {
                        Key::ArrowUp => current + step,
                        Key::ArrowDown => current - step,
                        Key::PageUp => current + step * 10.0,
                        Key::PageDown => current - step * 10.0,
                        Key::Home if min.is_some() => min.unwrap_or(current),
                        Key::End if max.is_some() => max.unwrap_or(current),
                        Key::Enter => match draft().and_then(|d| parse_number(&d)) {
                            Some(v) => v,
                            None => return,
                        },
                        _ => return,
                    };
                    e.prevent_default();
                    commit(next);
                },
                oninput: move |e: FormEvent| {
                    let text = e.value();
                    if let Some(v) = parse_number(&text) {
                        // Only report numbers in range while typing; out-of-range ones are
                        // clamped when the box loses focus.
                        if min.is_none_or(|m| v >= m) && max.is_none_or(|m| v <= m) && (v != current || empty) {
                            own.set(v);
                            on_value_change.call(v);
                        }
                    }
                    draft.set(Some(text));
                },
                onblur: move |e| {
                    match draft().map(|d| parse_number(&d)) {
                        Some(Some(v)) => commit(v),
                        Some(None) => draft.set(None),
                        None => {}
                    }
                    if let Some(f) = &onblur {
                        f.call(e);
                    }
                },
                onmounted: move |e| {
                    if let Some(f) = &onmounted {
                        f.call(e);
                    }
                },
                ..merge_attributes(vec![attributes!(input { class: Styles::dx_number_input_box }), attributes, wiring]),
            }
            IconButton {
                label: increase,
                variant: ButtonVariant::Outline,
                size: IconButtonSize::Sm,
                tabindex: "-1",
                disabled: disabled || at_max,
                onclick: move |_| commit(current + step),
                Plus { "aria-hidden": "true", size: "1rem" }
            }
        }
    }
}

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
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let described = field.described_by(description.is_some());
    let value = parse_number(&field.value());
    let wiring = attributes!(input {
        id: ids.control.clone(),
        aria_invalid: invalid.then_some("true"),
        aria_describedby: described,
    });
    rsx! {
        Field { invalid,
            FieldLabel { html_for: ids.control.clone(), "{label}" }
            NumberInput {
                label: label.clone(),
                value: value.unwrap_or(min.unwrap_or(0.0)),
                empty: value.is_none(),
                min,
                max,
                step,
                disabled,
                on_value_change: move |v: f64| field.set(format_number(v)),
                onblur: move |_| field.blur(),
                onmounted: move |e| field.mounted(e),
                attributes: merge_attributes(vec![attributes, wiring]),
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}

fn clamp(v: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let v = min.map_or(v, |m| v.max(m));
    max.map_or(v, |m| v.min(m))
}

/// `3` for whole numbers, `2.5` otherwise.
pub(crate) fn format_number(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{v:.0}")
    } else {
        v.to_string()
    }
}

/// Accepts `3`, ` 2.5 `, `-1` and `2,5`; `None` for anything else, including empty text.
pub(crate) fn parse_number(text: &str) -> Option<f64> {
    let text = text.trim().replace(',', ".");
    text.parse::<f64>().ok().filter(|v| v.is_finite())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_round_trip() {
        assert_eq!(format_number(3.0), "3");
        assert_eq!(format_number(2.5), "2.5");
        assert_eq!(parse_number(" 2,5 "), Some(2.5));
        assert_eq!(parse_number(""), None);
        assert_eq!(parse_number("abc"), None);
        assert_eq!(clamp(12.0, Some(1.0), Some(8.0)), 8.0);
        assert_eq!(clamp(-1.0, Some(1.0), None), 1.0);
    }
}
