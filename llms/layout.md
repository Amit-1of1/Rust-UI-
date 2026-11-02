# Layout

- Copy into your app: `atoll add layout` or `dx components add layout --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::layout::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

Page structure on the kit's foundations ([Foundations](../../../../docs/FOUNDATIONS.md)):

- `Container`: centres the page at `--layout-max-width` with the responsive page margin.
- `Stack`: children in a column or a row with an even `gap` in 4 px units (`gap: 4` is 16 px). Use it instead of margins between siblings.
- `Grid` and `GridItem`: the page grid (4 columns on phones, 8 from 768 px, 12 from 1024 px) with the matching gutter, `columns` for a fixed count, or `min_item: "16rem"` to fit as many columns as there is room for. `GridItem { span, span_m, span_l }` spans columns per breakpoint.
- `Text`: the type scale (`Display`, `Title1` to `Title4`, `Body1` to `Body3`, `Caption`) with line heights on the 4 px grid and the semantic text colours. `as_` picks the element, so a title can be a real heading.

## Component Structure

```rust
// Container centres the page and adds the page margin.
Container {
    // Stack spaces its children evenly, 4 px per gap unit.
    Stack { gap: 6,
        // Text sets a step of the type scale on the element you pick.
        Text { variant: TextVariant::Title1, as_: TextAs::H1, "Settings" }
        // Grid lays out the page columns, or fits columns of min_item width.
        Grid { min_item: "16rem",
            Card { "Profile" }
            Card { "Billing" }
        }
        // On the page grid, GridItem spans columns per breakpoint.
        Grid {
            GridItem { span: 4, span_m: 4, span_l: 8, "Main" }
            GridItem { span: 4, span_m: 4, span_l: 4, "Aside" }
        }
    }
}
```

## Accessibility

- Role and name: plain `div`s; `Text` renders the element you choose with `as_`, so pick headings by meaning (`H1` to `H4`) and the look with `variant`.
- Keyboard: nothing to focus.
- Reduced motion: no motion.
- Reading order follows the source order; `Grid` and `Stack` never reorder children.

## Source

Original to this repo; the spacing, grid and type tokens follow Reshaped (MIT).

## API

```rust
/// The spacing steps a `gap` can take, in 4 px units (`4` = 16 px): `--unit-x1` to
/// `--unit-x10`, then 12, 16, 20 and 24. Other values round down to the nearest step.
pub const UNITS: [u8; 15];

/// Centres page content at `--layout-max-width` with the responsive page margin
/// (`--layout-margin`: 16, 24 then 32 px as the window widens).
#[component]
pub fn Container(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Which way a [`Stack`] runs.
pub enum StackDirection {
    #[default]
    Column,
    Row,
}

/// How a [`Stack`] lines its children up across its direction (CSS `align-items`).
pub enum StackAlign {
    #[default]
    Stretch,
    Start,
    Center,
    End,
    Baseline,
}

/// How a [`Stack`] spreads its children along its direction (CSS `justify-content`).
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
) -> Element

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
) -> Element

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
) -> Element

/// A step of the type scale. Display and the titles grow from the m breakpoint up.
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

/// The colour role of a [`Text`], from the semantic foreground tokens.
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
) -> Element
```

## Example

```rust
use crate::components::layout::*;
use dioxus::prelude::*;

const DOT: &str = "width: 0.75rem; height: 0.75rem; border-radius: var(--radius-full); background: var(--color-background-primary);";
const CELL: &str = "padding: var(--unit-x3); border-radius: var(--radius-medium); background: var(--color-background-neutral-faded); box-shadow: inset 0 0 0 1px var(--color-border-neutral); font-size: var(--font-size-caption-1); text-align: center;";

#[component]
pub fn Demo() -> Element {
    rsx! {
        Stack { gap: 6,
            Stack { gap: 1,
                Text { variant: TextVariant::Display, as_: TextAs::Div, "Display" }
                Text { variant: TextVariant::Title1, as_: TextAs::Div, "Title 1" }
                Text { variant: TextVariant::Title2, as_: TextAs::Div, "Title 2" }
                Text { variant: TextVariant::Title3, as_: TextAs::Div, "Title 3" }
                Text { variant: TextVariant::Title4, as_: TextAs::Div, "Title 4" }
                Text { variant: TextVariant::Body1, as_: TextAs::P, "Body 1: 16 px on a 24 px line, for reading." }
                Text { as_: TextAs::P, "Body 2: 14 px on a 20 px line, the kit's default." }
                Text { variant: TextVariant::Caption, color: TextColor::Faded, "Caption: 12 px on 16 px, for hints and metadata." }
            }
            Grid { min_item: "10rem", gap: 3,
                for (name, step) in [("x1", 1u8), ("x2", 2), ("x4", 4), ("x6", 6), ("x8", 8)] {
                    Stack { key: "{name}", direction: StackDirection::Row, gap: step, align: StackAlign::Center,
                        span { style: DOT }
                        span { style: DOT }
                        Text { variant: TextVariant::Caption, color: TextColor::Faded, "gap {name}" }
                    }
                }
            }
            Grid {
                GridItem { span: 4, span_m: 4, span_l: 8,
                    div { style: CELL, "span 4 / 4 / 8" }
                }
                GridItem { span: 4, span_m: 4, span_l: 4,
                    div { style: CELL, "span 4 / 4 / 4" }
                }
                for i in 0..4u8 {
                    GridItem { key: "{i}", span: 2, span_m: 2, span_l: 3,
                        div { style: CELL, "2 / 2 / 3" }
                    }
                }
            }
        }
    }
}
```
