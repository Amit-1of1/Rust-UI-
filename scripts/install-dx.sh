#!/usr/bin/env bash
# Dioxus 0.8 branch: install stock `dx` 0.8.0-alpha.3, matching the `dioxus` pin. The 0.7 fork's
# fixes (docs/FORKS.md) are not ported to 0.8 yet, and stock 0.8's splitter panics on our apps
# ("assertion failed: !self.dead.contains(&id)"), so scripts/release-web.sh does not split here.
set -euo pipefail
cargo install dioxus-cli --version 0.8.0-alpha.3 --locked "$@"
