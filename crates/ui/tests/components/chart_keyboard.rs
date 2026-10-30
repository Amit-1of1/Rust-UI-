//! Charts with `keyboard: true` and the radar chart. The browser side (focus, keys, the
//! status line) is in the chart's contract.toml, run by tests/e2e/contracts.mjs.

use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

fn at(category: usize, series: usize) -> ChartCursor {
    ChartCursor { category, series }
}

#[test]
fn arrows_move_across_categories_and_series_and_stop_at_the_ends() {
    let c = ChartCursor::default();
    assert_eq!(c.apply(ChartKey::Previous, 6, 2), at(0, 0), "no wrap at the start");
    let c = c.apply(ChartKey::Next, 6, 2);
    assert_eq!(c, at(1, 0));
    let c = c.apply(ChartKey::NextSeries, 6, 2);
    assert_eq!(c, at(1, 1));
    assert_eq!(c.apply(ChartKey::NextSeries, 6, 2), at(1, 1), "no wrap at the last series");
    assert_eq!(c.apply(ChartKey::PreviousSeries, 6, 2), at(1, 0));
    assert_eq!(c.apply(ChartKey::Last, 6, 2), at(5, 1), "End keeps the series");
    assert_eq!(at(5, 1).apply(ChartKey::Next, 6, 2), at(5, 1));
    assert_eq!(at(4, 1).apply(ChartKey::First, 6, 2), at(0, 1));
}

#[test]
fn with_one_series_up_and_down_walk_the_points() {
    assert_eq!(at(2, 0).apply(ChartKey::NextSeries, 5, 1), at(3, 0));
    assert_eq!(at(2, 0).apply(ChartKey::PreviousSeries, 5, 1), at(1, 0));
}

#[test]
fn a_cursor_outside_shrunk_data_is_pulled_back() {
    assert_eq!(at(9, 3).apply(ChartKey::Previous, 4, 2), at(2, 1));
    assert_eq!(at(3, 0).apply(ChartKey::Next, 0, 0), at(0, 0), "empty chart");
}

#[test]
fn keys_map_to_moves() {
    assert_eq!(ChartKey::from_key(&Key::ArrowRight), Some(ChartKey::Next));
    assert_eq!(ChartKey::from_key(&Key::ArrowUp), Some(ChartKey::PreviousSeries));
    assert_eq!(ChartKey::from_key(&Key::End), Some(ChartKey::Last));
    assert_eq!(ChartKey::from_key(&Key::Tab), None, "Tab leaves the chart");
}

#[test]
fn a_keyboard_chart_is_one_named_tab_stop_with_a_hint_and_a_status_line() {
    let html = render(|| {
        rsx! {
            BarChart {
                data: vec![ChartDatum::new("Jan", 3.0), ChartDatum::new("Feb", 5.0)],
                keyboard: true,
                label: "Sales",
            }
        }
    });
    assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
    assert!(html.contains("role=\"group\""), "{html}");
    assert!(html.contains("aria-describedby=\"dx-chart-hint-"), "{html}");
    assert!(html.contains("Use the arrow keys to move between points."), "{html}");
    assert!(html.contains("role=\"status\""), "{html}");
    assert!(!html.contains("data-active"), "no cursor until focused: {html}");
}

#[test]
fn charts_without_keyboard_stay_out_of_the_tab_order() {
    let html = render(|| {
        rsx! {
            LineChart { data: vec![ChartDatum::new("Jan", 3.0)], label: "Sales" }
            PieChart { data: vec![ChartDatum::new("A", 1.0)], label: "Share" }
            RadarChart { data: vec![ChartDatum::new("A", 1.0)], label: "Profile" }
        }
    });
    assert!(!html.contains("tabindex"), "{html}");
    assert!(!html.contains("role=\"status\""), "{html}");
}

#[test]
fn radar_draws_one_shape_per_series_with_rings_and_labelled_axes() {
    let html = render(|| {
        rsx! {
            RadarChart {
                categories: vec!["Speed".into(), "Range".into(), "Price".into()],
                series: vec![
                    ChartSeries::new("A", [8.0, 4.0, 6.0]),
                    ChartSeries::new("B", [5.0, 9.0, 7.0]),
                ],
                max: 10.0,
                label: "Two cars",
            }
        }
    });
    assert!(html.contains("role=\"img\""), "{html}");
    assert!(html.contains("aria-label=\"Two cars\""), "{html}");
    // Four rings, two shapes.
    assert_eq!(html.matches("<polygon").count(), 6, "{html}");
    for text in [">Speed<", ">Range<", ">Price<", ">10<", ">5<"] {
        assert!(html.contains(text), "missing {text} in {html}");
    }
    assert!(html.contains("A: Speed 8, Range 4, Price 6."), "summary: {html}");
    assert!(html.contains("data-slot=\"chart-legend\""), "two series get a legend: {html}");
    assert!(html.contains("Speed, B: 5"), "point titles: {html}");
}

#[test]
fn radar_without_data_says_so() {
    let html = render(|| rsx! { RadarChart { label: "Nothing" } });
    assert!(html.contains("No data."), "{html}");
    assert!(!html.contains("<polygon"), "{html}");
}
