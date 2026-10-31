#!/usr/bin/env bash
# Catch API breaks before they merge: cargo-semver-checks against a base, plus our policy.
#
#   scripts/semver.sh                  compare this branch with origin/main
#   scripts/semver.sh <git ref>        compare with another ref (a release tag, say)
#   scripts/semver.sh --release        compare with the newest version on crates.io (when one
#                                      exists) and say which version bump the changes need
#
# A change that cargo-semver-checks calls breaking passes only when the branch also adds a
# migration note (changes/<topic>.migration.md) or a "### Breaking" entry in its fragment, so a
# break is always written down for the people upgrading (docs/VERSIONING.md). Breaks in items
# marked experimental still need the entry; the note can be one line.
# Needs cargo-semver-checks (`cargo install cargo-semver-checks --locked`).
set -euo pipefail
cd "$(dirname "$0")/.."

# Library crates with a public Rust API. form-derive is a proc macro (semver-checks can't read
# those); the CLI's library is internal (docs/STABILITY.md).
CRATES=(rust-ui-kit rust-ui-motion)

mode=ref
base=origin/main
case "${1:-}" in
  --release) mode=release ;;
  -h|--help) sed -n '2,13p' "$0"; exit 0 ;;
  "") ;;
  *) base=$1 ;;
esac

if ! cargo semver-checks --version >/dev/null 2>&1; then
  echo "cargo-semver-checks is not installed: cargo install cargo-semver-checks --locked" >&2
  exit 2
fi

args=()
for c in "${CRATES[@]}"; do args+=(--package "$c"); done
# The features apps turn on; `demos`, `blocks` and `contract-check` are internal.
args+=(--only-explicit-features --features web)

log=$(mktemp)
trap 'rm -f "$log"' EXIT
if [ "$mode" = release ]; then
  version=$(sed -nE 's/^version = "([^"]+)"/\1/p' crates/ui/Cargo.toml | head -1)
  if ! curl -fsS -A "rust-ui semver.sh" "https://crates.io/api/v1/crates/rust-ui-kit" >/dev/null 2>&1; then
    echo "rust-ui-kit is not on crates.io yet; comparing with the last tag instead"
    tag=$(git describe --tags --abbrev=0 --match 'v*' 2>/dev/null || true)
    [ -n "$tag" ] || { echo "no v* tag either: nothing to compare with (first release)"; exit 0; }
    args+=(--baseline-rev "$tag")
  fi
  # With a crates.io baseline (the default), semver-checks also says whether `version` is
  # bumped enough for what changed.
  cargo semver-checks "${args[@]}" 2>&1 | tee "$log"
  exit "${PIPESTATUS[0]}"
fi

git rev-parse -q --verify "$base^{commit}" >/dev/null || git fetch -q origin "${base#origin/}"
merge_base=$(git merge-base "$base" HEAD)
echo "== cargo semver-checks against $base (${merge_base:0:7})"
# The version is the same on both sides (it is bumped only in release commits), so check as if
# this were a patch release: anything that would need the next 0.x minor fails.
if cargo semver-checks "${args[@]}" --baseline-rev "$merge_base" --release-type patch 2>&1 | tee "$log"; then
  echo "no breaking API change"
  exit 0
fi
if ! grep -qE 'FAIL|requires new (major|minor) version' "$log"; then
  echo "cargo semver-checks failed before comparing (see above)" >&2
  exit 2
fi

# Breaking: the policy wants it written down in this branch.
added=$(git diff --name-only --diff-filter=AM "$merge_base"...HEAD -- 'changes/*.md')
documented=0
for f in $added; do
  case "$f" in
    *.migration.md) documented=1 ;;
    *) grep -q '^### Breaking' "$f" && documented=1 ;;
  esac
done
if [ "$documented" = 1 ]; then
  echo
  echo "Breaking change, documented in: $(tr '\n' ' ' <<<"$added")"
  echo "It ships in the next minor release (docs/VERSIONING.md); deprecate first where practical."
  exit 0
fi
cat >&2 <<'TXT'

This branch breaks the public API (above) and doesn't say so. Either:
  - keep the old item working and mark it #[deprecated] (docs/VERSIONING.md, "Deprecation"), or
  - add "### Breaking" to changes/<topic>.md and the upgrade steps to
    changes/<topic>.migration.md (changes/README.md).
TXT
exit 1
