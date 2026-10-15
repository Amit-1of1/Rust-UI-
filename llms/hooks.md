# Hooks

- Copy into your app: `rust-ui add hooks` or `dx components add hooks --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::hooks::*;` after copying, or `use ui::hooks::{...};` (by path, no glob) with the `rust-ui-kit` crate.

Hooks shared by the components and by apps.

The sidebar keeps its own `use_is_mobile` (from upstream, through `document::eval`, so it also works in the desktop webview); `use_media_query(MOBILE_QUERY)` is the web-sys equivalent. `rust-ui add hooks` copies them; a component that uses one (the command palette uses `use_hotkey`) brings this item along when you add it.

| Hook | What it does |
|---|---|
| `use_hotkey("k", f)` | Runs `f` on Ctrl+K (Cmd+K on macOS) anywhere on the page |
| `use_selection()` / `use_selected(sel, key)` | Row selection that re-renders only the rows that change |
| `use_media_query("(min-width: 1024px)")` | Follows a CSS media query |
| `MOBILE_QUERY` | `"(max-width: 767px)"`, the kit's phone breakpoint, for `use_media_query` |
| `use_copy_clipboard()` | Copies text; a flag stays true for two seconds after the copy succeeds |

The ones that listen to the browser (`use_hotkey`, `use_media_query`, `use_copy_clipboard`) are web only: they use Rust `web-sys` listeners, never JavaScript of ours, and remove them when the component unmounts. They are compiled in for `wasm32` builds with a `web` feature in your crate (the `dx new` template has one). On desktop and native they do nothing, so keep a visible way to do the same thing (a button for the shortcut, selectable text next to the copy button, a layout that works when the query is false).

## Accessibility

- Role and name: hooks render nothing. Announce what they change: give a copy button a visible "Copied" state, and show the shortcut next to the button it mirrors (`Kbd`).
- Keyboard: `use_hotkey` only fires on Ctrl or Cmd with no Alt or Shift, so it does not take keys that screen readers and text inputs use.
- Reduced motion: none.

## Source

`use_hotkey` and `Selection` are original to this repo. `use_media_query` and `use_copy_clipboard` follow Rust/UI's hooks (rust-ui/ui@43ddc65, MIT), rewritten to remove their listeners on unmount and to report a copy only after the browser confirms it.

## API

```rust
/// How long `copied` stays true after a successful copy.
pub const COPIED_FOR_MS: i32;

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
pub fn use_copy_clipboard() -> (Callback<String>, ReadSignal<bool>)

/// Is this key event Ctrl+`key` (or Cmd+`key` on macOS), with no Alt or Shift?
pub fn is_hotkey(key: &str, pressed: &str, ctrl: bool, meta: bool, alt: bool, shift: bool) -> bool

/// Runs `on_press` when Ctrl+`key` (Cmd+`key` on macOS) is pressed anywhere on the page,
/// for example `use_hotkey("k", move || open.set(true))` to open a command palette.
///
/// Web only: a Rust (web-sys) listener on `window`, no JavaScript (docs/GUARDRAILS.md).
/// On desktop and native it does nothing, so always keep a visible button that does the
/// same thing.
pub fn use_hotkey(key: &'static str, mut on_press: impl FnMut() + 'static)

/// Phone-sized viewports, matching the kit's CSS breakpoint: `use_media_query(MOBILE_QUERY)`.
pub const MOBILE_QUERY: &str;

/// Whether the CSS media `query` matches, updated when it changes (window resized, OS theme
/// switched, and so on): `let wide = use_media_query("(min-width: 1024px)");`.
///
/// Prefer a CSS `@media` rule when only styling changes; use this when the *markup* has to
/// differ, such as a sheet on phones and a popover on desktop.
///
/// Web only: a Rust (web-sys) listener on the `MediaQueryList`, removed when the component
/// unmounts. On desktop and native, and during server rendering, it is always `false`, so
/// make `false` the layout that works everywhere.
pub fn use_media_query(query: &'static str) -> ReadSignal<bool>

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

/// Creates an empty [`Selection`] owned by the calling component.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_selection<K: Hash + Eq + Clone + 'static>() -> Selection<K>

/// Whether the row `key` is selected. Re-renders the calling row only when its own state
/// changes. The key must not change for the life of the row (key the row by it).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_selected<K: Hash + Eq + Clone + 'static>(
    selection: Selection<K>,
    key: K,
) -> ReadSignal<bool>
```

## Example

```rust
use crate::components::button::{Button, ButtonVariant};
use crate::components::hooks::{use_copy_clipboard, use_media_query, MOBILE_QUERY};
use crate::components::kbd::{Kbd, KbdGroup};
use dioxus::prelude::*;

const INSTALL: &str = "rust-ui add hooks";

#[component]
pub fn Demo() -> Element {
    let mobile = use_media_query(MOBILE_QUERY);
    let dark = use_media_query("(prefers-color-scheme: dark)");
    let (copy, copied) = use_copy_clipboard();
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            p { style: "margin: 0;",
                "use_media_query(MOBILE_QUERY): "
                strong { if mobile() { "phone-sized" } else { "wider than a phone" } }
                ". use_media_query(prefers-color-scheme: dark): "
                strong { if dark() { "dark" } else { "light (or not on the web)" } }
                "."
            }
            div { style: "display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap;",
                code { "{INSTALL}" }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| copy.call(INSTALL.to_string()),
                    if copied() { "Copied" } else { "Copy" }
                }
            }
            p { style: "margin: 0;",
                "use_hotkey: "
                KbdGroup {
                    Kbd { "Ctrl" }
                    Kbd { "K" }
                }
                " opens the command palette above."
            }
        }
    }
}
```
