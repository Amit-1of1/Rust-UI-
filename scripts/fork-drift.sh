#!/usr/bin/env bash
# How far our forks have drifted from upstream, and whether the patches still apply.
#
#   scripts/fork-drift.sh               report: for each forked crate, the release we patch, the
#                                       newest compatible and newest overall release upstream
#   scripts/fork-drift.sh --try-rebase  also replay each fork's fix commits onto (a) the newest
#                                       compatible release, when there is one, and (b) upstream's
#                                       default branch, in throwaway clones. A fix that replays
#                                       as empty on (b) is already upstream and can be dropped
#
# Read-only: clones into target/fork-drift, pushes nothing. Exit code 0 even when there is
# drift; the weekly workflow (.github/workflows/fork-drift.yml) files the report as an issue.
# The table below must match docs/FORKS.md; the pins are checked against Cargo.toml and
# scripts/install-dx.sh, so a bump that forgets this file fails here.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)

TRY=0
case "${1:-}" in
  --try-rebase) TRY=1 ;;
  "") ;;
  -h|--help) sed -n '2,14p' "$0"; exit 0 ;;
  *) echo "unknown argument: $1" >&2; exit 2 ;;
esac

# crate | release we patch | upstream repo | upstream commit of that release | fork repo | package dir
FORKS="
blitz-dom|0.2.4|DioxusLabs/blitz|0a77560e59440d3c6b5f14b673b9c722a08a3d4c|Amit-1of1/blitz|packages/blitz-dom
blitz-traits|0.2.0|DioxusLabs/blitz|0a77560e59440d3c6b5f14b673b9c722a08a3d4c|Amit-1of1/blitz|packages/blitz-traits
blitz-net|0.2.1|DioxusLabs/blitz|0a77560e59440d3c6b5f14b673b9c722a08a3d4c|Amit-1of1/blitz|packages/blitz-net
blitz-paint|0.2.1|DioxusLabs/blitz|a1af09c0ce99e488f093743c450e9d3678d7c202|Amit-1of1/blitz|packages/blitz-paint
dioxus-native-dom|0.7.10|DioxusLabs/dioxus|57d6794ad60b949e5bd8aa282f6f8c3dc97a365e|Amit-1of1/dioxus|packages/native-dom
dioxus-cli|0.7.10|DioxusLabs/dioxus|57d6794ad60b949e5bd8aa282f6f8c3dc97a365e|Amit-1of1/dioxus|packages/wasm-split
dioxus-primitives|git|DioxusLabs/components|9a758255ea26e2b20c8cecf4c1c946feb9e71da7|Amit-1of1/dioxus-components|primitives
"

UA="atoll fork-drift.sh ($(git config remote.origin.url 2>/dev/null || echo local))"
api() { curl -fsS -A "$UA" "https://crates.io/api/v1/crates/$1"; }

# The commit each fork is pinned to, from the files that pin it.
pinned() {
  case "$1" in
    dioxus-cli) sed -nE 's/^REV=([0-9a-f]+)/\1/p' scripts/install-dx.sh ;;
    dioxus-primitives) grep -E '^dioxus-primitives *=' Cargo.toml | sed -nE 's/.* rev = "([0-9a-f]+)".*/\1/p' ;;
    *) grep -E "^$1 *=" Cargo.toml | sed -nE 's/.* rev = "([0-9a-f]+)".*/\1/p' ;;
  esac
}

# Newest stable release with the same 0.x minor (Cargo-compatible), and newest of any kind.
releases() { # crate version -> "compatible newest"
  api "$1" | python3 -c '
import sys, json
want = sys.argv[1].split(".")
vs = [v["num"] for v in json.load(sys.stdin)["versions"] if not v["yanked"]]
def key(v):
    core, _, pre = v.partition("-")
    return tuple(int(x) for x in core.split(".")) + ((1,) if not pre else (0, pre))
stable = [v for v in vs if "-" not in v]
compat = [v for v in stable if v.split(".")[:2] == want[:2]]
print(max(compat, key=key) if compat else sys.argv[1], max(vs, key=key))
' "$2"
}

# The upstream commit a crates.io release was cut from (its .cargo_vcs_info.json).
release_commit() { # crate version
  local f="$work/$1-$2.crate"
  [ -f "$f" ] || curl -fsSL -A "$UA" -o "$f" "https://crates.io/api/v1/crates/$1/$2/download"
  tar xzf "$f" -O "$1-$2/.cargo_vcs_info.json" | python3 -c 'import sys,json;print(json.load(sys.stdin)["git"]["sha1"])'
}

clone() { # repo -> dir with remotes origin (fork) and upstream fetched
  local dir="$work/${1//\//_}"
  if [ ! -d "$dir/.git" ]; then
    git clone -q --filter=blob:none "https://github.com/$1" "$dir"
  fi
  echo "$dir"
}

# Replay the fork's fix commits (base..pin, minus the "use the published manifest" commit and
# merges) onto a target commit. Prints "clean", "clean, N already upstream", or the conflicts.
replay() { # dir base pin target pkgdir newcrate
  local dir=$1 base=$2 pin=$3 target=$4 pkg=$5 newcrate=${6:-}
  git -C "$dir" checkout -q --detach "$target" 2>/dev/null || { echo "target $target not found"; return; }
  if [ -n "$newcrate" ] && [ -f "$newcrate" ]; then
    # Renderer branches start by using the published manifest (docs/FORKS.md, step 1).
    tar xzf "$newcrate" -O "$(basename "$newcrate" .crate)/Cargo.toml" >"$dir/$pkg/Cargo.toml"
    git -C "$dir" -c user.name=drift -c user.email=drift@localhost commit -qam "use the published manifest" || true
  fi
  local picks empty=0 n=0
  picks=$(git -C "$dir" log --reverse --no-merges --format='%H %s' "$base..$pin" -- "$pkg" | grep -v 'published .* manifest' | cut -d' ' -f1)
  for c in $picks; do
    n=$((n + 1))
    if ! git -C "$dir" -c user.name=drift -c user.email=drift@localhost cherry-pick -x "$c" >/dev/null 2>&1; then
      if git -C "$dir" diff --cached --quiet && [ -z "$(git -C "$dir" diff --name-only --diff-filter=U)" ]; then
        empty=$((empty + 1)); git -C "$dir" cherry-pick --skip >/dev/null 2>&1 || true; continue
      fi
      echo "conflict at $(git -C "$dir" log -1 --format='%h %s' "$c") in: $(git -C "$dir" diff --name-only --diff-filter=U | tr '\n' ' ')"
      git -C "$dir" cherry-pick --abort >/dev/null 2>&1 || true
      return
    fi
  done
  if [ "$empty" -gt 0 ]; then echo "clean, $empty of $n fix commits already upstream"; else echo "clean ($n fix commits)"; fi
}

work="$root/target/fork-drift"
mkdir -p "$work"

out=$(mktemp)
{
  echo "# Fork drift ($(date -u +%Y-%m-%d))"
  echo
  echo "Generated by \`scripts/fork-drift.sh$([ "$TRY" = 1 ] && echo " --try-rebase")\`. What to do with it: docs/FORKS.md, \"Moving to a new upstream release\"."
  echo
  echo "| Crate | We patch | Newest compatible | Newest upstream | Pinned fork commit |"
  echo "|---|---|---|---|---|"
} >"$out"

notes=()
status=0
while IFS='|' read -r crate ver repo base fork pkg; do
  [ -n "$crate" ] || continue
  pin=$(pinned "$crate")
  if [ -z "$pin" ]; then
    echo "$crate: no pin found in Cargo.toml / scripts/install-dx.sh" >&2; status=1; continue
  fi
  if [ "$ver" = git ]; then
    head=$(git ls-remote "https://github.com/$repo" HEAD | cut -f1)
    if [ "$head" = "$base" ]; then compat="(main = our base)"; else compat="main moved to ${head:0:7}"; fi
    newest=$(api "$crate" 2>/dev/null | python3 -c 'import sys,json;print(json.load(sys.stdin)["crate"]["max_version"])' 2>/dev/null || echo "?")
    newest="crates.io $newest"
  else
    read -r compat newest < <(releases "$crate" "$ver")
    [ "$compat" = "$ver" ] && compat="$ver (current)"
  fi
  echo "| \`$crate\` | $ver @ ${base:0:7} | $compat | $newest | \`${pin:0:7}\` |" >>"$out"

  if [ "$TRY" = 1 ]; then
    dir=$(clone "$fork")
    git -C "$dir" remote get-url upstream >/dev/null 2>&1 || git -C "$dir" remote add upstream "https://github.com/$repo"
    git -C "$dir" fetch -q upstream
    git -C "$dir" fetch -q origin "$pin" 2>/dev/null || true
    def=$(git -C "$dir" ls-remote upstream HEAD | cut -f1)
    if [ "$ver" != git ] && [[ "$compat" != *current* ]]; then
      target=$(release_commit "$crate" "$compat")
      notes+=("- \`$crate\` onto $compat (\`${target:0:7}\`): $(replay "$dir" "$base" "$pin" "$target" "$pkg" "$work/$crate-$compat.crate")")
    fi
    notes+=("- \`$crate\` onto upstream's default branch (\`${def:0:7}\`): $(replay "$dir" "$base" "$pin" "$def" "$pkg")")
  fi
done <<<"$FORKS"

if [ "${#notes[@]}" -gt 0 ]; then
  { echo; echo "## Replaying the fixes"; echo; printf '%s\n' "${notes[@]}"; } >>"$out"
fi
cat "$out"
[ -n "${GITHUB_STEP_SUMMARY:-}" ] && cat "$out" >>"$GITHUB_STEP_SUMMARY"
cp "$out" "$work/report.md"
rm -f "$out"
exit "$status"
