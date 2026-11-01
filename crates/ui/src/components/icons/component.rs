//! The kit's icon sets: every [Lucide](https://lucide.dev) icon, generated into `lucide.rs` by
//! `scripts/gen-lucide-icons.mjs` (no icon crate in between), and Atoll's own icons for the
//! stack (crates, wasm, apps, agents, motion...), generated into `atoll.rs` by
//! `scripts/gen-atoll-icons.mjs` from the SVGs in `crates/ui/icons/atoll/`.
//!
//! Every component and app imports icons from here (`crate::components::icons::Check`, or
//! `ui::icons::Check` in an app). Two ways to draw one:
//!
//! - a component per icon: `Check { size: IconSize::Sm }`;
//! - the shape data and [`Icon`]: `Icon { icon: lucide::CHECK, label: "Done" }`, for when the
//!   icon is chosen at runtime (a nav list, a status map).
//!
//! Each icon's data is a separate constant and each component a separate function, so only
//! the icons an app names end up in its binary.
//!
//! Icons are decorative by default (`aria-hidden="true"`). Give one a `label` and it becomes
//! `role="img"` with that name. Sizes come from the `--icon-size-*` tokens in `theme.css`,
//! the stroke from `--icon-stroke-width`, and icons that point along the reading direction
//! (arrows, chevrons, undo) mirror under `dir="rtl"`.

use dioxus::core::{AttributeValue, IntoAttributeValue};
use dioxus::prelude::*;

/// One shape of an icon, in Lucide's 24×24 coordinate space. Values are kept as Lucide's text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconNode {
    /// `<path d>`.
    Path { d: &'static str },
    /// `<circle cx cy r>`; `filled` draws it with `fill="currentColor"` (a dot).
    Circle {
        cx: &'static str,
        cy: &'static str,
        r: &'static str,
        filled: bool,
    },
    /// `<ellipse cx cy rx ry>`.
    Ellipse {
        cx: &'static str,
        cy: &'static str,
        rx: &'static str,
        ry: &'static str,
    },
    /// `<rect x y width height rx ry>`.
    Rect {
        x: &'static str,
        y: &'static str,
        width: &'static str,
        height: &'static str,
        rx: Option<&'static str>,
        ry: Option<&'static str>,
    },
    /// `<line x1 y1 x2 y2>`.
    Line {
        x1: &'static str,
        y1: &'static str,
        x2: &'static str,
        y2: &'static str,
    },
    /// `<polyline points>`.
    Polyline { points: &'static str },
    /// `<polygon points>`.
    Polygon { points: &'static str },
    /// `<path d>` filled with `currentColor` and no stroke (a solid shape; Atoll icons only).
    Fill { d: &'static str },
}

/// One icon's shape data, as generated from Lucide: `ui::icons::lucide::CHECK`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconData {
    /// Lucide's name (`"chevron-right"`), also rendered as `data-icon`.
    pub name: &'static str,
    /// Whether the icon points along the reading direction and is mirrored in RTL text.
    pub mirror_rtl: bool,
    /// The shapes, drawn in order.
    pub nodes: &'static [IconNode],
}

/// What the docs and tools know about one icon of a hand-drawn set ([`atoll::INDEX`](super::atoll::INDEX)):
/// its default accessible name, search keywords, category, and the action it is reserved for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconMeta {
    /// The icon's name (`"crate"`), the same as its [`IconData::name`].
    pub name: &'static str,
    /// The default accessible name: pass it as `label` when the icon stands alone and means
    /// something (`Icon { icon: atoll::CRATE, label: meta.label }`).
    pub label: &'static str,
    /// Search words beyond the name.
    pub keywords: &'static [&'static str],
    /// The group it is shown in (`"dev"`, `"system"`); one of [`atoll::CATEGORIES`](super::atoll::CATEGORIES).
    pub category: &'static str,
    /// The one action this icon means everywhere (`"delete"`), when it is a reserved system
    /// icon. Don't use a reserved icon for anything else.
    pub reserved: Option<&'static str>,
}

/// Width and height of an icon. The named steps read `--icon-size-xs` … `--icon-size-xl`
/// from the theme; a number is pixels, and a string is any CSS length (`"1rem"`, `"1em"`).
///
/// The default, [`IconSize::Lg`], is 24px like Lucide's own.
#[derive(Clone, PartialEq, Default)]
pub enum IconSize {
    /// `--icon-size-xs` (0.75rem).
    Xs,
    /// `--icon-size-sm` (1rem): inside buttons, inputs and menus.
    Sm,
    /// `--icon-size-md` (1.25rem).
    Md,
    /// `--icon-size-lg` (1.5rem, 24px). The default.
    #[default]
    Lg,
    /// `--icon-size-xl` (2rem).
    Xl,
    /// A fixed width and height (`16`, `"1rem"`).
    Custom(AttributeValue),
}

impl<T: IntoAttributeValue> From<T> for IconSize {
    fn from(value: T) -> Self {
        Self::Custom(value.into_value())
    }
}

impl IconSize {
    /// The `data-size` value the theme sizes, and the pixel size used when no theme is loaded.
    fn token(&self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Xs => Some(("xs", "12")),
            Self::Sm => Some(("sm", "16")),
            Self::Md => Some(("md", "20")),
            Self::Lg => Some(("lg", "24")),
            Self::Xl => Some(("xl", "32")),
            Self::Custom(_) => None,
        }
    }
}

/// Props of every per-icon component (`Check`, `ChevronRight`, ...).
#[derive(Clone, PartialEq, Props)]
pub struct IconProps {
    /// Width and height; a theme step ([`IconSize::Sm`]), pixels (`16`) or a CSS length (`"1rem"`).
    #[props(into, default)]
    pub size: IconSize,
    /// Accessible name. With one the icon is `role="img"`; without one it is `aria-hidden`.
    /// Leave it out when text next to the icon (or the control holding it) already says the same.
    #[props(into, default)]
    pub label: Option<String>,
    /// Override whether the icon mirrors in right-to-left text (default: the icon's own setting).
    #[props(default)]
    pub mirror_rtl: Option<bool>,
    /// Any SVG attribute (`stroke_width`, `class`, `style`, `"data-*"`), applied to the `<svg>`.
    #[props(extends = SvgAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Props of [`Icon`]: the icon's shape data plus everything in [`IconProps`].
#[derive(Clone, PartialEq, Props)]
pub struct IconDataProps {
    /// The icon's shape data, from [`lucide`](super::lucide).
    pub icon: IconData,
    /// Width and height; a theme step ([`IconSize::Sm`]), pixels (`16`) or a CSS length.
    #[props(into, default)]
    pub size: IconSize,
    /// Accessible name. With one the icon is `role="img"`; without one it is `aria-hidden`.
    #[props(into, default)]
    pub label: Option<String>,
    /// Override whether the icon mirrors in right-to-left text.
    #[props(default)]
    pub mirror_rtl: Option<bool>,
    /// Any SVG attribute, applied to the `<svg>`.
    #[props(extends = SvgAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Draws an icon from its shape data: `Icon { icon: lucide::CHECK, label: "Saved" }`.
///
/// Same props as the per-icon components; use this when the icon is a value (stored in a
/// list or chosen at runtime).
#[allow(non_snake_case)]
pub fn Icon(props: IconDataProps) -> Element {
    let IconDataProps {
        icon,
        size,
        label,
        mirror_rtl,
        attributes,
    } = props;
    lucide_icon(
        icon,
        IconProps {
            size,
            label,
            mirror_rtl,
            attributes,
        },
    )
}

/// Renders `icon` with `props`; the body of every generated icon component.
pub(crate) fn lucide_icon(icon: IconData, props: IconProps) -> Element {
    let IconProps {
        size,
        label,
        mirror_rtl,
        attributes,
    } = props;
    let has = |name: &str| attributes.iter().any(|a| a.name == name);
    let mut root = Vec::with_capacity(attributes.len() + 14);
    let mut put = |name: &'static str, value: AttributeValue| {
        if !has(name) {
            root.push(Attribute::new(name, value, None, false));
        }
    };
    let text = |s: &str| AttributeValue::Text(s.to_owned());

    put("xmlns", text("http://www.w3.org/2000/svg"));
    match (size.token(), size) {
        (Some((step, px)), _) => {
            put("width", text(px));
            put("height", text(px));
            put("data-size", text(step));
        }
        (None, IconSize::Custom(value)) => {
            put("width", value.clone());
            put("height", value);
        }
        (None, _) => {}
    }
    put("viewBox", text("0 0 24 24"));
    put("fill", text("none"));
    put("stroke", text("currentColor"));
    if !has("stroke-width") {
        // The theme's `--icon-stroke-width` applies only where the caller set no width.
        put("stroke-width", text("2"));
        put("data-stroke", text("theme"));
    }
    put("stroke-linecap", text("round"));
    put("stroke-linejoin", text("round"));
    put("data-icon", text(icon.name));
    if mirror_rtl.unwrap_or(icon.mirror_rtl) {
        put("data-mirror-rtl", text("true"));
    }
    match label.filter(|l| !l.is_empty()) {
        Some(label) => {
            put("role", text("img"));
            put("aria-label", AttributeValue::Text(label));
        }
        // A name passed as a plain attribute still counts as labelled.
        None if !has("aria-label") && !has("aria-labelledby") => {
            put("aria-hidden", text("true"));
        }
        None => put("role", text("img")),
    }
    root.extend(attributes);

    rsx! {
        svg { ..root,
            for shape in icon.nodes.iter() {
                {shape_node(shape)}
            }
        }
    }
}

fn shape_node(shape: &IconNode) -> Element {
    match *shape {
        IconNode::Path { d } => rsx! {
            path { d }
        },
        IconNode::Circle { cx, cy, r, filled } => rsx! {
            circle {
                cx,
                cy,
                r,
                fill: filled.then_some("currentColor"),
            }
        },
        IconNode::Ellipse { cx, cy, rx, ry } => rsx! {
            ellipse { cx, cy, rx, ry }
        },
        IconNode::Rect {
            x,
            y,
            width,
            height,
            rx,
            ry,
        } => rsx! {
            rect {
                x,
                y,
                width,
                height,
                rx,
                ry,
            }
        },
        IconNode::Line { x1, y1, x2, y2 } => rsx! {
            line { x1, y1, x2, y2 }
        },
        IconNode::Polyline { points } => rsx! {
            polyline { points }
        },
        IconNode::Polygon { points } => rsx! {
            polygon { points }
        },
        IconNode::Fill { d } => rsx! {
            path { d, fill: "currentColor", stroke: "none" }
        },
    }
}
