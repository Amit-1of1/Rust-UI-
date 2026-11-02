#!/usr/bin/env bash
# Model-check the TLA+ specs in proofs/tla/ with TLC (docs/notes/tla-models.md).
#   scripts/tla.sh            check every spec
#   scripts/tla.sh MotionValue   check one
# For each spec `X.tla`:
#   - `X.cfg` is the real code: every invariant and property must hold.
#   - `X_bug_*.cfg` switches on a deliberate mistake: TLC must report the property named in
#     the cfg's "expects <Name> to fail" comment. This shows the checks can catch the bug.
# Needs Java 11+. TLC (tla2tools.jar, MIT) is downloaded once to target/tla/, pinned by version
# and checksum; set TLA2TOOLS=/path/to/tla2tools.jar to use a local copy.
set -euo pipefail
cd "$(dirname "$0")/.."

VERSION=v1.7.4
SHA256=936a262061c914694dfd669a543be24573c45d5aa0ff20a8b96b23d01e050e88
JAR="${TLA2TOOLS:-target/tla/tla2tools-$VERSION.jar}"

if [ ! -f "$JAR" ]; then
  mkdir -p "$(dirname "$JAR")"
  curl -fsSL -o "$JAR.part" \
    "https://github.com/tlaplus/tlaplus/releases/download/$VERSION/tla2tools.jar"
  mv "$JAR.part" "$JAR"
fi
if [ -z "${TLA2TOOLS:-}" ]; then
  echo "$SHA256  $JAR" | sha256sum -c --quiet - || { rm -f "$JAR"; exit 1; }
fi

root=$PWD
case "$JAR" in /*) ;; *) JAR="$root/$JAR" ;; esac
tlc() { # cfg spec: TLC's output goes to $out, its exit status to $status
  status=0
  out=$(cd proofs/tla && java -XX:+UseParallelGC -cp "$JAR" tlc2.TLC \
    -workers auto -deadlock -metadir "$root/target/tla/states" -cleanup \
    -config "$1" "$2" 2>&1) || status=$?
}

specs=("$@")
if [ ${#specs[@]} -eq 0 ]; then
  for f in proofs/tla/*.tla; do specs+=("$(basename "$f" .tla)"); done
fi

failed=0
for spec in "${specs[@]}"; do
  echo "== $spec"
  tlc "$spec.cfg" "$spec.tla"
  if [ "$status" -eq 0 ] && grep -q "No error has been found" <<<"$out"; then
    grep -E "distinct states found" <<<"$out" | tail -1 | sed 's/^/   passes: /'
  else
    printf '%s\n' "$out"
    echo "FAIL: $spec.cfg should pass" >&2
    failed=1
  fi
  for cfg in proofs/tla/"$spec"_bug_*.cfg; do
    [ -e "$cfg" ] || continue
    name=$(basename "$cfg")
    expect=$(sed -n 's/.*expects \([A-Za-z]*\) to fail.*/\1/p' "$cfg" | head -1)
    if [ -z "$expect" ]; then
      echo "FAIL: $name has no 'expects <Name> to fail' comment" >&2
      failed=1
      continue
    fi
    tlc "$name" "$spec.tla"
    if grep -qE "(Invariant|Temporal property|Action property) $expect .*violated|$expect is violated" <<<"$out"; then
      echo "   $name: $expect fails, as it should"
    else
      printf '%s\n' "$out"
      echo "FAIL: $name should make $expect fail" >&2
      failed=1
    fi
  done
done
exit $failed
