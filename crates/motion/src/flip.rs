//! Layout animation by FLIP (First, Last, Invert, Play): when a list item ends up somewhere
//! else on the page, it starts at its old position and springs to the new one.
//!
//! Wrap the list in [`FlipList`] and each row in [`FlipItem`]. Rows re-measure whenever the list
//! re-renders and whenever another row is removed, so siblings slide into the gap.

use crate::{use_spring, Spring, SpringPreset};
use dioxus::core::use_after_render;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct FlipTick(Signal<u32>);

/// Bump the shared counter; ignored if its owner is already gone.
fn bump(mut t: Signal<u32>) {
    let _ = t.try_write().map(|mut w| *w = w.wrapping_add(1));
}

/// Groups [`FlipItem`]s so they re-measure together.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn FlipList(
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tick = use_context_provider(|| FlipTick(Signal::new(0)));
    // After the DOM for a new list is in place, ask every row to measure itself.
    use_after_render(move || {
        bump(tick.0);
    });
    rsx! {
        div { ..attributes, {children} }
    }
}

/// One row of a [`FlipList`]. Its position change is animated; its content is untouched.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn FlipItem(
    #[props(default)] preset: SpringPreset,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tick = try_use_context::<FlipTick>();
    let mut mounted = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let mut last_y = use_signal(|| None::<f64>);
    let mut offset = use_signal(|| 0.0_f32);
    let mut progress: Spring = use_spring(0.0, preset);

    // When this row goes away, nudge the survivors to re-measure.
    use_drop(move || {
        if let Some(FlipTick(t)) = tick {
            bump(t);
        }
    });

    use_effect(move || {
        if let Some(FlipTick(t)) = tick {
            let _ = t();
        }
        let Some(m) = mounted() else { return };
        spawn(async move {
            let Ok(rect) = m.get_client_rect().await else {
                return;
            };
            let y = rect.origin.y;
            if let Some(prev) = last_y.peek().as_ref().copied() {
                let moved = prev - y;
                if moved.abs() > 0.5 {
                    // Invert: start where we were (relative to where we are now), then play.
                    let current = *offset.peek() * (1.0 - progress.get_value());
                    offset.set(moved as f32 + current);
                    progress.reset();
                    progress.to(1.0);
                }
            }
            last_y.set(Some(y));
        });
    });

    let shown = offset() * (1.0 - progress.get_value());
    rsx! {
        // The outer element is measured; only the inner one is transformed.
        div {
            onmounted: move |e| mounted.set(Some(e.data())),
            ..attributes,
            div { style: "transform: translateY({shown}px);", {children} }
        }
    }
}
