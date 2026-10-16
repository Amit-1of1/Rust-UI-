#!/usr/bin/env bash
# Which open branches conflict with main, and with each other, without checking anything out.
#
#   scripts/pr-conflicts.sh                    every origin/claude/* branch not yet in main that
#                                              moved in the last 3 days
#   scripts/pr-conflicts.sh branch-a branch-b  just these (names on origin)
#
# A file marked "(generated)" is fixed by scripts/sync-main.sh on its own; anything else needs
# the branch's session. Pairs that conflict tell the owner which PR to merge first
# (docs/PROTOCOL.md). Read-only: it fetches, nothing else.
set -euo pipefail
cd "$(dirname "$0")/.."

case "${1:-}" in -h|--help) sed -n '2,10p' "$0"; exit 0 ;; esac

git fetch -q origin '+refs/heads/*:refs/remotes/origin/*'

branches=()
if [ $# -gt 0 ]; then
  for b in "$@"; do branches+=("${b#origin/}"); done
else
  since=$(date -d '3 days ago' +%s 2>/dev/null || date -v-3d +%s)
  while read -r ref when; do
    b=${ref#refs/remotes/origin/}
    [ "$when" -ge "$since" ] || continue
    git merge-base --is-ancestor "$ref" origin/main && continue
    branches+=("$b")
  done < <(git for-each-ref --format='%(refname) %(committerdate:unix)' 'refs/remotes/origin/claude/*')
fi

generated() {
  case "$1" in
    registry.json|Cargo.lock|apps/docs/src/generated.rs|llms.txt|llms-full.txt|llms/*) return 0 ;;
    crates/ui/src/components/*/component.json|component.json|crates/ui/tests/snapshots/*) return 0 ;;
  esac
  return 1
}

conflicts() {
  git merge-tree --write-tree --name-only --no-messages "origin/$1" "origin/$2" | tail -n +2 || true
}

show() {
  local any=0 f
  while IFS= read -r f; do
    [ -z "$f" ] && continue
    any=1
    if generated "$f"; then echo "    $f (generated)"; else echo "    $f"; fi
  done
  [ "$any" = 1 ] || echo "    none"
}

echo "== each branch against main"
for b in "${branches[@]}"; do
  behind=$(git rev-list --count "origin/$b..origin/main")
  echo "  $b ($behind commits behind main)"
  conflicts main "$b" | show
done

echo
echo "== pairs that conflict with each other"
pairs=0
for ((i = 0; i < ${#branches[@]}; i++)); do
  for ((j = i + 1; j < ${#branches[@]}; j++)); do
    out=$(conflicts "${branches[$i]}" "${branches[$j]}")
    [ -z "$out" ] && continue
    pairs=1
    echo "  ${branches[$i]} x ${branches[$j]}"
    printf '%s\n' "$out" | show
  done
done
[ "$pairs" = 1 ] || echo "  none"
