use dioxus::prelude::*;

#[css_module("/src/components/empty/style.css")]
struct Styles;

/// A placeholder for a list, table or page with nothing in it yet.
/// Compose with `EmptyHeader`, `EmptyMedia`, `EmptyTitle`, `EmptyDescription` and `EmptyContent`.
#[component]
pub fn Empty(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_empty, ..attributes, {children} }
    }
}

#[component]
pub fn EmptyHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_empty_header, ..attributes, {children} }
    }
}

/// Decorative media (an icon or illustration). Hidden from assistive tech.
#[component]
pub fn EmptyMedia(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_empty_media, "aria-hidden": "true", ..attributes, {children} }
    }
}

/// The heading. `level` is the heading level (default 3); pick the one that fits the page outline.
#[component]
pub fn EmptyTitle(
    #[props(default = 3)] level: u8,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: Styles::dx_empty_title,
            role: "heading",
            "aria-level": "{level.clamp(1, 6)}",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn EmptyDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        p { class: Styles::dx_empty_description, ..attributes, {children} }
    }
}

/// Actions, usually one or two buttons.
#[component]
pub fn EmptyContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_empty_content, ..attributes, {children} }
    }
}
