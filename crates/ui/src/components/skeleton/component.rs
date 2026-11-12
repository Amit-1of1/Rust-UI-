use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/skeleton/style.css")]
struct Styles;

/// The shape a [`Skeleton`] stands in for.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum SkeletonShape {
    /// A box with the theme's corner radius (cards, images, buttons).
    #[default]
    Rect,
    /// A circle (avatars, icons): give it a width; the height follows.
    Circle,
    /// A line of text: one line high at the current font size, so it matches the text it
    /// replaces. Set `lines` for a paragraph.
    Text,
}

impl SkeletonShape {
    fn class(self) -> &'static str {
        match self {
            SkeletonShape::Rect => "rect",
            SkeletonShape::Circle => "circle",
            SkeletonShape::Text => "text",
        }
    }
}

/// A placeholder shown while content loads. Size it with `style` (or a class) to the size of
/// what it replaces, so nothing moves when the content arrives. Decorative: mark the region
/// that is loading with `aria-busy="true"` instead.
#[component]
pub fn Skeleton(
    #[props(default)] shape: SkeletonShape,
    /// For `SkeletonShape::Text`: how many lines; the last one is shorter, like a paragraph.
    #[props(default = 1)]
    lines: usize,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_skeleton,
        "data-shape": shape.class(),
        aria_hidden: "true",
    });
    let merged = merge_attributes(vec![base, attributes]);
    if shape == SkeletonShape::Text && lines > 1 {
        return rsx! {
            div { class: Styles::dx_skeleton_lines, aria_hidden: "true",
                for i in 0..lines {
                    div { key: "{i}", ..merged.clone() }
                }
            }
        };
    }
    rsx! {
        div { ..merged }
    }
}
