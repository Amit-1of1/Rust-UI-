# Chart

- Copy into your app: `rust-ui add chart` or `dx components add chart --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::chart::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

`BarChart` and `LineChart` drawn as SVG from a list of `ChartDatum`. `describe()` builds the text alternative.

## Accessibility

- Role and name: `role="img"` with an `aria-label` summarising the data
- Keyboard: none (not interactive)
- Reduced motion: none

## Stability

Experimental. Multi-series charts and a legend are planned and will change the props; see `docs/STABILITY.md` in the kit repository.

## API

```rust
/// One point or bar.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct ChartDatum {
    pub label: String,
    pub value: f32,
}

/// Round `max` up to a "nice" axis limit (1, 2, 5 or 10 times a power of ten).
/// Non-positive and non-finite input gives 1, so an empty chart still has an axis.
pub fn nice_max(max: f32) -> f32

/// A plain-language summary for screen readers, since an SVG chart has no text structure.
pub fn describe(data: &[ChartDatum]) -> String

/// A bar chart drawn as SVG in the accent colour. `label` is read by screen readers along
/// with a summary of every value; hovering a bar shows its value.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn BarChart(
    data: Vec<ChartDatum>,
    #[props(into)] label: String,
    #[props(default = 200.0)] height: f32,
) -> Element

/// A line chart drawn as SVG with theme colors. Same accessibility contract as [`BarChart`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn LineChart(
    data: Vec<ChartDatum>,
    #[props(into)] label: String,
    #[props(default = 200.0)] height: f32,
) -> Element
```

## Example

```rust
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

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 2rem; grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));",
            BarChart { data: monthly(), label: "Visitors per month, January to June" }
            LineChart { data: monthly(), label: "Visitors per month, January to June (line)" }
        }
    }
}
```
