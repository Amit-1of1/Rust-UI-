#!/usr/bin/env bash
# Browser checks on the built web apps: axe, keyboard, accessibility tree and behaviour scripts
# in tests/e2e. scripts/check.sh --a11y runs this; CI runs it once per engine.
#   scripts/e2e.sh                          build the apps with dx, then test on Chromium
#   scripts/e2e.sh --browsers chromium,firefox,webkit
#   scripts/e2e.sh --prebuilt               use the existing target/dx/*/debug/web/public
#   scripts/e2e.sh --release                test release builds (target/dx/*/release)
# Needs `npm ci` in tests/e2e and each engine under PLAYWRIGHT_BROWSERS_PATH (this script never
# downloads one; `npx playwright install <engine>` does, and CI does it in .github/workflows/e2e.yml).
set -euo pipefail
cd "$(dirname "$0")/.."

BROWSERS="${BROWSERS:-chromium}"
BUILD=1
PROFILE=debug
while [ $# -gt 0 ]; do
  case "$1" in
    --browsers) BROWSERS="$2"; shift ;;
    --browsers=*) BROWSERS="${1#*=}" ;;
    --prebuilt) BUILD=0 ;;
    --release) PROFILE=release ;;
    -h|--help) sed -n '2,9p' "$0"; exit 0 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
  shift
done

# Every check runs even after one fails, so one run shows every engine difference.
FAILED=()
CURRENT=""
step() { CURRENT="$*"; printf '\n== %s\n' "$*"; }
# A hung script (an engine that stops delivering events) fails its step after 10 minutes.
LIMIT=(); command -v timeout >/dev/null && LIMIT=(timeout 600)
e2e() { (cd tests/e2e && "${LIMIT[@]}" "$@") || FAILED+=("$CURRENT"); }

PIDS=()
cleanup() { for p in "${PIDS[@]:-}"; do [ -n "$p" ] && kill "$p" 2>/dev/null || true; done; }
trap cleanup EXIT

[ -d tests/e2e/node_modules ] || (cd tests/e2e && npm ci)

# name | app dir | build output (relative to repo root) | port
APPS="gallery|apps/gallery|target/dx/gallery/$PROFILE/web/public|8765
site|apps/site|target/dx/site/$PROFILE/web/public|8767
docs|apps/docs|target/dx/docs/$PROFILE/web/public|8769
dashboard|apps/dashboard|target/dx/dashboard/$PROFILE/web/public|8771"

while IFS='|' read -r name dir out port; do
  if [ "$BUILD" = 1 ]; then
    step "build $name"
    if [ "$PROFILE" = release ]; then (cd "$dir" && dx build --platform web --release)
    else (cd "$dir" && dx build --platform web); fi
  fi
  [ -f "$out/index.html" ] || { echo "$out/index.html missing: build $name first" >&2; exit 1; }
  python3 -m http.server "$port" --directory "$out" >/dev/null 2>&1 &
  PIDS+=("$!")
done <<<"$APPS"
sleep 1

G=http://localhost:8765/ S=http://localhost:8767/ D=http://localhost:8769/ A=http://localhost:8771/

for engine in ${BROWSERS//,/ }; do
  export BROWSER="$engine"
  printf '\n######## %s\n' "$engine"

  for scheme in light dark; do
    step "[$engine] axe gallery ($scheme)"
    e2e env GALLERY_URL=$G READY_SELECTOR=.gallery-main node axe.mjs "$scheme"
    step "[$engine] axe site ($scheme)"
    e2e env GALLERY_URL=$S READY_SELECTOR=#pricing node axe.mjs "$scheme"
    step "[$engine] axe docs ($scheme)"
    e2e env GALLERY_URL=$D READY_SELECTOR=.docs-main node axe.mjs "$scheme"
  done
  for route in "#button" "#data_table" "#theming"; do
    step "[$engine] axe docs ($route)"
    e2e env GALLERY_URL="$D$route" READY_SELECTOR=.docs-main node axe.mjs light
  done
  step "[$engine] axe gallery (rtl)"
  e2e env DIR=rtl GALLERY_URL=$G READY_SELECTOR=.gallery-main node axe.mjs light

  step "[$engine] docs navigation behaviour";        e2e env GALLERY_URL=$D node docs.mjs
  step "[$engine] form behaviour (site)";            e2e env GALLERY_URL=$S node form.mjs
  step "[$engine] Ctrl+K hotkey (site)";             e2e env GALLERY_URL=$S READY_SELECTOR=#pricing node hotkey.mjs
  step "[$engine] otp + carousel (gallery)";         e2e env GALLERY_URL=$G node otp.mjs
  step "[$engine] multi-select + dropzone (gallery)"; e2e env GALLERY_URL=$G node pickers.mjs
  step "[$engine] drawer (gallery)";                 e2e env GALLERY_URL=$G node drawer.mjs
  step "[$engine] table (gallery)";                  e2e env GALLERY_URL=$G node table.mjs
  step "[$engine] select + combobox form fields";    e2e env GALLERY_URL=$G node form_fields.mjs
  step "[$engine] anchor positioning (gallery)";     e2e env GALLERY_URL=$G node anchor.mjs
  step "[$engine] keyboard per APG pattern";         e2e env GALLERY_URL=$G node keyboard.mjs
  step "[$engine] keyboard-only walkthrough";        e2e env GALLERY_URL=$G node walkthrough.mjs
  step "[$engine] accessibility tree (roles, names, states)"; e2e env GALLERY_URL=$G node a11y_tree.mjs
  step "[$engine] density presets (gallery)";        e2e env GALLERY_URL=$G node density.mjs
  step "[$engine] forced colors (gallery)";          e2e env GALLERY_URL=$G node forced_colors.mjs
  step "[$engine] light-dark theme, chips, link";    e2e env GALLERY_URL=$G node theme.mjs
  step "[$engine] Ctrl+K hotkey (gallery)";          e2e env GALLERY_URL=$G node hotkey.mjs
  step "[$engine] dashboard: axe on every page + flows"; e2e env GALLERY_URL=$A node dashboard.mjs

  # Contrast of the accent presets depends on the CSS, not the engine: Chromium only.
  if [ "$engine" = chromium ]; then
    for accent in blue green violet rose orange; do
      for scheme in light dark; do
        step "[$engine] axe gallery ($accent, $scheme)"
        e2e env ACCENT="$accent" GALLERY_URL=$G READY_SELECTOR=.gallery-main node axe.mjs "$scheme"
      done
    done
  fi
done

if [ ${#FAILED[@]} -gt 0 ]; then
  printf '\n%s browser check(s) failed:\n' "${#FAILED[@]}"
  printf '  %s\n' "${FAILED[@]}"
  exit 1
fi
printf '\nBrowser checks passed on: %s\n' "$BROWSERS"
