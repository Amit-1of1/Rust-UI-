### Added

- **ui:** Built-in translations of the kit's own words: `UiStrings::locale("fr")` (and `french()`, `german()`, `spanish()`, `portuguese()`, `arabic()`, `hebrew()`, `japanese()`, `chinese()`), `LOCALES`, and `text_direction(tag)` for `dir` (experimental). Drafted by an AI model; `tests/locales.rs` checks the placeholders.
- **docs:** `docs/RTL.md`, right-to-left pages and translations.
- **e2e:** `tests/e2e/rtl.mjs` checks the sheet edges, the switch thumb and the carousel with `dir="rtl"`.

### Changed

- **ui:** `SidebarSide::Left`/`Right` and `SheetSide::Left`/`Right` are the start and end edges, so on right-to-left pages the sidebar and side sheets open from the right. In left-to-right pages nothing changes.

### Fixed

- **ui:** Right-to-left pages: the desktop sidebar stayed on the left while the page left room for it on the right, overlapping the content; the switch thumb moved off the track when checked; sheets, the sidebar rail, stepper connectors, progress and shimmer animations moved the wrong way; the carousel's Next and Previous were swapped. Icons with `mirror_rtl` only flipped when `lang` was also a right-to-left language (the CSS minifier turns `:dir(rtl)` into `:lang()` checks); the kit now also matches the `dir` attribute.
