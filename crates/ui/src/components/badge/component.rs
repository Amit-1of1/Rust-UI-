use crate::components::icons::{BadgeCheck, X};
use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/badge/style.css")]
struct Styles;

/// A badge's role, shown by its colour. Success and warning use the theme's status tokens.
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgeVariant {
    #[default]
    Primary,
    /// A faded fill that draws less attention.
    Secondary,
    Destructive,
    Outline,
    Success,
    Warning,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "primary",
            BadgeVariant::Secondary => "secondary",
            BadgeVariant::Destructive => "destructive",
            BadgeVariant::Outline => "outline",
            BadgeVariant::Success => "success",
            BadgeVariant::Warning => "warning",
        }
    }
}

/// A badge's size: `Sm` for dense tables, `Lg` for marketing pages.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgeSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl BadgeSize {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeSize::Sm => "sm",
            BadgeSize::Default => "default",
            BadgeSize::Lg => "lg",
        }
    }
}

/// The props for the [`Badge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,

    #[props(default)]
    pub size: BadgeSize,

    /// A dot with no text, such as an "unread" marker. Give it an `aria-label` when it means
    /// something on its own; without one it is decorative and hidden from screen readers.
    #[props(default)]
    pub dot: bool,

    /// Adds a remove button after the text (for selected values and filters). It is named
    /// "Remove {dismiss_label}", so pass the badge's text as `dismiss_label`.
    #[props(default)]
    pub on_dismiss: Option<EventHandler<MouseEvent>>,

    /// What the remove button removes ("Rust" gives "Remove Rust"). Needed with `on_dismiss`.
    #[props(default, into)]
    pub dismiss_label: Option<String>,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let t = use_ui_strings();
    let named = props
        .attributes
        .iter()
        .any(|a| matches!(a.name, "aria-label" | "aria-labelledby"));
    let base = attributes!(span {
        class: Styles::dx_badge,
        "data-style": props.variant.class(),
        "data-size": props.size.class(),
        "data-dot": props.dot.then_some("true"),
        "padding": (!props.dot).then_some("true"),
        role: (props.dot && named).then_some("img"),
        aria_hidden: (props.dot && !named).then_some("true"),
    });
    let remove = props
        .dismiss_label
        .as_deref()
        .map(|label| fill(&t.remove_named, &[("label", label)]))
        .unwrap_or_else(|| t.close.clone());
    rsx! {
        span { ..merge_attributes(vec![base, props.attributes]),
            if !props.dot {
                {props.children}
            }
            if let Some(on_dismiss) = props.on_dismiss {
                button {
                    r#type: "button",
                    class: Styles::dx_badge_dismiss,
                    aria_label: remove,
                    onclick: move |e| on_dismiss.call(e),
                    X { "aria-hidden": "true", size: "0.75em" }
                }
            }
        }
    }
}

/// Where a [`BadgeAnchor`] puts its badge. `End` follows the reading direction (right in
/// English, left in Arabic).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgePlacement {
    #[default]
    TopEnd,
    BottomEnd,
    TopStart,
    BottomStart,
}

impl BadgePlacement {
    pub fn class(&self) -> &'static str {
        match self {
            BadgePlacement::TopEnd => "top-end",
            BadgePlacement::BottomEnd => "bottom-end",
            BadgePlacement::TopStart => "top-start",
            BadgePlacement::BottomStart => "bottom-start",
        }
    }
}

/// Pins a badge to the corner of another element, such as a count on an icon button or a
/// status dot on an avatar.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # use ui::icons::Bell;
/// # fn app() -> Element {
/// rsx! {
///     BadgeAnchor { badge: rsx! { Badge { size: BadgeSize::Sm, "3" } },
///         IconButton { label: "Notifications, 3 unread", variant: ButtonVariant::Outline,
///             Bell { "aria-hidden": "true" }
///         }
///     }
/// }
/// # }
/// ```
///
/// The badge is drawn over the element, so put what it says into the element's own name too
/// (as above) and mark it `aria-hidden` if it would be read twice.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn BadgeAnchor(
    /// The badge (or any small element) to pin.
    badge: Element,
    #[props(default)] placement: BadgePlacement,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span { ..merge_attributes(vec![attributes!(span { class: Styles::dx_badge_anchor }), attributes]),
            {children}
            span { class: Styles::dx_badge_anchored, "data-placement": placement.class(), {badge} }
        }
    }
}

#[component]
pub fn VerifiedIcon() -> Element {
    rsx! {
        BadgeCheck { "aria-hidden": "true",
            size: "12px",
            stroke: "var(--secondary-color-4)",
        }
    }
}
