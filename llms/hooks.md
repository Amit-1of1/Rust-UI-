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

/// Attributes that make an element an opener for [`ReturnFocus`]. Spread them on a
/// focusable element (`button { ..use_focus_opener(), }`), or merge them into a primitive's
/// `attributes`.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_focus_opener() -> Vec<Attribute>

/// Whether focus is inside an overlay's panel; see [`use_return_focus`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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
pub fn use_return_focus() -> (FocusInside, Vec<Attribute>)

/// Render inside an overlay's panel (it renders nothing): takes the opener when the panel
/// opens, and returns focus to it when the panel closes with focus inside. `inside: None`
/// means "always": for a modal panel, which keeps focus inside while it is open.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ReturnFocus(inside: Option<FocusInside>) -> Element

/// Is this key event Ctrl+`key` (or Cmd+`key` on macOS), with no Alt or Shift?
pub fn is_hotkey(key: &str, pressed: &str, ctrl: bool, meta: bool, alt: bool, shift: bool) -> bool

/// Runs `on_press` when Ctrl+`key` (Cmd+`key` on macOS) is pressed anywhere on the page,
/// for example `use_hotkey("k", move || open.set(true))` to open a command palette.
///
/// Web only: a Rust (web-sys) listener on `window`, no JavaScript (docs/GUARDRAILS.md).
/// On desktop and native it does nothing, so always keep a visible button that does the
/// same thing.
pub fn use_hotkey(key: &'static str, mut on_press: impl FnMut() + 'static)

/// A page-unique id such as `dx-field-3`, made once per component instance.
///
/// Ids are numbered per app, in render order, so a server-rendered page and the browser's
/// first render give each component the same id. A process-wide counter (what the kit used
/// before) keeps counting across server requests, so the server's `id`, `for` and
/// `aria-describedby` stop matching the ones the browser computes, and label clicks,
/// error announcements and focus-on-error break after hydration. With the `fullstack`
/// feature the server's id is also sent to the client, as `dioxus-primitives` does.
pub fn use_unique_id(prefix: &'static str) -> String

/// The next id from the same per-app counter as [`use_unique_id`], for code that is not a
/// hook: fields added from an event handler (`FormState::add_field`). Each call gives a new
/// id, so don't call it in a component body; use [`use_unique_id`] there.
pub fn next_unique_id(prefix: &str) -> String

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
/// make `false` the layout that works everywhere. In fullstack apps the browser also starts
/// from `false`, matching the server's HTML, and switches to the real value right after
/// hydration.
pub fn use_media_query(query: &'static str) -> ReadSignal<bool>

/// The `href` to render for `href`: inside a router with a base path (for example a site
/// served from `/my-app/`), an in-app path gets that prefix, so opening it in a new tab or
/// copying it works. Anything else is returned unchanged.
pub fn in_app_href(href: &str) -> String

/// Sends a plain left click on a link to `href` through the router, when there is one and
/// it knows the path. Returns `true` when it did (the browser's own navigation is then
/// cancelled). Clicks with a modifier key (new tab, new window), other buttons, links with
/// a `target`, and clicks whose default was already prevented are left alone.
pub fn follow_in_app(href: &str, event: &MouseEvent) -> bool

/// Whether `href` is the path the router is showing now, for `aria-current="page"`.
pub fn is_current_route(href: &str) -> bool

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
