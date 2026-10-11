use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;

#[css_module("/src/components/bottom_nav/style.css")]
struct Styles;

/// A mobile-style tab bar fixed to the bottom of its container. Wrap [`BottomNavItem`]s;
/// `label` names the landmark (for example "Main").
#[component]
pub fn BottomNav(
    #[props(into)] label: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let default_label = use_ui_strings().main_navigation;
    let label = label.unwrap_or(default_label);
    rsx! {
        nav { class: Styles::dx_bottom_nav, "aria-label": "{label}", ..attributes,
            ul { class: Styles::dx_bottom_nav_list, {children} }
        }
    }
}

/// One destination. With `href` it renders a link, otherwise a button. `current` marks the
/// active page (`aria-current="page"`).
#[component]
pub fn BottomNavItem(
    #[props(into)] label: String,
    #[props(into, default)] href: Option<String>,
    #[props(default)] current: bool,
    #[props(default)] onclick: Callback<MouseEvent>,
    /// Decorative icon; hidden from assistive tech.
    #[props(default)]
    icon: Option<Element>,
) -> Element {
    let aria_current = current.then_some("page");
    let content = rsx! {
        if let Some(icon) = icon {
            span { class: Styles::dx_bottom_nav_icon, "aria-hidden": "true", {icon} }
        }
        span { class: Styles::dx_bottom_nav_label, "{label}" }
    };
    rsx! {
        li { class: Styles::dx_bottom_nav_item,
            if let Some(href) = href {
                a {
                    class: Styles::dx_bottom_nav_link,
                    href: "{href}",
                    "aria-current": aria_current,
                    onclick: move |e| onclick.call(e),
                    {content}
                }
            } else {
                button {
                    class: Styles::dx_bottom_nav_link,
                    r#type: "button",
                    "aria-current": aria_current,
                    onclick: move |e| onclick.call(e),
                    {content}
                }
            }
        }
    }
}
