//! Pinch and pan to zoom: [`PinchZoom`] and the testable [`PinchTracker`] behind it.

use crate::gesture::{capture_pointer, now_ms};
use crate::physics::{
    pan_bounds, rubber_band_clamp, zoom_about, GlideConfig, SpringConfig, VelocityTracker,
};
use crate::{use_motion_value, MotionValue, SpringPreset};
use dioxus::prelude::*;
use std::rc::Rc;

/// What the pointers on a [`PinchTracker`] are doing, relative to when the gesture (re)started.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PinchFrame {
    /// One pointer, moved by `delta`.
    Pan { delta: (f64, f64) },
    /// Two pointers: their distance changed by `scale`, they turned by `rotation` radians, and
    /// their midpoint started at `anchor` and moved by `delta`.
    Pinch {
        scale: f64,
        rotation: f64,
        anchor: (f64, f64),
        delta: (f64, f64),
    },
}

/// `(pointer id, start, current)`.
type Tracked = (i32, (f64, f64), (f64, f64));

/// Tracks up to two pointers for pan and pinch. Whenever a pointer is added or lifted the
/// gesture restarts from where the pointers are, so going from one finger to two (or back)
/// never jumps.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PinchTracker {
    pointers: Vec<Tracked>,
}

impl PinchTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> usize {
        self.pointers.len()
    }

    fn rebase(&mut self) {
        for p in &mut self.pointers {
            p.1 = p.2;
        }
    }

    /// A pointer went down. Returns false (and ignores it) for a third finger.
    pub fn down(&mut self, id: i32, at: (f64, f64)) -> bool {
        if self.pointers.len() >= 2 || self.pointers.iter().any(|p| p.0 == id) {
            return false;
        }
        self.pointers.push((id, at, at));
        self.rebase();
        true
    }

    /// A pointer lifted (or was cancelled).
    pub fn up(&mut self, id: i32) {
        self.pointers.retain(|p| p.0 != id);
        self.rebase();
    }

    /// A pointer moved; returns the gesture so far, or `None` for a pointer we don't track.
    pub fn move_to(&mut self, id: i32, at: (f64, f64)) -> Option<PinchFrame> {
        let p = self.pointers.iter_mut().find(|p| p.0 == id)?;
        p.2 = at;
        Some(self.frame())
    }

    /// The midpoint of the tracked pointers now.
    pub fn centroid(&self) -> Option<(f64, f64)> {
        let n = self.pointers.len() as f64;
        if n == 0.0 {
            return None;
        }
        let (x, y) = self
            .pointers
            .iter()
            .fold((0.0, 0.0), |a, p| (a.0 + p.2 .0, a.1 + p.2 .1));
        Some((x / n, y / n))
    }

    fn frame(&self) -> PinchFrame {
        match self.pointers.as_slice() {
            [(_, s, c)] => PinchFrame::Pan {
                delta: (c.0 - s.0, c.1 - s.1),
            },
            [(_, s1, c1), (_, s2, c2)] => {
                let d0 = (s2.0 - s1.0).hypot(s2.1 - s1.1);
                let d1 = (c2.0 - c1.0).hypot(c2.1 - c1.1);
                let scale = if d0 > 1.0 { d1 / d0 } else { 1.0 };
                let a0 = (s2.1 - s1.1).atan2(s2.0 - s1.0);
                let a1 = (c2.1 - c1.1).atan2(c2.0 - c1.0);
                let anchor = ((s1.0 + s2.0) / 2.0, (s1.1 + s2.1) / 2.0);
                let now = ((c1.0 + c2.0) / 2.0, (c1.1 + c2.1) / 2.0);
                PinchFrame::Pinch {
                    scale,
                    rotation: a1 - a0,
                    anchor,
                    delta: (now.0 - anchor.0, now.1 - anchor.1),
                }
            }
            _ => PinchFrame::Pan { delta: (0.0, 0.0) },
        }
    }
}

/// `(offset x, offset y, scale)` of the zoomed content.
type View = (f64, f64, f64);

/// The view a gesture frame produces from the view it started at, in container coordinates.
/// Past `min..max` the scale stretches with resistance; panning stretches past the edges.
pub fn pinch_view(start: View, frame: PinchFrame, size: (f64, f64), min: f64, max: f64) -> View {
    let (ox, oy, s) = start;
    let (o, s) = match frame {
        PinchFrame::Pan { delta } => ((ox + delta.0, oy + delta.1), s),
        PinchFrame::Pinch {
            scale,
            anchor,
            delta,
            ..
        } => {
            let wanted = rubber_band_clamp(s * scale, min, max, (max - min).max(1.0), 0.5);
            let ((x, y), s2) = zoom_about((ox, oy), s, anchor, wanted / s);
            ((x + delta.0, y + delta.1), s2)
        }
    };
    let (lx, hx) = pan_bounds(size.0, s);
    let (ly, hy) = pan_bounds(size.1, s);
    (
        rubber_band_clamp(o.0, lx, hx, size.0, 0.35),
        rubber_band_clamp(o.1, ly, hy, size.1, 0.35),
        s,
    )
}

/// Zoom by `factor` about `anchor`, hard-limited to `min..max` and the pan bounds.
pub fn zoom_view(
    view: View,
    anchor: (f64, f64),
    factor: f64,
    size: (f64, f64),
    min: f64,
    max: f64,
) -> View {
    let (ox, oy, s) = view;
    let target = (s * factor).clamp(min, max);
    let ((x, y), s) = zoom_about((ox, oy), s, anchor, target / s);
    clamp_view((x, y, s), size)
}

/// Move `view` inside its pan bounds.
pub fn clamp_view((x, y, s): View, size: (f64, f64)) -> View {
    let (lx, hx) = pan_bounds(size.0, s);
    let (ly, hy) = pan_bounds(size.1, s);
    (x.clamp(lx, hx), y.clamp(ly, hy), s)
}

#[derive(Clone, Copy)]
struct ViewValues {
    x: MotionValue,
    y: MotionValue,
    s: MotionValue,
}

impl ViewValues {
    fn now(&self) -> View {
        (self.x.peek(), self.y.peek(), self.s.peek())
    }
    fn target(&self) -> View {
        (self.x.target(), self.y.target(), self.s.target())
    }
    fn set(&mut self, (x, y, s): View) {
        self.x.set(x);
        self.y.set(y);
        self.s.set(s);
    }
    fn spring(&mut self, (x, y, s): View, config: SpringConfig) {
        self.x.spring_with(x, config, self.x.velocity());
        self.y.spring_with(y, config, self.y.velocity());
        self.s.spring_with(s, config.with_rest(0.0005, 0.005), 0.0);
    }
}

/// Zoom into content with two fingers, a trackpad pinch (Ctrl + wheel), a double click, or
/// the keyboard (`+`, `-`, `0` to reset, arrows to pan once zoomed). One finger pans when
/// zoomed in and is thrown with momentum; letting go past the limits springs back.
///
/// The content keeps its layout size and is scaled with a transform. Give the container an
/// accessible name (`aria-label`).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn PinchZoom(
    #[props(default = 1.0)] min_scale: f64,
    #[props(default = 4.0)] max_scale: f64,
    /// Zoom factor per key press, double click target and wheel step cap.
    #[props(default = 1.5)]
    step: f64,
    #[props(default)] preset: SpringPreset,
    /// Called with the scale it will rest at after each gesture or key press.
    #[props(default)]
    on_zoom: EventHandler<f64>,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let min = min_scale.max(0.01);
    let max = max_scale.max(min);
    let step = step.max(1.01);
    let mut view = ViewValues {
        x: use_motion_value(0.0),
        y: use_motion_value(0.0),
        s: use_motion_value(1.0),
    };
    let spring: SpringConfig = preset.into();
    let mut tracker = use_signal(PinchTracker::new);
    let mut velocity = use_signal(VelocityTracker::new);
    // The view when the current gesture (re)started.
    let mut base = use_signal(|| (0.0, 0.0, 1.0));
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    // Container origin (client coordinates) and size.
    let mut rect = use_signal(|| ((0.0, 0.0), (300.0, 200.0)));
    let measure = move || {
        if let Some(m) = mounted() {
            spawn(async move {
                if let Ok(r) = m.get_client_rect().await {
                    rect.set(((r.origin.x, r.origin.y), (r.size.width, r.size.height)));
                }
            });
        }
    };
    let local = move |p: (f64, f64)| {
        let ((ox, oy), _) = *rect.peek();
        (p.0 - ox, p.1 - oy)
    };
    let size = move || rect.peek().1;
    let center = move || {
        let (w, h) = size();
        (w / 2.0, h / 2.0)
    };
    let mut zoom_to = move |target: View| {
        view.spring(target, spring);
        on_zoom.call(target.2);
    };
    let mut settle = move |throw: (f64, f64)| {
        let (x, y, s) = view.now();
        let anchor = tracker.peek().centroid().map(local).unwrap_or_else(center);
        let clamped_scale = s.clamp(min, max);
        if (clamped_scale - s).abs() > 1e-6 {
            // Pinched past a limit: spring back about the fingers.
            let target = zoom_view((x, y, s), anchor, clamped_scale / s, size(), min, max);
            zoom_to(target);
            return;
        }
        let (w, h) = size();
        let (lx, hx) = pan_bounds(w, s);
        let (ly, hy) = pan_bounds(h, s);
        let glide = |bounds| GlideConfig {
            bounds: Some(bounds),
            bounce: spring,
            ..GlideConfig::default()
        };
        view.x.glide(throw.0, glide((lx, hx)));
        view.y.glide(throw.1, glide((ly, hy)));
        on_zoom.call(s);
    };

    let (x, y, s) = (view.x.get(), view.y.get(), view.s.get());
    let percent = (s * 100.0).round();

    rsx! {
        div {
            tabindex: "0",
            role: "group",
            "aria-roledescription": "zoomable area",
            "data-scale": "{s:.3}",
            style: "overflow: hidden; touch-action: none; user-select: none; position: relative;",
            onmounted: move |e| {
                mounted.set(Some(e.data()));
                measure();
            },
            onpointerdown: move |e| {
                let p = e.client_coordinates();
                if tracker.write().down(e.pointer_id(), (p.x, p.y)) {
                    capture_pointer(&e);
                    measure();
                    view.x.stop();
                    view.y.stop();
                    view.s.stop();
                    base.set(view.now());
                    let mut v = velocity.write();
                    v.clear();
                    v.push(now_ms(), p.x, p.y);
                }
            },
            onpointermove: move |e| {
                let p = e.client_coordinates();
                let frame = tracker.write().move_to(e.pointer_id(), (p.x, p.y));
                let Some(frame) = frame else { return };
                if let Some(c) = tracker.peek().centroid() {
                    velocity.write().push(now_ms(), c.0, c.1);
                }
                let frame = match frame {
                    PinchFrame::Pinch { scale, rotation, anchor, delta } => PinchFrame::Pinch {
                        scale,
                        rotation,
                        anchor: local(anchor),
                        delta,
                    },
                    pan => pan,
                };
                let next = pinch_view(*base.peek(), frame, size(), min, max);
                view.set(next);
            },
            onpointerup: move |e| {
                let was = tracker.peek().count();
                let throw = if was == 1 { velocity.peek().velocity(now_ms()) } else { (0.0, 0.0) };
                if was == 0 {
                    return;
                }
                if tracker.peek().count() == 2 {
                    // Settle the pinch about the fingers before one of them leaves.
                    settle((0.0, 0.0));
                }
                tracker.write().up(e.pointer_id());
                velocity.write().clear();
                if tracker.peek().count() == 1 {
                    // Back to one finger: keep panning from here.
                    base.set(view.target());
                    if let Some(c) = tracker.peek().centroid() {
                        velocity.write().push(now_ms(), c.0, c.1);
                    }
                } else if was == 1 {
                    settle(throw);
                }
            },
            onpointercancel: move |e| {
                tracker.write().up(e.pointer_id());
                if tracker.peek().count() == 0 {
                    settle((0.0, 0.0));
                }
            },
            onwheel: move |e| {
                // Trackpad pinches arrive as Ctrl + wheel; plain wheel keeps scrolling the page.
                let m = e.modifiers();
                if !(m.ctrl() || m.meta()) {
                    return;
                }
                e.prevent_default();
                measure();
                let dy = e.delta().strip_units().y.clamp(-25.0, 25.0);
                let p = e.client_coordinates();
                let next = zoom_view(view.target(), local((p.x, p.y)), (-dy * 0.01).exp(), size(), min, max);
                view.set(next);
                on_zoom.call(next.2);
            },
            ondoubleclick: move |e| {
                measure();
                let p = e.client_coordinates();
                let (_, _, s) = view.target();
                let target = if s > min + 0.01 {
                    clamp_view((0.0, 0.0, min), size())
                } else {
                    zoom_view(view.target(), local((p.x, p.y)), step, size(), min, max)
                };
                zoom_to(target);
            },
            onkeydown: move |e| {
                let current = view.target();
                let pan = size().0.min(size().1) * 0.2;
                let target = match e.key() {
                    Key::Character(c) if c == "+" || c == "=" => {
                        zoom_view(current, center(), step, size(), min, max)
                    }
                    Key::Character(c) if c == "-" || c == "_" => {
                        zoom_view(current, center(), 1.0 / step, size(), min, max)
                    }
                    Key::Character(c) if c == "0" => clamp_view((0.0, 0.0, min), size()),
                    Key::Named(NamedKey::ArrowLeft) => clamp_view((current.0 + pan, current.1, current.2), size()),
                    Key::Named(NamedKey::ArrowRight) => clamp_view((current.0 - pan, current.1, current.2), size()),
                    Key::Named(NamedKey::ArrowUp) => clamp_view((current.0, current.1 + pan, current.2), size()),
                    Key::Named(NamedKey::ArrowDown) => clamp_view((current.0, current.1 - pan, current.2), size()),
                    _ => return,
                };
                if target != current {
                    e.prevent_default();
                    zoom_to(target);
                }
            },
            ..attributes,
            div {
                style: "transform: translate({x}px, {y}px) scale({s}); transform-origin: 0 0;",
                {children}
            }
            span {
                style: "position: absolute; width: 1px; height: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap;",
                "aria-live": "polite",
                "{percent}%"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_pointer_pans_and_two_pinch() {
        let mut t = PinchTracker::new();
        assert!(t.down(1, (0.0, 0.0)));
        assert_eq!(
            t.move_to(1, (10.0, 5.0)),
            Some(PinchFrame::Pan { delta: (10.0, 5.0) })
        );
        // Second finger: the gesture restarts from here.
        assert!(t.down(2, (110.0, 5.0)));
        let Some(PinchFrame::Pinch {
            scale,
            anchor,
            delta,
            rotation,
        }) = t.move_to(2, (210.0, 5.0))
        else {
            panic!("expected a pinch");
        };
        assert!((scale - 2.0).abs() < 1e-9);
        assert_eq!(anchor, (60.0, 5.0));
        assert_eq!(delta, (50.0, 0.0));
        assert_eq!(rotation, 0.0);
        // A third finger is ignored, an unknown pointer moving is too.
        assert!(!t.down(3, (0.0, 0.0)));
        assert_eq!(t.move_to(3, (1.0, 1.0)), None);
        // Lifting one finger goes back to a pan from where things are.
        t.up(1);
        assert_eq!(t.count(), 1);
        assert_eq!(
            t.move_to(2, (220.0, 5.0)),
            Some(PinchFrame::Pan { delta: (10.0, 0.0) })
        );
    }

    #[test]
    fn rotation_is_reported_in_radians() {
        let mut t = PinchTracker::new();
        t.down(1, (0.0, 0.0));
        t.down(2, (100.0, 0.0));
        let Some(PinchFrame::Pinch { rotation, .. }) = t.move_to(2, (0.0, 100.0)) else {
            panic!()
        };
        assert!((rotation - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    }

    #[test]
    fn pinching_zooms_about_the_fingers_and_resists_past_the_limit() {
        let size = (200.0, 200.0);
        let frame = PinchFrame::Pinch {
            scale: 2.0,
            rotation: 0.0,
            anchor: (100.0, 100.0),
            delta: (0.0, 0.0),
        };
        let (x, y, s) = pinch_view((0.0, 0.0, 1.0), frame, size, 1.0, 4.0);
        assert_eq!((x, y, s), (-100.0, -100.0, 2.0));
        let huge = PinchFrame::Pinch {
            scale: 10.0,
            rotation: 0.0,
            anchor: (100.0, 100.0),
            delta: (0.0, 0.0),
        };
        let (_, _, s) = pinch_view((0.0, 0.0, 1.0), huge, size, 1.0, 4.0);
        assert!(
            s > 4.0 && s < 10.0,
            "stretches past max with resistance: {s}"
        );
    }

    #[test]
    fn panning_at_scale_one_only_stretches() {
        let v = pinch_view(
            (0.0, 0.0, 1.0),
            PinchFrame::Pan { delta: (80.0, 0.0) },
            (200.0, 200.0),
            1.0,
            4.0,
        );
        assert!(v.0 > 0.0 && v.0 < 80.0, "{v:?}");
        assert_eq!(clamp_view(v, (200.0, 200.0)), (0.0, 0.0, 1.0));
    }

    #[test]
    fn keyboard_and_wheel_zoom_is_hard_limited() {
        let size = (200.0, 100.0);
        let v = zoom_view((0.0, 0.0, 1.0), (100.0, 50.0), 1.5, size, 1.0, 4.0);
        assert_eq!(v, (-50.0, -25.0, 1.5));
        let v = zoom_view((0.0, 0.0, 3.0), (100.0, 50.0), 10.0, size, 1.0, 4.0);
        assert_eq!(v.2, 4.0);
        let v = zoom_view((0.0, 0.0, 1.0), (100.0, 50.0), 0.1, size, 1.0, 4.0);
        assert_eq!(v, (0.0, 0.0, 1.0));
    }
}
