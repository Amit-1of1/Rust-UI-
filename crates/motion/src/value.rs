//! [`MotionValue`]: a number driven by our own physics ([`crate::physics`]) one frame at a time.
//!
//! Unlike [`crate::Spring`] (dioxus-motion's tween/spring engine), a `MotionValue` can be set
//! directly while a finger drags it, then handed to a spring or a momentum glide that starts at
//! the finger's speed. Frames come from the engine's timer (`requestAnimationFrame` on the web,
//! the Tokio runtime on desktop and native); without a timer the value jumps to its end.

use crate::gesture::now_ms;
use crate::physics::{Animation, Glide, GlideConfig, SpringConfig, SpringState};
use crate::reduced::use_reduced_motion_signal;
use crate::SpringPreset;
use dioxus::core::{current_scope_id, Runtime, ScopeId};
use dioxus::prelude::*;
use dioxus_motion::{MotionTime, TimeProvider};
use std::time::Duration;

/// A frame longer than this (a background tab, a debugger pause) counts as this long, so the
/// value doesn't teleport.
const MAX_FRAME_S: f64 = 0.064;

/// A spring- or momentum-driven `f64`. Copy it into event handlers freely.
///
/// Read it with [`MotionValue::get`] (re-renders as it moves). With reduced motion on, every
/// animation jumps straight to where it would have ended; [`MotionValue::set`] still follows
/// a dragging pointer exactly, because direct manipulation isn't decorative motion.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy)]
pub struct MotionValue {
    value: Signal<f64>,
    animating: Signal<bool>,
    // Not reactive: written every frame without re-rendering anyone.
    animation: CopyValue<Option<Animation>>,
    looping: CopyValue<bool>,
    scope: ScopeId,
    // Checked when an animation starts, so a change to the OS setting applies at once.
    reduced: Signal<bool>,
}

impl PartialEq for MotionValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Create a [`MotionValue`] starting at `initial` (non-finite values start at 0).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_motion_value(initial: f64) -> MotionValue {
    let reduced = use_reduced_motion_signal();
    let initial = if initial.is_finite() { initial } else { 0.0 };
    let value = use_signal(|| initial);
    let animating = use_signal(|| false);
    let animation = use_hook(|| CopyValue::new(None));
    let looping = use_hook(|| CopyValue::new(false));
    let scope = use_hook(current_scope_id);
    MotionValue {
        value,
        animating,
        animation,
        looping,
        scope,
        reduced,
    }
}

impl MotionValue {
    /// The current value. Reading it re-renders the component on every frame it moves.
    pub fn get(&self) -> f64 {
        (self.value)()
    }

    /// The current value, without subscribing.
    pub fn peek(&self) -> f64 {
        *self.value.peek()
    }

    /// Current speed in units per second (zero at rest).
    pub fn velocity(&self) -> f64 {
        self.animation
            .read()
            .as_ref()
            .map(Animation::velocity)
            .unwrap_or(0.0)
    }

    /// Where the value is heading (its current value at rest).
    pub fn target(&self) -> f64 {
        self.animation
            .read()
            .as_ref()
            .map(Animation::target)
            .unwrap_or_else(|| self.peek())
    }

    /// Whether an animation is running. Reactive: re-renders when it starts or stops.
    pub fn is_animating(&self) -> bool {
        (self.animating)()
    }

    /// Whether an animation is running, without subscribing.
    pub fn is_moving(&self) -> bool {
        self.animation.read().is_some()
    }

    /// Jump to `x` and stop any animation (e.g. to follow a dragging pointer).
    pub fn set(&mut self, x: f64) {
        if !x.is_finite() {
            return;
        }
        self.stop();
        if *self.value.peek() != x {
            self.value.set(x);
        }
    }

    /// Stop where it is.
    pub fn stop(&mut self) {
        if self.animation.write().take().is_some() {
            self.animating.set(false);
        }
    }

    /// Spring to `target` with a preset, keeping the current speed.
    pub fn spring_to(&mut self, target: f64, preset: SpringPreset) {
        self.spring_with(target, preset.into(), self.velocity());
    }

    /// Spring to `target` with any spring, starting at `velocity` units per second.
    pub fn spring_with(&mut self, target: f64, config: SpringConfig, velocity: f64) {
        let velocity = if velocity.is_finite() { velocity } else { 0.0 };
        let from = self.peek();
        self.animate(Animation::Spring(SpringState::new(
            config, from, target, velocity,
        )));
    }

    /// Coast from the current value at `velocity` (a throw), within `config`'s bounds and snap
    /// points.
    pub fn glide(&mut self, velocity: f64, config: GlideConfig) {
        let velocity = if velocity.is_finite() { velocity } else { 0.0 };
        self.animate(Animation::Glide(Glide::new(config, self.peek(), velocity)));
    }

    /// Run any [`Animation`]. Under reduced motion, jumps to its end instead.
    pub fn animate(&mut self, animation: Animation) {
        let end = animation.target();
        if !end.is_finite() {
            return;
        }
        if *self.reduced.peek() || animation.is_done() {
            self.set(end);
            return;
        }
        *self.animation.write() = Some(animation);
        if !*self.animating.peek() {
            self.animating.set(true);
        }
        if !*self.looping.read() {
            *self.looping.write() = true;
            let this = *self;
            Runtime::current().spawn(self.scope, this.run());
        }
    }

    /// One task per value: advances the animation each frame until it settles.
    async fn run(self) {
        let MotionValue {
            mut value,
            mut animating,
            mut animation,
            mut looping,
            ..
        } = self;
        let mut last = now_ms();
        loop {
            let timer = MotionTime::delay(Duration::from_millis(16)).await;
            let now = now_ms();
            let dt = ((now - last) / 1000.0).clamp(0.0, MAX_FRAME_S);
            last = now;
            // The owning component may have unmounted while we waited: then just stop.
            let Ok(mut slot) = animation.try_write() else {
                return;
            };
            let step = match slot.as_mut() {
                None => None,
                Some(a) if timer.is_err() => {
                    // No frame timer here: finish at once.
                    let end = a.target();
                    *slot = None;
                    Some((end, true))
                }
                Some(a) => {
                    a.advance(dt);
                    if a.is_done() {
                        let end = a.target();
                        *slot = None;
                        Some((end, true))
                    } else {
                        Some((a.position(), false))
                    }
                }
            };
            drop(slot);
            let Some((x, done)) = step else { break };
            if value.try_write().map(|mut v| *v = x).is_err() {
                return;
            }
            if done {
                let _ = animating.try_write().map(|mut a| *a = false);
                break;
            }
        }
        let _ = looping.try_write().map(|mut l| *l = false);
    }
}
