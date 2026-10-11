use dioxus::prelude::*;
use motion::{AnimatePresence, Pop, SpringPreset, Transition};

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn presets_are_ordered_from_soft_to_bouncy() {
    let gentle = SpringPreset::Gentle.spring();
    let snappy = SpringPreset::Snappy.spring();
    let bouncy = SpringPreset::Bouncy.spring();
    assert!(gentle.stiffness < snappy.stiffness);
    assert!(snappy.stiffness < bouncy.stiffness);
    // Damping ratio = c / (2 * sqrt(k * m)); lower means more overshoot.
    let ratio =
        |s: dioxus_motion::prelude::Spring| s.damping / (2.0 * (s.stiffness * s.mass).sqrt());
    assert!(ratio(bouncy) < 0.5, "bouncy should overshoot");
    assert!(
        ratio(snappy) > 0.8,
        "snappy should settle without much overshoot"
    );
    assert!(
        ratio(gentle) > 0.8,
        "gentle should settle without much overshoot"
    );
}

#[test]
fn every_preset_is_a_valid_animation() {
    for preset in [
        SpringPreset::Gentle,
        SpringPreset::Snappy,
        SpringPreset::Bouncy,
    ] {
        for reduced in [false, true] {
            Transition::new(preset)
                .resolve(reduced)
                .validate()
                .expect("preset config should validate");
        }
    }
}

#[test]
fn reduced_motion_replaces_the_spring_with_a_near_instant_tween() {
    let reduced = Transition::new(SpringPreset::Bouncy).resolve(true);
    assert!(reduced.get_duration() <= std::time::Duration::from_millis(5));
}

#[test]
fn pop_renders_its_children_in_the_initial_state() {
    let html = render(|| {
        rsx! {
            AnimatePresence {
                Pop { key: "{1}", id: "box", "Hello" }
            }
        }
    });
    assert!(html.contains("Hello"), "{html}");
    assert!(html.contains("id=\"box\""), "{html}");
    // Entering from invisible: the first frame must not flash at full opacity.
    assert!(html.contains("opacity"), "{html}");
}

#[test]
fn swipe_dismiss_is_keyboard_focusable_and_at_rest() {
    fn app() -> Element {
        rsx! {
            motion::SwipeDismiss { on_dismiss: |_| {}, "Card" }
        }
    }
    let html = render(app);
    assert!(html.contains("tabindex=\"0\""), "{html}");
    assert!(html.contains("translateX(0px)"), "{html}");
    assert!(html.contains("touch-action: pan-y"), "{html}");
}

#[test]
fn vertical_swipe_moves_on_the_y_axis_and_leaves_x_scrolling_alone() {
    fn app() -> Element {
        rsx! {
            motion::SwipeDismiss { on_dismiss: |_| {}, vertical: true, one_way: true, "Sheet" }
        }
    }
    let html = render(app);
    assert!(html.contains("translateY(0px)"), "{html}");
    assert!(html.contains("touch-action: pan-x"), "{html}");
}

#[test]
fn flip_item_wraps_content_without_changing_it() {
    fn app() -> Element {
        rsx! {
            motion::FlipList {
                motion::FlipItem { span { "row" } }
            }
        }
    }
    let html = render(app);
    assert!(html.contains("<span>row</span>"), "{html}");
    assert!(html.contains("translateY(0px)"), "{html}");
}
