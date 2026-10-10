use dioxus::prelude::*;

#[css_module("/src/components/callout/style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum CalloutVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
}

impl CalloutVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}

/// An inline note for docs and forms. Unlike `Alert` it is not announced when it appears, so use
/// it for static content; use `Alert` for messages that must interrupt.
#[component]
pub fn Callout(
    #[props(default)] variant: CalloutVariant,
    /// Optional bold heading.
    #[props(into, default)]
    title: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_callout,
            role: "note",
            "data-variant": variant.as_str(),
            ..attributes,
            if let Some(title) = title {
                p { class: Styles::dx_callout_title, "{title}" }
            }
            div { class: Styles::dx_callout_body, {children} }
        }
    }
}
