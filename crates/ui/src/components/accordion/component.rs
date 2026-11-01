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

/// The trigger's id for this item, so the panel can be a `region` named by its trigger (the
/// primitive gives the trigger `aria-controls` but leaves the panel unnamed). A trigger with
/// its own `id` needs the content to set `aria_labelledby` to that id.
#[derive(Clone)]
struct AccordionTriggerId(String);

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let id = crate::components::hooks::use_unique_id("rui-accordion-trigger");
    use_context_provider(|| AccordionTriggerId(id));
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
            id: props
                .id
                .or_else(|| try_use_context::<AccordionTriggerId>().map(|t| t.0)),
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
    // A named region, so moving into the panel with a screen reader reads its heading.
    let trigger = try_use_context::<AccordionTriggerId>().map(|t| t.0);
    rsx! {
        accordion::AccordionContent {
            style: "--collapsible-content-width: 140px",
            id: props.id,
            attributes: merge_attributes(vec![
                attributes!(div { class: Styles::dx_accordion_content, role: "region", aria_labelledby: trigger }),
                props.attributes,
            ]),
            {props.children}
        }
    }
}
