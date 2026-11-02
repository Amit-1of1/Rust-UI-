use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/spinner/style.css")]
struct Styles;

/// A [`Spinner`]'s size.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum SpinnerSize {
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

impl SpinnerSize {
    pub fn class(&self) -> &'static str {
        match self {
            SpinnerSize::Sm => "sm",
            SpinnerSize::Default => "default",
            SpinnerSize::Lg => "lg",
            SpinnerSize::Xl => "xl",
        }
    }
}

/// A loading indicator. `label` is read by screen readers. The ring takes the text colour
/// of its parent, so inside a destructive button or an alert it follows that colour.
#[component]
pub fn Spinner(
    /// Defaults to [`UiStrings::loading`](crate::components::ui_strings::UiStrings). Say what
    /// is loading when the spinner stands alone ("Loading invoices").
    #[props(into)]
    label: Option<String>,
    #[props(default)] size: SpinnerSize,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let default_label = use_ui_strings().loading;
    let label = label.unwrap_or(default_label);
    rsx! {
        span {
            role: "status",
            aria_label: label,
            "data-size": size.class(),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_spinner }), attributes])
        }
    }
}
