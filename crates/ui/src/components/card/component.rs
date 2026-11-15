use dioxus::core::AttributeValue;
use dioxus::prelude::*;

#[css_module("/src/components/card/style.css")]
struct Styles;

/// Adds the caller's `class` to ours. A plain `..attributes` spread would replace ours and
/// drop the card's styling.
fn merge_class(mut class: String, attributes: Vec<Attribute>) -> (String, Vec<Attribute>) {
    let mut rest = Vec::with_capacity(attributes.len());
    for attribute in attributes {
        match &attribute.value {
            AttributeValue::Text(extra) if attribute.name == "class" => {
                class.push(' ');
                class.push_str(extra);
            }
            _ => rest.push(attribute),
        }
    }
    (class, rest)
}

#[component]
pub fn Card(
    /// Puts a `CardMedia` beside the content instead of above it (stacked again on narrow
    /// screens).
    #[props(default)]
    horizontal: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card",
            "data-layout": horizontal.then_some("horizontal"),
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_header.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card-header",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_title.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card-title",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_description.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card-description",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardAction(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_action.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card-action",
            ..attributes,
            {children}
        }
    }
}

/// An edge-to-edge image, video or map in a card: no side padding, and as the first (or
/// last) child it meets the card's top (or bottom) edge with the card's rounded corners.
/// An `img` inside is cropped to fill it; give it an `alt`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn CardMedia(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_media.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card-media",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_content.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card-content",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_footer.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card-footer",
            ..attributes,
            {children}
        }
    }
}

/// Makes the whole card a link: the card's title goes inside, and the link's click area is
/// stretched over the card, so one press anywhere opens it while screen readers hear just the
/// title. Buttons in the card stay pressable above it.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn CardLink(
    #[props(into)] href: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_link.to_string(), attributes);
    let shown = crate::components::hooks::in_app_href(&href);
    rsx! {
        a {
            class,
            href: shown,
            "data-slot": "card-link",
            onclick: move |e: MouseEvent| {
                crate::components::hooks::follow_in_app(&href, &e);
            },
            ..attributes,
            {children}
        }
    }
}

/// Lays cards out in a grid that fits as many columns as there is room for, each at least
/// `min_width` wide, down to one column on phones. It adds no list semantics; give it
/// `role: "list"` and each card `role: "listitem"` when screen readers should count them.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn CardGroup(
    /// The narrowest a card may get before the grid drops a column (any CSS length).
    #[props(into, default = "16rem".to_string())]
    min_width: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let (class, attributes) = merge_class(Styles::dx_card_group.to_string(), attributes);
    rsx! {
        div {
            class,
            "data-slot": "card-group",
            style: "--dx-card-min: {min_width};",
            ..attributes,
            {children}
        }
    }
}
