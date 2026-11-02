use crate::components::sound::use_toggle_sound;
use dioxus::prelude::*;
use dioxus_primitives::switch::{self, SwitchProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/switch/style.css")]
struct Styles;

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    rsx! {
        switch::Switch {
            checked: props.checked,
            default_checked: props.default_checked,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            value: props.value,
            on_checked_change: use_toggle_sound(props.on_checked_change),
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_switch }), props.attributes]),
            switch::SwitchThumb { class: Styles::dx_switch_thumb }
        }
    }
}
