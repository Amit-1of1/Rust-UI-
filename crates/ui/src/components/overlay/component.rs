//! Modal layer for renderers without a real `position: fixed`.
//!
//! Blitz (dioxus-native) lays `position: fixed` out like `absolute`, and Taffy places an
//! absolute box relative to its *parent*, so a modal's backdrop only covers the element it
//! happens to sit in. With the `native` feature, [`ModalPortal`] moves its children into
//! [`OverlayLayer`], a window-sized box the app renders once at its root (the kit's
//! `ThemeStyles` does), so `position: fixed; inset: 0` means the window again. It also
//! paints last: Blitz only orders `z-index` among siblings.
//!
//! Without the feature both are no-ops: the markup stays where it is, and the web and the
//! desktop webview keep their real `position: fixed`.
//!
//! On native, portalled children are mounted under the layer, so they see the contexts
//! provided above the layer and inside the modal, not the ones between the layer and the
//! modal. The kit's own modals only use their own context.

#[cfg(not(feature = "native"))]
use dioxus::prelude::*;

#[cfg(feature = "native")]
mod imp {
    use dioxus::core::provide_root_context;
    use dioxus::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone, Copy)]
    struct Layer {
        items: Signal<Vec<(usize, Element)>>,
    }

    fn layer() -> Layer {
        try_consume_context::<Layer>().unwrap_or_else(|| {
            provide_root_context(Layer {
                items: Signal::new_in_scope(Vec::new(), ScopeId::ROOT),
            })
        })
    }

    #[component]
    pub fn ModalPortal(children: Element) -> Element {
        let mut layer = use_hook(layer);
        let id = use_hook(|| {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            NEXT.fetch_add(1, Ordering::Relaxed)
        });
        {
            let mut items = layer.items.write();
            match items.iter_mut().find(|(i, _)| *i == id) {
                Some(slot) => slot.1 = children,
                None => items.push((id, children)),
            }
        }
        use_drop(move || layer.items.write().retain(|(i, _)| *i != id));
        rsx! {}
    }

    /// Blitz hit-tests every element by its box (there is no `pointer-events`), so the layer
    /// itself is 0x0 and only its children, sized to the window, take clicks. They exist only
    /// while a modal is open: a closed kit modal renders nothing.
    #[component]
    pub fn OverlayLayer() -> Element {
        let layer = use_hook(layer);
        let items = layer.items.read().clone();
        rsx! {
            // The user-agent `body` margin would offset the layer from the window.
            style {
                "body {{ margin: 0; }} .dx-overlay-layer > * {{ top: 0; left: 0; width: 100vw; height: 100vh; }}"
            }
            div {
                class: "dx-overlay-layer",
                style: "position: absolute; top: 0; left: 0; width: 0; height: 0; z-index: 1000;",
                for (id , el) in items {
                    Fragment { key: "{id}", {el} }
                }
            }
        }
    }
}

#[cfg(feature = "native")]
pub use imp::{ModalPortal, OverlayLayer};

/// Renders `children` in place (web, desktop) or in the root [`OverlayLayer`] (native).
#[cfg(not(feature = "native"))]
#[component]
pub fn ModalPortal(children: Element) -> Element {
    children
}

/// The root layer modals are moved into on native; renders nothing elsewhere.
#[cfg(not(feature = "native"))]
#[component]
pub fn OverlayLayer() -> Element {
    rsx! {}
}
