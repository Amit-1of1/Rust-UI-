### Security

- **ci:** Workflows pin every action to a commit, run with a read-only token (except the sync bot and Pages, which need to write), and add a `cargo deny` job (advisories, licences, git sources) driven by `deny.toml`. (#PR)

### Changed

- **ci:** A newer push or sync-bot dispatch cancels the older CI and browser-check runs on the same PR branch (main always finishes), and the forked `dx` is cached between runs instead of being rebuilt for ten minutes each time. (#PR)
