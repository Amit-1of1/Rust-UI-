#!/usr/bin/env bash
# Release web build of one app, made as small as the tooling allows, then served from
#   target/dx/<app>/release/web/public
#   scripts/release-web.sh trial
# 1. `dx build --release`. Apps with a `wasm-split` feature (apps/trial) are also split, so their
#    `#[component(lazy)]` parts load on first use. The linker runs at -O0 for those: at -O2
#    wasm-ld tail-merges identical constants, and dx 0.7.10's splitter then zeroes bytes the
#    main module still reads (rsx node paths), which scrambles the first render.
# 2. `rust-ui postbuild`: drop wasm debug names, bundle the CSS into one file.
# Needs `dx` 0.7.10 and wasm-opt on PATH (offline: NO_DOWNLOADS=1, see docs/GUARDRAILS.md).
set -euo pipefail
cd "$(dirname "$0")/.."

app="${1:?usage: scripts/release-web.sh <app>}"
dir="apps/$app"
[ -f "$dir/Cargo.toml" ] || { echo "no app at $dir" >&2; exit 2; }
out="target/dx/$app/release/web/public"

split=()
if grep -q '^wasm-split *=' "$dir/Cargo.toml"; then
  split=(--wasm-split --features wasm-split "--rustc-args=-C link-arg=-O0")
fi

# dx adds to the output folder and never clears it; start clean so postbuild sees one build.
rm -rf "$out"
(cd "$dir" && dx build --release --platform web "${split[@]}")
cargo run -q -p rust-ui-cli -- postbuild "$out"
