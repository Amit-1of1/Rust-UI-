#!/usr/bin/env bash
# Install `dx` 0.7.10 from the Amit-1of1/dioxus fork: upstream 0.7.10 plus two fixes to the wasm
# splitter (docs/FORKS.md). Stock dx 0.7.10 pulls some lazy components back into the main
# wasm file and panics on others, so split release builds (scripts/release-web.sh) need this.
# Day-to-day `dx serve` and debug builds work with either.
set -euo pipefail
# Amit-1of1/dioxus, branch claude/project-thread-xwtlc7
REV=73a027cf834ec97d50e670a27227debccaba6d1e
cargo install --git https://github.com/Amit-1of1/dioxus --rev "$REV" dioxus-cli --locked "$@"
