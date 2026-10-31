//! Resizable panels: a row or column of panels with draggable dividers (the WAI-ARIA
//! "window splitter" pattern). Original to this repo; the API follows shadcn/ui's
//! `Resizable` (react-resizable-panels), which was not copied.

use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::rc::Rc;

#[css_module("/src/components/resizable/style.css")]
struct Styles;

/// How far one arrow-key press moves a divider, in percent of the group.
pub const RESIZE_STEP: f32 = 5.0;

/// Moves the divider between two neighbouring panels by `delta` percent, keeping both
/// inside their `(min, max)` limits and their total unchanged. Returns the new sizes.
///
/// ```
/// use ui::components::resizable::resize_pair;
/// assert_eq!(resize_pair((30.0, 70.0), 10.0, (10.0, 80.0), (10.0, 90.0)), (40.0, 60.0));
/// assert_eq!(resize_pair((30.0, 70.0), 90.0, (10.0, 80.0), (10.0, 90.0)), (80.0, 20.0));
/// ```
pub fn resize_pair(sizes: (f32, f32), delta: f32, a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    let total = sizes.0 + sizes.1;
    // `a` may not go below its min, above its max, or leave `b` outside its own range.
    let low = a.0.max(total - b.1);
    let high = a.1.min(total - b.0);
    if low > high {
        return sizes;
    }
    let first = (sizes.0 + delta).clamp(low, high);
    (first, total - first)
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ResizableDirection {
    /// Panels side by side, dividers dragged left and right.
    #[default]
    Horizontal,
    /// Panels stacked, dividers dragged up and down.
    Vertical,
}

#[derive(Clone, Copy, PartialEq)]
struct Drag {
    handle: usize,
    start: f64,
    sizes: (f32, f32),
    length: f64,
}

#[derive(Clone, Copy)]
struct Group {
    direction: ResizableDirection,
    /// Size of each panel in percent, in render order.
    sizes: Signal<Vec<f32>>,
    limits: Signal<Vec<(f32, f32)>>,
    /// Panel ids, empty when the panel has none.
    ids: Signal<Vec<String>>,
    handles: Signal<usize>,
    drag: Signal<Option<Drag>>,
    element: Signal<Option<Rc<MountedData>>>,
    on_layout: Callback<Vec<f32>>,
}

impl Group {
    fn move_divider(mut self, handle: usize, delta: f32) {
        let (a, b) = (handle, handle + 1);
        let sizes = self.sizes.peek().clone();
        let limits = self.limits.peek().clone();
        if b >= sizes.len() || b >= limits.len() {
            return;
        }
        let (x, y) = resize_pair((sizes[a], sizes[b]), delta, limits[a], limits[b]);
        if (x, y) != (sizes[a], sizes[b]) {
            {
                let mut write = self.sizes.write();
                write[a] = x;
                write[b] = y;
            }
            self.on_layout.call(self.sizes.peek().clone());
        }
    }

    fn set_first(self, handle: usize, size: f32) {
        let current = self.sizes.peek().get(handle).copied().unwrap_or(size);
        self.move_divider(handle, size - current);
    }
}

/// A row (or column) of [`ResizablePanel`]s with a [`ResizableHandle`] between each pair.
///
/// ```rust,ignore
/// ResizablePanelGroup {
///     ResizablePanel { default_size: 25.0, min_size: 15.0, "Folders" }
///     ResizableHandle { aria_label: "Resize folders" }
///     ResizablePanel { default_size: 75.0, "Messages" }
/// }
/// ```
///
/// Sizes are percentages of the group and should add up to 100. Give the group a size (a
/// height for horizontal groups, as with any flex row).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ResizablePanelGroup(
    #[props(default)] direction: ResizableDirection,
    /// Called with every panel's size (percent) after a divider moves.
    #[props(default)]
    on_layout: Callback<Vec<f32>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let group = use_context_provider(|| Group {
        direction,
        sizes: Signal::new(Vec::new()),
        limits: Signal::new(Vec::new()),
        ids: Signal::new(Vec::new()),
        handles: Signal::new(0),
        drag: Signal::new(None),
        element: Signal::new(None),
        on_layout,
    });
    let mut drag = group.drag;
    let mut element = group.element;
    let vertical = direction == ResizableDirection::Vertical;
    let base = attributes!(div {
        class: Styles::dx_resizable_group,
        "data-direction": if vertical { "vertical" } else { "horizontal" },
        "data-dragging": drag().is_some().then_some("true"),
    });
    rsx! {
        div {
            onmounted: move |e| element.set(Some(e.data())),
            onpointermove: move |e| {
                let Some(d) = *drag.peek() else { return };
                let point = e.client_coordinates();
                let now = if vertical { point.y } else { point.x };
                if d.length > 0.0 {
                    let delta = ((now - d.start) / d.length * 100.0) as f32;
                    let current = group.sizes.peek().get(d.handle).copied().unwrap_or(d.sizes.0);
                    group.move_divider(d.handle, d.sizes.0 + delta - current);
                }
            },
            onpointerup: move |_| drag.set(None),
            onpointercancel: move |_| drag.set(None),
            onpointerleave: move |_| drag.set(None),
            ..merge_attributes(vec![base, attributes]),
            {children}
        }
    }
}

/// One panel of a [`ResizablePanelGroup`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ResizablePanel(
    /// Starting size in percent of the group.
    #[props(default = 50.0)]
    default_size: f32,
    #[props(default = 0.0)] min_size: f32,
    #[props(default = 100.0)] max_size: f32,
    /// The panel's element id. The handle after it points at it with `aria-controls`, as
    /// the window splitter pattern asks; without an id there is no `aria-controls`.
    #[props(into)]
    id: Option<String>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let group = use_context::<Group>();
    let index = use_hook({
        let id = id.clone().unwrap_or_default();
        move || {
            let (mut sizes, mut limits, mut ids) = (group.sizes, group.limits, group.ids);
            let index = sizes.peek().len();
            sizes.write().push(default_size.clamp(min_size, max_size));
            limits.write().push((min_size, max_size));
            ids.write().push(id);
            index
        }
    });
    let size = group.sizes.read().get(index).copied().unwrap_or(default_size);
    let base = attributes!(div {
        class: Styles::dx_resizable_panel,
        id: id.clone(),
        style: format!("flex: {size} 1 0px;"),
    });
    rsx! {
        div { ..merge_attributes(vec![base, attributes]), {children} }
    }
}

/// The divider between two panels: drag it, or focus it and use the arrow keys (Home and End
/// jump to the limits). It is a focusable `role="separator"` whose value is the size of the
/// panel before it, as the WAI-ARIA window splitter pattern describes.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ResizableHandle(
    /// Read by screen readers, e.g. "Resize sidebar". Defaults to "Resize".
    #[props(into, default = "Resize".to_string())]
    aria_label: String,
    /// Show a grip in the middle of the divider.
    #[props(default)]
    with_handle: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let group = use_context::<Group>();
    let index = use_hook(move || {
        let mut handles = group.handles;
        let index = *handles.peek();
        handles.set(index + 1);
        index
    });
    let vertical = group.direction == ResizableDirection::Vertical;
    let sizes = group.sizes.read();
    let limits = group.limits.read();
    let size = sizes.get(index).copied().unwrap_or(50.0);
    // The range the divider can really move in: both neighbours' limits apply.
    let (min, max) = match (limits.get(index), limits.get(index + 1), sizes.get(index + 1)) {
        (Some(a), Some(b), Some(next)) => {
            let total = size + next;
            (a.0.max(total - b.1), a.1.min(total - b.0))
        }
        (Some(a), _, _) => *a,
        _ => (0.0, 100.0),
    };
    drop((sizes, limits));
    let controls = group.ids.read().get(index).filter(|id| !id.is_empty()).cloned();
    let mut drag = group.drag;
    let dragging = drag().is_some_and(|d| d.handle == index);
    let base = attributes!(div {
        class: Styles::dx_resizable_handle,
        role: "separator",
        tabindex: "0",
        // A divider between side-by-side panels is a vertical line.
        "aria-orientation": if vertical { "horizontal" } else { "vertical" },
        "aria-valuenow": format!("{}", size.round()),
        "aria-valuemin": format!("{}", min.round()),
        "aria-valuemax": format!("{}", max.round()),
        "aria-label": aria_label,
        "data-dragging": dragging.then_some("true"),
    });
    rsx! {
        div {
            "aria-controls": controls,
            onpointerdown: move |e: PointerEvent| {
                e.prevent_default();
                let point = e.client_coordinates();
                let start = if vertical { point.y } else { point.x };
                let element = group.element.peek().clone();
                let sizes = group.sizes.peek().clone();
                spawn(async move {
                    let Some(element) = element else { return };
                    let Ok(rect) = element.get_client_rect().await else { return };
                    let length = if vertical { rect.size.height } else { rect.size.width };
                    let pair = (
                        sizes.get(index).copied().unwrap_or(50.0),
                        sizes.get(index + 1).copied().unwrap_or(50.0),
                    );
                    drag.set(Some(Drag { handle: index, start, sizes: pair, length }));
                });
            },
            onkeydown: move |e: KeyboardEvent| {
                let step = if e.modifiers().shift() { RESIZE_STEP * 2.0 } else { RESIZE_STEP };
                let (less, more) = if vertical {
                    (Key::ArrowUp, Key::ArrowDown)
                } else {
                    (Key::ArrowLeft, Key::ArrowRight)
                };
                let key = e.key();
                if key == less {
                    group.move_divider(index, -step);
                } else if key == more {
                    group.move_divider(index, step);
                } else if key == Key::Home {
                    group.set_first(index, min);
                } else if key == Key::End {
                    group.set_first(index, max);
                } else {
                    return;
                }
                e.prevent_default();
            },
            ..merge_attributes(vec![base, attributes]),
            if with_handle {
                span { class: Styles::dx_resizable_grip, "aria-hidden": "true" }
            }
        }
    }
}
