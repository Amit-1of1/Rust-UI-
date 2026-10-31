//! `use_unique_id`: element ids that match between server rendering and hydration.

use dioxus::core::provide_root_context;
use dioxus::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// The app's id counter. It lives in the root context, so every `VirtualDom` (each server
/// request, the browser after hydration, each test) counts from 1 on its own.
#[derive(Clone)]
struct IdCounter(Rc<Cell<u64>>);

/// A page-unique id such as `dx-field-3`, made once per component instance.
///
/// Ids are numbered per app, in render order, so a server-rendered page and the browser's
/// first render give each component the same id. A process-wide counter (what the kit used
/// before) keeps counting across server requests, so the server's `id`, `for` and
/// `aria-describedby` stop matching the ones the browser computes, and label clicks,
/// error announcements and focus-on-error break after hydration. With the `fullstack`
/// feature the server's id is also sent to the client, as `dioxus-primitives` does.
pub fn use_unique_id(prefix: &'static str) -> String {
    #[allow(unused_mut)]
    let mut id = use_hook(|| next_unique_id(prefix));
    fullstack! {
        let sent = id.clone();
        id = use_server_cached(move || sent.clone());
    }
    id
}

/// The next id from the same per-app counter as [`use_unique_id`], for code that is not a
/// hook: fields added from an event handler (`FormState::add_field`). Each call gives a new
/// id, so don't call it in a component body; use [`use_unique_id`] there.
pub fn next_unique_id(prefix: &str) -> String {
    let counter = try_consume_context::<IdCounter>()
        .unwrap_or_else(|| provide_root_context(IdCounter(Rc::new(Cell::new(0)))));
    let n = counter.0.get() + 1;
    counter.0.set(n);
    format!("{prefix}-{n}")
}
