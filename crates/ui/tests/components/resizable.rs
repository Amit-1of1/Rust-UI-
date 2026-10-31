use super::render;
use dioxus::prelude::*;
use ui::components::resizable::resize_pair;
use ui::prelude::*;

#[test]
fn handles_are_window_splitters_that_name_the_panel_before_them() {
    let html = render(|| {
        rsx! {
            ResizablePanelGroup {
                ResizablePanel { id: "pane-a", default_size: 30.0, min_size: 10.0, max_size: 60.0, "A" }
                ResizableHandle { aria_label: "Resize A" }
                ResizablePanel { default_size: 70.0, "B" }
            }
        }
    });
    assert!(html.contains("role=\"separator\""), "{html}");
    assert!(html.contains("tabindex=\"0\""), "{html}");
    assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
    assert!(html.contains("aria-valuenow=\"30\""), "{html}");
    assert!(html.contains("aria-valuemin=\"10\""), "{html}");
    assert!(html.contains("aria-valuemax=\"60\""), "{html}");
    assert!(html.contains("aria-label=\"Resize A\""), "{html}");
    assert!(html.contains("aria-controls=\"pane-a\""), "{html}");
    assert!(html.contains("flex: 30 1 0px"), "{html}");
}

#[test]
fn a_divider_respects_both_panels_limits_and_keeps_the_total() {
    assert_eq!(resize_pair((50.0, 50.0), 5.0, (0.0, 100.0), (0.0, 100.0)), (55.0, 45.0));
    // `a` stops at its max.
    assert_eq!(resize_pair((50.0, 50.0), 40.0, (0.0, 60.0), (0.0, 100.0)), (60.0, 40.0));
    // `b` stops at its min, which stops `a` first.
    assert_eq!(resize_pair((50.0, 50.0), 40.0, (0.0, 100.0), (25.0, 100.0)), (75.0, 25.0));
    // Shrinking `a` below its min is refused.
    assert_eq!(resize_pair((20.0, 80.0), -15.0, (10.0, 100.0), (0.0, 100.0)), (10.0, 90.0));
    // Impossible limits leave the sizes alone.
    assert_eq!(resize_pair((50.0, 50.0), 5.0, (70.0, 100.0), (70.0, 100.0)), (50.0, 50.0));
}
