use crate::components::table::{
    Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow,
};
use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::prelude::*;
use std::f32::consts::{PI, TAU};
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/chart/style.css")]
struct Styles;

/// One point, bar or slice.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, PartialEq, Debug)]
pub struct ChartDatum {
    pub label: String,
    pub value: f32,
}

impl ChartDatum {
    pub fn new(label: impl Into<String>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// One named series for a multi-series chart: one value per category, in the order of the
/// chart's `categories`. Missing values count as zero.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, PartialEq, Debug)]
pub struct ChartSeries {
    pub name: String,
    pub values: Vec<f32>,
}

impl ChartSeries {
    pub fn new(name: impl Into<String>, values: impl Into<Vec<f32>>) -> Self {
        Self {
            name: name.into(),
            values: values.into(),
        }
    }
}

/// How a [`ChartLegend`] draws each entry's marker.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum LegendMarker {
    /// A filled square, for bars and slices.
    #[default]
    Square,
    /// A short line in the series' dash pattern, for line and area charts.
    Line,
}

const WIDTH: f32 = 400.0;
const PLOT_TOP: f32 = 12.0;
const PLOT_LEFT: f32 = 36.0;
const PLOT_RIGHT: f32 = 8.0;
const LABEL_SPACE: f32 = 24.0;
/// Series colours cycle through `--chart-1` … `--chart-5`.
const PALETTE: usize = 5;
/// Line and area series also differ by dash pattern, so they can be told apart without colour.
const DASHES: [&str; 4] = ["none", "6 4", "2 4", "10 4 2 4"];

/// Round `max` up to a "nice" axis limit (1, 2, 5 or 10 times a power of ten).
/// Non-positive and non-finite input gives 1, so an empty chart still has an axis.
pub fn nice_max(max: f32) -> f32 {
    if !max.is_finite() || max <= 0.0 {
        return 1.0;
    }
    let magnitude = 10f32.powf(max.log10().floor());
    let fraction = max / magnitude;
    let nice = if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice * magnitude
}

fn clean(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

/// A plain-language summary for screen readers, since an SVG chart has no text structure.
pub fn describe(data: &[ChartDatum]) -> String {
    if data.is_empty() {
        return "No data.".to_string();
    }
    data.iter()
        .map(|d| format!("{}: {}", d.label, d.value))
        .collect::<Vec<_>>()
        .join(", ")
}

/// [`describe`] for several series: `Desktop: Jan 186, Feb 305. Mobile: Jan 80, Feb 200.`
pub fn describe_series(categories: &[String], series: &[ChartSeries]) -> String {
    if series.is_empty() || categories.is_empty() {
        return "No data.".to_string();
    }
    series
        .iter()
        .map(|s| {
            let values = categories
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{c} {}", s.values.get(i).copied().unwrap_or(0.0)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}: {values}.", s.name)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Whole-number percentages of the total for each datum (negative and non-finite values count
/// as zero). An all-zero list gives zeros.
pub fn shares(data: &[ChartDatum]) -> Vec<f32> {
    let total: f32 = data.iter().map(|d| clean(d.value)).sum();
    data.iter()
        .map(|d| {
            if total > 0.0 {
                (clean(d.value) / total * 100.0).round()
            } else {
                0.0
            }
        })
        .collect()
}

/// [`describe`] for a pie or donut: each value with its share, `Free: 12 (40%), Pro: 18 (60%)`.
pub fn describe_shares(data: &[ChartDatum]) -> String {
    if data.is_empty() {
        return "No data.".to_string();
    }
    data.iter()
        .zip(shares(data))
        .map(|(d, share)| format!("{}: {} ({share}%)", d.label, d.value))
        .collect::<Vec<_>>()
        .join(", ")
}

/// A move of the keyboard cursor in a chart with `keyboard: true`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartKey {
    /// Right arrow: the next category (bar, point, slice or axis).
    Next,
    /// Left arrow.
    Previous,
    /// Down arrow: the next series in legend order. With one series, the next category.
    NextSeries,
    /// Up arrow.
    PreviousSeries,
    /// Home: the first category.
    First,
    /// End: the last category.
    Last,
}

impl ChartKey {
    /// The move a key makes, or `None` for keys a chart leaves alone.
    pub fn from_key(key: &Key) -> Option<Self> {
        Some(match key {
            Key::Named(NamedKey::ArrowRight) => Self::Next,
            Key::Named(NamedKey::ArrowLeft) => Self::Previous,
            Key::Named(NamedKey::ArrowDown) => Self::NextSeries,
            Key::Named(NamedKey::ArrowUp) => Self::PreviousSeries,
            Key::Named(NamedKey::Home) => Self::First,
            Key::Named(NamedKey::End) => Self::Last,
            _ => return None,
        })
    }
}

/// The mark under the keyboard cursor: a category (column, slice or radar axis) and a series.
/// Pure, so the moves are tested without a browser.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChartCursor {
    pub category: usize,
    pub series: usize,
}

impl ChartCursor {
    /// Where `key` moves the cursor in a chart of `categories` × `series` marks. Moves stop at
    /// the ends rather than wrap, so the first and last points are easy to find.
    pub fn apply(self, key: ChartKey, categories: usize, series: usize) -> Self {
        if categories == 0 || series == 0 {
            return Self::default();
        }
        let (last_c, last_s) = (categories - 1, series - 1);
        let (c, s) = (self.category.min(last_c), self.series.min(last_s));
        let key = match key {
            // One series: Up and Down walk the points too (a pie, a single line).
            ChartKey::NextSeries if series == 1 => ChartKey::Next,
            ChartKey::PreviousSeries if series == 1 => ChartKey::Previous,
            key => key,
        };
        let (category, series) = match key {
            ChartKey::Next => ((c + 1).min(last_c), s),
            ChartKey::Previous => (c.saturating_sub(1), s),
            ChartKey::NextSeries => (c, (s + 1).min(last_s)),
            ChartKey::PreviousSeries => (c, s.saturating_sub(1)),
            ChartKey::First => (0, s),
            ChartKey::Last => (last_c, s),
        };
        Self { category, series }
    }
}

/// Keyboard state shared by a chart's marks and its [`Explorer`].
#[derive(Clone, Copy, PartialEq)]
struct Nav {
    enabled: bool,
    cursor: Signal<ChartCursor>,
    focused: Signal<bool>,
}

impl Nav {
    fn current(&self) -> Option<ChartCursor> {
        (self.enabled && (self.focused)()).then(|| (self.cursor)())
    }

    fn active(&self, series: usize, category: usize) -> bool {
        self.current() == Some(ChartCursor { category, series })
    }

    fn active_category(&self) -> Option<usize> {
        self.current().map(|c| c.category)
    }
}

fn use_nav(enabled: bool) -> Nav {
    Nav {
        enabled,
        cursor: use_signal(ChartCursor::default),
        focused: use_signal(|| false),
    }
}

/// With `keyboard: true`, the chart's focusable wrapper: one tab stop named like the chart,
/// arrow keys move between marks, and the mark under the cursor is shown and announced in a
/// status line (`readout`).
#[component]
fn Explorer(
    nav: Nav,
    label: String,
    categories: usize,
    series: usize,
    /// What the mark under the cursor says, for each (series, category).
    titles: Vec<Vec<String>>,
    children: Element,
) -> Element {
    let strings = use_ui_strings();
    let hint_id = use_hook(|| {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        format!("dx-chart-hint-{}", NEXT.fetch_add(1, Ordering::Relaxed))
    });
    let Nav {
        mut cursor,
        mut focused,
        ..
    } = nav;
    if !nav.enabled {
        return children;
    }
    let readout = nav.current().and_then(|c| {
        let point = titles.get(c.series)?.get(c.category)?;
        Some(fill(
            &strings.chart_point,
            &[
                ("point", point),
                ("index", &(c.category + 1).to_string()),
                ("count", &categories.to_string()),
            ],
        ))
    });
    rsx! {
        div {
            class: Styles::dx_chart_explore,
            role: "group",
            tabindex: "0",
            "aria-label": "{label}",
            "aria-describedby": "{hint_id}",
            onfocus: move |_| focused.set(true),
            onblur: move |_| focused.set(false),
            onkeydown: move |e: KeyboardEvent| {
                let m = e.modifiers();
                if m.ctrl() || m.meta() || m.alt() {
                    return;
                }
                if let Some(key) = ChartKey::from_key(&e.key()) {
                    e.prevent_default();
                    let next = cursor.peek().apply(key, categories, series);
                    cursor.set(next);
                }
            },
            {children}
            p { id: "{hint_id}", class: "sr-only", "{strings.chart_keys_hint}" }
            p { class: Styles::dx_chart_readout, role: "status", "data-slot": "chart-readout",
                if let Some(readout) = readout {
                    "{readout}"
                }
            }
        }
    }
}

/// Categories and series after merging the single-series (`data`) and multi-series
/// (`categories` + `series`) props.
struct Model {
    categories: Vec<String>,
    series: Vec<ChartSeries>,
    /// Built from `data`: the summary and hover titles keep the single-series wording.
    single: bool,
}

impl Model {
    fn new(
        data: &[ChartDatum],
        categories: &[String],
        series: &[ChartSeries],
        value_name: &str,
    ) -> Self {
        if series.is_empty() {
            return Self {
                categories: data.iter().map(|d| d.label.clone()).collect(),
                series: vec![ChartSeries::new(
                    value_name,
                    data.iter().map(|d| d.value).collect::<Vec<_>>(),
                )],
                single: true,
            };
        }
        let longest = series.iter().map(|s| s.values.len()).max().unwrap_or(0);
        let mut categories = categories.to_vec();
        // A category without a name still gets a column; number it.
        for i in categories.len()..longest {
            categories.push((i + 1).to_string());
        }
        Self {
            categories,
            series: series.to_vec(),
            single: false,
        }
    }

    fn value(&self, series: usize, category: usize) -> f32 {
        clean(
            self.series[series]
                .values
                .get(category)
                .copied()
                .unwrap_or(0.0),
        )
    }

    fn raw(&self, series: usize, category: usize) -> f32 {
        self.series[series]
            .values
            .get(category)
            .copied()
            .unwrap_or(0.0)
    }

    /// Sum of the series below `series` in category `category` (the base of a stacked mark).
    fn below(&self, series: usize, category: usize) -> f32 {
        (0..series).map(|s| self.value(s, category)).sum()
    }

    fn axis_max(&self, stacked: bool) -> f32 {
        let count = self.categories.len();
        let max = if stacked {
            (0..count)
                .map(|c| self.below(self.series.len(), c))
                .fold(0.0, f32::max)
        } else {
            (0..self.series.len())
                .flat_map(|s| (0..count).map(move |c| (s, c)))
                .map(|(s, c)| self.value(s, c))
                .fold(0.0, f32::max)
        };
        nice_max(max)
    }

    fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    fn summary(&self, no_data: &str) -> String {
        if self.is_empty() {
            no_data.to_string()
        } else if self.single {
            self.categories
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{c}: {}", self.raw(0, i)))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            describe_series(&self.categories, &self.series)
        }
    }

    /// Hover text for one mark.
    fn mark_title(&self, series: usize, category: usize) -> String {
        let value = self.raw(series, category);
        if self.single {
            format!("{}: {value}", self.categories[category])
        } else {
            format!(
                "{}, {}: {value}",
                self.categories[category], self.series[series].name
            )
        }
    }

    /// Hover text for a whole category column (line and area charts).
    fn column_title(&self, category: usize) -> String {
        if self.single {
            return self.mark_title(0, category);
        }
        let values = (0..self.series.len())
            .map(|s| format!("{} {}", self.series[s].name, self.raw(s, category)))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}: {values}", self.categories[category])
    }

    /// [`mark_title`](Self::mark_title) for every mark, by series then category.
    fn titles(&self) -> Vec<Vec<String>> {
        (0..self.series.len())
            .map(|s| {
                (0..self.categories.len())
                    .map(|c| self.mark_title(s, c))
                    .collect()
            })
            .collect()
    }

    fn names(&self) -> Vec<String> {
        self.series.iter().map(|s| s.name.clone()).collect()
    }

    /// Rows for the data table: category, then one value per series.
    fn rows(&self) -> Vec<(String, Vec<String>)> {
        self.categories
            .iter()
            .enumerate()
            .map(|(c, name)| {
                let values = (0..self.series.len())
                    .map(|s| self.raw(s, c).to_string())
                    .collect();
                (name.clone(), values)
            })
            .collect()
    }
}

struct Plot {
    height: f32,
    top: f32,
    bottom: f32,
    axis_max: f32,
}

impl Plot {
    fn new(height: f32, axis_max: f32) -> Self {
        Self {
            height,
            top: PLOT_TOP,
            bottom: height - LABEL_SPACE,
            axis_max,
        }
    }

    fn y(&self, value: f32) -> f32 {
        let fraction = (clean(value) / self.axis_max).clamp(0.0, 1.0);
        self.bottom - fraction * (self.bottom - self.top)
    }

    fn slot(count: usize) -> f32 {
        (WIDTH - PLOT_LEFT - PLOT_RIGHT) / count.max(1) as f32
    }

    fn x_center(&self, index: usize, count: usize) -> f32 {
        PLOT_LEFT + Self::slot(count) * (index as f32 + 0.5)
    }

    fn gridlines(&self) -> Vec<(f32, f32)> {
        (0..=4)
            .map(|i| {
                let value = self.axis_max * i as f32 / 4.0;
                (self.y(value), value)
            })
            .collect()
    }
}

fn format_tick(value: f32) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{value:.1}")
    }
}

/// The `data-series` value that picks a colour (`--chart-1` … `--chart-5`).
fn palette_index(series: usize) -> usize {
    series % PALETTE
}

fn dash(series: usize) -> &'static str {
    DASHES[series % DASHES.len()]
}

#[component]
fn Grid(plot_height: f32, axis_max: f32) -> Element {
    let plot = Plot::new(plot_height, axis_max);
    rsx! {
        for (y, value) in plot.gridlines() {
            line { class: Styles::dx_chart_grid, x1: "{PLOT_LEFT}", x2: "{WIDTH - PLOT_RIGHT}", y1: "{y}", y2: "{y}" }
            text { class: Styles::dx_chart_tick, x: "{PLOT_LEFT - 6.0}", y: "{y + 3.0}", text_anchor: "end", "{format_tick(value)}" }
        }
    }
}

#[component]
fn CategoryLabels(plot_height: f32, categories: Vec<String>) -> Element {
    let plot = Plot::new(plot_height, 1.0);
    let count = categories.len();
    rsx! {
        for (i, category) in categories.iter().enumerate() {
            text { key: "{i}", class: Styles::dx_chart_tick, x: "{plot.x_center(i, count)}", y: "{plot.height - 6.0}", text_anchor: "middle", "{category}" }
        }
    }
}

/// One transparent column per category that shows every series' value on hover and
/// highlights the column under the pointer.
#[component]
fn HoverColumns(plot_height: f32, titles: Vec<String>, active: Option<usize>) -> Element {
    let plot = Plot::new(plot_height, 1.0);
    let slot = Plot::slot(titles.len());
    rsx! {
        for (i, title) in titles.iter().enumerate() {
            rect {
                key: "{i}",
                class: Styles::dx_chart_column,
                x: "{PLOT_LEFT + slot * i as f32}",
                y: "{plot.top}",
                width: "{slot}",
                height: "{plot.bottom - plot.top}",
                fill: "transparent",
                "data-active": (active == Some(i)).then_some("true"),
                title { "{title}" }
            }
        }
    }
}

/// The colour key for a multi-series chart, or for the slices of a pie. A list of names, each
/// with a marker in its series' colour (and, for [`LegendMarker::Line`], its dash pattern).
/// Charts render one themselves; use this to place a key elsewhere.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ChartLegend(items: Vec<String>, #[props(default)] marker: LegendMarker) -> Element {
    rsx! {
        ul { class: Styles::dx_chart_legend, "data-slot": "chart-legend",
            for (i, item) in items.iter().enumerate() {
                li { key: "{i}", class: Styles::dx_chart_legend_item, "data-series": palette_index(i),
                    if marker == LegendMarker::Line {
                        svg { class: Styles::dx_chart_legend_line, "aria-hidden": "true", view_box: "0 0 16 4", width: "16", height: "4",
                            line { x1: "0", x2: "16", y1: "2", y2: "2", stroke: "currentColor", stroke_width: "2", stroke_dasharray: dash(i) }
                        }
                    } else {
                        span { class: Styles::dx_chart_legend_square, "aria-hidden": "true" }
                    }
                    "{item}"
                }
            }
        }
    }
}

/// The chart's numbers as a table inside a "Show data" disclosure: a text alternative that can
/// be read cell by cell and reached by keyboard.
#[component]
fn ChartData(caption: String, columns: Vec<String>, rows: Vec<(String, Vec<String>)>) -> Element {
    let strings = use_ui_strings();
    rsx! {
        details { class: Styles::dx_chart_data, "data-slot": "chart-data",
            summary { class: Styles::dx_chart_data_toggle, "{strings.show_data}" }
            Table {
                TableCaption { "{caption}" }
                TableHeader {
                    TableRow {
                        TableHead { "{strings.chart_category}" }
                        for column in columns.iter() {
                            TableHead { numeric: true, "{column}" }
                        }
                    }
                }
                TableBody {
                    for (name, values) in rows.iter() {
                        TableRow {
                            TableCell { "{name}" }
                            for value in values.iter() {
                                TableCell { numeric: true, "{value}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Legend and data table under a chart, when asked for.
#[component]
fn Extras(
    caption: String,
    names: Vec<String>,
    marker: LegendMarker,
    legend: bool,
    table: bool,
    columns: Vec<String>,
    rows: Vec<(String, Vec<String>)>,
) -> Element {
    rsx! {
        if legend {
            ChartLegend { items: names, marker }
        }
        if table {
            ChartData { caption, columns, rows }
        }
    }
}

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
    /// One tab stop for the chart; the arrow keys then move between its marks (Left/Right
    /// across categories, Up/Down across series, Home/End), and the one under the cursor is
    /// highlighted and announced.
    #[props(default)]
    keyboard: bool,
) -> Element {
    let strings = use_ui_strings();
    let nav = use_nav(keyboard);
    let model = Model::new(&data, &categories, &series, &strings.chart_value);
    let plot = Plot::new(height, model.axis_max(stacked));
    let count = model.categories.len();
    let lanes = if stacked {
        1
    } else {
        model.series.len().max(1)
    };
    let group = (Plot::slot(count) * 0.7).max(2.0);
    let bar_width = (group / lanes as f32 - 1.0).max(1.0);
    let summary = model.summary(&strings.no_data);
    let legend = legend.unwrap_or(model.series.len() > 1);
    let bars = (0..model.series.len()).map(|s| {
        let marks = (0..count).map(|c| {
            let value = model.value(s, c);
            let (base, lane) = if stacked {
                (model.below(s, c), 0)
            } else {
                (0.0, s)
            };
            let x = plot.x_center(c, count) - group / 2.0 + lane as f32 * (group / lanes as f32);
            let y = plot.y(base + value);
            let bar_height = (plot.y(base) - y).max(0.0);
            (c, x, y, bar_height, model.mark_title(s, c))
        });
        (s, marks.collect::<Vec<_>>())
    });
    let chart = rsx! {
        Explorer { nav, label: label.clone(), categories: model.categories.len(), series: model.series.len(), titles: model.titles(),
            svg {
                class: Styles::dx_chart,
                role: "img",
                "aria-label": "{label}",
                view_box: "0 0 {WIDTH} {height}",
                title { "{label}" }
                desc { "{summary}" }
                Grid { plot_height: height, axis_max: plot.axis_max }
                for (s, marks) in bars {
                    g { key: "{s}", "data-series": palette_index(s),
                        for (c, x, y, bar_height, title) in marks {
                            rect { key: "{c}", class: Styles::dx_chart_bar, x: "{x}", y: "{y}", width: "{bar_width}", height: "{bar_height}", rx: "3",
                                "data-active": nav.active(s, c).then_some("true"),
                                // Shown on hover; the values are also in the chart's `<desc>`.
                                title { "{title}" }
                            }
                        }
                    }
                }
                CategoryLabels { plot_height: height, categories: model.categories.clone() }
            }
        }
    };
    if !legend && !table && !keyboard {
        return chart;
    }
    rsx! {
        div { class: Styles::dx_chart_root, "data-slot": "chart",
            {chart}
            Extras { caption: label.clone(), names: model.names(), marker: LegendMarker::Square, legend, table, columns: model.names(), rows: model.rows() }
        }
    }
}

/// Points of each series in plot coordinates; `stacked` adds the series below.
fn series_points(model: &Model, plot: &Plot, stacked: bool) -> Vec<Vec<(f32, f32)>> {
    let count = model.categories.len();
    (0..model.series.len())
        .map(|s| {
            (0..count)
                .map(|c| {
                    let base = if stacked { model.below(s, c) } else { 0.0 };
                    (plot.x_center(c, count), plot.y(base + model.value(s, c)))
                })
                .collect()
        })
        .collect()
}

fn polyline(points: &[(f32, f32)]) -> String {
    points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ")
}

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
    /// One tab stop for the chart; the arrow keys then move between its marks (Left/Right
    /// across categories, Up/Down across series, Home/End), and the one under the cursor is
    /// highlighted and announced.
    #[props(default)]
    keyboard: bool,
) -> Element {
    let strings = use_ui_strings();
    let nav = use_nav(keyboard);
    let model = Model::new(&data, &categories, &series, &strings.chart_value);
    let plot = Plot::new(height, model.axis_max(false));
    let lines = series_points(&model, &plot, false);
    let summary = model.summary(&strings.no_data);
    let legend = legend.unwrap_or(model.series.len() > 1);
    let titles: Vec<String> = (0..model.categories.len())
        .map(|c| model.column_title(c))
        .collect();
    let chart = rsx! {
        Explorer { nav, label: label.clone(), categories: model.categories.len(), series: model.series.len(), titles: model.titles(),
            svg {
                class: Styles::dx_chart,
                role: "img",
                "aria-label": "{label}",
                view_box: "0 0 {WIDTH} {height}",
                title { "{label}" }
                desc { "{summary}" }
                Grid { plot_height: height, axis_max: plot.axis_max }
                HoverColumns { plot_height: height, titles, active: nav.active_category() }
                for (s, points) in lines.iter().enumerate() {
                    g { key: "{s}", "data-series": palette_index(s),
                        if points.len() > 1 {
                            // The presentation attributes are a fallback for renderers that draw inline SVG
                            // without the page's CSS (Blitz); where CSS applies, the class wins.
                            polyline {
                                class: Styles::dx_chart_line,
                                points: "{polyline(points)}",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_dasharray: dash(s),
                            }
                        }
                        for (c, (x, y)) in points.iter().enumerate() {
                            circle { key: "{c}", class: Styles::dx_chart_dot, cx: "{x}", cy: "{y}",
                                r: if nav.active(s, c) { "5.5" } else { "3.5" },
                                "data-active": nav.active(s, c).then_some("true"),
                                title { "{model.mark_title(s, c)}" }
                            }
                        }
                    }
                }
                CategoryLabels { plot_height: height, categories: model.categories.clone() }
            }
        }
    };
    if !legend && !table && !keyboard {
        return chart;
    }
    rsx! {
        div { class: Styles::dx_chart_root, "data-slot": "chart",
            {chart}
            Extras { caption: label.clone(), names: model.names(), marker: LegendMarker::Line, legend, table, columns: model.names(), rows: model.rows() }
        }
    }
}

/// The filled region between `top` and `base` (both left to right).
fn area_path(top: &[(f32, f32)], base: &[(f32, f32)]) -> String {
    let mut path = String::new();
    for (i, (x, y)) in top.iter().enumerate() {
        path.push_str(&format!("{}{x:.1},{y:.1} ", if i == 0 { 'M' } else { 'L' }));
    }
    for (x, y) in base.iter().rev() {
        path.push_str(&format!("L{x:.1},{y:.1} "));
    }
    path.push('Z');
    path
}

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
    /// One tab stop for the chart; the arrow keys then move between its marks (Left/Right
    /// across categories, Up/Down across series, Home/End), and the one under the cursor is
    /// highlighted and announced.
    #[props(default)]
    keyboard: bool,
) -> Element {
    let strings = use_ui_strings();
    let nav = use_nav(keyboard);
    let model = Model::new(&data, &categories, &series, &strings.chart_value);
    let plot = Plot::new(height, model.axis_max(stacked));
    let tops = series_points(&model, &plot, stacked);
    let count = model.categories.len();
    let floor: Vec<(f32, f32)> = (0..count)
        .map(|c| (plot.x_center(c, count), plot.bottom))
        .collect();
    let areas: Vec<(String, String)> = tops
        .iter()
        .enumerate()
        .map(|(s, top)| {
            let base = if stacked && s > 0 {
                &tops[s - 1]
            } else {
                &floor
            };
            (area_path(top, base), polyline(top))
        })
        .collect();
    let summary = model.summary(&strings.no_data);
    let legend = legend.unwrap_or(model.series.len() > 1);
    let titles: Vec<String> = (0..count).map(|c| model.column_title(c)).collect();
    let chart = rsx! {
        Explorer { nav, label: label.clone(), categories: model.categories.len(), series: model.series.len(), titles: model.titles(),
            svg {
                class: Styles::dx_chart,
                role: "img",
                "aria-label": "{label}",
                view_box: "0 0 {WIDTH} {height}",
                title { "{label}" }
                desc { "{summary}" }
                Grid { plot_height: height, axis_max: plot.axis_max }
                HoverColumns { plot_height: height, titles, active: nav.active_category() }
                for (s, (area, line)) in areas.iter().enumerate() {
                    g { key: "{s}", "data-series": palette_index(s),
                        if count > 1 {
                            path { class: Styles::dx_chart_area, d: "{area}", fill: "currentColor", fill_opacity: "0.2" }
                            polyline {
                                class: Styles::dx_chart_line,
                                points: "{line}",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_dasharray: dash(s),
                            }
                            // The keyboard cursor's point; areas have no dots otherwise.
                            for (c, (x, y)) in tops[s].iter().enumerate() {
                                if nav.active(s, c) {
                                    circle { key: "{c}", class: Styles::dx_chart_dot, "data-active": "true", cx: "{x}", cy: "{y}", r: "5.5" }
                                }
                            }
                        } else {
                            // One category has no width to fill; show the point instead.
                            for (x, y) in tops[s].iter() {
                                circle { class: Styles::dx_chart_dot, cx: "{x}", cy: "{y}", r: "3.5" }
                            }
                        }
                    }
                }
                CategoryLabels { plot_height: height, categories: model.categories.clone() }
            }
        }
    };
    if !legend && !table && !keyboard {
        return chart;
    }
    rsx! {
        div { class: Styles::dx_chart_root, "data-slot": "chart",
            {chart}
            Extras { caption: label.clone(), names: model.names(), marker: LegendMarker::Line, legend, table, columns: model.names(), rows: model.rows() }
        }
    }
}

const PIE_SIZE: f32 = 200.0;
const PIE_CENTER: f32 = PIE_SIZE / 2.0;
const PIE_RADIUS: f32 = 96.0;
const DONUT_HOLE: f32 = 60.0;

/// A point on a circle, `angle` in radians clockwise from twelve o'clock.
fn polar(radius: f32, angle: f32) -> (f32, f32) {
    (
        PIE_CENTER + radius * angle.sin(),
        PIE_CENTER - radius * angle.cos(),
    )
}

/// The SVG path of one slice from `start` to `end` (radians clockwise from twelve o'clock),
/// with a hole of radius `inner` (0 for a pie). A full turn is drawn as two half arcs, since
/// an arc whose ends meet draws nothing.
pub(crate) fn slice_path(outer: f32, inner: f32, start: f32, end: f32) -> String {
    let sweep = (end - start).clamp(0.0, TAU);
    if sweep >= TAU - 1e-4 {
        let ring = |r: f32, flag: u8| {
            let (tx, ty) = polar(r, 0.0);
            let (bx, by) = polar(r, PI);
            format!("M{tx:.2},{ty:.2} A{r},{r} 0 1 {flag} {bx:.2},{by:.2} A{r},{r} 0 1 {flag} {tx:.2},{ty:.2} Z")
        };
        return if inner > 0.0 {
            // The hole runs the other way, so the nonzero fill rule leaves it empty.
            format!("{} {}", ring(outer, 1), ring(inner, 0))
        } else {
            ring(outer, 1)
        };
    }
    let end = start + sweep;
    let large = u8::from(sweep > PI);
    let (x1, y1) = polar(outer, start);
    let (x2, y2) = polar(outer, end);
    if inner > 0.0 {
        let (x3, y3) = polar(inner, end);
        let (x4, y4) = polar(inner, start);
        format!(
            "M{x1:.2},{y1:.2} A{outer},{outer} 0 {large} 1 {x2:.2},{y2:.2} L{x3:.2},{y3:.2} A{inner},{inner} 0 {large} 0 {x4:.2},{y4:.2} Z"
        )
    } else {
        format!(
            "M{PIE_CENTER},{PIE_CENTER} L{x1:.2},{y1:.2} A{outer},{outer} 0 {large} 1 {x2:.2},{y2:.2} Z"
        )
    }
}

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
    /// One tab stop; the arrow keys, Home and End then move between the slices.
    #[props(default)]
    keyboard: bool,
) -> Element {
    let strings = use_ui_strings();
    let nav = use_nav(keyboard);
    let total: f32 = data.iter().map(|d| clean(d.value)).sum();
    let percents = shares(&data);
    let inner = if donut { DONUT_HOLE } else { 0.0 };
    let mut start = 0.0;
    let slices: Vec<(usize, String, String)> = data
        .iter()
        .enumerate()
        .filter_map(|(i, d)| {
            let value = clean(d.value);
            if total <= 0.0 || value <= 0.0 {
                return None;
            }
            let sweep = value / total * TAU;
            let path = slice_path(PIE_RADIUS, inner, start, start + sweep);
            start += sweep;
            Some((
                i,
                path,
                format!("{}: {} ({}%)", d.label, d.value, percents[i]),
            ))
        })
        .collect();
    let summary = if total <= 0.0 {
        strings.no_data.clone()
    } else {
        describe_shares(&data)
    };
    let names: Vec<String> = data.iter().map(|d| d.label.clone()).collect();
    let slice_titles: Vec<String> = data
        .iter()
        .zip(&percents)
        .map(|(d, p)| format!("{}: {} ({p}%)", d.label, d.value))
        .collect();
    let rows: Vec<(String, Vec<String>)> = data
        .iter()
        .zip(&percents)
        .map(|(d, p)| (d.label.clone(), vec![d.value.to_string(), format!("{p}%")]))
        .collect();
    let center_y = if center_caption.is_some() {
        PIE_CENTER + 2.0
    } else {
        PIE_CENTER + 9.0
    };
    rsx! {
        div { class: Styles::dx_chart_root, "data-slot": "chart", style: "--chart-size: {size}px",
            Explorer { nav, label: label.clone(), categories: data.len(), series: 1usize, titles: vec![slice_titles],
                svg {
                    class: Styles::dx_chart_pie,
                    role: "img",
                    "aria-label": "{label}",
                    view_box: "0 0 {PIE_SIZE} {PIE_SIZE}",
                    title { "{label}" }
                    desc { "{summary}" }
                    if slices.is_empty() {
                        path { class: Styles::dx_chart_track, d: "{slice_path(PIE_RADIUS, inner, 0.0, TAU)}" }
                    }
                    for (i, path, title) in slices {
                        path { key: "{i}", class: Styles::dx_chart_slice, "data-series": palette_index(i), d: "{path}", fill: "currentColor",
                            "data-active": nav.active(0, i).then_some("true"),
                            title { "{title}" }
                        }
                    }
                    if donut {
                        if let Some(center) = center {
                            text { class: Styles::dx_chart_center, x: "{PIE_CENTER}", y: "{center_y}", text_anchor: "middle", "{center}" }
                        }
                        if let Some(caption) = center_caption {
                            text { class: Styles::dx_chart_caption, x: "{PIE_CENTER}", y: "{PIE_CENTER + 22.0}", text_anchor: "middle", "{caption}" }
                        }
                    }
                }
            }
            Extras {
                caption: label.clone(),
                names,
                marker: LegendMarker::Square,
                legend,
                table,
                columns: vec![strings.chart_value.clone(), strings.chart_share.clone()],
                rows,
            }
        }
    }
}

/// One radar axis: where its spoke ends, where its label goes, the label's anchor and text.
type RadarAxis = ((f32, f32), (f32, f32), &'static str, String);

const RADAR_SIZE: f32 = 300.0;
const RADAR_CENTER: f32 = RADAR_SIZE / 2.0;
const RADAR_RADIUS: f32 = 100.0;
/// Rings drawn at a quarter, half, three quarters and all of the axis limit.
const RADAR_RINGS: usize = 4;

/// Where axis `index` of `count` meets the circle of `radius`: the first axis points up, the
/// rest follow clockwise.
pub(crate) fn radar_point(radius: f32, index: usize, count: usize) -> (f32, f32) {
    let angle = TAU * index as f32 / count.max(1) as f32;
    (
        RADAR_CENTER + radius * angle.sin(),
        RADAR_CENTER - radius * angle.cos(),
    )
}

/// The `points` of a polygon through one value per axis, each as a fraction of the limit.
fn radar_polygon(fractions: &[f32]) -> String {
    let count = fractions.len();
    fractions
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let (x, y) = radar_point(RADAR_RADIUS * f.clamp(0.0, 1.0), i, count);
            format!("{x:.1},{y:.1}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A radar (spider) chart: one axis per category, all starting at the centre, and one closed
/// shape per series through its values. Good for comparing a few profiles across the same
/// measures. Give it `data` for one shape, or `categories` and `series` for several (each in
/// its own colour and dash pattern). The axis limit is `max`, or a round number above the
/// largest value. Same accessibility contract as [`LineChart`]; `size` caps the width in CSS
/// pixels.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn RadarChart(
    #[props(default)] data: Vec<ChartDatum>,
    #[props(default)] categories: Vec<String>,
    #[props(default)] series: Vec<ChartSeries>,
    #[props(into)] label: String,
    /// The value at the outer ring. Defaults to a round number above the largest value.
    #[props(default)]
    max: Option<f32>,
    #[props(default)] legend: Option<bool>,
    #[props(default)] table: bool,
    #[props(default = 300.0)] size: f32,
    /// One tab stop; Left/Right then move between the axes, Up/Down between the series.
    #[props(default)]
    keyboard: bool,
) -> Element {
    let strings = use_ui_strings();
    let nav = use_nav(keyboard);
    let model = Model::new(&data, &categories, &series, &strings.chart_value);
    let axis_max = max
        .filter(|m| m.is_finite() && *m > 0.0)
        .unwrap_or_else(|| model.axis_max(false));
    let count = model.categories.len();
    let summary = model.summary(&strings.no_data);
    let legend = legend.unwrap_or(model.series.len() > 1);
    let rings: Vec<(String, f32, f32)> = (1..=RADAR_RINGS)
        .map(|k| {
            let fraction = k as f32 / RADAR_RINGS as f32;
            let (_, y) = radar_point(RADAR_RADIUS * fraction, 0, 1);
            (radar_polygon(&vec![fraction; count]), y, axis_max * fraction)
        })
        .collect();
    let axes: Vec<RadarAxis> = model
        .categories
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let end = radar_point(RADAR_RADIUS, i, count);
            let (x, y) = radar_point(RADAR_RADIUS + 14.0, i, count);
            let angle = TAU * i as f32 / count.max(1) as f32;
            let anchor = if angle.sin() > 0.1 {
                "start"
            } else if angle.sin() < -0.1 {
                "end"
            } else {
                "middle"
            };
            // Labels above the centre sit on their point, those below hang from it.
            let y = y + if angle.cos() < -0.1 { 8.0 } else { 3.0 };
            (end, (x, y), anchor, name.clone())
        })
        .collect();
    let shapes: Vec<(String, Vec<(f32, f32)>)> = (0..model.series.len())
        .filter(|_| count > 0)
        .map(|s| {
            let fractions: Vec<f32> = (0..count)
                .map(|c| model.value(s, c) / axis_max)
                .collect();
            let points = fractions
                .iter()
                .enumerate()
                .map(|(c, f)| radar_point(RADAR_RADIUS * f.clamp(0.0, 1.0), c, count))
                .collect();
            (radar_polygon(&fractions), points)
        })
        .collect();
    rsx! {
        div { class: Styles::dx_chart_root, "data-slot": "chart", style: "--chart-size: {size}px",
            Explorer { nav, label: label.clone(), categories: count, series: model.series.len(), titles: model.titles(),
                svg {
                    class: Styles::dx_chart_radar,
                    role: "img",
                    "aria-label": "{label}",
                    view_box: "0 0 {RADAR_SIZE} {RADAR_SIZE}",
                    title { "{label}" }
                    desc { "{summary}" }
                    if count > 0 {
                        for (k, (ring, y, value)) in rings.iter().enumerate() {
                            g { key: "{k}",
                                polygon { class: Styles::dx_chart_grid, points: "{ring}", fill: "none" }
                                text { class: Styles::dx_chart_tick, x: "{RADAR_CENTER + 4.0}", y: "{y - 3.0}", "{format_tick(*value)}" }
                            }
                        }
                    }
                    for (i, ((x2, y2), (lx, ly), anchor, name)) in axes.iter().enumerate() {
                        g { key: "{i}",
                            line { class: Styles::dx_chart_grid, x1: "{RADAR_CENTER}", y1: "{RADAR_CENTER}", x2: "{x2}", y2: "{y2}" }
                            text { class: Styles::dx_chart_tick, x: "{lx}", y: "{ly}", text_anchor: "{anchor}", "{name}" }
                        }
                    }
                    for (s, (shape, points)) in shapes.iter().enumerate() {
                        g { key: "{s}", "data-series": palette_index(s),
                            polygon {
                                class: Styles::dx_chart_radar_area,
                                points: "{shape}",
                                fill: "currentColor",
                                fill_opacity: "0.2",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linejoin: "round",
                                stroke_dasharray: dash(s),
                            }
                            for (c, (x, y)) in points.iter().enumerate() {
                                circle { key: "{c}", class: Styles::dx_chart_dot, cx: "{x}", cy: "{y}",
                                    r: if nav.active(s, c) { "5.5" } else { "3" },
                                    "data-active": nav.active(s, c).then_some("true"),
                                    title { "{model.mark_title(s, c)}" }
                                }
                            }
                        }
                    }
                }
            }
            Extras { caption: label.clone(), names: model.names(), marker: LegendMarker::Line, legend, table, columns: model.names(), rows: model.rows() }
        }
    }
}

/// Fraction of `max` that `value` reaches, between 0 and 1 (0 for a non-positive `max`).
pub fn progress(value: f32, max: f32) -> f32 {
    if !max.is_finite() || max <= 0.0 {
        return 0.0;
    }
    (clean(value) / max).clamp(0.0, 1.0)
}

const RING_RADIUS: f32 = 88.0;

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
) -> Element {
    let strings = use_ui_strings();
    let fraction = progress(value, max);
    let percent = format!("{}%", (fraction * 100.0).round());
    let summary = fill(
        &strings.value_of_max,
        &[
            ("value", &value.to_string()),
            ("max", &max.to_string()),
            ("percent", &percent),
        ],
    );
    let circumference = TAU * RING_RADIUS;
    let offset = circumference * (1.0 - fraction);
    let center = center.unwrap_or_else(|| percent.clone());
    let center_y = if caption.is_some() {
        PIE_CENTER + 2.0
    } else {
        PIE_CENTER + 10.0
    };
    rsx! {
        svg {
            class: Styles::dx_chart_radial,
            role: "img",
            "aria-label": "{label}",
            view_box: "0 0 {PIE_SIZE} {PIE_SIZE}",
            style: "--chart-size: {size}px",
            title { "{label}" }
            desc { "{summary}" }
            circle { class: Styles::dx_chart_ring_track, cx: "{PIE_CENTER}", cy: "{PIE_CENTER}", r: "{RING_RADIUS}", fill: "none" }
            if fraction > 0.0 {
                circle {
                    class: Styles::dx_chart_ring,
                    "data-series": "0",
                    cx: "{PIE_CENTER}",
                    cy: "{PIE_CENTER}",
                    r: "{RING_RADIUS}",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_dasharray: "{circumference:.2} {circumference:.2}",
                    stroke_dashoffset: "{offset:.2}",
                    transform: "rotate(-90 {PIE_CENTER} {PIE_CENTER})",
                }
            }
            text { class: Styles::dx_chart_center, x: "{PIE_CENTER}", y: "{center_y}", text_anchor: "middle", "{center}" }
            if let Some(caption) = caption {
                text { class: Styles::dx_chart_caption, x: "{PIE_CENTER}", y: "{PIE_CENTER + 22.0}", text_anchor: "middle", "{caption}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_max_rounds_up_to_1_2_5_10() {
        assert_eq!(nice_max(0.7), 1.0);
        assert_eq!(nice_max(1.5), 2.0);
        assert_eq!(nice_max(3.0), 5.0);
        assert_eq!(nice_max(7.0), 10.0);
        assert_eq!(nice_max(42.0), 50.0);
        assert_eq!(nice_max(100.0), 100.0);
        assert_eq!(nice_max(101.0), 200.0);
    }

    #[test]
    fn nice_max_handles_degenerate_input() {
        for bad in [0.0, -5.0, f32::NAN, f32::INFINITY] {
            assert_eq!(nice_max(bad), 1.0, "{bad}");
        }
    }

    #[test]
    fn y_stays_inside_the_plot_even_for_bad_values() {
        let plot = Plot::new(200.0, 10.0);
        for value in [-5.0, 0.0, 10.0, 1e9, f32::NAN, f32::INFINITY] {
            let y = plot.y(value);
            assert!(y >= plot.top && y <= plot.bottom, "{value} -> {y}");
        }
    }

    #[test]
    fn describe_lists_every_value() {
        let data = vec![ChartDatum::new("Jan", 12.0), ChartDatum::new("Feb", 19.5)];
        assert_eq!(describe(&data), "Jan: 12, Feb: 19.5");
        assert_eq!(describe(&[]), "No data.");
    }

    fn two_series() -> (Vec<String>, Vec<ChartSeries>) {
        (
            vec!["Jan".into(), "Feb".into()],
            vec![
                ChartSeries::new("Desktop", [186.0, 305.0]),
                ChartSeries::new("Mobile", [80.0, 200.0]),
            ],
        )
    }

    #[test]
    fn describe_series_names_every_series() {
        let (categories, series) = two_series();
        assert_eq!(
            describe_series(&categories, &series),
            "Desktop: Jan 186, Feb 305. Mobile: Jan 80, Feb 200."
        );
        assert_eq!(describe_series(&[], &series), "No data.");
    }

    #[test]
    fn stacked_axis_fits_the_tallest_stack() {
        let (categories, series) = two_series();
        let model = Model::new(&[], &categories, &series, "Value");
        assert_eq!(model.axis_max(false), 500.0);
        assert_eq!(model.axis_max(true), 1000.0);
        assert_eq!(model.below(1, 1), 305.0);
    }

    #[test]
    fn missing_categories_and_values_are_filled_in() {
        let series = vec![
            ChartSeries::new("A", [1.0, 2.0, 3.0]),
            ChartSeries::new("B", [4.0]),
        ];
        let model = Model::new(&[], &["x".to_string()], &series, "Value");
        assert_eq!(model.categories, ["x", "2", "3"]);
        assert_eq!(model.value(1, 2), 0.0);
    }

    #[test]
    fn shares_round_and_ignore_bad_values() {
        let data = vec![
            ChartDatum::new("a", 1.0),
            ChartDatum::new("b", 3.0),
            ChartDatum::new("c", -2.0),
            ChartDatum::new("d", f32::NAN),
        ];
        assert_eq!(shares(&data), [25.0, 75.0, 0.0, 0.0]);
        assert_eq!(shares(&[ChartDatum::new("z", 0.0)]), [0.0]);
        assert_eq!(describe_shares(&data[..2]), "a: 1 (25%), b: 3 (75%)");
    }

    #[test]
    fn slices_use_large_arcs_past_half_a_turn() {
        assert!(slice_path(96.0, 0.0, 0.0, 1.0).contains(" 0 0 1 "));
        assert!(slice_path(96.0, 0.0, 0.0, 4.0).contains(" 0 1 1 "));
        // A donut slice runs back along the hole.
        assert!(slice_path(96.0, 60.0, 0.0, 1.0).contains("A60,60 0 0 0"));
    }

    #[test]
    fn a_full_turn_is_two_half_arcs() {
        let pie = slice_path(96.0, 0.0, 0.0, TAU);
        assert_eq!(pie.matches('A').count(), 2, "{pie}");
        let ring = slice_path(96.0, 60.0, 0.0, TAU);
        assert_eq!(ring.matches('A').count(), 4, "{ring}");
    }

    #[test]
    fn radar_axes_start_at_twelve_and_go_clockwise() {
        let (x, y) = radar_point(100.0, 0, 4);
        assert!((x - RADAR_CENTER).abs() < 0.01 && (y - (RADAR_CENTER - 100.0)).abs() < 0.01);
        let (x, y) = radar_point(100.0, 1, 4);
        assert!((x - (RADAR_CENTER + 100.0)).abs() < 0.01 && (y - RADAR_CENTER).abs() < 0.01);
        assert_eq!(radar_polygon(&[0.0, 0.0, 0.0]).split(' ').count(), 3);
        // Out-of-range fractions stay on the chart.
        assert_eq!(radar_polygon(&[2.0]), radar_polygon(&[1.0]));
    }

    #[test]
    fn progress_is_clamped() {
        assert_eq!(progress(50.0, 200.0), 0.25);
        assert_eq!(progress(500.0, 100.0), 1.0);
        assert_eq!(progress(-1.0, 100.0), 0.0);
        assert_eq!(progress(5.0, 0.0), 0.0);
        assert_eq!(progress(f32::NAN, 10.0), 0.0);
    }
}
