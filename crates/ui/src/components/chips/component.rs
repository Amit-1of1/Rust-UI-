use crate::components::icons::Check;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/chips/style.css")]
struct Styles;

/// A labelled group of [`Chip`]s, for filters and multi-choice tags.
#[component]
pub fn Chips(
    /// Names the group for screen readers, e.g. "Filter by status".
    #[props(into)]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            role: "group",
            "aria-label": label,
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_chips }), attributes]),
            {children}
        }
    }
}

/// One on/off chip: a native checkbox inside its label, so Tab reaches it, Space toggles it
/// and screen readers announce "checked". A check mark shows the state as well as the colour.
///
/// Uncontrolled with `default_checked`, or controlled with `checked` plus `on_change`.
#[component]
pub fn Chip(
    /// Controlled state. Leave unset to let the chip keep its own.
    checked: Option<bool>,
    /// Starting state when uncontrolled.
    #[props(default)]
    default_checked: bool,
    #[props(default)] disabled: bool,
    /// Form field name and value, when the chips sit in a `<form>`.
    #[props(into)]
    name: Option<String>,
    #[props(into)] value: Option<String>,
    #[props(default)] on_change: EventHandler<bool>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut own = use_signal(|| default_checked);
    let on = checked.unwrap_or_else(|| *own.read());
    rsx! {
        label {
            "data-state": if on { "on" } else { "off" },
            "data-disabled": disabled.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_chip }), attributes]),
            input {
                class: Styles::dx_chip_input,
                r#type: "checkbox",
                checked: on,
                disabled,
                name,
                value,
                onchange: move |e: FormEvent| {
                    let next = e.checked();
                    own.set(next);
                    on_change.call(next);
                },
            }
            span { class: Styles::dx_chip_check, "aria-hidden": "true",
                Check { size: "0.875rem", stroke_width: 3 }
            }
            span { {children} }
        }
    }
}
