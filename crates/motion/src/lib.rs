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

mod flip;
mod gesture;
mod preset;
mod reduced;

pub use flip::{FlipItem, FlipList};
pub use gesture::{SwipeDirection, SwipeDismiss, SwipeOutcome, SwipeTracker};
pub use preset::{SpringPreset, Transition};
pub use reduced::use_reduced_motion;

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
