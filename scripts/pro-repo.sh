#!/usr/bin/env bash
# Build the private Pro repository locally (docs/pro/README.md). Nothing is pushed.
#   scripts/pro-repo.sh <dir>          a clone of this repo set up as the Pro repo: marker file,
#                                      the Pro licence, `upstream` pointing at the public repo
#   scripts/pro-repo.sh <dir> --demo   the same, then marks one block Pro and walks the buyer's
#                                      path: `rust-ui list`, then `rust-ui add` into a throwaway app
# Afterwards: create the private repo on GitHub, `git -C <dir> remote add origin <its URL>` and
# push. Keep it in step with the public kit with `git -C <dir> pull upstream main`.
set -euo pipefail
cd "$(dirname "$0")/.."
here=$PWD

dir="${1:-}"
demo=0
[ "${2:-}" = --demo ] && demo=1
if [ -z "$dir" ] || [ "$dir" = -h ] || [ "$dir" = --help ]; then
  sed -n '2,8p' "$0"; exit 2
fi
if [ -e "$dir" ]; then
  echo "$dir already exists; pick a new folder" >&2; exit 1
fi

step() { printf '\n== %s\n' "$*"; }

step "clone"
git clone -q --no-hardlinks -- "$here" "$dir"
upstream=$(git -C "$here" remote get-url origin 2>/dev/null || echo "$here")
git -C "$dir" remote remove origin
git -C "$dir" remote add upstream "$upstream"
git -C "$dir" checkout -q -B main

step "mark it as the Pro repo"
cat >"$dir/.rust-ui-pro" <<'MARK'
This is the private Pro repository. Pro items are the component and block folders that hold
a LICENSE-PRO.md (copy docs/pro/LICENSE-PRO.md into the folder). See docs/pro/README.md.
MARK
git -C "$dir" add .rust-ui-pro
git -C "$dir" -c user.name="rust-ui pro-repo.sh" -c user.email=pro-repo@localhost \
  commit -q -m "Set up as the Pro repository"
echo "$dir is ready; upstream = $upstream"

if [ "$demo" = 1 ]; then
  cli() { cargo run -q --manifest-path "$here/Cargo.toml" -p rust-ui-cli -- "$@"; }
  step "demo: mark login_01 as Pro"
  cp "$here/docs/pro/LICENSE-PRO.md" "$dir/crates/ui/src/blocks/login_01/LICENSE-PRO.md"
  (cd "$dir" && cli registry >/dev/null)
  grep -A3 '"name": "login_01"' "$dir/registry.json"
  step "demo: rust-ui list"
  cli --source "$dir" list | grep -E '\(.*pro' 
  step "demo: rust-ui add login_01 into a throwaway app"
  app=$(mktemp -d)
  (cd "$app" && cli --source "$dir" add login_01 | tail -2)
  test -f "$app/src/blocks/login_01/LICENSE-PRO.md"
  echo "licence copied with the block: $app/src/blocks/login_01/LICENSE-PRO.md"
  rm -rf "$app"
  echo
  echo "Demo changes are left uncommitted in $dir (git -C $dir checkout . && git -C $dir clean -fd to drop them)."
fi
