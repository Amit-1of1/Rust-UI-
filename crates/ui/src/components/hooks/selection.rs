//! `Selection`: row selection that re-renders only the rows that change.

use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_selection<K: Hash + Eq + Clone + 'static>() -> Selection<K> {
    use_hook(|| Selection {
        keys: CopyValue::new(HashSet::new()),
        rows: CopyValue::new(HashMap::new()),
        count: Signal::new(0),
    })
}

/// Whether the row `key` is selected. Re-renders the calling row only when its own state
/// changes. The key must not change for the life of the row (key the row by it).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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
