//! Hooks that reach outside the component tree.

use dioxus::prelude::*;

/// Is this key event Ctrl+`key` (or Cmd+`key` on macOS), with no Alt or Shift?
pub fn is_hotkey(key: &str, pressed: &str, ctrl: bool, meta: bool, alt: bool, shift: bool) -> bool {
    (ctrl || meta) && !alt && !shift && pressed.eq_ignore_ascii_case(key)
}

/// Runs `on_press` when Ctrl+`key` (Cmd+`key` on macOS) is pressed anywhere on the page,
/// for example `use_hotkey("k", move || open.set(true))` to open a command palette.
///
/// Web only: a Rust (web-sys) listener on `window`, no JavaScript (docs/GUARDRAILS.md).
/// On desktop and native it does nothing, so always keep a visible button that does the
/// same thing.
pub fn use_hotkey(key: &'static str, mut on_press: impl FnMut() + 'static) {
    let callback = use_callback(move |()| on_press());
    let pressed = use_signal(|| 0u32);
    use_effect(move || {
        if pressed() > 0 {
            callback.call(());
        }
    });

    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    {
        let listener = use_hook(move || web::listen(key, pressed));
        use_drop(move || web::unlisten(&listener));
    }
    #[cfg(not(all(target_arch = "wasm32", feature = "web")))]
    let _ = (key, pressed);
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod web {
    use dioxus::prelude::*;
    use std::rc::Rc;
    use wasm_bindgen::{closure::Closure, JsCast};
    use web_sys::KeyboardEvent;

    pub type Listener = Rc<Closure<dyn FnMut(KeyboardEvent)>>;

    pub fn listen(key: &'static str, mut pressed: Signal<u32>) -> Listener {
        let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            if super::is_hotkey(
                key,
                &e.key(),
                e.ctrl_key(),
                e.meta_key(),
                e.alt_key(),
                e.shift_key(),
            ) {
                // Stop the browser's own Ctrl+K (focus the address bar).
                e.prevent_default();
                pressed += 1;
            }
        });
        if let Some(window) = web_sys::window() {
            let _ = window
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        }
        Rc::new(closure)
    }

    pub fn unlisten(listener: &Listener) {
        if let Some(window) = web_sys::window() {
            let _ = window.remove_event_listener_with_callback(
                "keydown",
                listener.as_ref().as_ref().unchecked_ref(),
            );
        }
    }
}

/// Moves focus from the focused element to `selector` inside its nearest ancestor with
/// class `container`, e.g. back to a menu's trigger when Escape closes the menu.
/// Web only; does nothing elsewhere.
pub(crate) fn refocus_within(container: &str, selector: &str) {
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    {
        use wasm_bindgen::JsCast;
        let target = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element())
            .and_then(|el| el.closest(&format!(".{container}")).ok().flatten())
            .and_then(|root| root.query_selector(selector).ok().flatten())
            .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());
        if let Some(el) = target {
            let _ = el.focus();
        }
    }
    #[cfg(not(all(target_arch = "wasm32", feature = "web")))]
    let _ = (container, selector);
}

#[cfg(test)]
mod tests {
    use super::is_hotkey;

    #[test]
    fn ctrl_or_cmd_and_the_key_only() {
        assert!(is_hotkey("k", "k", true, false, false, false));
        assert!(is_hotkey("k", "K", false, true, false, false));
        assert!(!is_hotkey("k", "k", false, false, false, false));
        assert!(!is_hotkey("k", "j", true, false, false, false));
        assert!(!is_hotkey("k", "k", true, false, true, false));
        assert!(!is_hotkey("k", "k", true, false, false, true));
    }
}
