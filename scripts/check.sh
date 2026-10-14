#!/usr/bin/env bash
# One-command gate. Everything CLAUDE.md asks for before pushing, plus the registry check.
#   scripts/check.sh           fmt, clippy, tests, wasm builds, registry, doctor
#   scripts/check.sh --a11y    also build gallery + site, serve them and run axe (light + dark)
#   scripts/check.sh --release also make split release builds of gallery + dashboard
#                              (scripts/release-web.sh, needs dx from scripts/install-dx.sh)
#                              and run their browser checks against those
# The a11y step needs `dx` and `npm ci` in tests/e2e (and Chromium via PLAYWRIGHT_BROWSERS_PATH).
set -euo pipefail
cd "$(dirname "$0")/.."

A11Y=0
RELEASE=0
for arg in "$@"; do
  case "$arg" in
    --a11y) A11Y=1 ;;
    --release) RELEASE=1 ;;
    -h|--help) sed -n '2,8p' "$0"; exit 0 ;;
    *) echo "unknown argument: $arg" >&2; exit 2 ;;
  esac
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
  PIDS=()
  cleanup() { for p in "${PIDS[@]:-}"; do [ -n "$p" ] && kill "$p" 2>/dev/null || true; done; }
  trap cleanup EXIT

  [ -d tests/e2e/node_modules ] || (cd tests/e2e && npm ci)

  # name | app dir | build output (relative to repo root) | port | element that means "rendered"
  for spec in "gallery|apps/gallery|target/dx/gallery/debug/web/public|8765|.gallery-main > .gallery-section:last-child" \
              "site|apps/site|target/dx/site/debug/web/public|8767|#pricing" \
              "docs|apps/docs|target/dx/docs/debug/web/public|8769|.docs-main"; do
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
    if [ "$name" = docs ]; then
      for route in "#button" "#data_table" "#theming"; do
        step "axe docs ($route)"
        (cd tests/e2e && GALLERY_URL="http://localhost:$port/$route" READY_SELECTOR="$ready" node axe.mjs light)
      done
      step "docs navigation behaviour"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node docs.mjs)
    fi
    if [ "$name" = site ]; then
      step "form behaviour (site)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node form.mjs)
      step "Ctrl+K hotkey (site)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" READY_SELECTOR="$ready" node hotkey.mjs)
    fi
    if [ "$name" = gallery ]; then
      step "otp + carousel behaviour (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node otp.mjs)
      step "multi-select + dropzone behaviour (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node pickers.mjs)
      step "drawer behaviour (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node drawer.mjs)
      step "table behaviour (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node table.mjs)
      step "select + combobox form fields (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node form_fields.mjs)
      step "axe $name (rtl)"
      (cd tests/e2e && DIR=rtl GALLERY_URL="http://localhost:$port/" READY_SELECTOR="$ready" node axe.mjs light)
      step "anchor positioning (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node anchor.mjs)
      step "keyboard per APG pattern (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node keyboard.mjs)
      step "density presets (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node density.mjs)
      step "forced colors (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node forced_colors.mjs)
      step "light-dark theme, chips, link (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node theme.mjs)
      step "Ctrl+K hotkey (gallery)"
      (cd tests/e2e && GALLERY_URL="http://localhost:$port/" node hotkey.mjs)
      for accent in blue green violet rose orange; do
        for scheme in light dark; do
          step "axe $name ($accent, $scheme)"
          (cd tests/e2e && ACCENT="$accent" GALLERY_URL="http://localhost:$port/" READY_SELECTOR="$ready" node axe.mjs "$scheme")
        done
      done
    fi
  done

  # apps/dashboard: a real app on the kit. Its script runs axe (light + dark) on every page and
  # overlay, plus the add / edit / delete / filter / Ctrl+K flows.
  step "build dashboard"
  (cd apps/dashboard && dx build --platform web)
  python3 -m http.server 8771 --directory target/dx/dashboard/debug/web/public >/dev/null 2>&1 &
  PIDS+=("$!")
  sleep 1
  step "dashboard: axe on every page + flows"
  (cd tests/e2e && GALLERY_URL="http://localhost:8771/" node dashboard.mjs)

  # apps/booking: the third real app (routed, long forms). Its script runs axe (light + dark) on
  # every route and overlay, plus search, filters, booking, the checkout steps and cancelling.
  step "build booking"
  (cd apps/booking && dx build --platform web)
  python3 -m http.server 8773 --directory target/dx/booking/debug/web/public >/dev/null 2>&1 &
  PIDS+=("$!")
  sleep 1
  step "booking: axe on every route + flows"
  (cd tests/e2e && GALLERY_URL="http://localhost:8773/" node booking.mjs)
fi

if [ "$RELEASE" = 1 ]; then
  RPIDS=()
  rcleanup() { for p in "${RPIDS[@]:-}"; do [ -n "$p" ] && kill "$p" 2>/dev/null || true; done; }
  trap rcleanup EXIT
  [ -d tests/e2e/node_modules ] || (cd tests/e2e && npm ci)
  # The lazy parts load after the first screen; every check waits for what it uses.
  step "split release build: dashboard"; scripts/release-web.sh dashboard
  python3 -m http.server 8772 --directory target/dx/dashboard/release/web/public >/dev/null 2>&1 &
  RPIDS+=("$!")
  step "split release build: gallery"; scripts/release-web.sh gallery
  python3 -m http.server 8766 --directory target/dx/gallery/release/web/public >/dev/null 2>&1 &
  RPIDS+=("$!")
  sleep 1
  step "dashboard (split): axe on every page + flows"
  (cd tests/e2e && GALLERY_URL="http://localhost:8772/" node dashboard.mjs)
  for scheme in light dark; do
    step "axe gallery (split, $scheme)"
    (cd tests/e2e && GALLERY_URL="http://localhost:8766/" node axe.mjs "$scheme")
  done
  for t in table keyboard pickers drawer otp hotkey form_fields; do
    step "$t behaviour (gallery, split)"
    (cd tests/e2e && GALLERY_URL="http://localhost:8766/" node "$t.mjs")
  done
fi

printf '\nAll checks passed.\n'
