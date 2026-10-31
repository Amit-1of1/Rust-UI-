### Added

- **cli:** `atoll motion [--css <file>] [--all]` (experimental): classes every animated property in the kit's stylesheets, or an app's, as composite, paint or layout, and fails on layout animations that no `atoll-motion: layout` comment explains. `crates/cli/tests/motion.rs` runs it on the kit in `cargo test`.
- **e2e:** `tests/e2e/frames.mjs` on Chromium, Firefox and WebKit: every component contract row's animations and transitions, checked for what they actually animate, for reduced motion (nothing over 1 ms), and on Chromium against the browser's own compositor verdict (70 of 70 composite-only animations on the compositor) with frame times at 4x CPU slowdown. Notes: docs/notes/motion-audit.md.

### Changed

- **ui:** The progress bar slides a full-width bar with `translateX` instead of animating its `width`, so value changes run on the compositor. Button and the sidebar rail transition named properties instead of `all`, and toggle group items `border-color` instead of `border`.
