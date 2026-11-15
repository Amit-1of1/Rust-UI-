use crate::components::hooks::{follow_in_app, in_app_href, is_current_route};
use crate::components::icons::ExternalLink;
use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/link/style.css")]
struct Styles;

/// A [`TextLink`]'s colour role.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum LinkVariant {
    /// The kit's link colour, medium weight.
    #[default]
    Default,
    /// Secondary text colour, for footers and meta lines.
    Muted,
    /// Takes the colour and weight of the text around it (in an alert, a heading, a
    /// coloured message), keeping only the underline.
    Inherit,
    /// The error text colour, for "Delete account" style links.
    Destructive,
}

impl LinkVariant {
    pub fn class(&self) -> &'static str {
        match self {
            LinkVariant::Default => "default",
            LinkVariant::Muted => "muted",
            LinkVariant::Inherit => "inherit",
            LinkVariant::Destructive => "destructive",
        }
    }
}

/// An inline text link: a plain `<a href>` with the kit's colours and focus ring.
///
/// Named `TextLink` so it does not clash with the router's `Link` when both preludes are
/// imported. With the kit's `router` feature, a path `href` inside a `Router` navigates in
/// the app and is marked `aria-current="page"` while it is the current route.
#[component]
pub fn TextLink(
    #[props(into)] href: String,
    /// Opens in a new tab, with `rel="noopener noreferrer"`, an icon, and
    /// "(opens in a new tab)" for screen readers.
    #[props(default)]
    external: bool,
    /// Marks the link for the page you are on (`aria-current="page"`), e.g. in a nav list.
    #[props(default)]
    current: bool,
    #[props(default)] variant: LinkVariant,
    /// Not followable for now: no `href`, `aria-disabled="true"`, out of the tab order and
    /// drawn dimmed. Screen readers still find it as a link in the text.
    #[props(default)]
    disabled: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let t = use_ui_strings();
    if disabled {
        return rsx! {
            a {
                role: "link",
                aria_disabled: "true",
                "data-variant": variant.class(),
                ..merge_attributes(vec![attributes!(div { class: Styles::dx_link }), attributes]),
                {children}
            }
        };
    }
    let current = current || (!external && is_current_route(&href));
    let shown = in_app_href(&href);
    rsx! {
        a {
            href: shown,
            target: external.then_some("_blank"),
            rel: external.then_some("noopener noreferrer"),
            "aria-current": current.then_some("page"),
            "data-variant": variant.class(),
            onclick: move |e| {
                if !external {
                    follow_in_app(&href, &e);
                }
            },
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_link }), attributes]),
            {children}
            if external {
                ExternalLink { class: Styles::dx_link_icon, size: "0.875em", "aria-hidden": "true" }
                span { class: Styles::dx_link_sr_only, " {t.opens_in_new_tab}" }
            }
        }
    }
}
