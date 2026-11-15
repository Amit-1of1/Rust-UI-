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

mod resize_proofs {
    use proptest::prelude::*;
    use ui::components::resizable::resize_pair;

    fn any_f32() -> impl Strategy<Value = f32> {
        prop_oneof![
            any::<f32>(),
            prop::sample::select(vec![0.0, 100.0, f32::INFINITY, f32::NEG_INFINITY, f32::NAN]),
            0.0..100.0f32,
        ]
    }

    proptest! {
        #![proptest_config(ProptestConfig { cases: 20_000, ..ProptestConfig::default() })]

        /// Limits are props, so any float can arrive: never a panic (NaN limits used to make
        /// `f32::clamp` panic), and a non-finite drag leaves the panels as they were.
        #[test]
        fn never_panics(s0 in any_f32(), s1 in any_f32(), d in any_f32(),
                        a0 in any_f32(), a1 in any_f32(), b0 in any_f32(), b1 in any_f32()) {
            let out = resize_pair((s0, s1), d, (a0, a1), (b0, b1));
            if !d.is_finite() {
                prop_assert!(out.0.to_bits() == s0.to_bits() && out.1.to_bits() == s1.to_bits());
            }
        }

        /// Sane panels: the pair keeps its total, and both stay inside their limits whenever
        /// that is possible.
        #[test]
        fn keeps_the_total_and_the_limits(
            s0 in 0.0..100.0f32, d in -150.0..150.0f32,
            a in (0.0..50.0f32, 50.0..100.0f32), b in (0.0..50.0f32, 50.0..100.0f32),
        ) {
            let s1 = 100.0 - s0;
            let (x, y) = resize_pair((s0, s1), d, a, b);
            prop_assert!(((x + y) - (s0 + s1)).abs() <= 1e-3, "{x} + {y}");
            let feasible = a.0.max(100.0 - b.1) <= a.1.min(100.0 - b.0);
            if feasible {
                prop_assert!(x >= a.0 - 1e-3 && x <= a.1 + 1e-3, "{x} outside {a:?}");
                prop_assert!(y >= b.0 - 1e-3 && y <= b.1 + 1e-3, "{y} outside {b:?}");
            } else {
                prop_assert_eq!((x, y), (s0, s1));
            }
        }
    }
}
