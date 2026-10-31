#!/usr/bin/env bash
# One-command gate. Everything CLAUDE.md asks for before pushing, plus the registry check.
#   scripts/check.sh           fmt, clippy, tests, wasm builds, registry, doctor
#   scripts/check.sh --a11y    also build the web apps and run the browser checks (scripts/e2e.sh)
#   scripts/check.sh --a11y --browsers chromium,firefox,webkit   the same on each engine
#   scripts/check.sh --release also run the browser checks on release builds, split where the
#                              app has a `wasm-split` feature (scripts/release-web.sh, needs dx
#                              from scripts/install-dx.sh); this is what CI and Pages ship
# The browser steps need `dx`, `npm ci` in tests/e2e and the engines under PLAYWRIGHT_BROWSERS_PATH.
# On a work branch it first fails if the branch is behind origin/main (run scripts/sync-main.sh)
# or edits CHANGELOG.md / docs/MIGRATION.md directly (use changes/). SKIP_SYNC_CHECK=1 skips the
# first, FOLDING_CHANGELOG=1 the second (only for the PR that runs `rust-ui changelog`).
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

# Parallel sessions (docs/GUARDRAILS.md): stay on top of main, and keep shared files untouched.
branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo HEAD)
if [ "$branch" != main ] && [ "$branch" != HEAD ] && [ -z "${CI:-}" ]; then
  if [ "${SKIP_SYNC_CHECK:-0}" != 1 ]; then
    step "up to date with main"
    if git fetch -q origin main 2>/dev/null; then
      if ! git merge-base --is-ancestor origin/main HEAD; then
        echo "$branch is behind origin/main. Run scripts/sync-main.sh, then check again." >&2
        exit 1
      fi
    else
      echo "could not fetch origin/main; not checked"
    fi
  fi
  if [ "${FOLDING_CHANGELOG:-0}" != 1 ] && git rev-parse -q --verify origin/main >/dev/null; then
    step "changelog entries are fragments"
    if ! git diff --quiet origin/main...HEAD -- CHANGELOG.md docs/MIGRATION.md; then
      echo "This branch edits CHANGELOG.md or docs/MIGRATION.md. Put the entries in" >&2
      echo "changes/<topic>.md (and <topic>.migration.md) instead; see changes/README.md." >&2
      exit 1
    fi
  fi
fi

step "fmt";     cargo fmt --all --check
step "clippy";  cargo clippy --workspace --all-targets -- -D warnings
step "tests";   cargo test --workspace --exclude native
# On its own: in a workspace build the gallery's `web` feature reaches the motion crate,
# which then calls into JavaScript off wasm and panics in the native drawer test.
step "native (headless Blitz)"; cargo test -p native
step "wasm32";  cargo build -p gallery -p site -p docs -p dashboard -p booking -p showcase --target wasm32-unknown-unknown
step "registry, component.json, llms.txt"; cargo run -q -p rust-ui-cli -- registry --check
step "changelog fragments"; cargo run -q -p rust-ui-cli -- changelog --check
step "doctor (errors fail, warnings are listed)"; cargo run -q -p rust-ui-cli -- doctor | tail -3
if command -v cargo-deny >/dev/null; then
  step "dependencies: advisories, licences, sources (deny.toml)"; cargo deny check -s
else
  step "dependencies: skipped (cargo install cargo-deny --locked; CI runs it)"
fi

if [ "$A11Y" = 1 ]; then
  scripts/e2e.sh --browsers "$BROWSERS"
fi

if [ "$RELEASE" = 1 ]; then
  step "browser checks on release builds"
  scripts/e2e.sh --release --browsers "$BROWSERS"
fi

printf '\nAll checks passed.\n'
