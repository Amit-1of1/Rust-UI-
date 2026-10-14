use dioxus::prelude::*;
use dioxus_primitives::label::{self, LabelProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
#[css_module("/src/components/label/style.css")]
struct Styles;

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label::Label {
            html_for: props.html_for,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_label }), props.attributes]),
            {props.children}
        }
    }
}
