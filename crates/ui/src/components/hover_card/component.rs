use dioxus::prelude::*;
use dioxus_primitives::hover_card::{
    self, HoverCardContentProps, HoverCardProps, HoverCardTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
#[css_module("/src/components/hover_card/style.css")]
struct Styles;

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    rsx! {
        hover_card::HoverCard {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_hover_card }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    rsx! {
        hover_card::HoverCardTrigger {
            id: props.id,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_hover_card_trigger }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    rsx! {
        hover_card::HoverCardContent {
            side: props.side,
            align: props.align,
            id: props.id,
            force_mount: props.force_mount,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_hover_card_content }), props.attributes]),
            {props.children}
        }
    }
}
