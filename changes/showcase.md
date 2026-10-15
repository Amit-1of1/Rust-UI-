### Added

- **apps:** `apps/showcase`, one codebase for the web, the desktop and phones: the same pages on the web (WebAssembly) and on native (Blitz, no webview), with a side rail on wide windows and a bottom tab bar at phone width. Headless native tests at desktop and phone size (`cargo test -p native --test showcase`) and browser checks (`tests/e2e/showcase.mjs`); per-target status in `apps/showcase/README.md`.
