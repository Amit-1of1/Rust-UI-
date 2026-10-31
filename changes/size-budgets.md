### Added

- **e2e:** Size and startup budgets for the six release web apps (`bench/budgets.json`, `tests/e2e/budgets.mjs`, run by `scripts/e2e.sh --release` in the Chromium job): a PR fails when an app's first screen, main wasm or total download (brotli) grows more than 10%, its first screen needs more requests, or it takes more than twice as long to start with the CPU slowed 4x. Failures list the files that grew. Notes: docs/notes/size-budgets.md. (#77)
