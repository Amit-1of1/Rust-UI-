# Working in this repo

- Read `docs/GUARDRAILS.md` first and follow it; it lists what must not be done (no compiler forks, no custom renderer, pinned git deps, license checks, keep primitive markup intact, reduced motion, gallery coverage).
- Before pushing run `scripts/check.sh` (add `--a11y` for UI changes); it runs: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo build -p gallery --target wasm32-unknown-unknown`.
- Every copied file gets a row in `THIRD_PARTY.md`.
- New components: start with `cargo run -p rust-ui-cli -- new <name>`, which creates all of the following. They are added to `crates/ui/src/components/`, the prelude in `crates/ui/src/lib.rs`, a `<name>_demo.rs` (rendered in `apps/gallery/src/main.rs` and smoke-tested in `crates/ui/tests/demos.rs`), and a render test in `crates/ui/tests/render.rs`.
