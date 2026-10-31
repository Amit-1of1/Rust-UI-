### Security

- **ci:** Workflows pin every action to a commit, run with a read-only token (except the sync bot and Pages, which need to write), and add a `cargo deny` job (advisories, licences, git sources) driven by `deny.toml`. (#64)

### Changed

- **ci:** A newer push or sync-bot dispatch cancels the older CI and browser-check runs on the same PR branch (main always finishes), and the forked `dx` is cached between runs instead of being rebuilt for ten minutes each time. (#64)

### Fixed

- **gallery:** On Android the gallery no longer aborts when the system changes dark mode, font size, density, locale or keyboard while it runs: its own `AndroidManifest.xml` lets the activity handle every configuration change instead of being recreated (dx's default only covers rotation). This was the intermittent crash in the Android emulator check. (#64)
