use dioxus::prelude::*;

#[css_module("/src/components/spinner/style.css")]
struct Styles;

/// A loading indicator. `label` is read by screen readers.
#[component]
pub fn Spinner(
    #[props(into, default = "Loading".to_string())] label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span { class: Styles::dx_spinner, role: "status", aria_label: label, ..attributes }
    }
}
