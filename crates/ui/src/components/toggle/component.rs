use dioxus::prelude::*;
use dioxus_primitives::toggle::{self, ToggleProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/toggle/style.css")]
struct Styles;

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    rsx! {
        toggle::Toggle {
            pressed: props.pressed,
            default_pressed: props.default_pressed,
            disabled: props.disabled,
            on_pressed_change: props.on_pressed_change,
            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_toggle }), props.attributes]),
            {props.children}
        }
    }
}
