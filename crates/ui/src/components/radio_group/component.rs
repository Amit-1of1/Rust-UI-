use dioxus::prelude::*;
use dioxus_primitives::radio_group::{self, RadioGroupProps, RadioItemProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/radio_group/style.css")]
struct Styles;

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    rsx! {
        radio_group::RadioGroup {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_radio_group }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn RadioItem(props: RadioItemProps) -> Element {
    rsx! {
        radio_group::RadioItem {
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            // `class` is a named prop on the primitive's item, not one of its attributes.
            class: match props.class {
                Some(class) => format!("{} {class}", Styles::dx_radio_item),
                None => Styles::dx_radio_item.to_string(),
            },
            attributes: props.attributes,
            {props.children}
        }
    }
}
