//! `use_copy_clipboard`: copy text and show "Copied" for a moment.

use dioxus::prelude::*;

/// How long `copied` stays true after a successful copy.
pub const COPIED_FOR_MS: i32 = 2000;

/// Copies text to the clipboard. Returns the copy callback and a flag that is true for
/// [`COPIED_FOR_MS`] after the browser confirms the copy, for a "Copied" label:
///
/// ```rust,ignore
/// let (copy, copied) = use_copy_clipboard();
/// rsx! { Button { onclick: move |_| copy.call("cargo add rust-ui".into()),
///     if copied() { "Copied" } else { "Copy" } } }
/// ```
///
/// Announce the result for screen readers too (a toast, or the button's own text as above).
///
/// Web only: the async Clipboard API through web-sys, which needs a secure context (https or
/// localhost). On desktop and native it does nothing and `copied` stays false, so keep the
/// text itself selectable.
pub fn use_copy_clipboard() -> (Callback<String>, ReadSignal<bool>) {
    let copied = use_signal(|| false);
    // Each copy gets a number; a timer only clears the flag it set, not a newer copy's.
    let generation = use_hook(|| CopyValue::new(0u32));
    let copy = use_callback(move |text: String| {
        let mut generation = generation;
        generation += 1;
        #[cfg(all(target_arch = "wasm32", feature = "web"))]
        web::copy(&text, copied, generation);
        #[cfg(not(all(target_arch = "wasm32", feature = "web")))]
        let _ = (text, copied);
    });
    (copy, copied.into())
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod web {
    use dioxus::prelude::*;
    use wasm_bindgen::{closure::Closure, JsCast};
    use wasm_bindgen_futures::JsFuture;

    pub fn copy(text: &str, mut copied: Signal<bool>, generation: CopyValue<u32>) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let mine = *generation.peek();
        let promise = window.navigator().clipboard().write_text(text);
        spawn(async move {
            let ok = JsFuture::from(promise).await.is_ok();
            copied.set(ok);
            if !ok {
                return;
            }
            // Frees itself after it runs.
            let reset = Closure::once_into_js(move || {
                if *generation.peek() == mine {
                    copied.set(false);
                }
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                reset.unchecked_ref(),
                super::COPIED_FOR_MS,
            );
        });
    }
}
