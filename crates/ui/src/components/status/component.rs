use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/status/style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum StatusKind {
    #[default]
    Neutral,
    Active,
    Warning,
    Error,
    Info,
}

impl StatusKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Active => "active",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Info => "info",
        }
    }
}

/// A coloured dot with a text label. The label is required: colour alone never carries the
/// meaning. `pulse` adds a soft ring animation (off under reduced motion).
#[component]
pub fn Status(
    #[props(default)] kind: StatusKind,
    #[props(default)] pulse: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span {
            "data-kind": kind.as_str(),
            "data-pulse": pulse.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_status }), attributes]),
            span { class: Styles::dx_status_dot, "aria-hidden": "true" }
            {children}
        }
    }
}
