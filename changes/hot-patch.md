### Fixed

- `dx serve --hot-patch` works with the kit. Rust edits show without a rebuild or reload: 0.87 s in an `atoll create` starter (15.6 s with plain `dx serve`), 4.2 s in the gallery's app crate (about 15 s). It needs `dx` from `scripts/install-dx.sh`, now pinned to a fork commit that fixes five hot-patch bugs, among them the wasm-bindgen 0.2.129 parse failure that stopped every app from starting (docs/notes/hot-patch.md, docs/FORKS.md). (#80)

### Added

- `bench/hot-patch.mjs` measures save-to-screen time for Rust edits under `dx serve` with or without `--hot-patch`, on the gallery or any app (`FILE=... TEXT=...`). (#80)
