use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;

#[css_module("/src/components/chart/style.css")]
struct Styles;

/// One point or bar.
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

const WIDTH: f32 = 400.0;
const PLOT_TOP: f32 = 12.0;
const PLOT_LEFT: f32 = 36.0;
const PLOT_RIGHT: f32 = 8.0;
const LABEL_SPACE: f32 = 24.0;

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

/// Largest finite, non-negative value in `data`.
fn max_value(data: &[ChartDatum]) -> f32 {
    data.iter()
        .map(|d| d.value)
        .filter(|v| v.is_finite())
        .fold(0.0, f32::max)
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

struct Plot {
    height: f32,
    top: f32,
    bottom: f32,
    axis_max: f32,
}

impl Plot {
    fn new(height: f32, data: &[ChartDatum]) -> Self {
        Self {
            height,
            top: PLOT_TOP,
            bottom: height - LABEL_SPACE,
            axis_max: nice_max(max_value(data)),
        }
    }

    fn y(&self, value: f32) -> f32 {
        let fraction = (clean(value) / self.axis_max).clamp(0.0, 1.0);
        self.bottom - fraction * (self.bottom - self.top)
    }

    fn x_center(&self, index: usize, count: usize) -> f32 {
        let slot = (WIDTH - PLOT_LEFT - PLOT_RIGHT) / count.max(1) as f32;
        PLOT_LEFT + slot * (index as f32 + 0.5)
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

#[component]
fn Grid(plot_height: f32, data: Vec<ChartDatum>) -> Element {
    let plot = Plot::new(plot_height, &data);
    rsx! {
        for (y, value) in plot.gridlines() {
            line { class: Styles::dx_chart_grid, x1: "{PLOT_LEFT}", x2: "{WIDTH - PLOT_RIGHT}", y1: "{y}", y2: "{y}" }
            text { class: Styles::dx_chart_tick, x: "{PLOT_LEFT - 6.0}", y: "{y + 3.0}", text_anchor: "end", "{format_tick(value)}" }
        }
    }
}

/// A bar chart drawn as SVG in the accent colour. `label` is read by screen readers along
/// with a summary of every value; hovering a bar shows its value.
#[component]
pub fn BarChart(
    data: Vec<ChartDatum>,
    #[props(into)] label: String,
    #[props(default = 200.0)] height: f32,
) -> Element {
    let plot = Plot::new(height, &data);
    let count = data.len();
    let slot = (WIDTH - PLOT_LEFT - PLOT_RIGHT) / count.max(1) as f32;
    let bar_width = (slot * 0.6).max(2.0);
    let no_data = use_ui_strings().no_data;
    let summary = if data.is_empty() {
        no_data
    } else {
        describe(&data)
    };
    rsx! {
        svg {
            class: Styles::dx_chart,
            role: "img",
            "aria-label": "{label}",
            view_box: "0 0 {WIDTH} {height}",
            title { "{label}" }
            desc { "{summary}" }
            Grid { plot_height: height, data: data.clone() }
            for (i, datum) in data.iter().enumerate() {
                {
                    let x = plot.x_center(i, count) - bar_width / 2.0;
                    let y = plot.y(datum.value);
                    let bar_height = (plot.bottom - y).max(0.0);
                    rsx! {
                        rect { key: "{datum.label}", class: Styles::dx_chart_bar, x: "{x}", y: "{y}", width: "{bar_width}", height: "{bar_height}", rx: "3",
                            // Shown on hover; the values are also in the chart's `<desc>`.
                            title { "{datum.label}: {datum.value}" }
                        }
                        text { class: Styles::dx_chart_tick, x: "{plot.x_center(i, count)}", y: "{plot.height - 6.0}", text_anchor: "middle", "{datum.label}" }
                    }
                }
            }
        }
    }
}

/// A line chart drawn as SVG with theme colors. Same accessibility contract as [`BarChart`].
#[component]
pub fn LineChart(
    data: Vec<ChartDatum>,
    #[props(into)] label: String,
    #[props(default = 200.0)] height: f32,
) -> Element {
    let plot = Plot::new(height, &data);
    let count = data.len();
    let points: Vec<(f32, f32)> = data
        .iter()
        .enumerate()
        .map(|(i, d)| (plot.x_center(i, count), plot.y(d.value)))
        .collect();
    let path = points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ");
    let no_data = use_ui_strings().no_data;
    let summary = if data.is_empty() {
        no_data
    } else {
        describe(&data)
    };
    rsx! {
        svg {
            class: Styles::dx_chart,
            role: "img",
            "aria-label": "{label}",
            view_box: "0 0 {WIDTH} {height}",
            title { "{label}" }
            desc { "{summary}" }
            Grid { plot_height: height, data: data.clone() }
            if points.len() > 1 {
                // The presentation attributes are a fallback for renderers that draw inline SVG
                // without the page's CSS (Blitz); where CSS applies, the class wins.
                polyline {
                    class: Styles::dx_chart_line,
                    points: "{path}",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
            for (i, (x, y)) in points.iter().enumerate() {
                circle { key: "{data[i].label}", class: Styles::dx_chart_dot, cx: "{x}", cy: "{y}", r: "3.5",
                    title { "{data[i].label}: {data[i].value}" }
                }
                text { class: Styles::dx_chart_tick, x: "{x}", y: "{plot.height - 6.0}", text_anchor: "middle", "{data[i].label}" }
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
        let data = vec![ChartDatum::new("a", 10.0)];
        let plot = Plot::new(200.0, &data);
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
}
