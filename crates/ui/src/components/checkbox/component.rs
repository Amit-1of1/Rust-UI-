use crate::components::icons::Check;
use dioxus::prelude::*;
/// Re-exported so apps can read and set the checked state without depending on
/// dioxus-primitives themselves.
pub use dioxus_primitives::checkbox::CheckboxState;
use dioxus_primitives::checkbox::{self, CheckboxProps};

#[css_module("/src/components/checkbox/style.css")]
struct Styles;

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        checkbox::Checkbox {
            class: Styles::dx_checkbox,
            checked: props.checked,
            default_checked: props.default_checked,
            required: props.required,
            disabled: props.disabled,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            checkbox::CheckboxIndicator { class: Styles::dx_checkbox_indicator,
                Check { "aria-hidden": "true", size: "1rem" }
            }
        }
    }
}
