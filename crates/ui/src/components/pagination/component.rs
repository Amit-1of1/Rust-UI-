use crate::components::hooks::{follow_in_app, in_app_href};
use crate::components::icons::{ChevronLeft, ChevronRight, Ellipsis};
use crate::components::ui_strings::use_ui_strings;
use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
#[css_module("/src/components/pagination/style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum PaginationLinkSize {
    #[default]
    Icon,
    Default,
}

impl PaginationLinkSize {
    pub fn class(&self) -> &'static str {
        match self {
            PaginationLinkSize::Icon => "icon",
            PaginationLinkSize::Default => "default",
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum PaginationLinkKind {
    Previous,
    Next,
}

impl PaginationLinkKind {
    pub fn attr(&self) -> &'static str {
        match self {
            PaginationLinkKind::Previous => "previous",
            PaginationLinkKind::Next => "next",
        }
    }
}

#[component]
pub fn Pagination(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        nav {
            "data-slot": "pagination",
            role: "navigation",
            aria_label: "pagination",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_pagination }), attributes]),
            {children}
        }
    }
}

#[component]
pub fn PaginationContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        ul {
            "data-slot": "pagination-content",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_pagination_content }), attributes]),
            {children}
        }
    }
}

#[component]
pub fn PaginationItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        li {
            "data-slot": "pagination-item",
            ..attributes,
            {children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationLinkProps {
    #[props(default)]
    pub is_active: bool,
    #[props(default)]
    pub size: PaginationLinkSize,
    #[props(default)]
    pub data_kind: Option<PaginationLinkKind>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn PaginationLink(mut props: PaginationLinkProps) -> Element {
    let aria_current = if props.is_active { Some("page") } else { None };
    let data_kind = props.data_kind.map(|kind| kind.attr());
    // With the kit's `router` feature, a path `href` goes through the router (hooks/router.rs).
    let mut href = None;
    for a in props.attributes.iter_mut().filter(|a| a.name == "href") {
        if let AttributeValue::Text(t) = &a.value {
            href = Some(t.clone());
            a.value = AttributeValue::Text(in_app_href(t));
        }
    }
    rsx! {
        a {
            "data-slot": "pagination-link",
            "data-active": props.is_active,
            "data-size": props.size.class(),
            "data-kind": data_kind,
            aria_current: aria_current,
            onclick: move |event: MouseEvent| {
                if let Some(f) = &props.onclick {
                    f.call(event.clone());
                }
                if let Some(href) = &href {
                    follow_in_app(href, &event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &props.onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &props.onmouseup {
                    f.call(event);
                }
            },
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_pagination_link }), props.attributes]),
            {props.children}
        }
    }
}

#[component]
pub fn PaginationPrevious(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    let t = use_ui_strings();
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            aria_label: t.go_to_previous_page,
            data_kind: Some(PaginationLinkKind::Previous),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            ChevronLeft { "aria-hidden": "true", size: "1rem" }
            span { class: Styles::dx_pagination_label, {t.previous} }
        }
    }
}

#[component]
pub fn PaginationNext(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    let t = use_ui_strings();
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            aria_label: t.go_to_next_page,
            data_kind: Some(PaginationLinkKind::Next),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            span { class: Styles::dx_pagination_label, {t.next} }
            ChevronRight { "aria-hidden": "true", size: "1rem" }
        }
    }
}

#[component]
pub fn PaginationEllipsis(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span {
            "data-slot": "pagination-ellipsis",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_pagination_ellipsis }), attributes]),
            Ellipsis { "aria-hidden": "true", size: "1rem" }
            span { class: Styles::dx_sr_only, "More pages" }
        }
    }
}

/// Client-side paging: Previous, numbered pages and Next as buttons, for a list that pages
/// in place (no URL per page). `page` is 1-based. Previous and Next are `aria-disabled` on
/// the first and last page, so they keep focus but do nothing; long ranges collapse to
/// "1 … 4 5 6 … 20" (`siblings` pages each side of the current one).
///
/// For pages that are URLs, compose [`Pagination`] with [`PaginationLink`] instead.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn Pager(
    page: usize,
    pages: usize,
    on_page_change: EventHandler<usize>,
    #[props(default = 1)] siblings: usize,
    /// Goes on the `<nav>`: give it an `aria-label` ("Pages of results").
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let t = use_ui_strings();
    let pages = pages.max(1);
    let page = page.clamp(1, pages);
    let first = page == 1;
    let last = page == pages;
    rsx! {
        Pagination { attributes,
            PaginationContent {
                PaginationItem {
                    button {
                        r#type: "button",
                        class: "{Styles::dx_pagination_link} {Styles::dx_pagination_button}",
                        "data-size": PaginationLinkSize::Default.class(),
                        "data-kind": PaginationLinkKind::Previous.attr(),
                        "data-active": false,
                        aria_label: t.go_to_previous_page.clone(),
                        aria_disabled: first.then_some("true"),
                        onclick: move |_| {
                            if !first {
                                on_page_change.call(page - 1);
                            }
                        },
                        ChevronLeft { "aria-hidden": "true", size: "1rem" }
                        span { class: Styles::dx_pagination_label, {t.previous.clone()} }
                    }
                }
                for (i, item) in page_items(page, pages, siblings).into_iter().enumerate() {
                    PaginationItem { key: "{i}-{item:?}",
                        if let Some(p) = item {
                            button {
                                r#type: "button",
                                class: "{Styles::dx_pagination_link} {Styles::dx_pagination_button}",
                                "data-size": PaginationLinkSize::Icon.class(),
                                "data-active": p == page,
                                aria_current: (p == page).then_some("page"),
                                onclick: move |_| {
                                    if p != page {
                                        on_page_change.call(p);
                                    }
                                },
                                "{p}"
                            }
                        } else {
                            PaginationEllipsis {}
                        }
                    }
                }
                PaginationItem {
                    button {
                        r#type: "button",
                        class: "{Styles::dx_pagination_link} {Styles::dx_pagination_button}",
                        "data-size": PaginationLinkSize::Default.class(),
                        "data-kind": PaginationLinkKind::Next.attr(),
                        "data-active": false,
                        aria_label: t.go_to_next_page.clone(),
                        aria_disabled: last.then_some("true"),
                        onclick: move |_| {
                            if !last {
                                on_page_change.call(page + 1);
                            }
                        },
                        span { class: Styles::dx_pagination_label, {t.next.clone()} }
                        ChevronRight { "aria-hidden": "true", size: "1rem" }
                    }
                }
            }
        }
    }
}

/// The page numbers [`Pager`] shows, `None` for a gap: the first and last page, and
/// `siblings` pages each side of `page`. A gap of one page shows that page instead.
pub fn page_items(page: usize, pages: usize, siblings: usize) -> Vec<Option<usize>> {
    let pages = pages.max(1);
    let page = page.clamp(1, pages);
    let lo = page.saturating_sub(siblings).max(1);
    let hi = (page + siblings).min(pages);
    let mut items = vec![Some(1)];
    match lo {
        0..=2 => {}
        3 => items.push(Some(2)),
        _ => items.push(None),
    }
    items.extend((lo.max(2)..=hi).map(Some));
    if hi < pages {
        match pages - hi {
            1 => {}
            2 => items.push(Some(pages - 1)),
            _ => items.push(None),
        }
        items.push(Some(pages));
    }
    items
}

#[cfg(test)]
mod tests {
    use super::page_items;

    fn show(page: usize, pages: usize) -> String {
        page_items(page, pages, 1)
            .iter()
            .map(|i| i.map_or("…".to_string(), |p| p.to_string()))
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn page_items_collapse_long_ranges() {
        assert_eq!(show(1, 1), "1");
        assert_eq!(show(1, 3), "1 2 3");
        assert_eq!(show(1, 5), "1 2 … 5");
        assert_eq!(show(3, 5), "1 2 3 4 5");
        assert_eq!(show(6, 20), "1 … 5 6 7 … 20");
        assert_eq!(show(20, 20), "1 … 19 20");
        assert_eq!(show(4, 20), "1 2 3 4 5 … 20");
    }
}
