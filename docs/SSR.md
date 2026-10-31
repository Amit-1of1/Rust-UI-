# Server rendering and hydration

The kit works in Dioxus fullstack apps (`dioxus = { features = ["fullstack"] }`, server
functions, server-side rendering then hydration in the browser). This page lists what the kit
does for that and what the app still has to do. It was written after building a fullstack
trial app ("Ledgerly", 2026-10-31) from a fresh project with only the public docs.

## Set up

```toml
[dependencies]
dioxus = { version = "0.7.10", features = ["router", "fullstack"] }
ui = { git = "https://github.com/Amit-1of1/Rust-UI-", rev = "<commit>", package = "atoll-ui", features = ["router"] }

[features]
default = ["web"]
web = ["dioxus/web", "ui/web"]
server = ["dioxus/server"]
```

Build and run with `dx serve --fullstack` (or `dx build --fullstack`). Turn on `ui/web` only
for the browser build; the server build compiles the kit without it.

## What the kit does

- **Ids match between the server and the browser.** Labels (`for`), `aria-describedby`,
  `aria-controls` and focus-on-error all use generated ids. The kit numbers them per app in
  render order (`hooks::use_unique_id`), so each server request starts again from 1 and the
  browser's first render computes the same ids. With `fullstack` on, the server's ids are also
  sent to the browser, as `dioxus-primitives` does for its own (`dxc-<n>`).
  `crates/ui/tests/ssr.rs` renders every demo twice in one process and fails if an id
  changes. (Before 2026-10-31 these came from process-wide counters: the second request's
  HTML said `dx-field-7` while the browser computed `dx-field-1`.)
- **Media queries hydrate with the server's value.** The server cannot know the window size,
  the colour scheme or "reduce motion", so `use_media_query`, `use_theme().is_dark()` and
  `use_reduced_motion()` render `false` there. In a fullstack app the browser also starts
  from `false` (so its first render matches the HTML) and switches to the real value right
  after hydration. Make `false` the layout that works everywhere (desktop sidebar, motion on).
- **The theme can be remembered without a flash.** `PersistTheme {}` plus
  `document::Script { {THEME_INIT_SCRIPT} }` (theme_customizer docs): the script runs in the
  `<head>` before the first paint and applies the saved mode, accent, density and radius.

## What the app has to do

- **Don't send big tables in the page.** Everything a `use_server_future` returns is
  serialized into the HTML for hydration. The trial's customers page with 20,000 rows in a
  client-side `DataTable` was a 4.4 MB HTML response. `DataTable`'s server mode
  (`total_rows` + `on_query_change`, data_table docs) fetches one page at a time: the same
  page became 44 KB, and paging, sorting and filtering still worked after hydration.
- **Keep the browser and server crates in step.** A fresh project resolves the newest
  `wasm-bindgen`, and `dx` refuses to build when its `wasm-bindgen-cli` differs
  ("project requires version 0.2.130 but version 0.2.129 is installed"). Install the CLI
  version `cargo tree -i wasm-bindgen` names, or pin the crate:
  `cargo update -p wasm-bindgen --precise <version>`.
- **Set `lang` and `dir` on `<html>`** for translated and right-to-left pages; overlays
  (dialogs, sheets, menus) are rendered at the page root and read the direction from there.

## Checked

`crates/ui/tests/ssr.rs` (every demo and block). The Ledgerly trial was run with
`dx build --fullstack` and driven in headless Chromium: label clicks and focus-on-error work
after hydration, and a reload in dark mode paints dark before the app loads.
