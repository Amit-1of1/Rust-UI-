//! Layout and shared-element animation: [`LayoutGroup`] and [`LayoutItem`].
//!
//! A [`LayoutItem`] measures itself after renders. When its box moved or changed size, it
//! starts drawn where it was (a translate and scale transform) and springs into its new box.
//! Give two items the same `layout_id` in one [`LayoutGroup`] and, when one disappears and the
//! other appears, the new one grows out of the old one's box: a tab underline slides to the
//! new tab, a thumbnail grows into a detail card.
//!
//! The item's own element is the one transformed, and its measurement is corrected for the
//! transform it has at that moment, so an interrupted animation carries on from where it is.
//! Size changes scale the element and its children; text inside a resizing item stretches
//! while it moves. Use it for boxes, highlights and images; use [`crate::FlipItem`] for rows
//! of text that only change position.

use crate::physics::SpringConfig;
use crate::{use_motion_value, MotionValue, SpringPreset};
use dioxus::core::use_after_render;
use dioxus::prelude::*;
use dioxus_motion::{MotionTime, TimeProvider};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

/// A box in page pixels (viewport position plus the page's scroll offset).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl LayoutRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Whether `other` is somewhere else, ignoring sub-pixel noise.
    pub fn differs_from(&self, other: &Self) -> bool {
        (self.x - other.x).abs() > 0.5
            || (self.y - other.y).abs() > 0.5
            || (self.width - other.width).abs() > 0.5
            || (self.height - other.height).abs() > 0.5
    }
}

/// The transform that draws a box laid out at `to` where `from` is, eased out by `progress`
/// (0 = drawn at `from`, 1 = at rest in `to`).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutDelta {
    pub dx: f64,
    pub dy: f64,
    pub sx: f64,
    pub sy: f64,
}

impl Default for LayoutDelta {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl LayoutDelta {
    pub const IDENTITY: Self = Self {
        dx: 0.0,
        dy: 0.0,
        sx: 1.0,
        sy: 1.0,
    };

    /// Draw `to` at `from` (with `transform-origin: 0 0`).
    pub fn between(from: LayoutRect, to: LayoutRect) -> Self {
        let ratio = |a: f64, b: f64| if b > 0.5 && a > 0.0 { a / b } else { 1.0 };
        Self {
            dx: from.x - to.x,
            dy: from.y - to.y,
            sx: ratio(from.width, to.width),
            sy: ratio(from.height, to.height),
        }
    }

    /// `(translate x, translate y, scale x, scale y)` at `progress`.
    pub fn at(&self, progress: f64) -> (f64, f64, f64, f64) {
        let rest = 1.0 - progress;
        (
            self.dx * rest,
            self.dy * rest,
            self.sx + (1.0 - self.sx) * progress,
            self.sy + (1.0 - self.sy) * progress,
        )
    }

    /// Where a box laid out at `layout` is drawn at `progress`.
    pub fn visual(&self, layout: LayoutRect, progress: f64) -> LayoutRect {
        let (tx, ty, sx, sy) = self.at(progress);
        LayoutRect::new(
            layout.x + tx,
            layout.y + ty,
            layout.width * sx,
            layout.height * sy,
        )
    }

    /// The layout box of an element measured (transform included) at `visual` at `progress`.
    pub fn layout(&self, visual: LayoutRect, progress: f64) -> LayoutRect {
        let (tx, ty, sx, sy) = self.at(progress);
        let div = |a: f64, s: f64| if s.abs() > 1e-6 { a / s } else { a };
        LayoutRect::new(
            visual.x - tx,
            visual.y - ty,
            div(visual.width, sx),
            div(visual.height, sy),
        )
    }

    /// The CSS transform at `progress`, or `None` when it is the identity.
    pub fn css(&self, progress: f64) -> Option<String> {
        let (tx, ty, sx, sy) = self.at(progress);
        let still = tx.abs() < 0.01
            && ty.abs() < 0.01
            && (sx - 1.0).abs() < 1e-4
            && (sy - 1.0).abs() < 1e-4;
        (!still).then(|| {
            format!("transform: translate({tx:.2}px, {ty:.2}px) scale({sx:.4}, {sy:.4}); transform-origin: 0 0;")
        })
    }
}

/// Re-measures one item without animating.
type Snapshot = Rc<dyn Fn()>;

#[derive(Clone, Copy)]
struct LayoutCtx {
    /// Bumped after the group renders: items measure and animate any move.
    tick: Signal<u32>,
    /// Bumped to re-measure without animating, where measuring can't be done on the spot.
    quiet: Signal<u32>,
    /// Last box (as drawn) of each `layout_id` whose item went away.
    last: CopyValue<HashMap<String, LayoutRect>>,
    items: CopyValue<HashMap<u64, Snapshot>>,
    next_key: CopyValue<u64>,
}

impl LayoutCtx {
    /// Record every item's box as it is now, so the next change animates from here.
    fn snapshot(&self) {
        let items: Vec<Snapshot> = self.items.read().values().cloned().collect();
        for item in items {
            item();
        }
    }
}

/// How far the page is scrolled. Boxes are kept in page coordinates so a box remembered before
/// the page scrolled still lines up afterwards (scrolling doesn't re-render anything).
fn page_scroll() -> (f64, f64) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .map(|w| (w.scroll_x().unwrap_or(0.0), w.scroll_y().unwrap_or(0.0)))
            .unwrap_or((0.0, 0.0))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        (0.0, 0.0)
    }
}

/// The element's box right now, transform included, where that can be read synchronously (the
/// web). Elsewhere `None`, and callers fall back to the asynchronous `get_client_rect`.
fn rect_now(m: &MountedData) -> Option<LayoutRect> {
    #[cfg(target_arch = "wasm32")]
    {
        let el = m.downcast::<web_sys::Element>()?;
        let r = el.get_bounding_client_rect();
        let (sx, sy) = page_scroll();
        Some(LayoutRect::new(
            r.x() + sx,
            r.y() + sy,
            r.width(),
            r.height(),
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = m;
        None
    }
}

fn bump(mut t: Signal<u32>) {
    let _ = t.try_write().map(|mut w| *w = w.wrapping_add(1));
}

/// Scope for [`LayoutItem`]s: they re-measure together after each render of the group, and
/// items with the same `layout_id` hand their box to each other.
///
/// A pointer or key press inside the group first records where every item is (it arrives
/// before the click or change it leads to), so put the controls that move the items inside
/// the group. It renders a `display: contents` wrapper.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn LayoutGroup(children: Element) -> Element {
    let ctx = use_context_provider(|| LayoutCtx {
        tick: Signal::new(0),
        quiet: Signal::new(0),
        last: CopyValue::new(HashMap::new()),
        items: CopyValue::new(HashMap::new()),
        next_key: CopyValue::new(0),
    });
    use_after_render(move || bump(ctx.tick));
    rsx! {
        div {
            style: "display: contents;",
            onpointerdown: move |_| ctx.snapshot(),
            onkeydown: move |_| ctx.snapshot(),
            {children}
        }
    }
}

/// One item's measurement state; `Copy`, so closures can share it.
#[derive(Clone, Copy)]
struct Item {
    group: Option<LayoutCtx>,
    id: CopyValue<Option<String>>,
    preset: CopyValue<SpringPreset>,
    mounted: Signal<Option<Rc<MountedData>>>,
    layout: Signal<Option<LayoutRect>>,
    delta: Signal<LayoutDelta>,
    progress: MotionValue,
}

impl Item {
    fn rect_now(&self) -> Option<LayoutRect> {
        self.mounted.peek().as_ref().and_then(|m| rect_now(m))
    }

    /// Take a measurement of the element as drawn. With `animate`, a move since the last
    /// measurement springs from the old box to the new one; without, it is just recorded.
    fn apply(mut self, drawn: LayoutRect, animate: bool) {
        // Unmounted while measuring.
        if self.layout.try_peek().is_err() {
            return;
        }
        let p = self.progress.peek();
        let d = *self.delta.peek();
        let now = d.layout(drawn, p);
        let previous = *self.layout.peek();
        let from = match previous {
            // Moved: start from where the old box is drawn right now.
            Some(prev) if animate && prev.differs_from(&now) => Some(d.visual(prev, p)),
            Some(_) => None,
            // First measurement: grow out of a vanished twin, if there is one.
            None => match (self.group, self.id.read().clone()) {
                (Some(mut g), Some(id)) => g.last.write().remove(&id),
                _ => None,
            },
        };
        self.layout.set(Some(now));
        if let Some(from) = from.filter(|f| f.differs_from(&now)) {
            self.delta.set(LayoutDelta::between(from, now));
            self.progress.set(0.0);
            let spring = SpringConfig::from(*self.preset.read()).with_rest(0.0005, 0.01);
            self.progress.spring_with(1.0, spring, 0.0);
        }
    }

    /// Measure now if possible, else as soon as the renderer answers.
    fn measure(self, animate: bool) {
        if let Some(r) = self.rect_now() {
            self.apply(r, animate);
            return;
        }
        let Some(m) = self.mounted.peek().clone() else {
            return;
        };
        spawn(async move {
            if let Ok(r) = m.get_client_rect().await {
                let (sx, sy) = page_scroll();
                let drawn = LayoutRect::new(
                    r.origin.x + sx,
                    r.origin.y + sy,
                    r.size.width,
                    r.size.height,
                );
                self.apply(drawn, animate);
            }
        });
    }
}

/// An element whose position and size changes animate (Motion's `layout` and `layoutId`).
/// Web and desktop webview only: it needs element measurement.
///
/// Under reduced motion it jumps to its new box.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn LayoutItem(
    /// Items with the same id in one [`LayoutGroup`] are the same thing in different places.
    #[props(default)]
    layout_id: Option<String>,
    #[props(default)] preset: SpringPreset,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let group = try_use_context::<LayoutCtx>();
    let mut item = Item {
        group,
        id: use_hook(|| CopyValue::new(layout_id.clone())),
        preset: use_hook(|| CopyValue::new(preset)),
        mounted: use_signal(|| None),
        layout: use_signal(|| None),
        delta: use_signal(|| LayoutDelta::IDENTITY),
        progress: use_motion_value(1.0),
    };
    if *item.id.peek() != layout_id {
        *item.id.write() = layout_id.clone();
    }
    if *item.preset.peek() != preset {
        *item.preset.write() = preset;
    }
    let own_tick = use_signal(|| 0_u32);
    // Re-measure after our own renders too, but not on every frame of our own animation.
    use_after_render(move || {
        if !item.progress.is_moving() {
            bump(own_tick);
        }
    });

    // Let the group snapshot us before an interaction; measure on the spot when possible.
    let key = use_hook(move || {
        let mut g = group?;
        let key = *g.next_key.peek();
        *g.next_key.write() += 1;
        let snapshot: Snapshot = Rc::new(move || match item.rect_now() {
            Some(r) => item.apply(r, false),
            None => bump(g.quiet),
        });
        g.items.write().insert(key, snapshot);
        Some(key)
    });

    // Leave our box behind for the next item with this id.
    use_drop(move || {
        let Some(mut g) = group else { return };
        if let Some(key) = key {
            let _ = g.items.try_write().map(|mut items| items.remove(&key));
        }
        let id = item.id.try_read().ok().and_then(|id| id.clone());
        let Some(layout) = item.layout.try_peek().ok().and_then(|l| *l) else {
            return;
        };
        if let Some(id) = id {
            let drawn = item
                .rect_now()
                .unwrap_or_else(|| item.delta.peek().visual(layout, item.progress.peek()));
            let _ = g.last.try_write().map(|mut last| last.insert(id, drawn));
        }
        bump(g.tick);
    });

    // Which signal woke the effect decides whether a move is animated.
    let mut seen = use_hook(|| CopyValue::new((u32::MAX, u32::MAX, u32::MAX)));
    use_effect(move || {
        let ticks = (
            own_tick(),
            group.map_or(0, |g| (g.tick)()),
            group.map_or(0, |g| (g.quiet)()),
        );
        if item.mounted.read().is_none() {
            return;
        }
        let before = *seen.peek();
        *seen.write() = ticks;
        let first = before == (u32::MAX, u32::MAX, u32::MAX);
        item.measure(!first && (ticks.0 != before.0 || ticks.1 != before.1));
        if first {
            // Stylesheets and fonts often arrive after the first render; record the settled
            // box quietly so the first real change doesn't animate from the unstyled one.
            spawn(async move {
                if MotionTime::delay(Duration::from_millis(500)).await.is_ok() {
                    item.measure(false);
                }
            });
        }
    });

    // An empty style attribute is not written to the DOM, so the last frame would stick.
    let style = (item.delta)()
        .css(item.progress.get())
        .unwrap_or_else(|| "transform: none;".to_string());
    let mut mounted = item.mounted;
    rsx! {
        div {
            style,
            onmounted: move |e| mounted.set(Some(e.data())),
            ..attributes,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_draws_the_new_box_over_the_old_one_then_lets_go() {
        let from = LayoutRect::new(10.0, 20.0, 50.0, 10.0);
        let to = LayoutRect::new(110.0, 20.0, 100.0, 20.0);
        let d = LayoutDelta::between(from, to);
        assert_eq!(d.visual(to, 0.0), from);
        assert_eq!(d.visual(to, 1.0), to);
        let half = d.visual(to, 0.5);
        assert_eq!(half.x, 60.0);
        assert_eq!(half.width, 75.0);
        assert_eq!(d.css(1.0), None);
        assert!(d
            .css(0.0)
            .is_some_and(|c| c.contains("scale(0.5000, 0.5000)")));
    }

    #[test]
    fn measuring_mid_animation_recovers_the_layout_box() {
        let from = LayoutRect::new(0.0, 0.0, 40.0, 40.0);
        let to = LayoutRect::new(200.0, 100.0, 80.0, 20.0);
        let d = LayoutDelta::between(from, to);
        for p in [0.0, 0.3, 0.9, 1.2] {
            let drawn = d.visual(to, p);
            let back = d.layout(drawn, p);
            assert!(!back.differs_from(&to), "{p}: {back:?}");
        }
    }

    #[test]
    fn empty_boxes_do_not_divide_by_zero() {
        let d = LayoutDelta::between(LayoutRect::default(), LayoutRect::new(5.0, 5.0, 0.0, 0.0));
        assert_eq!((d.sx, d.sy), (1.0, 1.0));
        assert!(d.layout(LayoutRect::default(), 0.0).x.is_finite());
    }

    #[test]
    fn sub_pixel_changes_do_not_count() {
        let a = LayoutRect::new(0.0, 0.0, 10.0, 10.0);
        assert!(!a.differs_from(&LayoutRect::new(0.3, 0.2, 10.4, 9.8)));
        assert!(a.differs_from(&LayoutRect::new(1.0, 0.0, 10.0, 10.0)));
    }
}
