### Added

- **tooling:** `/run-rust-ui` Claude Code skill (`.claude/skills/run-rust-ui/`): `driver.mjs` serves a web app's `dx build` output and drives it in headless Chromium with steps (`goto:`, `click:`, `fill:`, `theme:`, `aria:`, `ss:`), plus the fresh-container setup that lets `dx` build offline (`NO_DOWNLOADS=1`).
