#!/usr/bin/env bash
# Build an app outside this workspace against the kit as crates.io would serve it.
#
#   scripts/consumer-check.sh            package the crates, build tests/consumer for the web
#                                        (wasm32) and the host, and list where every crate came from
#   KEEP=1 scripts/consumer-check.sh     keep the scratch app and print its path
#
# What it proves (docs/PUBLISHING.md): the packaged crates build; an app needs no `[patch]` of
# Dioxus or Blitz, and gets stock `dioxus` from crates.io; the only git source left is our
# primitives fork, until `rust-ui-primitives` is published. Then it checks that none is left.
# Read-only for the repo: it packages into target/package and works in a temp directory.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)

step() { printf '\n== %s\n' "$*"; }

# The primitives line of the workspace manifest: git + rev until published, version after.
line=$(grep -E '^dioxus-primitives *=' Cargo.toml)
field() { sed -nE "s/.* $1 = \"([^\"]+)\".*/\1/p" <<<"$line"; }
prim_git=$(field git); prim_rev=$(field rev)
patch_args=()
if [ -n "$prim_git" ]; then
  # Unpublished: let packaging resolve the renamed fork crates from git. crates.io never sees
  # this; it is the same source the workspace builds from.
  for c in rust-ui-primitives rust-ui-attributes; do
    patch_args+=(--config "patch.crates-io.$c.git=\"$prim_git\"" --config "patch.crates-io.$c.rev=\"$prim_rev\"")
  done
fi

crates=(rust-ui-form-derive rust-ui-motion rust-ui-kit)
step "package ${crates[*]}"
# --no-verify: the next step builds the packaged sources for real, outside the workspace.
pkg_args=()
for c in "${crates[@]}"; do pkg_args+=(-p "$c"); done
cargo package "${pkg_args[@]}" --no-verify --allow-dirty "${patch_args[@]}" 2>&1 | grep -E '^ *(Packag|warning|error)' || true

work=$(mktemp -d)
[ "${KEEP:-0}" = 1 ] || trap 'rm -rf "$work"' EXIT
version=$(sed -nE 's/^version = "([^"]+)"/\1/p' crates/ui/Cargo.toml | head -1)
for c in "${crates[@]}"; do
  tar xzf "target/package/$c-$version.crate" -C "$work"
done

app="$work/app"
mkdir -p "$app"
cp -r tests/consumer/src "$app/"
{
  cat <<TOML
[package]
name = "consumer"
version = "0.0.0"
edition = "2021"
publish = false

# Its own workspace: nothing from the kit's workspace (its [patch] section included) applies.
[workspace]

[dependencies]
dioxus = "0.7"
ui = { package = "rust-ui-kit", version = "$version" }

[features]
default = ["web"]
web = ["dioxus/web", "ui/web"]

# The packaged crates stand in for crates.io until they are published.
[patch.crates-io]
TOML
  for c in "${crates[@]}"; do echo "$c = { path = \"../$c-$version\" }"; done
  if [ -n "$prim_git" ]; then
    echo "rust-ui-primitives = { git = \"$prim_git\", rev = \"$prim_rev\" }"
    echo "rust-ui-attributes = { git = \"$prim_git\", rev = \"$prim_rev\" }"
  fi
} >"$app/Cargo.toml"

cd "$app"
# Share the kit's target dir so a second run is quick; the app is its own workspace either way.
export CARGO_TARGET_DIR="$root/target/consumer"

step "consumer: web (wasm32)"
cargo build -q --target wasm32-unknown-unknown

step "consumer: host, no renderer (server-side rendering, tests)"
cargo check -q --no-default-features

step "where every crate comes from"
# `(*)` marks a crate already listed; `(proc-macro)` its kind. Neither is a source.
tree=$(cargo tree -q --target all -e normal,build --prefix none --format '{p}' |
  sed -E 's/ \((\*|proc-macro)\)//g' | sort -u)
git_sources=$(grep -F '(https://' <<<"$tree" | grep -F '#' || true)
grep -E '^dioxus v' <<<"$tree"
if grep -qE '^dioxus v.*\(' <<<"$tree"; then
  echo "dioxus does not come from crates.io" >&2; exit 1
fi
if [ -n "$git_sources" ]; then
  echo "git sources:"; sed 's/^/  /' <<<"$git_sources"
  unexpected=$(grep -vE '^rust-ui-(primitives|attributes) ' <<<"$git_sources" || true)
  if [ -n "$unexpected" ]; then
    echo "unexpected git dependencies (only the unpublished primitives fork may be one):" >&2
    echo "$unexpected" >&2; exit 1
  fi
  if [ -z "$prim_git" ]; then
    echo "primitives are published, so no git source should be left" >&2; exit 1
  fi
else
  echo "no git sources: everything comes from crates.io (or the packaged crates)"
fi
for c in blitz-dom blitz-paint dioxus-native-dom; do
  if grep -qE "^$c v.*\(https://github.com/Amit-1of1" <<<"$tree"; then
    echo "$c comes from a fork; an app should never need our renderer forks for web" >&2; exit 1
  fi
done

[ "${KEEP:-0}" = 1 ] && echo "kept: $app"
printf '\nConsumer check passed.\n'
