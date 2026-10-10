# Working in this repo

- Read `docs/GUARDRAILS.md` first and follow it; it lists what must not be done (no compiler forks, no custom renderer, pinned git deps, license checks, keep primitive markup intact, reduced motion, gallery coverage).
- Before pushing: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo build -p gallery --target wasm32-unknown-unknown`.
- Every copied file gets a row in `THIRD_PARTY.md`.
- New components are added to `crates/ui/src/components/`, the prelude in `crates/ui/src/lib.rs`, a demo in `apps/gallery/src/main.rs`, and a render test in `crates/ui/tests/render.rs`.
