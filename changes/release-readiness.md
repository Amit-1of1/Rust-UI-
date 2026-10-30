### Changed

- **ui:** The kit's primitives are now `rust-ui-primitives`, our DioxusLabs/components fork published under its own name so the kit can go on crates.io. The library is still `dioxus_primitives`, so code doesn't change. `component.json` files name `rust-ui-primitives`, so copied components and the crate use the same primitives. New internal feature `contract-check` for this repo's builds. (#62)

### Added

- `scripts/publish.sh` (dry run by default) publishes every crate in dependency order. `scripts/consumer-check.sh` builds an outside app from the packaged crates on stock Dioxus, and CI runs it on every PR. `scripts/semver.sh` (cargo-semver-checks) fails a PR that breaks the public API without a migration note. `scripts/fork-drift.sh` and a weekly workflow report upstream releases and whether the fork fixes still apply. (#62)
