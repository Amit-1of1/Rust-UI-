use crate::components::hooks::{follow_in_app, in_app_href};
use crate::components::icons::Ellipsis;
use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/breadcrumb/style.css")]
struct Styles;

#[component]
pub fn Breadcrumb(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let label = use_ui_strings().breadcrumb;
    rsx! {
        nav { aria_label: label, ..merge_attributes(vec![attributes!(div { class: Styles::dx_breadcrumb }), attributes]),
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
        li { ..merge_attributes(vec![attributes!(div { class: Styles::dx_breadcrumb_item }), attributes]), {children} }
    }
}

#[component]
pub fn BreadcrumbLink(
    #[props(into, default = "#".to_string())] href: String,
    /// Can't be followed for now: no `href`, `aria-disabled="true"`, out of the tab order and
    /// drawn dimmed. Still read as a link in the trail.
    #[props(default)]
    disabled: bool,
    /// For in-app navigation without a router: call `prevent_default()` and switch the view.
    /// With the kit's `router` feature, a path `href` goes through the router by itself.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    if disabled {
        return rsx! {
            a {
                role: "link",
                aria_disabled: "true",
                ..merge_attributes(vec![attributes!(div { class: Styles::dx_breadcrumb_link }), attributes]),
                {children}
            }
        };
    }
    let shown = in_app_href(&href);
    rsx! {
        a {
            href: shown,
            onmounted: move |e| {
                if let Some(f) = &onmounted {
                    f.call(e);
                }
            },
            onclick: move |e: MouseEvent| {
                if let Some(f) = &onclick {
                    f.call(e.clone());
                }
                follow_in_app(&href, &e);
            },
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_breadcrumb_link }), attributes]),
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
        span { aria_current: "page", ..merge_attributes(vec![attributes!(div { class: Styles::dx_breadcrumb_page }), attributes]), {children} }
    }
}

/// The divider between items: "/" unless you pass your own, such as a `ChevronRight` icon or
/// "›". Hidden from assistive technology.
#[component]
pub fn BreadcrumbSeparator(#[props(default)] children: Element) -> Element {
    // An empty `children` is a lone placeholder node, not an empty template.
    let custom = children.as_ref().is_ok_and(|node| {
        !node.template.roots.is_empty()
            && !matches!(
                node.dynamic_nodes.as_ref(),
                [dioxus::dioxus_core::DynamicNode::Placeholder(_)]
            )
    });
    rsx! {
        li { class: Styles::dx_breadcrumb_separator, role: "presentation", "aria-hidden": "true",
            if custom {
                {children}
            } else {
                "/"
            }
        }
    }
}

/// Stands in for the items a collapsed trail hides. A button: pressing it calls `on_click`
/// (show the hidden items, or open a menu of them). [`BreadcrumbTrail`] uses it for you.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn BreadcrumbEllipsis(
    /// Its name, such as "Show 3 more"; read instead of the dots.
    #[props(into)]
    label: String,
    on_click: EventHandler<MouseEvent>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            aria_label: label,
            onclick: move |e| on_click.call(e),
            ..merge_attributes(vec![attributes!(button { class: Styles::dx_breadcrumb_ellipsis }), attributes]),
            Ellipsis { "aria-hidden": "true", size: "1rem" }
        }
    }
}

/// One step of a [`BreadcrumbTrail`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, PartialEq, Debug)]
pub struct Crumb {
    pub label: String,
    /// `None` for the current page (the last step).
    pub href: Option<String>,
    pub disabled: bool,
}

impl Crumb {
    /// A step that links to `href`.
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
            disabled: false,
        }
    }

    /// The page you are on: not a link.
    pub fn page(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            disabled: false,
        }
    }

    /// The same step, shown but not followable.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Which crumbs a trail of `count` shows when at most `max` fit: the first one, then the last
/// `max - 1`, with a gap (`None`) between. `max` below 2 counts as 2, and a gap never hides
/// just one crumb (that crumb is shown instead).
///
/// ```
/// use ui::components::breadcrumb::visible_crumbs;
/// assert_eq!(visible_crumbs(6, 3), vec![Some(0), None, Some(4), Some(5)]);
/// assert_eq!(visible_crumbs(3, 3), vec![Some(0), Some(1), Some(2)]);
/// ```
pub fn visible_crumbs(count: usize, max: usize) -> Vec<Option<usize>> {
    let max = max.max(2);
    if count <= max + 1 {
        return (0..count).map(Some).collect();
    }
    let tail = max - 1;
    let mut out = vec![Some(0), None];
    out.extend((count - tail..count).map(Some));
    out
}

/// A breadcrumb trail built from a list, which collapses the middle into a "…" button when
/// there are more than `max_items` steps. Pressing the button shows every step and moves
/// focus to the first one it revealed.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     BreadcrumbTrail { max_items: 3, items: vec![
///         Crumb::link("Home", "/"),
///         Crumb::link("Projects", "/projects"),
///         Crumb::link("Atoll", "/projects/atoll"),
///         Crumb::link("Settings", "/projects/atoll/settings"),
///         Crumb::page("Members"),
///     ] }
/// }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn BreadcrumbTrail(
    items: Vec<Crumb>,
    /// How many steps to show before collapsing (the first and the last `max_items - 1`).
    #[props(default = usize::MAX)]
    max_items: usize,
    /// A custom separator, drawn between every two steps.
    #[props(default)]
    separator: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let t = use_ui_strings();
    let mut expanded = use_signal(|| false);
    let collapsed = visible_crumbs(items.len(), max_items);
    let hidden = items.len() - collapsed.iter().flatten().count();
    let shown = if expanded() {
        (0..items.len()).map(Some).collect()
    } else {
        collapsed
    };
    let count = shown.len();
    rsx! {
        Breadcrumb { attributes,
            for (pos, slot) in shown.into_iter().enumerate() {
                if let Some(i) = slot {
                    BreadcrumbItem { key: "crumb-{i}",
                        if items[i].href.is_none() {
                            BreadcrumbPage { "{items[i].label}" }
                        } else {
                            BreadcrumbLink {
                                href: items[i].href.clone().unwrap_or_default(),
                                disabled: items[i].disabled,
                                // The first crumb the "…" button revealed takes focus, so
                                // keyboard users land where the hidden steps begin.
                                onmounted: move |e: MountedEvent| {
                                    if expanded() && hidden > 0 && i == 1 {
                                        spawn(async move {
                                            _ = e.data().set_focus(true).await;
                                        });
                                    }
                                },
                                "{items[i].label}"
                            }
                        }
                    }
                } else {
                    BreadcrumbItem { key: "gap-{pos}",
                        BreadcrumbEllipsis {
                            label: fill(&t.show_more_crumbs, &[("count", &hidden.to_string())]),
                            on_click: move |_| expanded.set(true),
                        }
                    }
                }
                if pos + 1 < count {
                    if let Some(sep) = separator.clone() {
                        BreadcrumbSeparator { key: "sep-{pos}", {sep} }
                    } else {
                        BreadcrumbSeparator { key: "sep-{pos}" }
                    }
                }
            }
        }
    }
}
