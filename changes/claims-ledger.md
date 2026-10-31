### Added

- **ui:** keyboard contracts for `Sheet`, `Drawer`, `Tooltip` and `ToggleGroup`; every row runs in the browser and on native Blitz, like the other 24.
- **docs:** `docs/CLAIMS.md`, a ledger of every public claim (README, docs home page, pricing, launch copy) with its evidence and the command that reproduces it; `crates/cli/tests/claims.rs` fails when a counted claim drifts from the repository.

### Fixed

- **docs:** the home page counted 83 components (it included the hooks, icons and string tables); it now says 80 and shows how many keyboard contracts there are. Claims that every component has a keyboard contract now say what is true. README's native section said Blitz had never been run, and the benchmark numbers are re-measured.
