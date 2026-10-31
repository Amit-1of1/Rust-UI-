#!/usr/bin/env bash
# Build every `atoll create --template` app the way a user would get it.
#
#   scripts/template-check.sh              every template: create it outside this repo, build
#                                          it for the web (wasm32) and check the desktop build
#   scripts/template-check.sh dashboard    just these templates
#   KEEP=1 scripts/template-check.sh       keep the apps and print where they are
#
# Each app is its own workspace with `--path` to this checkout, so it tests the templates
# as they are now and nothing from this repo's workspace (features, [patch]) leaks in
# unless the template copies it on purpose.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)
start=$(date +%s)
step() { printf '\n== %s (%ss)\n' "$*" "$(( $(date +%s) - start ))"; }

cargo build -q -p atoll-cli
atoll="$root/target/debug/atoll"

if [ $# -gt 0 ]; then
  templates=("$@")
else
  templates=(dashboard booking marketing showcase)
fi

work=$(mktemp -d)
[ "${KEEP:-0}" = 1 ] || trap 'rm -rf "$work"' EXIT
# One target folder for all of them: they share most of their dependencies.
export CARGO_TARGET_DIR="$root/target/templates"

desktop=1
if [ "$(uname)" = Linux ] && ! pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
  desktop=0
  echo "desktop checks skipped: no webkit2gtk-4.1 here (CI installs libwebkit2gtk-4.1-dev)"
  [ "${CI:-}" = true ] && exit 1
fi

for t in "${templates[@]}"; do
  step "atoll create my-$t --template $t"
  (cd "$work" && "$atoll" create "my-$t" --template "$t" --path "$root" >/dev/null)
  app="$work/my-$t"
  if grep -q 'workspace' "$app/Cargo.toml"; then
    echo "my-$t/Cargo.toml still points into the kit's workspace" >&2
    exit 1
  fi
  step "my-$t: web build (wasm32)"
  (cd "$app" && cargo build -q --target wasm32-unknown-unknown)
  if [ "$desktop" = 1 ]; then
    step "my-$t: desktop build check"
    (cd "$app" && cargo check -q --no-default-features --features desktop)
  fi
done

step "all templates build"
[ "${KEEP:-0}" = 1 ] && echo "apps kept in $work"
exit 0
