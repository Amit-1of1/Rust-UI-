#!/usr/bin/env bash
# Run docs/GETTING_STARTED.md against this checkout, the way a new user would.
#
#   scripts/getting-started-check.sh        create an app with `atoll create`, build it for the web
#                                           (wasm32), check the desktop build, then compile the
#                                           guide's own code sample in it; prints how long it took
#   KEEP=1 scripts/getting-started-check.sh keep the app and print its path
#
# The app is its own workspace outside this repo, so nothing here (the [patch] section
# included) reaches it. It uses `--path` to this checkout instead of crates.io or git, so it
# tests the kit as it is now; the generated Cargo.toml is otherwise what users get.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)
start=$(date +%s)
step() { printf '\n== %s (%ss)\n' "$*" "$(( $(date +%s) - start ))"; }

step "build the atoll CLI"
cargo build -q -p atoll-cli
atoll="$root/target/debug/atoll"

work=$(mktemp -d)
[ "${KEEP:-0}" = 1 ] || trap 'rm -rf "$work"' EXIT
export CARGO_TARGET_DIR="$root/target/getting-started"

step "atoll create my-app"
(cd "$work" && "$atoll" create my-app --path "$root")
app="$work/my-app"
grep -q 'ui = { package = "atoll-ui"' "$app/Cargo.toml"

cd "$app"
step "web build (wasm32), as dx serve does"
cargo build -q --target wasm32-unknown-unknown

step "desktop build check"
if [ "$(uname)" = Linux ] && ! pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
  echo "skipped: no webkit2gtk-4.1 here (CI installs libwebkit2gtk-4.1-dev and libxdo-dev)"
  [ "${CI:-}" = true ] && exit 1
else
  cargo check -q --no-default-features --features desktop
fi

step "the guide's code sample compiles"
{
  echo 'use dioxus::prelude::*;'
  echo 'use ui::prelude::*;'
  echo 'fn main() { dioxus::launch(App); }'
  # The first ```rust block of step 3, minus its own `use` lines.
  awk '/^## 3\./{s=1} s&&/^```rust/{c=1;next} c&&/^```/{exit} c' "$root/docs/GETTING_STARTED.md" | grep -v '^use '
} >src/main.rs
cargo build -q --target wasm32-unknown-unknown

step "done"
[ "${KEEP:-0}" = 1 ] && echo "app kept at $app"
echo "getting-started check passed in $(( $(date +%s) - start ))s"
