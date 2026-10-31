#!/usr/bin/env bash
# Edit-compile times for a wasm app built with the kit (default: the gallery, which uses every
# component, so it is the slowest case). Prints one tab-separated line per step:
#
#   bench/build-times.sh [package]        # e.g. bench/build-times.sh dashboard
#
# Steps: a cold build (after `cargo clean`), a no-op rebuild, a rebuild after editing one kit
# component, a rebuild after editing the app, then
# the same cold and app-edit builds in release. Edits are a trailing comment, undone with
# `git checkout`, so run it on a clean tree. Dependencies are fetched first and not timed.
# `cargo clean` deletes target/: expect the cold steps to take minutes.
set -euo pipefail
cd "$(dirname "$0")/.."
PKG=${1:-gallery}
APP_MAIN=apps/$PKG/src/main.rs
KIT_FILE=crates/ui/src/components/button/component.rs
if ! git diff --quiet -- "$APP_MAIN" "$KIT_FILE"; then
  echo "commit or stash changes to $APP_MAIN and $KIT_FILE first" >&2
  exit 1
fi
trap 'git checkout -q -- "$APP_MAIN" "$KIT_FILE"' EXIT
WASM=(--target wasm32-unknown-unknown)

step() {
  local label=$1
  shift
  local start end
  start=$(date +%s.%N)
  "$@" >/dev/null 2>&1
  end=$(date +%s.%N)
  printf '%s\t%.1f s\n' "$label" "$(echo "$end - $start" | bc)"
}
touch_file() { echo "// build-times probe" >>"$1"; }
undo() { git checkout -q -- "$1"; }

cargo fetch >/dev/null
cargo clean >/dev/null 2>&1
printf 'package\t%s\ncpus\t%s\nrustc\t%s\n' "$PKG" "$(nproc)" "$(rustc --version)"
step "cold, dev" cargo build -p "$PKG" "${WASM[@]}"
step "no change, dev" cargo build -p "$PKG" "${WASM[@]}"
touch_file "$KIT_FILE"
step "edit a kit component, dev" cargo build -p "$PKG" "${WASM[@]}"
undo "$KIT_FILE"
touch_file "$APP_MAIN"
step "edit the app, dev" cargo build -p "$PKG" "${WASM[@]}"
undo "$APP_MAIN"
step "cold, release" cargo build -p "$PKG" "${WASM[@]}" --release
touch_file "$APP_MAIN"
step "edit the app, release" cargo build -p "$PKG" "${WASM[@]}" --release
undo "$APP_MAIN"
