use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectMultiProps, SelectOptionProps, SelectProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::sync::atomic::{AtomicUsize, Ordering};

pub use dioxus_primitives::select::SelectGroup;

#[css_module("/src/components/select/style.css")]
struct Styles;

/// How the trigger button is named. `aria-label` and `aria-labelledby` given to `Select`
/// arrive with the root's attributes; on the root `div` they would name nothing, so they
/// are moved to the trigger. The trigger also lists itself in `aria-labelledby`, so a
/// screen reader announces the label and the current value ("Fruit, Apple"), the way a
/// native `<select>` does. A plain `aria-label` on a button would replace the value.
struct TriggerName {
    root: Vec<Attribute>,
    trigger_id: String,
    label_id: String,
    label: Option<String>,
    labelledby: Option<String>,
}

fn use_trigger_name(attributes: Vec<Attribute>) -> TriggerName {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    let id = use_hook(|| NEXT_ID.fetch_add(1, Ordering::Relaxed));
    let (trigger_id, label_id) = (
        format!("rui-select-{id}-trigger"),
        format!("rui-select-{id}-label"),
    );
    let mut label = None;
    let mut by = None;
    let root = attributes
        .into_iter()
        .filter(|a| {
            let slot = match a.name {
                "aria-label" => &mut label,
                "aria-labelledby" => &mut by,
                _ => return true,
            };
            if let AttributeValue::Text(t) = &a.value {
                *slot = Some(t.clone()).filter(|t| !t.trim().is_empty());
            }
            false
        })
        .collect();
    let labelledby = match (&by, &label) {
        (Some(ids), _) => Some(format!("{ids} {trigger_id}")),
        (None, Some(_)) => Some(format!("{label_id} {trigger_id}")),
        (None, None) => None,
    };
    TriggerName {
        root,
        label: label.filter(|_| by.is_none()),
        trigger_id,
        label_id,
        labelledby,
    }
}

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select
    });
    let name = use_trigger_name(props.attributes);
    let merged = merge_attributes(vec![base, name.root]);

    rsx! {
        select::Select {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            if let Some(label) = name.label {
                span { id: name.label_id, hidden: true, "{label}" }
            }
            select::SelectTrigger {
                class: Styles::dx_select_trigger,
                id: name.trigger_id,
                aria_labelledby: name.labelledby,
                select::SelectValue {}
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: Styles::dx_select_list,
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select
    });
    let name = use_trigger_name(props.attributes);
    let merged = merge_attributes(vec![base, name.root]);

    rsx! {
        select::SelectMulti {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            if let Some(label) = name.label {
                span { id: name.label_id, hidden: true, "{label}" }
            }
            select::SelectTrigger {
                class: Styles::dx_select_trigger,
                id: name.trigger_id,
                aria_labelledby: name.labelledby,
                select::SelectValue {}
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: Styles::dx_select_list,
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_group_label
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectGroupLabel {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_option
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            select::SelectItemIndicator {
                Check {
                    size: "1rem",
                    stroke: "var(--secondary-color-5)",
                }
            }
        }
    }
}
