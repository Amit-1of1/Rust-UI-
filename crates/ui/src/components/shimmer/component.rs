use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/shimmer/style.css")]
struct Styles;

/// Wraps content that is still loading and sweeps a highlight across it. While `loading` is
/// true the wrapper is marked `aria-busy`. Under reduced motion the sweep becomes a static tint.
#[component]
pub fn Shimmer(
    #[props(default = true)] loading: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-loading": loading.then_some("true"),
            "aria-busy": loading.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_shimmer }), attributes]),
            {children}
        }
    }
}
