//! Put focus back on the control that opened an overlay when the overlay closes.
//!
//! The dialog and popover primitives do this on the web with a script (they remember
//! `document.activeElement`). Native has no script, and Dioxus has no "which element has
//! focus" API, so focus stayed on the removed panel there. This does the same in Rust,
//! through element handles, so it works on every renderer:
//!
//! - an opener (the kit's `Button`, `PopoverTrigger`, the pickers' triggers) calls
//!   [`use_focus_opener`] and spreads the attributes it returns: while the control has focus
//!   it is the page's current opener;
//! - an overlay calls [`use_return_focus`], spreads its attributes on the panel and renders
//!   [`ReturnFocus`] inside the panel. When the panel opens it takes the current opener; when
//!   it closes with focus still inside it (Escape, a Cancel button), focus goes back there.
//!   A close that already moved focus elsewhere (a click on another control) is left alone.

use dioxus::core::{provide_root_context, spawn_forever};
use dioxus::prelude::*;
use std::rc::Rc;

/// The control that has focus now, if it is an opener.
#[derive(Clone, Copy)]
struct Opener(Signal<Option<Rc<MountedData>>>);

fn opener() -> Opener {
    try_consume_context::<Opener>().unwrap_or_else(|| {
        provide_root_context(Opener(Signal::new_in_scope(None, ScopeId::ROOT)))
    })
}

/// Attributes that make an element an opener for [`ReturnFocus`]. Spread them on a
/// focusable element (`button { ..use_focus_opener(), }`), or merge them into a primitive's
/// `attributes`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_focus_opener() -> Vec<Attribute> {
    let mut el = use_signal(|| None::<Rc<MountedData>>);
    let Opener(mut current) = use_hook(opener);
    // Dioxus 0.8 builds event attributes with the `Vec<Attribute>` builder methods.
    Vec::<Attribute>::new()
        .onmounted(move |e: MountedEvent| el.set(Some(e.data())))
        .onfocus(move |_: FocusEvent| {
            if let Some(el) = el.peek().clone() {
                current.set(Some(el));
            }
        })
        .onblur(move |_: FocusEvent| {
            let mine = match (&*current.peek(), &*el.peek()) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                _ => false,
            };
            if mine {
                current.set(None);
            }
        })
}

/// Whether focus is inside an overlay's panel; see [`use_return_focus`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, PartialEq)]
pub struct FocusInside(Signal<bool>);

/// For an overlay that stays mounted while its panel opens and closes: the state to pass
/// to [`ReturnFocus`], and the attributes to spread on the panel (they track whether focus
/// is inside it).
///
/// Dioxus runs one listener per event name on an element, so only spread these on a panel
/// whose primitive has no `onfocusin`/`onfocusout` of its own (the popover content). The
/// dialog primitives trap focus with exactly those two listeners; a modal dialog passes
/// `inside: None` to [`ReturnFocus`] instead, since its focus is always inside while open.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_return_focus() -> (FocusInside, Vec<Attribute>) {
    let mut inside = use_signal(|| false);
    let attrs = Vec::<Attribute>::new()
        .onfocusin(move |_: FocusEvent| inside.set(true))
        .onfocusout(move |_: FocusEvent| inside.set(false));
    (FocusInside(inside), attrs)
}

/// Render inside an overlay's panel (it renders nothing): takes the opener when the panel
/// opens, and returns focus to it when the panel closes with focus inside. `inside: None`
/// means "always": for a modal panel, which keeps focus inside while it is open.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ReturnFocus(inside: Option<FocusInside>) -> Element {
    let to = use_hook(|| opener().0.peek().clone());
    use_drop(move || {
        let had_focus = match inside {
            Some(FocusInside(mut inside)) => {
                let had = *inside.peek();
                inside.set(false);
                had
            }
            None => true,
        };
        if let (true, Some(el)) = (had_focus, to.clone()) {
            // This scope is going away; the task must outlive it.
            spawn_forever(async move {
                _ = el.set_focus(true).await;
            });
        }
    });
    rsx! {}
}
