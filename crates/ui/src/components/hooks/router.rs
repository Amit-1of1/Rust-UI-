//! In-app navigation for the kit's links when the Dioxus router is in use (feature `router`).
//!
//! The kit's links (`TextLink`, `BreadcrumbLink`, `PaginationLink`) render a plain `<a href>`.
//! With the `router` feature on and a `Router` above them, a plain left click on a path the
//! router knows goes through the router instead of loading the page again, the way the
//! router's own `Link` behaves. Without the feature, or outside a router, they stay plain
//! links. Experimental (docs/STABILITY.md).

use dioxus::prelude::*;

/// The `href` to render for `href`: inside a router with a base path (for example a site
/// served from `/my-app/`), an in-app path gets that prefix, so opening it in a new tab or
/// copying it works. Anything else is returned unchanged.
pub fn in_app_href(href: &str) -> String {
    #[cfg(feature = "router")]
    if is_path(href) {
        if let Some(router) = dioxus::router::try_router() {
            if router.prefix().is_some() && is_route(href) {
                return router.prefix().unwrap_or_default() + href;
            }
        }
    }
    href.to_string()
}

/// Sends a plain left click on a link to `href` through the router, when there is one and
/// it knows the path. Returns `true` when it did (the browser's own navigation is then
/// cancelled). Clicks with a modifier key (new tab, new window), other buttons, links with
/// a `target`, and clicks whose default was already prevented are left alone.
pub fn follow_in_app(href: &str, event: &MouseEvent) -> bool {
    #[cfg(feature = "router")]
    {
        use dioxus::html::input_data::MouseButton;
        if !event.default_action_enabled()
            || !event.modifiers().is_empty()
            || event.trigger_button() != Some(MouseButton::Primary)
            || !is_path(href)
            || !is_route(href)
        {
            return false;
        }
        if let Some(router) = dioxus::router::try_router() {
            event.prevent_default();
            router.push(href.to_string());
            return true;
        }
    }
    #[cfg(not(feature = "router"))]
    let _ = (href, event);
    false
}

/// Whether `href` is the path the router is showing now, for `aria-current="page"`.
pub fn is_current_route(href: &str) -> bool {
    #[cfg(feature = "router")]
    if is_path(href) {
        if let Some(router) = dioxus::router::try_router() {
            return router.full_route_string() == href;
        }
    }
    let _ = href;
    false
}

/// The router's current path, or `None` without the `router` feature or outside a `Router`.
/// Reading it subscribes the calling component to navigation.
pub(crate) fn current_route() -> Option<String> {
    #[cfg(feature = "router")]
    {
        dioxus::router::try_router().map(|r| r.full_route_string())
    }
    #[cfg(not(feature = "router"))]
    None
}

/// A same-site absolute path: `/stays/3`, not `//cdn.example`, `#top`, `https://..` or `mailto:`.
#[cfg(feature = "router")]
fn is_path(href: &str) -> bool {
    href.starts_with('/') && !href.starts_with("//")
}

/// Whether the app's route enum parses `href` (the same test the router's `Link` uses).
#[cfg(feature = "router")]
fn is_route(href: &str) -> bool {
    use dioxus::router::NavigationTarget;
    matches!(
        <NavigationTarget>::from(href),
        NavigationTarget::Internal(_)
    )
}
