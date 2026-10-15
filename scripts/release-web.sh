#!/usr/bin/env bash
# Release web build of one app, made as small as the tooling allows, then served from
#   target/dx/<app>/release/web/public
#   scripts/release-web.sh gallery [extra dx build args, e.g. --base-path Rust-UI-/gallery]
# 1. `dx build --release`. Apps with a `wasm-split` feature (gallery, dashboard, trial) are also
#    split, so their `#[component(lazy)]` parts load later. This needs dx from
#    scripts/install-dx.sh (stock 0.7.10's splitter drops or crashes on lazy parts). The linker
#    runs at -O0 for those: at -O2 wasm-ld tail-merges identical constants, and the splitter
#    then zeroes bytes the main module still reads (rsx node paths), which scrambles the first
#    render.
# 2. `rust-ui postbuild`: drop wasm debug names, bundle the CSS into one file.
# Needs `dx` and wasm-opt on PATH (offline: NO_DOWNLOADS=1, see docs/GUARDRAILS.md).
set -euo pipefail
cd "$(dirname "$0")/.."

app="${1:?usage: scripts/release-web.sh <app> [dx build args]}"
shift
dir="apps/$app"
[ -f "$dir/Cargo.toml" ] || { echo "no app at $dir" >&2; exit 2; }
out="target/dx/$app/release/web/public"

split=()
if grep -q '^wasm-split *=' "$dir/Cargo.toml"; then
  split=(--wasm-split --features wasm-split "--rustc-args=-C link-arg=-O0")
fi

# dx adds to the output folder and never clears it; start clean so postbuild sees one build.
rm -rf "$out"
(cd "$dir" && dx build --release --platform web "${split[@]}" "$@")

cargo run -q -p rust-ui-cli -- postbuild "$out"

# Stock dx 0.7.10 leaves some lazy parts as near-empty files (their code stays in the main
# file). Fail loudly instead of shipping a split that saves nothing.
if [ ${#split[@]} -gt 0 ]; then
  for f in "$out"/assets/module_*.wasm; do
    if [ "$(wc -c <"$f")" -lt 1024 ]; then
      echo "error: $(basename "$f") is almost empty: its code stayed in the main wasm." >&2
      echo "       Build with dx from scripts/install-dx.sh (docs/FORKS.md)." >&2
      exit 1
    fi
  done
fi
