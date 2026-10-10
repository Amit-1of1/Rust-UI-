use dioxus::prelude::*;

#[css_module("/src/components/alert/style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

impl AlertVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
        }
    }
}

/// A callout. Announced to screen readers (`role="alert"`) when it appears.
#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_alert,
            role: "alert",
            "data-slot": "alert",
            "data-variant": variant.as_str(),
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn AlertTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_alert_title, "data-slot": "alert-title", ..attributes, {children} }
    }
}

#[component]
pub fn AlertDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_alert_description,
            "data-slot": "alert-description",
            ..attributes,
            {children}
        }
    }
}
