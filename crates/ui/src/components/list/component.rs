use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/list/style.css")]
struct Styles;

/// How a [`List`] marks its items.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ListMarker {
    /// Bullets (an unordered list).
    #[default]
    Bullet,
    /// 1, 2, 3 (an ordered list).
    Number,
    /// a, b, c (an ordered list).
    Letter,
    /// i, ii, iii (an ordered list).
    Roman,
    /// No marker, for lists of rich rows (an unordered list that keeps its list role).
    None,
}

impl ListMarker {
    fn ordered(self) -> bool {
        matches!(self, Self::Number | Self::Letter | Self::Roman)
    }

    fn class(self) -> &'static str {
        match self {
            Self::Bullet => "bullet",
            Self::Number => "number",
            Self::Letter => "letter",
            Self::Roman => "roman",
            Self::None => "none",
        }
    }
}

/// A bulleted, numbered or plain list: a real `<ul>` or `<ol>`, so screen readers say "list,
/// 3 items" and count as you move through it. Items hold any content, components included.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     List { marker: ListMarker::Number,
///         ListItem { "Install the CLI" }
///         ListItem { "Run " Kbd { "atoll create" } }
///     }
/// }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn List(
    #[props(default)] marker: ListMarker,
    /// Space between items: `true` for lists of rows or cards.
    #[props(default)]
    spaced: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(ul {
        class: Styles::dx_list,
        "data-marker": marker.class(),
        "data-spaced": spaced.then_some("true"),
        // Safari drops the list role from a list drawn without markers; give it back.
        role: (marker == ListMarker::None).then_some("list"),
    });
    let merged = merge_attributes(vec![base, attributes]);
    if marker.ordered() {
        rsx! {
            ol { ..merged, {children} }
        }
    } else {
        rsx! {
            ul { ..merged, {children} }
        }
    }
}

/// One item of a [`List`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ListItem(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        li { ..merge_attributes(vec![attributes!(li { class: Styles::dx_list_item }), attributes]), {children} }
    }
}
