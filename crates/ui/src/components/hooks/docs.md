# Hooks

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
