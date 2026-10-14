//! Swipe gesture maths and the `SwipeDismiss` component.
//!
//! The decision logic lives in [`SwipeTracker`] with no UI in it so it can be unit tested.
//! The thresholds follow the usual swipe-to-dismiss rule: travel far enough, or flick fast enough.

use crate::{use_spring, Spring, SpringPreset};
use dioxus::prelude::*;

/// Milliseconds on a monotonic clock.
fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::sync::OnceLock;
        use std::time::Instant;
        static START: OnceLock<Instant> = OnceLock::new();
        START.get_or_init(Instant::now).elapsed().as_secs_f64() * 1000.0
    }
}

/// A pause this long between the last move and the release cancels the flick.
const STALE_MS: f64 = 100.0;

/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipeDirection {
    Left,
    Right,
}

/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwipeOutcome {
    /// Released far enough, or fast enough, in this direction.
    Dismiss(SwipeDirection),
    /// Released too early: spring back.
    Cancel,
}

/// Tracks one horizontal drag.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwipeTracker {
    start_x: f64,
    last_x: f64,
    last_t: f64,
    /// Smoothed horizontal velocity in px/ms.
    velocity: f64,
    /// Fraction of the element width that counts as "far enough".
    pub distance_ratio: f64,
    /// Speed in px/ms that counts as a flick.
    pub flick_speed: f64,
}

impl SwipeTracker {
    pub fn new(x: f64, t_ms: f64) -> Self {
        Self {
            start_x: x,
            last_x: x,
            last_t: t_ms,
            velocity: 0.0,
            distance_ratio: 0.4,
            flick_speed: 0.5,
        }
    }

    /// Record a pointer move; returns the offset from where the drag began.
    pub fn update(&mut self, x: f64, t_ms: f64) -> f64 {
        let dt = t_ms - self.last_t;
        if dt > 0.0 {
            let instant = (x - self.last_x) / dt;
            // Smooth so one jittery sample doesn't decide the outcome.
            self.velocity = 0.6 * instant + 0.4 * self.velocity;
        }
        self.last_x = x;
        self.last_t = t_ms;
        x - self.start_x
    }

    /// Take the release position as the final one without touching the velocity: a browser
    /// can coalesce or drop the last moves before the release, and the release is where the
    /// pointer really ended up.
    pub fn release_at(&mut self, x: f64) {
        self.last_x = x;
    }

    pub fn offset(&self) -> f64 {
        self.last_x - self.start_x
    }

    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    /// Velocity in px/ms at release time `t_ms`; zero if the pointer had already stopped.
    pub fn release_velocity(&self, t_ms: f64) -> f64 {
        if t_ms - self.last_t > STALE_MS {
            0.0
        } else {
            self.velocity
        }
    }

    /// Decide what the release at `t_ms` means for an element `width` pixels wide.
    ///
    /// A pointer that stopped moving before it was released is not a flick.
    pub fn finish(&self, width: f64, t_ms: f64) -> SwipeOutcome {
        let velocity = self.release_velocity(t_ms);
        let offset = self.offset();
        let direction = if offset >= 0.0 {
            SwipeDirection::Right
        } else {
            SwipeDirection::Left
        };
        let far = offset.abs() >= width.max(1.0) * self.distance_ratio;
        // A flick only counts if it is moving the same way as the drag so far.
        let flick = velocity.abs() >= self.flick_speed && velocity * offset > 0.0;
        if far || flick {
            SwipeOutcome::Dismiss(direction)
        } else {
            SwipeOutcome::Cancel
        }
    }
}

/// Keep receiving this pointer's events after it leaves the element (web only; no JS needed).
/// Captures on the event target (a child of the element is fine: its events bubble to us); with
/// Dioxus' delegated listeners `current_target` would be the document root, not the element.
fn capture_pointer(e: &Event<PointerData>) {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(ev) = e.data().downcast::<web_sys::PointerEvent>() {
            if let Some(el) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            {
                let _ = el.set_pointer_capture(ev.pointer_id());
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = e;
}

/// Drag an element sideways (or up and down with `vertical`) to dismiss it, like a toast or a
/// bottom drawer. Keyboard users press Delete, Backspace or Escape while it has focus.
///
/// With `one_way` only the positive direction (right, or down when vertical) can dismiss and the
/// element does not follow the pointer the other way: right for a bottom drawer you pull down.
///
/// `on_dismiss` is called after the exit animation finishes (or at once under reduced motion).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn SwipeDismiss(
    on_dismiss: EventHandler<()>,
    /// Width used for the "far enough" test and the exit distance when it can't be measured.
    #[props(default = 320.0)]
    width: f64,
    /// Drag up and down instead of left and right. `width` then means the height.
    #[props(default)]
    vertical: bool,
    /// Only dismiss toward the right (or down); the other way springs back.
    #[props(default)]
    one_way: bool,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    // `progress` runs 0 -> 1 after release; the element moves from `from` to `to` along it.
    let mut progress: Spring = use_spring(0.0, SpringPreset::Snappy);
    let mut from = use_signal(|| 0.0_f32);
    let mut to = use_signal(|| 0.0_f32);
    let mut tracker = use_signal(|| None::<SwipeTracker>);
    // While the pointer is down the element follows it exactly; the spring takes over on release.
    let mut dragging = use_signal(|| false);
    let mut dismissed = use_signal(|| false);
    let mut notified = use_signal(|| false);

    // `velocity` is the pointer speed in px/ms at release, so a throw keeps its momentum.
    let mut release = move |start: f32, end: f32, velocity: f64| {
        dragging.set(false);
        from.set(start);
        to.set(end);
        progress.reset();
        progress.to(1.0);
        let distance = (end - start) as f64;
        if distance.abs() > 1.0 {
            // Progress units per second; negative if the pointer was moving away from the target.
            progress.set_velocity((velocity * 1000.0 / distance) as f32);
        }
    };
    let mut dismiss = move |start: f32, dir: SwipeDirection, velocity: f64| {
        if dismissed() {
            return;
        }
        dismissed.set(true);
        let sign = if dir == SwipeDirection::Right {
            1.0
        } else {
            -1.0
        };
        release(start, sign * width as f32 * 1.2, velocity);
    };
    // Tell the parent once the exit spring has settled.
    use_effect(move || {
        if dismissed() && !progress.is_running() && progress.get_value() > 0.5 && !notified() {
            notified.set(true);
            on_dismiss.call(());
        }
    });

    let shown = if dragging() {
        tracker().map(|t| t.offset() as f32).unwrap_or(0.0)
    } else {
        from() + (to() - from()) * progress.get_value()
    };
    let shown = if one_way { shown.max(0.0) } else { shown };
    let (axis, touch_action) = if vertical {
        ("Y", "pan-x")
    } else {
        ("X", "pan-y")
    };
    let coordinate = move |e: &Event<PointerData>| {
        let point = e.client_coordinates();
        if vertical {
            point.y
        } else {
            point.x
        }
    };
    let opacity = (1.0 - (shown.abs() / (width as f32 * 1.2))).clamp(0.0, 1.0);

    rsx! {
        div {
            tabindex: "0",
            style: "transform: translate{axis}({shown}px); opacity: {opacity}; touch-action: {touch_action}; cursor: grab;",
            onpointerdown: move |e| {
                if dismissed() {
                    return;
                }
                capture_pointer(&e);
                progress.reset();
                from.set(0.0);
                to.set(0.0);
                tracker.set(Some(SwipeTracker::new(coordinate(&e), now_ms())));
                dragging.set(true);
            },
            onpointermove: move |e| {
                if let Some(mut t) = tracker() {
                    t.update(coordinate(&e), now_ms());
                    tracker.set(Some(t));
                }
            },
            onpointerup: move |e| {
                if let Some(mut t) = tracker.take() {
                    t.release_at(coordinate(&e));
                    let start = t.offset() as f32;
                    let now = now_ms();
                    let velocity = t.release_velocity(now);
                    match t.finish(width, now) {
                        SwipeOutcome::Dismiss(SwipeDirection::Left) if one_way => {
                            release(start, 0.0, velocity);
                        }
                        SwipeOutcome::Dismiss(dir) => dismiss(start, dir, velocity),
                        SwipeOutcome::Cancel => release(start, 0.0, velocity),
                    }
                }
            },
            onpointerleave: move |_| {
                if tracker().is_some() {
                    let start = tracker().map(|t| t.offset() as f32).unwrap_or(0.0);
                    tracker.set(None);
                    release(start, 0.0, 0.0);
                }
            },
            onkeydown: move |e| {
                if matches!(e.key(), Key::Delete | Key::Backspace | Key::Escape) {
                    e.prevent_default();
                    dismiss(0.0, SwipeDirection::Right, 0.0);
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

    #[test]
    fn release_position_counts_when_moves_were_dropped() {
        // Only one move arrived, but the pointer was released far enough down.
        let mut t = SwipeTracker::new(0.0, 0.0);
        t.update(20.0, 16.0);
        t.release_at(160.0);
        assert_eq!(t.offset(), 160.0);
        assert_eq!(
            t.finish(320.0, 500.0),
            SwipeOutcome::Dismiss(SwipeDirection::Right)
        );
    }

    #[test]
    fn short_slow_drag_cancels() {
        let mut t = SwipeTracker::new(0.0, 0.0);
        t.update(20.0, 200.0);
        assert_eq!(t.finish(300.0, t.last_t), SwipeOutcome::Cancel);
    }

    #[test]
    fn long_drag_dismisses_in_its_direction() {
        let mut t = SwipeTracker::new(300.0, 0.0);
        t.update(100.0, 800.0);
        assert_eq!(
            t.finish(300.0, t.last_t),
            SwipeOutcome::Dismiss(SwipeDirection::Left)
        );
        let mut t = SwipeTracker::new(0.0, 0.0);
        t.update(150.0, 900.0);
        assert_eq!(
            t.finish(300.0, t.last_t),
            SwipeOutcome::Dismiss(SwipeDirection::Right)
        );
    }

    #[test]
    fn fast_flick_dismisses_even_when_short() {
        let mut t = SwipeTracker::new(0.0, 0.0);
        t.update(10.0, 5.0);
        t.update(40.0, 15.0);
        t.update(70.0, 25.0);
        assert!(t.velocity() > 0.5);
        assert_eq!(
            t.finish(300.0, t.last_t),
            SwipeOutcome::Dismiss(SwipeDirection::Right)
        );
    }

    #[test]
    fn flick_then_pause_does_not_dismiss() {
        let mut t = SwipeTracker::new(0.0, 0.0);
        t.update(20.0, 10.0);
        t.update(40.0, 20.0);
        assert!(t.velocity() > 0.5);
        // Held still for half a second before letting go.
        assert_eq!(t.finish(300.0, 520.0), SwipeOutcome::Cancel);
        assert_eq!(
            t.finish(300.0, 25.0),
            SwipeOutcome::Dismiss(SwipeDirection::Right)
        );
    }

    #[test]
    fn release_velocity_is_zero_once_the_pointer_has_stopped() {
        let mut t = SwipeTracker::new(0.0, 0.0);
        t.update(30.0, 10.0);
        t.update(60.0, 20.0);
        assert!(t.release_velocity(25.0) > 0.5);
        assert_eq!(t.release_velocity(500.0), 0.0);
    }

    #[test]
    fn flick_against_the_drag_does_not_dismiss() {
        let mut t = SwipeTracker::new(0.0, 0.0);
        t.update(60.0, 400.0);
        // Fast move back toward the start, but still right of it.
        t.update(40.0, 410.0);
        t.update(20.0, 420.0);
        assert!(t.velocity() < 0.0 && t.offset() > 0.0);
        assert_eq!(t.finish(300.0, t.last_t), SwipeOutcome::Cancel);
    }

    #[test]
    fn zero_width_does_not_divide_by_zero() {
        let t = SwipeTracker::new(0.0, 0.0);
        assert_eq!(t.finish(0.0, t.last_t), SwipeOutcome::Cancel);
    }
}
