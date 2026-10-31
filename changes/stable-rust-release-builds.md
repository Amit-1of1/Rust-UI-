### Fixed

- Release web builds work on current stable Rust (1.98+, LLVM 23). `strip = true` in the release profile made LLVM drop the `__wasm_bindgen_unstable` section, so `dx build --release`, `scripts/release-web.sh` and the GitHub Pages deploy failed. The workspace and apps made by `atoll create` now use `strip = "debuginfo"`; `atoll postbuild` still removes the wasm names. If your own app has `strip = true` under `[profile.release]`, change it to `strip = "debuginfo"`.
