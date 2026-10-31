#!/usr/bin/env bash
# Proves the registry works with Dioxus' own installer: copies the repo into a throwaway git
# registry, runs `dx components add --all --git <it>` in a fresh crate, and compiles the result.
# Slow (a full dependency build), so it is not part of scripts/check.sh. Needs `dx` and network
# access to crates.io and github.com.
set -euo pipefail
cd "$(dirname "$0")/.."
ROOT=$PWD
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
export NO_DOWNLOADS=1

mkdir "$WORK/registry"
cp --parents $(git ls-files; git ls-files -o --exclude-standard) "$WORK/registry/"
URL="file://$WORK/registry"
(cd "$WORK/registry" && ATOLL_REGISTRY_GIT="$URL" cargo run -q --manifest-path "$ROOT/Cargo.toml" -p atoll-cli -- --source . registry >/dev/null \
  && git init -q && git add -A && git -c user.email=ci@example.com -c user.name=ci commit -q -m registry)

dx components clean >/dev/null 2>&1 || true
mkdir "$WORK/app" && cd "$WORK/app"
cargo init -q --name dx_compat_probe .
cargo add -q dioxus --features web
dx components add --all --git "$URL"
# The drawer depends on the motion crate from the registry's git URL; use the local copy.
printf '\n[patch."%s"]\natoll-motion = { path = "%s/crates/motion" }\n' "$URL" "$ROOT" >> Cargo.toml
printf '#![allow(dead_code, unused)]\nmod components;\nfn main() {}\n' > src/main.rs
cargo check -q
echo "dx components add --all: ok"
