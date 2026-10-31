#!/usr/bin/env bash
# Publish the kit to crates.io, in dependency order. Dry run unless --publish is given.
#
#   scripts/publish.sh                 dry run: package every crate (the fork's two included),
#                                      build an outside app from the packages
#                                      (scripts/consumer-check.sh) and list what --publish would do
#   scripts/publish.sh --publish       publish for real; needs CARGO_REGISTRY_TOKEN, a clean tree
#                                      and HEAD tagged v<version>. Owner only (docs/PUBLISHING.md)
#
# Order: atoll-attributes, atoll-primitives (from the fork commit pinned in Cargo.toml),
# atoll-form-derive, atoll-motion, atoll-ui, atoll-cli. A crate already on crates.io
# at this version is skipped, so a run that stopped half way can be started again.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)

PUBLISH=0
case "${1:-}" in
  --publish) PUBLISH=1 ;;
  "" ) ;;
  -h|--help) sed -n '2,12p' "$0"; exit 0 ;;
  *) echo "unknown argument: $1" >&2; exit 2 ;;
esac

step() { printf '\n== %s\n' "$*"; }
on_crates_io() { # name version
  curl -fsS -A "atoll publish.sh ($(git config remote.origin.url))" \
    "https://crates.io/api/v1/crates/$1/$2" >/dev/null 2>&1
}

version=$(sed -nE 's/^version = "([^"]+)"/\1/p' crates/ui/Cargo.toml | head -1)
line=$(grep -E '^dioxus-primitives *=' Cargo.toml)
field() { sed -nE "s/.* $1 = \"([^\"]+)\".*/\1/p" <<<"$line"; }
prim_pkg=$(field package); prim_ver=$(field version); prim_git=$(field git); prim_rev=$(field rev)
: "${prim_pkg:=dioxus-primitives}"

step "versions"
echo "kit crates: $version"
echo "primitives: $prim_pkg $prim_ver${prim_git:+ (built from $prim_git @ ${prim_rev:0:7})}"
for c in atoll-form-derive atoll-motion atoll-ui atoll-cli; do
  v=$(sed -nE 's/^version = "([^"]+)"/\1/p' "$(cargo metadata -q --no-deps --format-version 1 |
    python3 -c "import sys,json;print(next(p['manifest_path'] for p in json.load(sys.stdin)['packages'] if p['name']=='$c'))")" | head -1)
  [ "$v" = "$version" ] || { echo "$c is $v, not $version: the crates share one version (docs/VERSIONING.md)" >&2; exit 1; }
done

if [ "$PUBLISH" = 1 ]; then
  step "preconditions"
  [ -n "${CARGO_REGISTRY_TOKEN:-}" ] || { echo "CARGO_REGISTRY_TOKEN is not set" >&2; exit 1; }
  git diff --quiet HEAD || { echo "the tree has changes; publish from a clean checkout" >&2; exit 1; }
  git tag --points-at HEAD | grep -qx "v$version" || {
    echo "HEAD is not tagged v$version; tag the release commit first (docs/VERSIONING.md)" >&2; exit 1; }
  if grep -q '^publish = false' crates/ui/Cargo.toml; then
    echo "crates/ui/Cargo.toml still says publish = false; remove it in the release PR (docs/PUBLISHING.md)" >&2
    exit 1
  fi
fi

# 1-2. The primitives fork, from the exact commit the workspace builds against.
if [ -n "$prim_git" ]; then
  fork="$root/target/publish/dioxus-components"
  step "primitives fork at ${prim_rev:0:7}"
  if [ ! -d "$fork/.git" ]; then
    git clone -q --filter=blob:none "$prim_git" "$fork"
  fi
  git -C "$fork" fetch -q origin "$prim_rev" 2>/dev/null || git -C "$fork" fetch -q origin
  git -C "$fork" -c advice.detachedHead=false checkout -q "$prim_rev"
  for c in atoll-attributes atoll-primitives; do
    if on_crates_io "$c" "$prim_ver"; then echo "$c $prim_ver is on crates.io; skipped"; continue; fi
    if [ "$PUBLISH" = 1 ]; then
      (cd "$fork" && cargo publish -p "$c")
    fi
  done
  if [ "$PUBLISH" = 0 ]; then
    # Packages both and builds each from its tarball, the attributes crate standing in for
    # crates.io while it is unpublished.
    (cd "$fork" && cargo package -q -p atoll-attributes -p atoll-primitives)
    echo "packaged and verified: atoll-attributes, atoll-primitives"
  fi
elif ! on_crates_io "$prim_pkg" "$prim_ver"; then
  echo "$prim_pkg $prim_ver is not on crates.io, and Cargo.toml has no git source for it" >&2; exit 1
fi

# 3-5. The kit's crates. The dry run builds them from their packages in an outside app.
step "kit crates"
if [ "$PUBLISH" = 0 ]; then
  scripts/consumer-check.sh
  cargo package -q -p atoll-cli --allow-dirty
  echo "packaged and verified: atoll-cli"
else
  for c in atoll-form-derive atoll-motion atoll-ui atoll-cli; do
    if on_crates_io "$c" "$version"; then echo "$c $version is on crates.io; skipped"; continue; fi
    cargo publish -p "$c"
  done
fi

step "next"
if [ "$PUBLISH" = 0 ]; then
  cat <<TXT
Dry run passed. --publish would publish, in order:
  atoll-attributes $prim_ver, atoll-primitives $prim_ver   (fork @ ${prim_rev:0:7})
  atoll-form-derive, atoll-motion, atoll-ui, atoll-cli $version
Before that (docs/PUBLISHING.md): make the repo public, remove
publish = false from crates/ui/Cargo.toml, tag v$version.
TXT
else
  cat <<TXT
Published. Now, in a PR: delete git and rev from the dioxus-primitives line in Cargo.toml and
run \`cargo run -p atoll-cli -- registry\`, so component.json asks for crates.io versions.
scripts/consumer-check.sh then fails if any git source is left.
TXT
fi
