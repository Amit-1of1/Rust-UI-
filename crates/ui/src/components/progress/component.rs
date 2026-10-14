use dioxus::prelude::*;
use dioxus_primitives::progress::{self, ProgressProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/progress/style.css")]
struct Styles;

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        progress::Progress {
            value: props.value,
            max: props.max,
            attributes: merge_attributes(vec![attributes!(div { class: Styles::dx_progress }), props.attributes]),
            progress::ProgressIndicator { class: Styles::dx_progress_indicator }
        }
    }
}
