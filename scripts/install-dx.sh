#!/usr/bin/env bash
# Install `dx` 0.7.10 from the Amit-1of1/dioxus fork: upstream 0.7.10 plus fixes for the wasm
# splitter and for hot-patching (docs/FORKS.md, docs/notes/hot-patch.md).
# - Split release builds (scripts/release-web.sh) need it: stock dx 0.7.10 pulls some lazy
#   components back into the main wasm file and panics on others.
# - `dx serve --hot-patch` needs it: stock dx 0.7.10 can't read the wasm that wasm-bindgen
#   0.2.129 writes ("failed to parse import section: exceptions proposal not enabled").
# Plain `dx serve` and debug builds work with either.
set -euo pipefail
# Amit-1of1/dioxus, branch claude/project-thread-hgbr1g (on top of claude/project-thread-xwtlc7)
REV=5ab24501d5a3a6ebf66a04cf099ce5cbbf5b96a9
# --force: replace any other `dx` already on PATH (a stock one, or one a CI cache restored
# into ~/.cargo/bin), which otherwise stops cargo with "binary `dx` already exists".
cargo install --git https://github.com/Amit-1of1/dioxus --rev "$REV" dioxus-cli --locked --force "$@"
