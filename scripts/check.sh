#!/usr/bin/env bash
# One-command gate. Everything CLAUDE.md asks for before pushing, plus the registry check.
#   scripts/check.sh           fmt, clippy, tests, wasm builds, registry, doctor
#   scripts/check.sh --a11y    also build the web apps and run the browser checks (scripts/e2e.sh)
#   scripts/check.sh --a11y --browsers chromium,firefox,webkit   the same on each engine
#   scripts/check.sh --release also run the browser checks on release builds, split where the
#                              app has a `wasm-split` feature (scripts/release-web.sh, needs dx
#                              from scripts/install-dx.sh); this is what CI and Pages ship
# The browser steps need `dx`, `npm ci` in tests/e2e and the engines under PLAYWRIGHT_BROWSERS_PATH.
set -euo pipefail
cd "$(dirname "$0")/.."

A11Y=0
RELEASE=0
BROWSERS="${BROWSERS:-chromium}"
while [ $# -gt 0 ]; do
  case "$1" in
    --a11y) A11Y=1 ;;
    --release) RELEASE=1 ;;
    --browsers) BROWSERS="$2"; shift ;;
    --browsers=*) BROWSERS="${1#*=}" ;;
    -h|--help) sed -n '2,9p' "$0"; exit 0 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
  shift
done

step() { printf '\n== %s\n' "$*"; }

step "fmt";     cargo fmt --all --check
step "clippy";  cargo clippy --workspace --all-targets -- -D warnings
step "tests";   cargo test --workspace --exclude native
# On its own: in a workspace build the gallery's `web` feature reaches the motion crate,
# which then calls into JavaScript off wasm and panics in the native drawer test.
step "native (headless Blitz)"; cargo test -p native
step "wasm32";  cargo build -p gallery -p site -p docs -p dashboard -p booking --target wasm32-unknown-unknown
step "registry"; cargo run -q -p rust-ui-cli -- registry --check
step "doctor (errors fail, warnings are listed)"; cargo run -q -p rust-ui-cli -- doctor | tail -3

if [ "$A11Y" = 1 ]; then
  scripts/e2e.sh --browsers "$BROWSERS"
fi

if [ "$RELEASE" = 1 ]; then
  step "browser checks on release builds"
  scripts/e2e.sh --release --browsers "$BROWSERS"
fi

printf '\nAll checks passed.\n'
