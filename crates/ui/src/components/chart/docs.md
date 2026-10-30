# Chart

Charts drawn as SVG in Rust, coloured from theme tokens. No JavaScript chart library.

- `BarChart`, `LineChart`, `AreaChart`: give `data: Vec<ChartDatum>` for one series, or `categories: Vec<String>` and `series: Vec<ChartSeries>` for several. Bars sit side by side or, with `stacked: true`, on top of each other; areas overlap translucently or stack.
- `PieChart`: shares of a total from `data`. `donut: true` cuts a hole, with `center` and `center_caption` text inside it.
- `RadarChart`: one axis per category around a circle and one closed shape per series, for comparing profiles across the same measures. Takes `data`, or `categories` and `series`, like `LineChart`; `max` sets the value at the outer ring (otherwise a round number above the largest value).
- `RadialChart`: one `value` out of `max` (default 100) as a ring, with the percentage (or `center`) in the middle and an optional `caption`.
- `ChartLegend`: the colour key. Charts with two or more series, and pie charts, show one; `legend: false` hides it, `legend: Some(true)` forces it.
- `table: true` adds a "Show data" disclosure with the numbers as a table.
- `keyboard: true` (bar, line, area, pie and radar charts) makes the chart one tab stop: the arrow keys move between points, and the point under the cursor is outlined and named in a status line under the chart ("Feb: 305. 2 of 6."). Off by default, so a page full of charts doesn't fill the tab order; turn it on for charts people need to read point by point.

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
RadarChart {
    categories: vec!["Speed".into(), "Battery".into(), "Camera".into()],
    series: vec![ChartSeries::new("Phone A", [8.0, 6.0, 9.0]), ChartSeries::new("Phone B", [6.0, 9.0, 6.0])],
    max: 10.0,
    keyboard: true,
    label: "Two phones scored out of 10",
}
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
- Focus: without `keyboard` a chart is a static image, not a tab stop. With it, the chart is a `role="group"` named by `label` and described by a hint ("Use the arrow keys to move between points.", `UiStrings::chart_keys_hint`); the cursor starts on the first point when focus arrives, and a `role="status"` line reads the point (`UiStrings::chart_point`). The keys are in the table below. The "Show data" toggle is a native `<summary>` (Enter or Space).
- Reduced motion: bars grow, areas, lines, radar shapes and slices fade in, and the radial ring slides to a new value only under `prefers-reduced-motion: no-preference`.

## Stability

Experimental; see `docs/STABILITY.md` in the kit repository.
