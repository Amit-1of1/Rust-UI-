//! Exact-value tests for the motion physics and the drag model, written to kill the mutants
//! `cargo mutants` found surviving (docs/notes/mutation-testing.md). The older tests mostly
//! checked direction or sign; these pin the numbers, so a flipped operator or a dropped guard
//! fails here. Each expected value is worked out from the formula in the comment above it.

use motion::physics::{
    rubber_band, rubber_band_clamp, Animation, Glide, GlideConfig, SpringConfig, SpringState,
    VelocityTracker,
};
use motion::{DragBounds, DragModel, SpringPreset};

fn close(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

fn spring() -> SpringConfig {
    SpringConfig::new(170.0, 26.0, 1.0)
}

// --- springs ---

#[test]
fn an_undamped_spring_settles_at_the_ten_second_cap() {
    // It never settles, so the loop stops at the first step at or past 10 s.
    let t = SpringConfig::new(100.0, 0.0, 1.0).settle_time(100.0, 0.0);
    assert!((10.0..10.0 + 1.0 / 240.0).contains(&t), "{t}");
}

#[test]
fn a_spring_reports_its_real_speed() {
    let mut s = SpringState::new(spring(), 0.0, 100.0, 0.0);
    assert_eq!(s.velocity(), 0.0);
    s.advance(0.05);
    let v = s.velocity();
    // The speed is the slope of the position: compare with a central difference.
    let at = |t: f64| {
        let mut p = SpringState::new(spring(), 0.0, 100.0, 0.0);
        p.advance(t);
        p.position()
    };
    let slope = (at(0.05 + 1e-6) - at(0.05 - 1e-6)) / 2e-6;
    assert!(v > 1.0, "{v}");
    assert!(close(v, slope, 1e-3 * slope.abs()), "{v} vs {slope}");
    assert_eq!(Animation::Spring(s).velocity(), v);
}

#[test]
fn time_never_runs_backwards_or_from_nan() {
    let mut s = SpringState::new(spring(), 0.0, 100.0, 0.0);
    s.advance(0.05);
    let before = s.position();
    for dt in [-1.0, f64::NAN, f64::INFINITY, 0.0] {
        s.advance(dt);
        assert_eq!(s.position(), before, "spring, dt = {dt}");
    }
    let mut g = Glide::new(GlideConfig::default(), 0.0, 1000.0);
    g.advance(0.1);
    let before = g.position();
    for dt in [-1.0, f64::NAN, f64::INFINITY, 0.0] {
        g.advance(dt);
        assert_eq!(g.position(), before, "glide, dt = {dt}");
    }
}

#[test]
fn a_spring_at_rest_on_its_target_is_done() {
    let s = SpringState::new(spring(), 5.0, 5.0, 0.0);
    assert!(s.is_done());
    assert!(Animation::Spring(s).is_done());
    assert!(!SpringState::new(spring(), 0.0, 5.0, 0.0).is_done());
}

// --- glides ---

#[test]
fn power_scales_how_far_a_throw_goes() {
    // from + v * time_constant * power = 0 + 100 * 0.325 * 2 = 65.
    let c = GlideConfig {
        power: 2.0,
        ..GlideConfig::default()
    };
    assert!(close(c.projected(0.0, 100.0), 65.0, 1e-12));
}

#[test]
fn reversed_bounds_are_ignored_not_a_panic() {
    let c = GlideConfig {
        bounds: Some((10.0, 0.0)),
        ..GlideConfig::default()
    };
    assert_eq!(c.resting_place(5.0, 0.0), 5.0);
}

#[test]
fn a_glide_starts_where_it_was_thrown_at_the_throw_speed() {
    let mut g = Glide::new(GlideConfig::default(), 0.0, 1000.0);
    assert_eq!(g.position(), 0.0);
    assert!(close(g.velocity(), 1000.0, 1e-9), "{}", g.velocity());
    assert!(!g.is_done());
    // After 0.5 s: speed v * e^(-t / tc), distance left amplitude * e^(-t / tc).
    g.advance(0.5);
    let decay = (-0.5f64 / 0.325).exp();
    assert!(
        close(g.velocity(), 1000.0 * decay, 1e-9),
        "{}",
        g.velocity()
    );
    assert!(close(g.position(), 325.0 * (1.0 - decay), 1e-9));
    assert!(!g.is_done(), "{} left", 325.0 * decay);
    g.advance(4.5);
    assert!(g.is_done());
    assert!(close(g.position(), g.target(), 0.5));
    assert_eq!(Animation::Glide(g.clone()).velocity(), g.velocity());
}

#[test]
fn a_glide_with_nothing_to_travel_is_done_at_once() {
    assert!(Glide::new(GlideConfig::default(), 0.0, 0.0).is_done());
    assert!(Animation::Glide(Glide::new(GlideConfig::default(), 3.0, 0.0)).is_done());
}

// --- pointer speed ---

#[test]
fn pointer_speed_is_exact() {
    // 10 units in 50 ms is 200 units per second (off the origin, so x - x0 is checked).
    let mut t = VelocityTracker::new();
    t.push(0.0, 4.0, 3.0);
    t.push(50.0, 14.0, -2.0);
    assert_eq!(t.velocity(50.0), (200.0, -100.0));
    t.clear();
    assert_eq!(t.velocity(50.0), (0.0, 0.0));
}

#[test]
fn a_bad_sample_is_skipped() {
    // Each bad sample would be the start of the window if it were kept.
    for bad in [
        (120.0, f64::NAN, 0.0),
        (120.0, 0.0, f64::INFINITY),
        (f64::NAN, 0.0, 0.0),
    ] {
        let mut t = VelocityTracker::new();
        t.push(0.0, 0.0, 0.0);
        t.push(bad.0, bad.1, bad.2);
        t.push(150.0, 15.0, 0.0);
        // Only the first and last count: 15 units in 150 ms.
        assert_eq!(t.velocity(150.0), (100.0, 0.0), "{bad:?}");
        assert_eq!(t.len(), 2);
    }
}

#[test]
fn a_repeated_timestamp_keeps_the_history() {
    let mut t = VelocityTracker::new();
    t.push(0.0, 0.0, 0.0);
    t.push(50.0, 10.0, 0.0);
    t.push(50.0, 10.0, 0.0);
    assert_eq!(t.velocity(50.0), (200.0, 0.0));
}

#[test]
fn a_gap_longer_than_the_window_uses_the_sample_before_it() {
    // Nothing inside the last 100 ms but the newest sample: measure from the one before.
    let mut t = VelocityTracker::new();
    t.push(1000.0, 0.0, 0.0);
    t.push(1150.0, 30.0, 0.0);
    assert_eq!(t.velocity(1150.0), (200.0, 0.0));
}

#[test]
fn a_pointer_still_for_100_ms_has_no_speed() {
    let mut t = VelocityTracker::new();
    t.push(0.0, 0.0, 0.0);
    t.push(50.0, 10.0, 0.0);
    assert_eq!(t.velocity(149.0), (200.0, 0.0));
    assert_eq!(t.velocity(150.0), (0.0, 0.0));
}

#[test]
fn the_history_stays_short() {
    // Trimming keeps only the window, so a long drag doesn't grow memory or slow down.
    let mut t = VelocityTracker::new();
    assert!(t.is_empty());
    t.push(0.0, 0.0, 0.0);
    assert!(!t.is_empty());
    t.push(1.0, 1.0, 0.0);
    assert_eq!(t.len(), 2);
    for i in 2..10_000 {
        t.push(f64::from(i), f64::from(i), 0.0);
    }
    // One sample a millisecond: the 100 ms window holds 101, plus the one before it.
    assert!((100..=102).contains(&t.len()), "{} samples kept", t.len());
    assert!(close(t.velocity(9_999.0).0, 1000.0, 1e-6));
}

// --- rubber banding ---

#[test]
fn rubber_band_is_exact() {
    // (1 - 1 / (|o| * c / d + 1)) * d = (1 - 1 / 1.5) * 100 = 33.33...
    assert!(close(rubber_band(100.0, 100.0, 0.5), 100.0 / 3.0, 1e-9));
    assert!(close(rubber_band(-100.0, 100.0, 0.5), -100.0 / 3.0, 1e-9));
    // Below the range: 0 + band(-50) = -(1 - 1 / 1.25) * 100 = -20; above: 100 + 20.
    assert!(close(
        rubber_band_clamp(-50.0, 0.0, 100.0, 100.0, 0.5),
        -20.0,
        1e-9
    ));
    assert!(close(
        rubber_band_clamp(150.0, 0.0, 100.0, 100.0, 0.5),
        120.0,
        1e-9
    ));
    // Off the origin: 10 + band(-50) = -10.
    assert!(close(
        rubber_band_clamp(-40.0, 10.0, 110.0, 100.0, 0.5),
        -10.0,
        1e-9
    ));
    assert_eq!(rubber_band_clamp(40.0, 0.0, 100.0, 100.0, 0.5), 40.0);
}

// --- dragging ---

#[test]
fn dragging_past_an_edge_stretches_by_the_box_size() {
    // Box 300 wide and 200 tall, away from the origin, elastic 0.35, 100 past the edge:
    // edge + (1 - 1 / (100 * 0.35 / size + 1)) * size.
    let m = DragModel {
        bounds: Some(DragBounds::new(50.0, 350.0, 20.0, 220.0)),
        ..DragModel::default()
    };
    let want_x = 350.0 + (1.0 - 1.0 / (100.0 * 0.35 / 300.0 + 1.0)) * 300.0;
    let want_y = 220.0 + (1.0 - 1.0 / (100.0 * 0.35 / 200.0 + 1.0)) * 200.0;
    assert!(close(m.follow((50.0, 20.0), (400.0, 0.0)).0, want_x, 1e-9));
    assert!(close(m.follow((50.0, 20.0), (0.0, 300.0)).1, want_y, 1e-9));
}

#[test]
fn the_models_spring_drives_the_bounce_back() {
    let at = |spring: SpringConfig| {
        let m = DragModel {
            bounds: Some(DragBounds::new(0.0, 300.0, 0.0, 300.0)),
            spring,
            ..DragModel::default()
        };
        let (mut x, _) = m.release((400.0, 0.0), (0.0, 0.0));
        x.advance(0.1);
        x.position()
    };
    let snappy = at(SpringPreset::Snappy.into());
    let gentle = at(SpringPreset::Gentle.into());
    assert!((snappy - gentle).abs() > 1.0, "{snappy} vs {gentle}");
}

#[test]
fn a_nan_snap_point_is_never_chosen() {
    let m = DragModel {
        snap_points: vec![(f64::NAN, 0.0), (0.0, f64::NAN)],
        ..DragModel::default()
    };
    assert_eq!(m.resting_place((5.0, 5.0), (0.0, 0.0)), (5.0, 5.0));
}

#[test]
fn release_snaps_to_the_nearest_point_to_the_landing() {
    // Landing at (50, 0): (60, 0) is 10 away, (-40, 0) is 90.
    let m = DragModel {
        snap_points: vec![(-40.0, 0.0), (60.0, 0.0)],
        ..DragModel::default()
    };
    assert_eq!(m.resting_place((50.0, 0.0), (0.0, 0.0)), (60.0, 0.0));
    // The same in y.
    let m = DragModel {
        snap_points: vec![(0.0, -40.0), (0.0, 60.0)],
        ..DragModel::default()
    };
    assert_eq!(m.resting_place((0.0, 50.0), (0.0, 0.0)), (0.0, 60.0));
}

#[test]
fn arrow_keys_step_to_the_next_snap_point_ahead() {
    let m = DragModel {
        snap_points: vec![(0.0, 0.0), (100.0, 0.0), (200.0, 0.0)],
        ..DragModel::default()
    };
    assert_eq!(m.key_step((100.0, 0.0), (1.0, 0.0), 10.0), (200.0, 0.0));
    assert_eq!(m.key_step((100.0, 0.0), (-1.0, 0.0), 10.0), (0.0, 0.0));
    assert_eq!(m.key_step((200.0, 0.0), (1.0, 0.0), 10.0), (200.0, 0.0));
    // A point less than half a pixel ahead is where you already are.
    let m = DragModel {
        snap_points: vec![(100.4, 0.0), (200.0, 0.0)],
        ..DragModel::default()
    };
    assert_eq!(m.key_step((100.0, 0.0), (1.0, 0.0), 10.0), (200.0, 0.0));
    // Off the line: the nearest point ahead, measured from where you are.
    let m = DragModel {
        snap_points: vec![(150.0, 80.0), (300.0, 0.0)],
        ..DragModel::default()
    };
    assert_eq!(m.key_step((100.0, 0.0), (1.0, 0.0), 10.0), (150.0, 80.0));
    // A point beside you is not ahead, in either axis.
    let m = DragModel {
        snap_points: vec![(0.0, 10.0), (100.0, 0.0), (10.0, 0.0), (0.0, 100.0)],
        ..DragModel::default()
    };
    assert_eq!(m.key_step((0.0, 0.0), (1.0, 0.0), 10.0), (10.0, 0.0));
    assert_eq!(m.key_step((0.0, 0.0), (0.0, 1.0), 10.0), (0.0, 10.0));
    // Up and down, from off the origin.
    let m = DragModel {
        snap_points: vec![(5.0, 0.0), (5.0, 100.0), (5.0, 200.0)],
        ..DragModel::default()
    };
    assert_eq!(m.key_step((5.0, 100.0), (0.0, 1.0), 10.0), (5.0, 200.0));
    assert_eq!(m.key_step((5.0, 100.0), (0.0, -1.0), 10.0), (5.0, 0.0));
    // A half-strength key (a gamepad stick halfway): "ahead" is weighed by the strength, so a
    // point 0.8 px ahead counts as 0.4 and is not ahead.
    let m = DragModel {
        snap_points: vec![(0.8, 0.0), (50.0, 0.0), (0.0, 0.8), (0.0, 50.0)],
        ..DragModel::default()
    };
    assert_eq!(m.key_step((0.0, 0.0), (0.5, 0.0), 10.0), (50.0, 0.0));
    assert_eq!(m.key_step((0.0, 0.0), (0.0, 0.5), 10.0), (0.0, 50.0));
}

#[test]
fn arrow_keys_step_exactly() {
    let m = DragModel::default();
    assert_eq!(m.key_step((10.0, 10.0), (1.0, 0.0), 5.0), (15.0, 10.0));
    assert_eq!(m.key_step((10.0, 10.0), (0.0, -1.0), 5.0), (10.0, 5.0));
    assert_eq!(m.key_step((10.0, 10.0), (3.0, 0.0), 5.0), (15.0, 10.0));
}
