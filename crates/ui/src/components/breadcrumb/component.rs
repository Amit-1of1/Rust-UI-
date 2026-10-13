use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;

#[css_module("/src/components/breadcrumb/style.css")]
struct Styles;

#[component]
pub fn Breadcrumb(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let label = use_ui_strings().breadcrumb;
    rsx! {
        nav { aria_label: label, class: Styles::dx_breadcrumb, ..attributes,
            ol { class: Styles::dx_breadcrumb_list, {children} }
        }
    }
}

#[component]
pub fn BreadcrumbItem(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        li { class: Styles::dx_breadcrumb_item, ..attributes, {children} }
    }
}

#[component]
pub fn BreadcrumbLink(
    #[props(into, default = "#".to_string())] href: String,
    /// For in-app navigation without a router: call `prevent_default()` and switch the view.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        a {
            class: Styles::dx_breadcrumb_link,
            href,
            onclick: move |e| {
                if let Some(f) = &onclick {
                    f.call(e);
                }
            },
            ..attributes,
            {children}
        }
    }
}

/// The current page: not a link, and marked with `aria-current="page"`.
#[component]
pub fn BreadcrumbPage(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span { class: Styles::dx_breadcrumb_page, aria_current: "page", ..attributes, {children} }
    }
}

/// The divider between items. Hidden from assistive technology.
#[component]
pub fn BreadcrumbSeparator() -> Element {
    rsx! {
        li { class: Styles::dx_breadcrumb_separator, role: "presentation", aria_hidden: "true", "/" }
    }
}
