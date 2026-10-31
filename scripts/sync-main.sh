#!/usr/bin/env bash
# Bring the latest main into the current branch, and regenerate what is generated instead of
# merging it by hand.
#
#   scripts/sync-main.sh          fetch origin/main, merge it, fix generated files, commit
#   scripts/sync-main.sh --continue   after resolving the remaining conflicts by hand
#
# Generated files (registry.json, every component.json, apps/docs/src/generated.rs, llms.txt and
# llms/, Cargo.lock, markup snapshots) are never resolved by hand: the script takes main's copy and regenerates
# it from the merged sources. Cargo.toml files where both sides only added lines (two new
# dependencies at the same spot) keep both, if cargo still reads the result. Anything else that
# conflicts is listed and left for you; fix it, `git add` it, then run the script again with --continue.
# First it merges origin/<your branch> if someone else (the sync-prs workflow) pushed to it.
# It never pushes, rebases or force-pushes. Run scripts/check.sh afterwards, then push.
# The rules around it are in docs/PROTOCOL.md.
set -euo pipefail
cd "$(dirname "$0")/.."

CONTINUE=0
case "${1:-}" in
  --continue) CONTINUE=1 ;;
  "") ;;
  -h|--help) sed -n '2,16p' "$0"; exit 0 ;;
  *) echo "unknown argument: $1" >&2; exit 2 ;;
esac

BASE="${SYNC_BASE:-main}"
branch=$(git rev-parse --abbrev-ref HEAD)
if [ "$branch" = "HEAD" ] || [ "$branch" = "$BASE" ]; then
  echo "sync-main: check out your work branch first (now on '$branch')" >&2
  exit 2
fi

# Set when a snapshot conflicted, so --continue knows to regenerate them.
MARK="$(git rev-parse --git-dir)/sync-main-snapshots"

# Paths that are generated from other files in the repo.
is_generated() {
  case "$1" in
    registry.json|Cargo.lock|apps/docs/src/generated.rs|llms.txt|llms-full.txt|llms/*) return 0 ;;
    crates/ui/src/components/*/component.json|component.json) return 0 ;;
    crates/ui/tests/snapshots/*) return 0 ;;
    crates/ui/assets/*.scoped.css) return 0 ;;
  esac
  return 1
}

regenerate_and_commit() {
  local snapshots="$1"
  echo "== regenerating registry.json, component.json files, llms pages and the docs index"
  cargo run -q -p atoll-cli -- registry >/dev/null
  echo "== refreshing Cargo.lock"
  cargo metadata -q --format-version 1 >/dev/null
  if [ "$snapshots" = 1 ]; then
    echo "== regenerating markup snapshots (review their diff before pushing)"
    UPDATE_SNAPSHOTS=1 cargo test -q -p atoll-ui --test snapshots >/dev/null
  fi
  git add -A registry.json Cargo.lock apps/docs/src/generated.rs component.json \
    crates/ui/src/components crates/ui/tests/snapshots llms.txt llms-full.txt llms \
    crates/ui/assets/theme.scoped.css crates/ui/assets/reset.scoped.css
  if git rev-parse -q --verify MERGE_HEAD >/dev/null; then
    git commit -q --no-edit
  elif ! git diff --cached --quiet; then
    git commit -q -m "Regenerate generated files after merging $BASE"
  fi
  echo
  echo "Merged origin/$BASE into $branch. Next: scripts/check.sh, then push."
}

if [ "$CONTINUE" = 1 ]; then
  if ! git rev-parse -q --verify MERGE_HEAD >/dev/null; then
    echo "sync-main: no merge in progress" >&2
    exit 2
  fi
  if [ -n "$(git diff --name-only --diff-filter=U)" ]; then
    echo "sync-main: still unresolved:" >&2
    git diff --name-only --diff-filter=U | sed 's/^/  /' >&2
    exit 1
  fi
  snapshots=0
  [ -f "$MARK" ] && snapshots=1
  rm -f "$MARK"
  regenerate_and_commit "$snapshots"
  exit 0
fi

if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "sync-main: commit or stash your changes first" >&2
  exit 2
fi

# Pick up commits pushed to this branch from elsewhere (the sync-prs workflow merges main into
# open PR branches after every merge), so the next push is not rejected.
if git fetch -q origin "$branch" 2>/dev/null && ! git merge-base --is-ancestor "origin/$branch" HEAD; then
  echo "== merging origin/$branch (pushed from elsewhere) into $branch"
  if ! git merge -q --no-edit "origin/$branch"; then
    echo "sync-main: origin/$branch conflicts with your local commits; resolve, commit, run again" >&2
    exit 1
  fi
fi

git fetch -q origin "$BASE"
if git merge-base --is-ancestor "origin/$BASE" HEAD; then
  echo "Already up to date with origin/$BASE."
  exit 0
fi

echo "== merging origin/$BASE into $branch"
if git merge -q --no-edit "origin/$BASE"; then
  regenerate_and_commit 0
  exit 0
fi

# Cargo.toml where both sides added lines at the same place: keep both sides' lines. Checked
# below with `cargo metadata`; a duplicate key or a changed line puts the conflict back.
union_toml() {
  local f="$1"
  git cat-file -e ":1:$f" 2>/dev/null || return 1
  local tmp
  tmp=$(mktemp -d)
  git show ":2:$f" >"$tmp/ours"; git show ":1:$f" >"$tmp/base"; git show ":3:$f" >"$tmp/theirs"
  git merge-file --union -p "$tmp/ours" "$tmp/base" "$tmp/theirs" >"$f"
  rm -rf "$tmp"
}

snapshots=0
left=()
unioned=()
while IFS= read -r f; do
  [ -z "$f" ] && continue
  case "$f" in
    Cargo.toml|*/Cargo.toml)
      if union_toml "$f"; then unioned+=("$f"); continue; fi ;;
  esac
  if is_generated "$f"; then
    # Main's copy; regenerated below from the merged sources.
    git checkout -q --theirs -- "$f" 2>/dev/null || git rm -q -- "$f"
    git add -- "$f" 2>/dev/null || true
    case "$f" in crates/ui/tests/snapshots/*) snapshots=1 ;; esac
    echo "   generated: $f (taken from $BASE, regenerated)"
  else
    left+=("$f")
  fi
done < <(git diff --name-only --diff-filter=U)

if [ "${#unioned[@]}" -gt 0 ]; then
  if cargo metadata -q --format-version 1 --no-deps >/dev/null 2>&1; then
    for f in "${unioned[@]}"; do
      git add -- "$f"
      echo "   Cargo.toml: $f (both sides' added lines kept; review the diff)"
    done
  else
    for f in "${unioned[@]}"; do
      git checkout -q -m -- "$f"
      left+=("$f")
    done
  fi
fi

if [ "${#left[@]}" -gt 0 ]; then
  if [ "$snapshots" = 1 ]; then touch "$MARK"; fi
  echo
  echo "Resolve these by hand (both sides changed them), then 'git add' them and run"
  echo "scripts/sync-main.sh --continue:"
  for f in "${left[@]}"; do
    case "$f" in
      CHANGELOG.md|docs/MIGRATION.md)
        echo "  $f   (move your entry to changes/<topic>.md; see changes/README.md)" ;;
      *) echo "  $f" ;;
    esac
  done
  exit 1
fi

regenerate_and_commit "$snapshots"
