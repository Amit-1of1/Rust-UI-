### Security

- **ci:** Workflows pin every action to a commit, run with a read-only token (except the sync bot and Pages, which need to write), and add a `cargo deny` job (advisories, licences, git sources) driven by `deny.toml`. (#64)

### Changed

- **ci:** A newer push or sync-bot dispatch cancels the older CI and browser-check runs on the same PR branch (main always finishes), and the forked `dx` is cached between runs instead of being rebuilt for ten minutes each time. (#64)

### Fixed

- **gallery:** On Android the gallery's activity now handles dark-mode, font-size, density, locale and keyboard changes itself instead of being destroyed and recreated (dx's default manifest only covers rotation), because the app cannot yet survive a recreated activity. Changes Android always forces a recreate for (such as a new system overlay right after boot) still crash it; see `apps/showcase/README.md`. (#64)
