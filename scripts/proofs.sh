#!/usr/bin/env bash
# Exhaustive proofs that are too slow for a debug `cargo test`: each is an `#[ignore]`d test
# that checks every value of a finite domain (all 16.7M colours, all 2^32 floats), run here in
# release. CI runs this in the "Proofs" workflow. docs/notes/exhaustive-proofs.md says what each
# one proves.
#   scripts/proofs.sh          run them all
#   scripts/proofs.sh theme    only those whose name contains "theme"
set -euo pipefail
cd "$(dirname "$0")/.."

# "<package> <test target> <test name filter>" per line. A filter that matches no test (the
# test hasn't merged yet) is an error, so a renamed proof can't silently stop running.
proofs=(
  "atoll-cli theme_proof every_one_of_16_7_million_brand_colours_gets_a_readable_preset"
  "atoll-cli theme_proof generator_and_proof_agree_on_every_colour"
  "atoll-cli theme_proof every_colour_survives_the_hsl_round_trip"
)

failed=0
for p in "${proofs[@]}"; do
  read -r pkg target name <<<"$p"
  case "$name" in *"${1:-}"*) ;; *) continue ;; esac
  echo "== $pkg --test $target $name"
  start=$SECONDS
  out=$(cargo test --release -q -p "$pkg" --test "$target" "$name" -- --ignored --exact 2>&1) || {
    printf '%s\n' "$out"
    echo "FAIL: $name" >&2
    failed=1
    continue
  }
  if ! grep -q "test result: ok. 1 passed" <<<"$out"; then
    printf '%s\n' "$out"
    echo "FAIL: $name matched no test (renamed?)" >&2
    failed=1
    continue
  fi
  echo "   proved in $((SECONDS - start)) s"
done
exit $failed
