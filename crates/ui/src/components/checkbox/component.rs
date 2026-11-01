use crate::components::icons::{Check, Minus};
use dioxus::prelude::*;
/// Re-exported so apps can read and set the checked state without depending on
/// dioxus-primitives themselves.
pub use dioxus_primitives::checkbox::CheckboxState;
use dioxus_primitives::checkbox::{self, CheckboxProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/checkbox/style.css")]
struct Styles;

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        checkbox::Checkbox {
            checked: props.checked,
            default_checked: props.default_checked,
            required: props.required,
            disabled: props.disabled,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_checkbox }), props.attributes]),
            checkbox::CheckboxIndicator { class: Styles::dx_checkbox_indicator,
                // One of the two shows, from the checkbox's `data-state` (style.css).
                Check { class: Styles::dx_checkbox_check, "aria-hidden": "true", size: "1rem" }
                Minus { class: Styles::dx_checkbox_mixed, "aria-hidden": "true", size: "1rem" }
            }
        }
    }
}

/// A set of related checkboxes under one visible label: a `<fieldset>` with a `<legend>`, so a
/// screen reader reads the group's name when focus enters it. Put each `Checkbox` with its
/// `Label` inside. For a "select all" box, give it `CheckboxState::Indeterminate` while only
/// some are checked.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn CheckboxGroup(
    /// The group's visible name ("Notify me about").
    #[props(into)]
    label: String,
    /// Shows the group as invalid; pair it with a text error that says what to fix.
    #[props(default)]
    invalid: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        fieldset {
            "data-invalid": invalid.then_some("true"),
            ..merge_attributes(vec![attributes!(fieldset { class: Styles::dx_checkbox_group }), attributes]),
            legend { class: Styles::dx_checkbox_group_label, {label} }
            {children}
        }
    }
}
