### Added

- **test:** New crate `atoll-test`: render a screen headless (Blitz, the kit's real CSS) inside `cargo test`, find elements by role and name, label, text, placeholder or test id, click, type, press keys, move the app's clock (`advance`, `wait_for`) and assert on what a user sees. Guide in `docs/TESTING.md` (experimental). (#93)
- **cli:** `atoll setup-tests` adds `atoll-test` to an app's `[dev-dependencies]` from the same source as its kit, plus the kit's renderer fixes (`[patch.crates-io]`) (experimental). (#93)

### Fixed

- **motion:** dioxus-motion's browser timer is now only turned on for wasm builds, so an app's host `cargo test` no longer panics ("cannot access imported statics on non-wasm targets") in animated components (drawer, presence) when the app's `web` feature is on. The `web` feature of `atoll-motion` stays and does nothing extra. (#93)
