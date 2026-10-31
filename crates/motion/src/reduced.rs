use dioxus::core::{provide_root_context, ScopeId};
use dioxus::prelude::*;

/// The app's one copy of the setting, shared through the root context.
#[derive(Clone, Copy)]
struct ReducedMotion(Signal<bool>);

/// Whether the user asked the OS to reduce motion (`prefers-reduced-motion: reduce`).
///
/// Reactive: a component that reads it re-renders when the setting changes while the app is
/// open. The media query is read and watched once per app, not once per component, so a
/// long animated list doesn't query it per row. Off-web (desktop, tests) this is `false`.
pub fn use_reduced_motion() -> bool {
    use_reduced_motion_signal()()
}

/// The setting as a signal, for code that checks it later (in an event handler) without
/// subscribing the component.
pub(crate) fn use_reduced_motion_signal() -> Signal<bool> {
    use_hook(|| {
        try_consume_context::<ReducedMotion>()
            .unwrap_or_else(|| provide_root_context(ReducedMotion(watch())))
            .0
    })
}

fn watch() -> Signal<bool> {
    let signal = Signal::new_in_scope(prefers_reduced_motion(), ScopeId::ROOT);
    #[cfg(target_arch = "wasm32")]
    web::listen(signal);
    signal
}

#[cfg(target_arch = "wasm32")]
fn prefers_reduced_motion() -> bool {
    web::query().is_some_and(|q| q.matches())
}

#[cfg(not(target_arch = "wasm32"))]
fn prefers_reduced_motion() -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
mod web {
    use dioxus::prelude::*;
    use web_sys::wasm_bindgen::{closure::Closure, JsCast, JsValue};
    use web_sys::MediaQueryList;

    pub fn query() -> Option<MediaQueryList> {
        web_sys::window()?
            .match_media("(prefers-reduced-motion: reduce)")
            .ok()
            .flatten()
    }

    /// Follow the OS setting for as long as the page lives (one listener per app).
    pub fn listen(mut signal: Signal<bool>) {
        let Some(mql) = query() else { return };
        let source = mql.clone();
        let on_change = Closure::<dyn FnMut(JsValue)>::new(move |_| {
            let now = source.matches();
            if *signal.peek() != now {
                signal.set(now);
            }
        });
        if mql
            .add_event_listener_with_callback("change", on_change.as_ref().unchecked_ref())
            .is_ok()
        {
            on_change.forget();
        }
    }
}
