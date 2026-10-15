### Added

- **cli:** `rust-ui update [names] [--dry-run]` updates copied components and blocks to the registry's current version with a three-way merge, keeping your edits and writing `<<<<<<< yours` conflict markers where both sides changed the same lines; `--dry-run` prints a unified diff and writes nothing. New global `--lock` (default `rust-ui-lock.json`). Experimental. (#44)

### Changed

- **cli:** `rust-ui add` now records what it copied in `rust-ui-lock.json` (source, commit, file hashes) and keeps untouched copies in `.rust-ui/base/`, for `update` to merge against. Commit both. (#44)
