use crate::components::icons::ChevronDown;
use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    self, AccordionContentProps, AccordionItemProps, AccordionProps, AccordionTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/accordion/style.css")]
struct Styles;

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        accordion::Accordion {
            width: "15rem",
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_accordion }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        accordion::AccordionItem {
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_accordion_item }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        accordion::AccordionTrigger {
            id: props.id,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_accordion_trigger }), props.attributes]),
            {props.children}
            ChevronDown { "aria-hidden": "true",
                class: Styles::dx_accordion_expand_icon,
                size: "20px",
                stroke: "var(--secondary-color-4)",
            }
        }
    }
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    rsx! {
        accordion::AccordionContent {
            style: "--collapsible-content-width: 140px",
            id: props.id,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_accordion_content }), props.attributes]),
            {props.children}
        }
    }
}
