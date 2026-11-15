use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/alert/style.css")]
struct Styles;

/// An alert's role, shown by its colour (and, with `icon`, by an icon, so colour is not the
/// only cue).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Success,
    Warning,
    Info,
}

impl AlertVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

/// How an [`Alert`] is announced.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum AlertLive {
    /// `role="alert"`: read at once, interrupting. For errors and anything urgent.
    #[default]
    Assertive,
    /// `role="status"`: read when the screen reader is idle. For success and progress notes.
    Polite,
    /// No live region: for a message that is on the page from the start (a live region only
    /// announces changes, and many alerts on load are noise).
    Off,
}

/// A message about the page or a part of it. Announced to screen readers (`role="alert"`)
/// when it appears, unless `live` says otherwise.
#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    /// How it is announced: at once (default), politely, or not at all.
    #[props(default)]
    live: AlertLive,
    /// An icon before the text, such as `CircleAlert`. Mark it `aria-hidden`; the title says
    /// what the icon shows.
    #[props(default)]
    icon: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let role = match live {
        AlertLive::Assertive => Some("alert"),
        AlertLive::Polite => Some("status"),
        AlertLive::Off => None,
    };
    let has_icon = icon.is_some();
    rsx! {
        div {
            role,
            "data-slot": "alert",
            "data-variant": variant.as_str(),
            "data-icon": has_icon.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_alert }), attributes]),
            if let Some(icon) = icon {
                span { class: Styles::dx_alert_icon, aria_hidden: "true", {icon} }
            }
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
        div { "data-slot": "alert-title", ..merge_attributes(vec![attributes!(div { class: Styles::dx_alert_title }), attributes]), {children} }
    }
}

#[component]
pub fn AlertDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "alert-description",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_alert_description }), attributes]),
            {children}
        }
    }
}

/// Buttons or links that answer the alert ("Retry", "Undo", "View details"), below the text.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn AlertActions(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "alert-actions",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_alert_actions }), attributes]),
            {children}
        }
    }
}
