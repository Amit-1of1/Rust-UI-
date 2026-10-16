### Added

- **repo:** `docs/PROTOCOL.md`, the protocol parallel sessions follow; the "Sync open PRs with main" workflow, which merges main into every open PR branch after each merge (with `scripts/sync-main.sh`), starts CI on it and comments on PRs that still conflict; `scripts/pr-conflicts.sh` lists what each open branch conflicts with.

### Changed

- **repo:** The workspace lists its members as `crates/*` and `apps/*`. `scripts/sync-main.sh` first merges the branch's own remote copy, and keeps both sides' added lines in a conflicted `Cargo.toml` when cargo still reads it. CI can be started by hand (`workflow_dispatch`).
