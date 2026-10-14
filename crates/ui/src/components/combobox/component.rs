use crate::components::icons::{Check, ChevronsUpDown};
use dioxus::prelude::*;
use dioxus_primitives::combobox::{
    self, default_combobox_filter, ComboboxEmptyProps, ComboboxOptionProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/combobox/style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    #[props(default)]
    pub default_value: Option<T>,

    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    #[props(default)]
    pub default_query: ReadSignal<String>,

    #[props(default)]
    pub on_query_change: Callback<String>,

    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    #[props(default)]
    pub placeholder: ReadSignal<String>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(default)]
    pub list_aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

/// Attributes that make it a form control. They belong on the text input (the focusable
/// control a `<label for>` points at), not on the wrapper `div`.
const INPUT_ATTRIBUTES: [&str; 5] = [
    "id",
    "aria-labelledby",
    "aria-describedby",
    "aria-invalid",
    "aria-required",
];

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox
    });
    let (input_attributes, root): (Vec<Attribute>, Vec<Attribute>) = props
        .attributes
        .into_iter()
        .partition(|a| INPUT_ATTRIBUTES.contains(&a.name));
    let input_id = input_attributes.iter().find_map(|a| match &a.value {
        dioxus::core::AttributeValue::Text(id) if a.name == "id" => Some(id.clone()),
        _ => None,
    });
    let input_attributes: Vec<Attribute> = input_attributes
        .into_iter()
        .filter(|a| a.name != "id")
        .collect();
    let merged = merge_attributes(vec![base, root]);

    rsx! {
        combobox::Combobox {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: merged,
            div { class: Styles::dx_combobox_input_wrapper,
                combobox::ComboboxInput {
                    class: Styles::dx_combobox_input,
                    placeholder: props.placeholder,
                    id: input_id,
                    aria_label: props.aria_label.clone(),
                    attributes: input_attributes,
                }
                ChevronsUpDown { "aria-hidden": "true",
                    class: Styles::dx_combobox_expand_icon,
                    size: "16px",
                }
            }
            combobox::ComboboxList {
                class: Styles::dx_combobox_list,
                aria_label: props.list_aria_label.clone(),
                {props.children}
            }
        }
    }
}

#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_empty
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxEmpty {
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_option
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            combobox::ComboboxItemIndicator {
                Check { "aria-hidden": "true",
                    class: Styles::dx_combobox_check_icon,
                    size: "16px",
                }
            }
        }
    }
}
