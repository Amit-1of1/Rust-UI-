use dioxus::prelude::*;

/// Whether the user asked the OS to reduce motion (`prefers-reduced-motion: reduce`).
///
/// Read once when the component mounts. Off-web (desktop, tests) this is `false`.
pub fn use_reduced_motion() -> bool {
    use_hook(prefers_reduced_motion)
}

#[cfg(target_arch = "wasm32")]
fn prefers_reduced_motion() -> bool {
    web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .map(|query| query.matches())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
fn prefers_reduced_motion() -> bool {
    false
}
