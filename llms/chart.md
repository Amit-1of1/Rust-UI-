# Chart

- Copy into your app: `rust-ui add chart` or `dx components add chart --git https://github.com/Amit-1of1/Rust-UI-` (also copies `table`, `ui_strings`).
- Import: `use crate::components::chart::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

Charts drawn as SVG in Rust, coloured from theme tokens. No JavaScript chart library.

- `BarChart`, `LineChart`, `AreaChart`: give `data: Vec<ChartDatum>` for one series, or `categories: Vec<String>` and `series: Vec<ChartSeries>` for several. Bars sit side by side or, with `stacked: true`, on top of each other; areas overlap translucently or stack.
- `PieChart`: shares of a total from `data`. `donut: true` cuts a hole, with `center` and `center_caption` text inside it.
- `RadialChart`: one `value` out of `max` (default 100) as a ring, with the percentage (or `center`) in the middle and an optional `caption`.
- `ChartLegend`: the colour key. Charts with two or more series, and pie charts, show one; `legend: false` hides it, `legend: Some(true)` forces it.
- `table: true` adds a "Show data" disclosure with the numbers as a table.

```rust
AreaChart {
    categories: vec!["Jan".into(), "Feb".into(), "Mar".into()],
    series: vec![
        ChartSeries::new("Desktop", [186.0, 305.0, 237.0]),
        ChartSeries::new("Mobile", [80.0, 200.0, 120.0]),
    ],
    stacked: true,
    table: true,
    label: "Visitors by device, January to March",
}
PieChart { data: plans, donut: true, center: "42", center_caption: "Customers", label: "Customers by plan" }
RadialChart { value: 89.6, max: 100.0, caption: "of target", label: "Q3 revenue against target" }
```

## Colours

Series 1 uses `--chart-1` when you set it and the accent (`--accent-color`) otherwise; series 2 to 5 use `--chart-2` … `--chart-5` from `theme.css`, and a sixth series starts again at 1. Every shipped colour keeps 3:1 against the page in light and dark. Set the tokens on `:root` or on any container to recolour charts:

```css
.sales { --chart-1: #2563eb; --chart-2: #f59e0b; }
```

## Accessibility

- Role and name: each chart is `role="img"` named by `label`, with a `<desc>` that lists every value (`describe`, `describe_series`, `describe_shares`). Radial charts read "64 of 100 (64%)" (`UiStrings::value_of_max`).
- Hover: each bar, point and slice has a `<title>` with its value; line and area charts also show every series' value for the column under the pointer.
- Not by colour alone: line and area series also differ by dash pattern, matched in the legend; `table: true` gives a keyboard-reachable table of the numbers.
- Keyboard: the chart itself is not interactive; the "Show data" toggle is a native `<summary>` (Enter or Space).
- Reduced motion: bars grow, areas, lines and slices fade in, and the radial ring slides to a new value only under `prefers-reduced-motion: no-preference`.

## Stability

Experimental; see `docs/STABILITY.md` in the kit repository.

## API

```rust
/// One point, bar or slice.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct ChartDatum {
    pub label: String,
    pub value: f32,
}

/// One named series for a multi-series chart: one value per category, in the order of the
/// chart's `categories`. Missing values count as zero.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct ChartSeries {
    pub name: String,
    pub values: Vec<f32>,
}

/// How a [`ChartLegend`] draws each entry's marker.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum LegendMarker {
    /// A filled square, for bars and slices.
    #[default]
    Square,
    /// A short line in the series' dash pattern, for line and area charts.
    Line,
}

/// Round `max` up to a "nice" axis limit (1, 2, 5 or 10 times a power of ten).
/// Non-positive and non-finite input gives 1, so an empty chart still has an axis.
pub fn nice_max(max: f32) -> f32

/// A plain-language summary for screen readers, since an SVG chart has no text structure.
pub fn describe(data: &[ChartDatum]) -> String

/// [`describe`] for several series: `Desktop: Jan 186, Feb 305. Mobile: Jan 80, Feb 200.`
pub fn describe_series(categories: &[String], series: &[ChartSeries]) -> String

/// Whole-number percentages of the total for each datum (negative and non-finite values count
/// as zero). An all-zero list gives zeros.
pub fn shares(data: &[ChartDatum]) -> Vec<f32>

/// [`describe`] for a pie or donut: each value with its share, `Free: 12 (40%), Pro: 18 (60%)`.
pub fn describe_shares(data: &[ChartDatum]) -> String

/// The colour key for a multi-series chart, or for the slices of a pie. A list of names, each
/// with a marker in its series' colour (and, for [`LegendMarker::Line`], its dash pattern).
/// Charts render one themselves; use this to place a key elsewhere.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ChartLegend(items: Vec<String>, #[props(default)] marker: LegendMarker) -> Element {
    rsx!

/// A bar chart drawn as SVG. Give it `data` for one series, or `categories` and `series` for
/// several: side by side, or `stacked`. `label` is read by screen readers along with a summary
/// of every value; hovering a bar shows its value. A legend appears when there are two or
/// more series; `table: true` adds a "Show data" table.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn BarChart(
    #[props(default)] data: Vec<ChartDatum>,
    #[props(default)] categories: Vec<String>,
    #[props(default)] series: Vec<ChartSeries>,
    #[props(into)] label: String,
    #[props(default = 200.0)] height: f32,
    #[props(default)] stacked: bool,
    #[props(default)] legend: Option<bool>,
    #[props(default)] table: bool,
) -> Element

/// A line chart drawn as SVG with theme colours: `data` for one line, `categories` and
/// `series` for several (each in its own colour and dash pattern). Same accessibility
/// contract as [`BarChart`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn LineChart(
    #[props(default)] data: Vec<ChartDatum>,
    #[props(default)] categories: Vec<String>,
    #[props(default)] series: Vec<ChartSeries>,
    #[props(into)] label: String,
    #[props(default = 200.0)] height: f32,
    #[props(default)] legend: Option<bool>,
    #[props(default)] table: bool,
) -> Element

/// An area chart: lines with the region below them filled. Several series overlap
/// translucently, or sit on top of each other with `stacked`. Same props and accessibility
/// contract as [`LineChart`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn AreaChart(
    #[props(default)] data: Vec<ChartDatum>,
    #[props(default)] categories: Vec<String>,
    #[props(default)] series: Vec<ChartSeries>,
    #[props(into)] label: String,
    #[props(default = 200.0)] height: f32,
    #[props(default)] stacked: bool,
    #[props(default)] legend: Option<bool>,
    #[props(default)] table: bool,
) -> Element

/// A pie chart, or with `donut: true` a ring with an optional `center` value and
/// `center_caption` in the hole. Each slice is a share of the total; hovering a slice shows
/// its value and percentage, the legend names the slices and `table: true` adds a
/// "Show data" table. `size` caps the width in CSS pixels.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn PieChart(
    data: Vec<ChartDatum>,
    #[props(into)] label: String,
    #[props(default)] donut: bool,
    #[props(default, into)] center: Option<String>,
    #[props(default, into)] center_caption: Option<String>,
    #[props(default = true)] legend: bool,
    #[props(default)] table: bool,
    #[props(default = 200.0)] size: f32,
) -> Element

/// Fraction of `max` that `value` reaches, between 0 and 1 (0 for a non-positive `max`).
pub fn progress(value: f32, max: f32) -> f32

/// A radial (ring) chart for one value out of `max`: a goal, a quota, a score. The centre shows
/// the percentage unless `center` gives other text; `caption` goes under it. Changes animate
/// along the ring, except under reduced motion.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn RadialChart(
    value: f32,
    #[props(default = 100.0)] max: f32,
    #[props(into)] label: String,
    #[props(default, into)] center: Option<String>,
    #[props(default, into)] caption: Option<String>,
    #[props(default = 160.0)] size: f32,
) -> Element
```

## Example

```rust
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::chart::*;
use dioxus::prelude::*;

fn monthly() -> Vec<ChartDatum> {
    [
        ("Jan", 186.0),
        ("Feb", 305.0),
        ("Mar", 237.0),
        ("Apr", 73.0),
        ("May", 209.0),
        ("Jun", 214.0),
    ]
    .into_iter()
    .map(|(label, value)| ChartDatum::new(label, value))
    .collect()
}

fn months() -> Vec<String> {
    ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
        .map(String::from)
        .to_vec()
}

fn devices() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("Desktop", [186.0, 305.0, 237.0, 73.0, 209.0, 214.0]),
        ChartSeries::new("Mobile", [80.0, 200.0, 120.0, 190.0, 130.0, 140.0]),
    ]
}

fn browsers() -> Vec<ChartDatum> {
    [
        ("Chrome", 275.0),
        ("Safari", 200.0),
        ("Firefox", 187.0),
        ("Edge", 173.0),
        ("Other", 90.0),
    ]
    .into_iter()
    .map(|(label, value)| ChartDatum::new(label, value))
    .collect()
}

#[component]
pub fn Demo() -> Element {
    let mut goal = use_signal(|| 64.0_f32);
    rsx! {
        div { style: "display: grid; gap: 2rem; grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));",
            BarChart { data: monthly(), label: "Visitors per month, January to June" }
            LineChart { data: monthly(), label: "Visitors per month, January to June (line)" }
            AreaChart {
                categories: months(),
                series: devices(),
                stacked: true,
                table: true,
                label: "Visitors by device, January to June, stacked",
            }
            BarChart { categories: months(), series: devices(), label: "Visitors by device, January to June" }
            LineChart { categories: months(), series: devices(), label: "Visitors by device, January to June (lines)" }
            PieChart {
                data: browsers(),
                donut: true,
                center: "935",
                center_caption: "Visitors",
                table: true,
                label: "Visitors by browser",
            }
            PieChart { data: browsers(), label: "Visitors by browser (pie)" }
            div { style: "display: grid; gap: 0.75rem; justify-items: center;",
                RadialChart { value: goal(), label: "Monthly goal reached", caption: "of goal" }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| goal.set(if goal() >= 100.0 { 12.0 } else { (goal() + 22.0).min(100.0) }),
                    "Add progress"
                }
            }
        }
    }
}
```
