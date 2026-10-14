use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/spinner/style.css")]
struct Styles;

/// A loading indicator. `label` is read by screen readers.
#[component]
pub fn Spinner(
    /// Defaults to [`UiStrings::loading`](crate::components::ui_strings::UiStrings).
    #[props(into)]
    label: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let default_label = use_ui_strings().loading;
    let label = label.unwrap_or(default_label);
    rsx! {
        span { role: "status", aria_label: label, ..merge_attributes(vec![attributes!(div { class: Styles::dx_spinner }), attributes]) }
    }
}
