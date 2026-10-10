#!/usr/bin/env bash
# One-command gate. Everything CLAUDE.md asks for before pushing, plus the registry check.
#   scripts/check.sh           fmt, clippy, tests, wasm builds, registry
#   scripts/check.sh --a11y    also build gallery + site, serve them and run axe (light + dark)
# The a11y step needs `dx` and `npm ci` in tests/e2e (and Chromium via PLAYWRIGHT_BROWSERS_PATH).
set -euo pipefail
cd "$(dirname "$0")/.."

A11Y=0
for arg in "$@"; do
  case "$arg" in
    --a11y) A11Y=1 ;;
    -h|--help) sed -n '2,5p' "$0"; exit 0 ;;
    *) echo "unknown argument: $arg" >&2; exit 2 ;;
  esac
done

step() { printf '\n== %s\n' "$*"; }

step "fmt";     cargo fmt --all --check
step "clippy";  cargo clippy --workspace --all-targets -- -D warnings
step "tests";   cargo test --workspace
step "wasm32";  cargo build -p gallery -p site --target wasm32-unknown-unknown
step "registry"; cargo run -q -p rust-ui-cli -- registry --check

if [ "$A11Y" = 1 ]; then
  PIDS=()
  cleanup() { for p in "${PIDS[@]:-}"; do [ -n "$p" ] && kill "$p" 2>/dev/null || true; done; }
  trap cleanup EXIT

  [ -d tests/e2e/node_modules ] || (cd tests/e2e && npm ci)

  # name | app dir | build output (relative to repo root) | port | element that means "rendered"
  for spec in "gallery|apps/gallery|target/dx/gallery/debug/web/public|8765|.gallery-main" \
              "site|apps/site|target/dx/site/debug/web/public|8767|#pricing"; do
    IFS='|' read -r name dir out port ready <<<"$spec"
    step "build $name"
    (cd "$dir" && dx build --platform web)
    python3 -m http.server "$port" --directory "$out" >/dev/null 2>&1 &
    PIDS+=("$!")
    sleep 1
    for scheme in light dark; do
      step "axe $name ($scheme)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" READY_SELECTOR="$ready" node axe.mjs "$scheme")
    done
    if [ "$name" = gallery ]; then
      for accent in blue green violet rose orange; do
        for scheme in light dark; do
          step "axe $name ($accent, $scheme)"
          (cd tests/e2e && ACCENT="$accent" GALLERY_URL="http://localhost:$port/" READY_SELECTOR="$ready" node axe.mjs "$scheme")
        done
      done
    fi
  done
fi

printf '\nAll checks passed.\n'
