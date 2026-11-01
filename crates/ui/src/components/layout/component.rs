//! Layout pieces on the kit's foundations (docs/FOUNDATIONS.md): a page [`Container`], a
//! [`Stack`] of children spaced in 4 px units, a responsive column [`Grid`] and [`Text`] on the
//! type scale. All of it is CSS on the theme tokens; nothing measures the page in Rust.

use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/layout/style.css")]
struct Styles;

/// The spacing steps a `gap` can take, in 4 px units (`4` = 16 px): `--unit-x1` to
/// `--unit-x10`, then 12, 16, 20 and 24. Other values round down to the nearest step.
pub const UNITS: [u8; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 16, 20, 24];

fn unit(n: u8) -> String {
    UNITS
        .iter()
        .rev()
        .find(|&&u| u <= n)
        .copied()
        .unwrap_or(0)
        .to_string()
}

/// Centres page content at `--layout-max-width` with the responsive page margin
/// (`--layout-margin`: 16, 24 then 32 px as the window widens).
#[component]
pub fn Container(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_container }), attributes]),
            {children}
        }
    }
}

/// Which way a [`Stack`] runs.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub enum StackDirection {
    #[default]
    Column,
    Row,
}

/// How a [`Stack`] lines its children up across its direction (CSS `align-items`).
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub enum StackAlign {
    #[default]
    Stretch,
    Start,
    Center,
    End,
    Baseline,
}

/// How a [`Stack`] spreads its children along its direction (CSS `justify-content`).
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub enum StackJustify {
    #[default]
    Start,
    Center,
    End,
    Between,
}

/// Children in a column (or a row) with an even gap from the spacing scale. Use it instead
/// of margins between siblings.
///
/// ```rust,ignore
/// Stack { gap: 3, Label { "Email" } Input {} }
/// Stack { direction: StackDirection::Row, gap: 2, justify: StackJustify::End, Button { "Cancel" } Button { "Save" } }
/// ```
#[component]
pub fn Stack(
    /// Column (default) or row.
    #[props(default)]
    direction: StackDirection,
    /// Space between children in 4 px units (default 4 = 16 px).
    #[props(default = 4)]
    gap: u8,
    #[props(default)] align: StackAlign,
    #[props(default)] justify: StackJustify,
    /// Let a row wrap onto more lines when it runs out of room.
    #[props(default)]
    wrap: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let direction = match direction {
        StackDirection::Column => "column",
        StackDirection::Row => "row",
    };
    let align = match align {
        StackAlign::Stretch => "stretch",
        StackAlign::Start => "start",
        StackAlign::Center => "center",
        StackAlign::End => "end",
        StackAlign::Baseline => "baseline",
    };
    let justify = match justify {
        StackJustify::Start => "start",
        StackJustify::Center => "center",
        StackJustify::End => "end",
        StackJustify::Between => "between",
    };
    rsx! {
        div {
            "data-direction": direction,
            "data-gap": unit(gap),
            "data-align": align,
            "data-justify": justify,
            "data-wrap": wrap.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_stack }), attributes]),
            {children}
        }
    }
}

/// A column grid. Without `columns` it follows the page grid: 4 columns on phones, 8 from the
/// m breakpoint (768 px), 12 from l (1024 px), with the matching gutter. With `min_item`, it
/// fits as many columns of at least that width as there is room for (a card list).
#[component]
pub fn Grid(
    /// A fixed number of columns, 1 to 12.
    columns: Option<u8>,
    /// Space between cells in 4 px units; the page gutter when unset.
    gap: Option<u8>,
    /// Smallest column width for an automatic grid, e.g. `"16rem"`.
    min_item: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let columns = columns.map(|c| c.clamp(1, 12).to_string());
    rsx! {
        div {
            "data-columns": columns,
            "data-gap": gap.map(unit),
            "data-auto": min_item.is_some().then_some("true"),
            style: min_item.map(|w| format!("--dx-grid-min-item: {w}")),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_grid }), attributes]),
            {children}
        }
    }
}

/// A cell in a [`Grid`] that spans several columns, optionally different per breakpoint:
/// `GridItem { span: 4, span_m: 4, span_l: 6 }` is full width on phones, half from m up.
#[component]
pub fn GridItem(
    /// Columns to span (1 to 12) on every screen.
    span: Option<u8>,
    /// Columns from the m breakpoint (768 px) up.
    span_m: Option<u8>,
    /// Columns from the l breakpoint (1024 px) up.
    span_l: Option<u8>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let s = |v: Option<u8>| v.map(|c| c.clamp(1, 12).to_string());
    rsx! {
        div {
            "data-span": s(span),
            "data-span-m": s(span_m),
            "data-span-l": s(span_l),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_grid_item }), attributes]),
            {children}
        }
    }
}

/// A step of the type scale. Display and the titles grow from the m breakpoint up.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub enum TextVariant {
    Display,
    Title1,
    Title2,
    Title3,
    Title4,
    Body1,
    #[default]
    Body2,
    Body3,
    Caption,
}

impl TextVariant {
    /// The token suffix: `title-1` for `--font-size-title-1`.
    pub fn name(self) -> &'static str {
        match self {
            TextVariant::Display => "display",
            TextVariant::Title1 => "title-1",
            TextVariant::Title2 => "title-2",
            TextVariant::Title3 => "title-3",
            TextVariant::Title4 => "title-4",
            TextVariant::Body1 => "body-1",
            TextVariant::Body2 => "body-2",
            TextVariant::Body3 => "body-3",
            TextVariant::Caption => "caption-1",
        }
    }
}

/// The colour role of a [`Text`], from the semantic foreground tokens.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub enum TextColor {
    /// Inherit the surrounding colour.
    #[default]
    Inherit,
    Neutral,
    /// Secondary text: hints, captions, metadata.
    Faded,
    Primary,
    Critical,
    Positive,
    Warning,
}

/// Which element a [`Text`] renders, so a title can be a real heading.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub enum TextAs {
    #[default]
    Span,
    P,
    Div,
    H1,
    H2,
    H3,
    H4,
}

/// Text on the type scale: size, line height on the 4 px grid, weight and colour from the
/// tokens. Pick `variant` for how it looks and `as_` for what it is.
///
/// ```rust,ignore
/// Text { variant: TextVariant::Title2, as_: TextAs::H2, "Billing" }
/// Text { color: TextColor::Faded, "Billed yearly." }
/// ```
#[component]
pub fn Text(
    #[props(default)] variant: TextVariant,
    #[props(default)] color: TextColor,
    #[props(default)] as_: TextAs,
    /// 400 to 700; the variant's own weight when unset (titles are bold, body is regular).
    weight: Option<u16>,
    /// Cut off with an ellipsis after this many lines.
    max_lines: Option<u8>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let color = match color {
        TextColor::Inherit => None,
        TextColor::Neutral => Some("neutral"),
        TextColor::Faded => Some("faded"),
        TextColor::Primary => Some("primary"),
        TextColor::Critical => Some("critical"),
        TextColor::Positive => Some("positive"),
        TextColor::Warning => Some("warning"),
    };
    let mut style = String::new();
    if let Some(w) = weight {
        style.push_str(&format!("font-weight: {};", w.clamp(100, 900)));
    }
    if let Some(n) = max_lines {
        style.push_str(&format!("--dx-text-lines: {};", n.max(1)));
    }
    let style = (!style.is_empty()).then_some(style);
    let attrs = merge_attributes(vec![attributes!(span { class: Styles::dx_text }), attributes]);
    let variant = variant.name();
    let clamp = max_lines.map(|_| "true");
    match as_ {
        TextAs::Span => rsx! { span { "data-variant": variant, "data-color": color, "data-clamp": clamp, style, ..attrs, {children} } },
        TextAs::P => rsx! { p { "data-variant": variant, "data-color": color, "data-clamp": clamp, style, ..attrs, {children} } },
        TextAs::Div => rsx! { div { "data-variant": variant, "data-color": color, "data-clamp": clamp, style, ..attrs, {children} } },
        TextAs::H1 => rsx! { h1 { "data-variant": variant, "data-color": color, "data-clamp": clamp, style, ..attrs, {children} } },
        TextAs::H2 => rsx! { h2 { "data-variant": variant, "data-color": color, "data-clamp": clamp, style, ..attrs, {children} } },
        TextAs::H3 => rsx! { h3 { "data-variant": variant, "data-color": color, "data-clamp": clamp, style, ..attrs, {children} } },
        TextAs::H4 => rsx! { h4 { "data-variant": variant, "data-color": color, "data-clamp": clamp, style, ..attrs, {children} } },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaps_round_down_to_a_step() {
        assert_eq!(unit(4), "4");
        assert_eq!(unit(11), "10");
        assert_eq!(unit(15), "12");
        assert_eq!(unit(200), "24");
    }
}
