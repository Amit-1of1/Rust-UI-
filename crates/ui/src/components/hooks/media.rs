//! `use_media_query`: follow a CSS media query from Rust.

use dioxus::prelude::*;

/// Phone-sized viewports, matching the kit's CSS breakpoint: `use_media_query(MOBILE_QUERY)`.
pub const MOBILE_QUERY: &str = "(max-width: 767px)";

/// Whether the CSS media `query` matches, updated when it changes (window resized, OS theme
/// switched, and so on): `let wide = use_media_query("(min-width: 1024px)");`.
///
/// Prefer a CSS `@media` rule when only styling changes; use this when the *markup* has to
/// differ, such as a sheet on phones and a popover on desktop.
///
/// Web only: a Rust (web-sys) listener on the `MediaQueryList`, removed when the component
/// unmounts. On desktop and native, and during server rendering, it is always `false`, so
/// make `false` the layout that works everywhere. In fullstack apps the browser also starts
/// from `false`, matching the server's HTML, and switches to the real value right after
/// hydration.
pub fn use_media_query(query: &'static str) -> ReadSignal<bool> {
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    {
        // A server-rendered page was rendered with `false`; hydrating with the browser's
        // value would build different markup than the HTML on the page. So in fullstack
        // apps start from `false` and read the real value once hydration is done.
        #[allow(unused_mut)]
        let mut hydrating = false;
        fullstack! { hydrating = true; }
        let mut matches = use_signal(|| !hydrating && web::matches(query));
        if hydrating {
            use_effect(move || {
                let now = web::matches(query);
                if *matches.peek() != now {
                    matches.set(now);
                }
            });
        }
        let listener = use_hook(move || web::listen(query, matches));
        use_drop(move || web::unlisten(&listener));
        matches.into()
    }
    #[cfg(not(all(target_arch = "wasm32", feature = "web")))]
    {
        let _ = query;
        use_signal(|| false).into()
    }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod web {
    use dioxus::prelude::*;
    use std::rc::Rc;
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};
    use web_sys::MediaQueryList;

    pub type Listener = Option<Rc<(MediaQueryList, Closure<dyn FnMut(JsValue)>)>>;

    fn list(query: &str) -> Option<MediaQueryList> {
        web_sys::window()?.match_media(query).ok().flatten()
    }

    pub fn matches(query: &str) -> bool {
        list(query).is_some_and(|l| l.matches())
    }

    pub fn listen(query: &str, mut matches: Signal<bool>) -> Listener {
        let mql = list(query)?;
        let source = mql.clone();
        let closure = Closure::<dyn FnMut(JsValue)>::new(move |_| {
            let now = source.matches();
            if *matches.peek() != now {
                matches.set(now);
            }
        });
        mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .ok()?;
        Some(Rc::new((mql, closure)))
    }

    pub fn unlisten(listener: &Listener) {
        if let Some(pair) = listener {
            let (mql, closure) = pair.as_ref();
            let _ =
                mql.remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
        }
    }
}
