//! Hooks that reach outside the component tree.

use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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

/// Which rows of a list are selected, with updates that touch only the rows that change.
///
/// A plain `Signal<HashSet<K>>` re-renders every row that reads it whenever the selection
/// changes, so selecting one row of 1,000 diffs all 1,000. Here each row owns a `bool`
/// signal (from [`use_selected`]) and `Selection` flips only the rows that actually
/// change: selecting one row costs two row renders, whatever the list size. This is
/// SolidJS's `createSelector` idea on Dioxus signals.
///
/// ```rust,ignore
/// let sel = use_selection::<usize>();
/// // in each row component:
/// let selected = use_selected(sel, id);
/// rsx! { tr { class: if selected() { "selected" }, onclick: move |_| sel.select_only(id) } }
/// ```
pub struct Selection<K: 'static> {
    keys: CopyValue<HashSet<K>>,
    rows: CopyValue<HashMap<K, Signal<bool>>>,
    count: Signal<usize>,
}

impl<K: 'static> Clone for Selection<K> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<K: 'static> Copy for Selection<K> {}
impl<K: 'static> PartialEq for Selection<K> {
    fn eq(&self, other: &Self) -> bool {
        self.keys == other.keys
    }
}

/// Creates an empty [`Selection`] owned by the calling component.
pub fn use_selection<K: Hash + Eq + Clone + 'static>() -> Selection<K> {
    use_hook(|| Selection {
        keys: CopyValue::new(HashSet::new()),
        rows: CopyValue::new(HashMap::new()),
        count: Signal::new(0),
    })
}

/// Whether the row `key` is selected. Re-renders the calling row only when its own state
/// changes. The key must not change for the life of the row (key the row by it).
pub fn use_selected<K: Hash + Eq + Clone + 'static>(
    selection: Selection<K>,
    key: K,
) -> ReadSignal<bool> {
    let flag = use_signal(|| selection.keys.peek().contains(&key));
    let k = key.clone();
    use_hook(move || selection.rows.write_unchecked().insert(k, flag));
    use_drop(move || {
        let mut rows = selection.rows.write_unchecked();
        // Only remove our own entry: a newer row with the same key may have replaced it.
        if rows.get(&key) == Some(&flag) {
            rows.remove(&key);
        }
    });
    flag.into()
}

impl<K: Hash + Eq + Clone + 'static> Selection<K> {
    fn flip(&self, key: &K, on: bool) {
        if let Some(mut flag) = self.rows.peek().get(key).copied() {
            if *flag.peek() != on {
                flag.set(on);
            }
        }
    }

    /// Selects or deselects one row.
    pub fn set(&self, key: K, on: bool) {
        let changed = if on {
            self.keys.write_unchecked().insert(key.clone())
        } else {
            self.keys.write_unchecked().remove(&key)
        };
        if changed {
            self.flip(&key, on);
            let mut count = self.count;
            count.set(self.keys.peek().len());
        }
    }

    /// Flips one row.
    pub fn toggle(&self, key: K) {
        let on = !self.keys.peek().contains(&key);
        self.set(key, on);
    }

    /// Selects `key` and deselects everything else.
    pub fn select_only(&self, key: K) {
        let others: Vec<K> = self
            .keys
            .peek()
            .iter()
            .filter(|k| **k != key)
            .cloned()
            .collect();
        for k in others {
            self.set(k, false);
        }
        self.set(key, true);
    }

    /// Deselects everything.
    pub fn clear(&self) {
        let all: Vec<K> = self.keys.peek().iter().cloned().collect();
        for k in all {
            self.set(k, false);
        }
    }

    /// Whether `key` is selected, without subscribing to anything.
    pub fn contains(&self, key: &K) -> bool {
        self.keys.peek().contains(key)
    }

    /// The selected keys, without subscribing to anything.
    pub fn keys(&self) -> Vec<K> {
        self.keys.peek().iter().cloned().collect()
    }

    /// How many rows are selected. Reading it subscribes to the count only.
    pub fn len(&self) -> usize {
        (self.count)()
    }

    /// Whether nothing is selected. Reading it subscribes to the count only.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
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
