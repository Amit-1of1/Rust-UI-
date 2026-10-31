### Added

- **ui:** `PersistTheme` saves `use_theme()`'s mode, accent, density and radius in `localStorage` and restores them on the next visit; `THEME_INIT_SCRIPT` applies them in the page head before the first paint (experimental).
- **ui:** `hooks::use_unique_id(prefix)`: page-unique element ids numbered per app, so server-rendered HTML and the browser agree.
- **ui:** `DataTable` server mode: `total_rows`, `on_query_change(TableQuery)` and `loading`, for tables paged, sorted and filtered on the server (experimental).
- **ui:** `FormState::add_field` / `remove_field`: fields added while the form is open, such as line items, validated and focused with the rest (experimental).
- **docs:** `docs/SSR.md`, server rendering and hydration in fullstack apps.

### Fixed

- **ui:** Fullstack apps: the ids of form fields, multi-selects, carousels, selects and tabs came from process-wide counters, so after the first request the server's `id`, `for` and `aria-describedby` no longer matched what the browser computed when hydrating; label clicks, error announcements and focus-on-error broke. They are now numbered per app (`tests/ssr.rs`).
- **ui, motion:** Fullstack apps: `use_media_query`, `use_theme().is_dark()` and `use_reduced_motion()` start from the server's value (`false`) when hydrating and switch to the browser's value right after, instead of building different markup than the server sent.
