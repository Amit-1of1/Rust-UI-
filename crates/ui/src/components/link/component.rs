use crate::components::icons::ExternalLink;
use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/link/style.css")]
struct Styles;

/// An inline text link: a plain `<a href>` with the kit's colours and focus ring.
///
/// Named `TextLink` so it does not clash with the router's `Link` when both preludes are
/// imported. For in-app routes, pass the router's `Link` the same class, or use this with
/// a path `href`.
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
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let t = use_ui_strings();
    rsx! {
        a {
            href,
            target: external.then_some("_blank"),
            rel: external.then_some("noopener noreferrer"),
            "aria-current": current.then_some("page"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_link }), attributes]),
            {children}
            if external {
                ExternalLink { class: Styles::dx_link_icon, size: "0.875em", "aria-hidden": "true" }
                span { class: Styles::dx_link_sr_only, " {t.opens_in_new_tab}" }
            }
        }
    }
}
