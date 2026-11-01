//! Free dragging with momentum: [`Draggable`] and the testable [`DragModel`] behind it.
//!
//! While the pointer is down the element follows it exactly (with rubber-band resistance past
//! its bounds). On release it is thrown: it coasts at the pointer's speed and bounces off the
//! bounds, springs to the nearest snap point, or springs home.

use crate::gesture::{capture_pointer, now_ms};
use crate::physics::{
    rubber_band_clamp, Animation, Glide, GlideConfig, SpringConfig, SpringState, VelocityTracker,
};
use crate::{use_motion_value, SpringPreset};
use dioxus::prelude::*;

/// Which way a [`Draggable`] may move.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DragAxis {
    #[default]
    Both,
    /// Left and right only.
    X,
    /// Up and down only.
    Y,
}

/// How far a [`Draggable`] may rest from where it started, in pixels (`left` and `top` are
/// usually negative).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DragBounds {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

impl DragBounds {
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Self {
        Self {
            left: left.min(right),
            right: right.max(left),
            top: top.min(bottom),
            bottom: bottom.max(top),
        }
    }

    /// `x` pixels either side and `y` pixels above and below the start.
    pub fn around(x: f64, y: f64) -> Self {
        Self::new(-x.abs(), x.abs(), -y.abs(), y.abs())
    }

    pub fn clamp(&self, (x, y): (f64, f64)) -> (f64, f64) {
        (
            x.clamp(self.left, self.right),
            y.clamp(self.top, self.bottom),
        )
    }
}

/// Where a drag came to rest, reported by [`Draggable`]'s `on_drag_end`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DragEnd {
    /// Resting offset from the start, in pixels.
    pub x: f64,
    pub y: f64,
    /// Pointer speed at release, px/s (zero for keyboard moves).
    pub velocity: (f64, f64),
}

/// The decisions a [`Draggable`] makes, without any UI.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Debug, PartialEq)]
pub struct DragModel {
    pub axis: DragAxis,
    pub bounds: Option<DragBounds>,
    /// 0 stops dead at the bounds; higher stretches further past them (0.55 feels like iOS).
    pub elastic: f64,
    /// Coast after release instead of stopping where the pointer let go.
    pub momentum: bool,
    /// Spring back to the start on release.
    pub snap_to_origin: bool,
    /// Resting places; the release goes to the one nearest where the throw would have landed.
    pub snap_points: Vec<(f64, f64)>,
    /// The spring for snapping and bounces.
    pub spring: SpringConfig,
}

impl Default for DragModel {
    fn default() -> Self {
        Self {
            axis: DragAxis::Both,
            bounds: None,
            elastic: 0.35,
            momentum: true,
            snap_to_origin: false,
            snap_points: Vec::new(),
            spring: SpringPreset::Snappy.into(),
        }
    }
}

impl DragModel {
    fn locks(&self) -> (bool, bool) {
        (self.axis == DragAxis::Y, self.axis == DragAxis::X)
    }

    /// Where the element is while dragging: `start` is its offset when the drag began and
    /// `delta` how far the pointer has moved since.
    pub fn follow(&self, start: (f64, f64), delta: (f64, f64)) -> (f64, f64) {
        let (lock_x, lock_y) = self.locks();
        let mut x = if lock_x { start.0 } else { start.0 + delta.0 };
        let mut y = if lock_y { start.1 } else { start.1 + delta.1 };
        if let Some(b) = self.bounds {
            let w = (b.right - b.left).max(100.0);
            let h = (b.bottom - b.top).max(100.0);
            x = rubber_band_clamp(x, b.left, b.right, w, self.elastic);
            y = rubber_band_clamp(y, b.top, b.bottom, h, self.elastic);
        }
        (x, y)
    }

    /// Where a release at `at` moving at `velocity` (px/s) comes to rest.
    pub fn resting_place(&self, at: (f64, f64), velocity: (f64, f64)) -> (f64, f64) {
        let (x, y) = self.release(at, velocity);
        (x.target(), y.target())
    }

    /// The x and y animations to run after a release.
    pub fn release(&self, at: (f64, f64), velocity: (f64, f64)) -> (Animation, Animation) {
        let (lock_x, lock_y) = self.locks();
        let velocity = (
            if lock_x { 0.0 } else { velocity.0 },
            if lock_y { 0.0 } else { velocity.1 },
        );
        let spring = |from: f64, to: f64, v: f64| {
            Animation::Spring(SpringState::new(self.spring, from, to, v))
        };
        let glide_config = |bounds: Option<(f64, f64)>| GlideConfig {
            bounds,
            bounce: self.spring,
            ..GlideConfig::default()
        };
        if self.snap_to_origin {
            return (spring(at.0, 0.0, velocity.0), spring(at.1, 0.0, velocity.1));
        }
        if !self.snap_points.is_empty() {
            let probe = glide_config(None);
            let landing = (
                probe.projected(at.0, velocity.0),
                probe.projected(at.1, velocity.1),
            );
            let target = self
                .snap_points
                .iter()
                .copied()
                .filter(|p| p.0.is_finite() && p.1.is_finite())
                .min_by(|a, b| distance(*a, landing).total_cmp(&distance(*b, landing)))
                .unwrap_or(at);
            let target = self.bounds.map_or(target, |b| b.clamp(target));
            return (
                spring(at.0, target.0, velocity.0),
                spring(at.1, target.1, velocity.1),
            );
        }
        let (bx, by) = match self.bounds {
            Some(b) => (Some((b.left, b.right)), Some((b.top, b.bottom))),
            None => (None, None),
        };
        let (vx, vy) = if self.momentum { velocity } else { (0.0, 0.0) };
        (
            Animation::Glide(Glide::new(glide_config(bx), at.0, vx)),
            Animation::Glide(Glide::new(glide_config(by), at.1, vy)),
        )
    }

    /// Where an arrow key moves the element: `step` pixels in `direction`, kept in bounds and
    /// on the allowed axis. With snap points it moves to the next one that way instead.
    pub fn key_step(&self, at: (f64, f64), direction: (f64, f64), step: f64) -> (f64, f64) {
        let (lock_x, lock_y) = self.locks();
        let direction = (
            if lock_x {
                0.0
            } else {
                direction.0.signum() * direction.0.abs().min(1.0)
            },
            if lock_y {
                0.0
            } else {
                direction.1.signum() * direction.1.abs().min(1.0)
            },
        );
        if direction == (0.0, 0.0) {
            return at;
        }
        if !self.snap_points.is_empty() {
            // The nearest snap point that lies in the pressed direction.
            let ahead = self
                .snap_points
                .iter()
                .copied()
                .filter(|p| {
                    let d = (p.0 - at.0, p.1 - at.1);
                    d.0 * direction.0 + d.1 * direction.1 > 0.5
                })
                .min_by(|a, b| distance(*a, at).total_cmp(&distance(*b, at)));
            return ahead.unwrap_or(at);
        }
        let next = (at.0 + direction.0 * step, at.1 + direction.1 * step);
        self.bounds.map_or(next, |b| b.clamp(next))
    }
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

/// Drag an element anywhere with the pointer and throw it: it keeps the pointer's momentum,
/// bounces off `bounds`, and can snap to points. Keyboard users move it with the arrow keys
/// (Shift moves four times as far, Home returns it to the start).
///
/// Give it an accessible name (`aria-label`) and say what dragging does nearby. Under reduced
/// motion it still follows the pointer, but releases land at once instead of coasting.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn Draggable(
    #[props(default)] axis: DragAxis,
    /// Resting limits relative to the start. Without them it can be thrown anywhere.
    #[props(default)]
    bounds: Option<DragBounds>,
    /// Rubber-band looseness past `bounds`, 0..1.
    #[props(default = 0.35)]
    elastic: f64,
    /// Keep moving after release at the pointer's speed.
    #[props(default = true)]
    momentum: bool,
    /// Spring back to the start on release.
    #[props(default)]
    snap_to_origin: bool,
    /// Resting places (offsets from the start); releases and arrow keys go to one of them.
    #[props(default)]
    snap_points: Vec<(f64, f64)>,
    /// How the snaps and bounces feel.
    #[props(default)]
    preset: SpringPreset,
    /// Pixels per arrow-key press.
    #[props(default = 16.0)]
    step: f64,
    /// Called on every release and key move with where it will come to rest.
    #[props(default)]
    on_drag_end: EventHandler<DragEnd>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let model = DragModel {
        axis,
        bounds,
        elastic,
        momentum,
        snap_to_origin,
        snap_points,
        spring: preset.into(),
    };
    let model = use_memo(use_reactive!(|model| model));
    let mut x = use_motion_value(0.0);
    let mut y = use_motion_value(0.0);
    // (pointer id, pointer start, element offset at start)
    let mut session = use_signal(|| None::<(i32, (f64, f64), (f64, f64))>);
    let mut tracker = use_signal(VelocityTracker::new);

    let mut release = move |velocity: (f64, f64)| {
        let at = (x.peek(), y.peek());
        let (ax, ay) = model.peek().release(at, velocity);
        on_drag_end.call(DragEnd {
            x: ax.target(),
            y: ay.target(),
            velocity,
        });
        x.animate(ax);
        y.animate(ay);
    };
    let mut end_drag = move |velocity: (f64, f64)| {
        if session.take().is_some() {
            tracker.write().clear();
            release(velocity);
        }
    };

    let touch_action = match axis {
        DragAxis::Both => "none",
        DragAxis::X => "pan-y",
        DragAxis::Y => "pan-x",
    };
    let (tx, ty) = (x.get(), y.get());
    let dragging = session().is_some();
    let cursor = if dragging { "grabbing" } else { "grab" };

    rsx! {
        div {
            tabindex: "0",
            role: "group",
            "aria-roledescription": "draggable",
            "data-dragging": if dragging { "true" } else { "false" },
            style: "transform: translate({tx}px, {ty}px); touch-action: {touch_action}; cursor: {cursor}; user-select: none;",
            onpointerdown: move |e| {
                if session.peek().is_some() {
                    return;
                }
                capture_pointer(&e);
                let p = e.client_coordinates();
                x.stop();
                y.stop();
                let mut t = tracker.write();
                t.clear();
                t.push(now_ms(), p.x, p.y);
                session.set(Some((e.pointer_id(), (p.x, p.y), (x.peek(), y.peek()))));
            },
            onpointermove: move |e| {
                let Some((id, start, origin)) = *session.peek() else { return };
                if e.pointer_id() != id {
                    return;
                }
                let p = e.client_coordinates();
                tracker.write().push(now_ms(), p.x, p.y);
                let (nx, ny) = model.peek().follow(origin, (p.x - start.0, p.y - start.1));
                x.set(nx);
                y.set(ny);
            },
            onpointerup: move |e| {
                if session.peek().is_some_and(|s| s.0 == e.pointer_id()) {
                    let p = e.client_coordinates();
                    let now = now_ms();
                    let v = {
                        let mut t = tracker.write();
                        t.push(now, p.x, p.y);
                        t.velocity(now)
                    };
                    end_drag(v);
                }
            },
            onpointercancel: move |_| end_drag((0.0, 0.0)),
            // Without pointer capture (off the web) a drag can't follow the pointer outside.
            onpointerleave: move |_| {
                if !cfg!(target_arch = "wasm32") {
                    end_drag((0.0, 0.0));
                }
            },
            onkeydown: move |e| {
                let far = if e.modifiers().shift() { 4.0 } else { 1.0 };
                let direction = match e.key() {
                    Key::Named(NamedKey::ArrowLeft) => (-1.0, 0.0),
                    Key::Named(NamedKey::ArrowRight) => (1.0, 0.0),
                    Key::Named(NamedKey::ArrowUp) => (0.0, -1.0),
                    Key::Named(NamedKey::ArrowDown) => (0.0, 1.0),
                    Key::Named(NamedKey::Home) => {
                        e.prevent_default();
                        x.spring_to(0.0, preset);
                        y.spring_to(0.0, preset);
                        on_drag_end.call(DragEnd { x: 0.0, y: 0.0, velocity: (0.0, 0.0) });
                        return;
                    }
                    _ => return,
                };
                let at = (x.target(), y.target());
                let next = model.peek().key_step(at, direction, step * far);
                if next != at {
                    e.prevent_default();
                    x.spring_to(next.0, preset);
                    y.spring_to(next.1, preset);
                    on_drag_end.call(DragEnd { x: next.0, y: next.1, velocity: (0.0, 0.0) });
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settle((mut x, mut y): (Animation, Animation)) -> (f64, f64) {
        for _ in 0..600 {
            x.advance(1.0 / 60.0);
            y.advance(1.0 / 60.0);
        }
        (x.position(), y.position())
    }

    #[test]
    fn follows_the_pointer_inside_bounds_and_stretches_past_them() {
        let m = DragModel {
            bounds: Some(DragBounds::around(100.0, 50.0)),
            ..DragModel::default()
        };
        assert_eq!(m.follow((0.0, 0.0), (40.0, -20.0)), (40.0, -20.0));
        let (x, _) = m.follow((0.0, 0.0), (300.0, 0.0));
        assert!(x > 100.0 && x < 300.0, "rubber band past the edge: {x}");
        let stiff = DragModel { elastic: 0.0, ..m };
        assert_eq!(stiff.follow((0.0, 0.0), (300.0, 0.0)).0, 100.0);
    }

    #[test]
    fn a_locked_axis_never_moves() {
        let m = DragModel {
            axis: DragAxis::X,
            ..DragModel::default()
        };
        assert_eq!(m.follow((5.0, 7.0), (30.0, 90.0)), (35.0, 7.0));
        let (_, y) = m.resting_place((35.0, 7.0), (0.0, 4000.0));
        assert_eq!(y, 7.0);
    }

    #[test]
    fn a_throw_coasts_and_stays_in_bounds() {
        let m = DragModel {
            bounds: Some(DragBounds::around(200.0, 200.0)),
            ..DragModel::default()
        };
        // Slow throw: coasts a little past where it was let go.
        let (x, _) = settle(m.release((0.0, 0.0), (300.0, 0.0)));
        assert!(x > 50.0 && x < 200.0, "{x}");
        // Hard throw: ends on the edge, not past it.
        let (x, y) = settle(m.release((0.0, 0.0), (5000.0, -5000.0)));
        assert!(
            (x - 200.0).abs() < 0.05 && (y + 200.0).abs() < 0.05,
            "{x} {y}"
        );
        // No momentum: stays where it was let go.
        let still = DragModel {
            momentum: false,
            ..m
        };
        assert_eq!(
            still.resting_place((30.0, 40.0), (5000.0, 0.0)),
            (30.0, 40.0)
        );
    }

    #[test]
    fn released_past_an_edge_springs_back() {
        let m = DragModel {
            bounds: Some(DragBounds::around(100.0, 100.0)),
            ..DragModel::default()
        };
        let (x, _) = settle(m.release((140.0, 0.0), (0.0, 0.0)));
        assert!((x - 100.0).abs() < 0.05, "{x}");
    }

    #[test]
    fn snap_points_pick_the_landing_spot_of_the_throw() {
        let m = DragModel {
            axis: DragAxis::X,
            snap_points: vec![(0.0, 0.0), (150.0, 0.0), (300.0, 0.0)],
            ..DragModel::default()
        };
        assert_eq!(m.resting_place((20.0, 0.0), (0.0, 0.0)), (0.0, 0.0));
        // Let go at 20 px but thrown right at 600 px/s: projects to ~215, nearest 150.
        assert_eq!(m.resting_place((20.0, 0.0), (600.0, 0.0)), (150.0, 0.0));
        assert_eq!(m.resting_place((20.0, 0.0), (3000.0, 0.0)), (300.0, 0.0));
        let (x, _) = settle(m.release((20.0, 0.0), (600.0, 0.0)));
        assert!((x - 150.0).abs() < 0.05);
    }

    #[test]
    fn snap_to_origin_always_goes_home() {
        let m = DragModel {
            snap_to_origin: true,
            ..DragModel::default()
        };
        assert_eq!(m.resting_place((80.0, -30.0), (2000.0, 0.0)), (0.0, 0.0));
    }

    #[test]
    fn arrow_keys_step_within_bounds_or_between_snap_points() {
        let m = DragModel {
            bounds: Some(DragBounds::around(20.0, 20.0)),
            ..DragModel::default()
        };
        assert_eq!(m.key_step((0.0, 0.0), (1.0, 0.0), 16.0), (16.0, 0.0));
        assert_eq!(m.key_step((16.0, 0.0), (1.0, 0.0), 16.0), (20.0, 0.0));
        let snaps = DragModel {
            axis: DragAxis::X,
            snap_points: vec![(0.0, 0.0), (150.0, 0.0), (300.0, 0.0)],
            ..DragModel::default()
        };
        assert_eq!(snaps.key_step((0.0, 0.0), (1.0, 0.0), 16.0), (150.0, 0.0));
        assert_eq!(snaps.key_step((150.0, 0.0), (-1.0, 0.0), 16.0), (0.0, 0.0));
        assert_eq!(snaps.key_step((300.0, 0.0), (1.0, 0.0), 16.0), (300.0, 0.0));
        // Up and down do nothing on a horizontal-only drag.
        assert_eq!(snaps.key_step((150.0, 0.0), (0.0, 1.0), 16.0), (150.0, 0.0));
    }
}
