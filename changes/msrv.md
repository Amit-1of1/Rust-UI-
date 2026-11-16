### Changed

- The minimum Rust version is now **1.89** (was declared 1.88, which never built: `dioxus-motion` and `safe_arch` need 1.89). A new `MSRV` CI job builds the published crates, on the host and for `wasm32-unknown-unknown`, with the exact version `rust-version` declares, so the two can't drift again.
