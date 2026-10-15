# Chart

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
