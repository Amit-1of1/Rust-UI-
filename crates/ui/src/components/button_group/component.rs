use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/button_group/style.css")]
struct Styles;

/// Joins `Button`s (and inputs) into one segmented control. Give it an `aria-label` so the
/// group is announced; it carries `role="group"`.
#[component]
pub fn ButtonGroup(
    #[props(default)] vertical: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            role: "group",
            "data-orientation": if vertical { "vertical" } else { "horizontal" },
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_button_group }), attributes]),
            {children}
        }
    }
}

/// A non-interactive label segment inside a `ButtonGroup`.
#[component]
pub fn ButtonGroupText(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span { ..merge_attributes(vec![attributes!(div { class: Styles::dx_button_group_text }), attributes]), {children} }
    }
}
