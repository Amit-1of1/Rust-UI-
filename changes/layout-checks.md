### Added

- **tests:** WCAG 2.2 layout checks over every demo on the headless native renderer, in `cargo test -p native --test layout`: target size (2.5.8), focus not obscured (2.4.11), reflow at 320px (1.4.10), text spacing (1.4.12), text 40% longer and right to left. Accepted findings and the Blitz differences behind most of them are in `docs/notes/layout-checks.md`. (#50)

### Fixed

- **ui:** `DataTable` no longer pushes the page wider than a phone screen inside a grid or flex parent; its table scrolls instead. (#50)
- **ui:** `Pagination` wraps instead of running past a narrow window. (#50)
- **ui:** `TimePicker` segments are at least 24x24px (WCAG 2.5.8); the picker keeps its height. (#50)
- **apps:** at phone size the showcase scrolls a focused element above its bottom tab bar instead of under it. (#50)
