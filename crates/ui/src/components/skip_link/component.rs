use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::rc::Rc;

#[css_module("/src/components/skip_link/style.css")]
struct Styles;

/// The id [`SkipLink`] points at and [`MainContent`] carries unless told otherwise.
pub const MAIN_CONTENT_ID: &str = "main-content";

/// A "Skip to content" link for keyboard users: hidden until it has focus, then shown at
/// the top corner of the nearest positioned ancestor. Put it first in the page, before the
/// header and navigation, and point it at the page's [`MainContent`].
#[component]
pub fn SkipLink(
    /// The id of the element to skip to, without `#`.
    #[props(into, default = MAIN_CONTENT_ID.to_string())]
    target: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    /// The link text. Defaults to `UiStrings::skip_to_content`.
    children: Option<Element>,
) -> Element {
    let t = use_ui_strings();
    rsx! {
        a {
            href: "#{target}",
            ..merge_attributes(vec![attributes!(a { class: Styles::dx_skip_link }), attributes]),
            if let Some(children) = children {
                {children}
            } else {
                "{t.skip_to_content}"
            }
        }
    }
}

/// The page's `<main>` landmark: the target of [`SkipLink`] (`id="main-content"`,
/// focusable from script with `tabindex="-1"`).
///
/// With the kit's `router` feature, inside a `Router`, it also takes focus after an in-app
/// navigation (not on the first load), so keyboard and screen reader users start at the new
/// page instead of on the link they used. Give the page a heading inside it.
#[component]
pub fn MainContent(
    #[props(into, default = MAIN_CONTENT_ID.to_string())] id: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut el = use_signal(|| None::<Rc<MountedData>>);
    // Reading the route subscribes this component to it, so the effect below sees every
    // navigation.
    let route = current_route();
    let mut seen = use_signal(|| route.clone());
    use_effect(use_reactive!(|route| {
        if *seen.peek() != route {
            seen.set(route);
            if let Some(el) = el.peek().clone() {
                spawn(async move {
                    _ = el.set_focus(true).await;
                });
            }
        }
    }));
    rsx! {
        main {
            id,
            tabindex: "-1",
            onmounted: move |e| el.set(Some(e.data())),
            ..merge_attributes(vec![attributes!(main { class: Styles::dx_main_content }), attributes]),
            {children}
        }
    }
}

/// The router's current path, or `None` without the `router` feature or outside a `Router`.
fn current_route() -> Option<String> {
    #[cfg(feature = "router")]
    {
        dioxus::router::try_router().map(|r| r.full_route_string())
    }
    #[cfg(not(feature = "router"))]
    None
}
