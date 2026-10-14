//! The generated Lucide set: every icon renders, and the icon rules hold (decorative by
//! default, labelled icons are images, theme sizes, RTL mirroring).

use dioxus::prelude::*;
use ui::icons::{lucide, ChevronRight, Icon, IconData, IconNode, IconSize, Search};

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn render_icon(icon: IconData) -> String {
    let mut dom =
        VirtualDom::new_with_props(|p: IconOnly| rsx! { Icon { icon: p.0 } }, IconOnly(icon));
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[derive(Clone, PartialEq)]
struct IconOnly(IconData);

#[test]
fn every_lucide_icon_renders_its_shapes() {
    assert!(lucide::ALL.len() > 1800, "{}", lucide::ALL.len());
    let mut names = std::collections::HashSet::new();
    for icon in lucide::ALL.iter() {
        assert!(names.insert(icon.name), "duplicate {}", icon.name);
        assert!(!icon.nodes.is_empty(), "{} has no shapes", icon.name);
        let html = render_icon(*icon);
        assert!(html.starts_with("<svg"), "{html}");
        assert!(
            html.contains(&format!("data-icon=\"{}\"", icon.name)),
            "{html}"
        );
        assert!(html.contains("viewBox=\"0 0 24 24\""), "{html}");
        let shapes = [
            "<path",
            "<circle",
            "<ellipse",
            "<rect",
            "<line",
            "<polyline",
            "<polygon",
        ]
        .iter()
        .map(|t| html.matches(&format!("{t} ")).count())
        .sum::<usize>();
        assert_eq!(shapes, icon.nodes.len(), "{}: {html}", icon.name);
    }
}

#[test]
fn icons_are_decorative_by_default() {
    let html = render(|| rsx! { Search {} });
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
    assert!(!html.contains("role="), "{html}");
}

#[test]
fn a_label_makes_an_image() {
    let html = render(|| rsx! { Icon { icon: lucide::CHECK, label: "Saved" } });
    assert!(
        html.contains("role=\"img\"") && html.contains("aria-label=\"Saved\""),
        "{html}"
    );
    assert!(!html.contains("aria-hidden"), "{html}");
}

#[test]
fn caller_attributes_are_not_duplicated() {
    let html = render(|| rsx! { Search { "aria-hidden": "true", stroke_width: "1.5" } });
    assert_eq!(html.matches("aria-hidden").count(), 1, "{html}");
    assert_eq!(html.matches("stroke-width").count(), 1, "{html}");
    assert!(html.contains("stroke-width=\"1.5\""), "{html}");
    // A fixed stroke opts out of the theme's stroke token.
    assert!(!html.contains("data-stroke"), "{html}");
}

#[test]
fn sizes_come_from_theme_steps_or_values() {
    let html = render(|| rsx! { Search { size: IconSize::Sm } });
    assert!(
        html.contains("data-size=\"sm\"") && html.contains("width=\"16\""),
        "{html}"
    );
    let html = render(|| rsx! { Search { size: "1rem" } });
    assert!(
        html.contains("width=\"1rem\"") && !html.contains("data-size"),
        "{html}"
    );
    let html = render(|| rsx! { Search {} });
    assert!(
        html.contains("data-size=\"lg\"") && html.contains("width=\"24\""),
        "{html}"
    );
}

#[test]
fn directional_icons_mirror_in_rtl() {
    let cases = [
        (lucide::CHEVRON_RIGHT, true),
        (lucide::ARROW_LEFT, true),
        (lucide::UNDO_2, true),
        (lucide::CHEVRON_DOWN, false),
        (lucide::ARROW_UP, false),
        (lucide::SEARCH, false),
    ];
    for (icon, mirrors) in cases {
        assert_eq!(icon.mirror_rtl, mirrors, "{}", icon.name);
    }
    let html = render(|| rsx! { ChevronRight {} });
    assert!(html.contains("data-mirror-rtl=\"true\""), "{html}");
    let html = render(|| rsx! { ChevronRight { mirror_rtl: false } });
    assert!(!html.contains("data-mirror-rtl"), "{html}");
}

#[test]
fn filled_dots_and_optional_rect_radii() {
    let dots = lucide::CHART_SCATTER
        .nodes
        .iter()
        .filter(|n| matches!(n, IconNode::Circle { filled: true, .. }))
        .count();
    assert_eq!(dots, 5);
    let html = render(|| rsx! { Icon { icon: lucide::CHART_SCATTER } });
    assert_eq!(html.matches("fill=\"currentColor\"").count(), 5, "{html}");
}

#[test]
fn old_lucide_names_still_work() {
    // Renamed upstream (alert-circle -> circle-alert); both names draw the same icon.
    assert_eq!(lucide::ALERT_CIRCLE, lucide::CIRCLE_ALERT);
    let old = render(|| rsx! { ui::icons::AlertCircle {} });
    let new = render(|| rsx! { ui::icons::CircleAlert {} });
    assert_eq!(old, new);
}
