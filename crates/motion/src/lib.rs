//! Motion for the Rust UI kit.
//!
//! This is the only place app and component code should touch the animation engine
//! (`dioxus-motion`, a one-maintainer crate). Swapping the engine later means changing
//! this crate, not every component (docs/GUARDRAILS.md).
//!
//! * [`SpringPreset`]: named spring feels instead of raw stiffness/damping numbers.
//! * [`use_reduced_motion`]: the OS "reduce motion" setting.
//! * [`Transition`]: a preset that collapses to an instant change when motion is reduced.
//! * [`Pop`]: enter/exit animation for children of an [`AnimatePresence`].
//! * [`FlipList`] and [`FlipItem`]: siblings slide into place when a row is added or removed.
//! * [`SwipeDismiss`]: drag sideways (or press Delete) to dismiss.
//! * [`use_spring`]: a spring-driven `f32`, e.g. for hover and press effects.
//! * [`physics`]: our own spring solver (closed form), momentum glides, velocity tracking,
//!   rubber banding and zoom maths, all plain Rust and unit tested.
//! * [`use_motion_value`]: a value driven by that physics; set it while dragging, then spring
//!   or throw it with the pointer's speed.
//! * [`Draggable`]: drag and throw with momentum, bounds, rubber banding and snap points.
//! * [`PinchZoom`]: pinch, Ctrl + wheel, double click and keyboard zoom with pan.
//! * [`LayoutGroup`] and [`LayoutItem`]: position and size changes animate, and items sharing
//!   a `layout_id` morph into each other (shared-element transitions).
//! * [`css`]: the same presets traced as CSS `linear()` easings, for CSS-animated overlays.
//!
//! # Stability
//!
//! Stable: [`SpringPreset`], [`Transition`], [`use_reduced_motion`], [`Pop`], and
//! [`AnimatePresence`], [`use_is_present`] and [`PresenceMode`] as used with `Pop`.
//! Experimental: [`use_spring`], [`FlipList`]/[`FlipItem`], [`SwipeDismiss`] and its types,
//! [`physics`], [`use_motion_value`], [`Draggable`], [`PinchZoom`], [`LayoutGroup`]/[`LayoutItem`]
//! and their types, and the other items re-exported from `dioxus-motion` (they change when the
//! engine does).
//! Internal: [`css`]. Details in docs/STABILITY.md.
#![forbid(unsafe_code)]

pub mod css;
mod drag;
mod flip;
mod gesture;
mod layout;
pub mod physics;
mod pinch;
mod preset;
mod reduced;
mod value;

pub use drag::{DragAxis, DragBounds, DragEnd, DragModel, Draggable};
pub use flip::{FlipItem, FlipList};
pub use gesture::{SwipeDirection, SwipeDismiss, SwipeOutcome, SwipeTracker};
pub use layout::{LayoutDelta, LayoutGroup, LayoutItem, LayoutRect};
pub use pinch::{clamp_view, pinch_view, zoom_view, PinchFrame, PinchTracker, PinchZoom};
pub use preset::{SpringPreset, Transition};
pub use reduced::use_reduced_motion;
pub use value::{use_motion_value, MotionValue};

// Engine re-exports. AnimatePresence, use_is_present and PresenceMode are stable; the rest is
// experimental: their types are dioxus-motion's and change with it (docs/STABILITY.md).
pub use dioxus_motion::prelude::{
    use_is_present, use_motion, AnimatePresence, AnimationConfig, AnimationError, MotionHandle,
    MotionStyle, PresenceMode,
};

use dioxus::prelude::*;
use dioxus_motion::prelude::{use_presence_style, AnimationManager, PresenceConfig};

/// Fade, scale and slide a child in and out. Render it as a keyed child of
/// [`AnimatePresence`] so the exit animation can finish before the child unmounts.
///
/// Honors `prefers-reduced-motion`: the child still appears and disappears, but without travel.
#[component]
pub fn Pop(
    /// How the motion feels.
    #[props(default)]
    preset: SpringPreset,
    /// Vertical travel in pixels while entering and exiting.
    #[props(default = 12.0)]
    distance: f32,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let reduced = use_reduced_motion();
    let transition = Transition::new(preset).resolve(reduced);
    let distance = if reduced { 0.0 } else { distance };
    let config = PresenceConfig::new(
        MotionStyle::new(0.0)
            .y(distance)
            .scale(if reduced { 1.0 } else { 0.96 }),
        MotionStyle::new(1.0).y(0.0).scale(1.0),
        MotionStyle::new(0.0)
            .y(-distance)
            .scale(if reduced { 1.0 } else { 0.96 }),
        transition,
    );
    let style = use_presence_style(config)?;
    let css = style.get_value().to_css();

    rsx! {
        div { style: "{css}", ..attributes, {children} }
    }
}

/// A spring-driven `f32` that eases toward whatever you set, keeping its momentum when
/// retargeted. Reads as `value.get_value()`; move it with [`Spring::to`].
///
/// With reduced motion on, [`Spring::to`] jumps straight to the target.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy)]
pub struct Spring {
    motion: MotionHandle<f32>,
    reduced: bool,
    preset: SpringPreset,
}

impl Spring {
    /// Current animated value. Reading it re-renders the component as it changes.
    pub fn get_value(&self) -> f32 {
        self.motion.get_value()
    }

    /// Whether the spring is still moving.
    pub fn is_running(&self) -> bool {
        self.motion.is_running()
    }

    /// Set the spring's current speed (units per second). Only meaningful right after [`Spring::to`].
    pub fn set_velocity(&mut self, velocity: f32) {
        // Fails if the spring isn't running (e.g. reduced motion made it instant); that's fine.
        let _ = self.motion.set_velocity(velocity);
    }

    /// Jump back to the starting value and stop.
    pub fn reset(&mut self) {
        self.motion.reset();
    }

    /// Animate toward `target`.
    pub fn to(&mut self, target: f32) {
        let config = Transition::new(self.preset).resolve(self.reduced);
        if let Err(error) = self.motion.animate_to(target, config) {
            // A bad target (NaN, infinity) leaves the spring where it was.
            eprintln!("motion: ignoring invalid spring target {target}: {error}");
        }
    }
}

/// Create a [`Spring`] starting at `initial`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_spring(initial: f32, preset: SpringPreset) -> Spring {
    let reduced = use_reduced_motion();
    // `use_motion` only fails for a non-finite initial value; fall back to zero.
    let initial = if initial.is_finite() { initial } else { 0.0 };
    let motion = use_motion(initial).unwrap_or_else(|_| unreachable!("finite initial value"));
    Spring {
        motion,
        reduced,
        preset,
    }
}
