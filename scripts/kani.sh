#!/usr/bin/env bash
# Kani proofs (bounded model checking of the real Rust code; docs/notes/kani.md).
#   scripts/kani.sh            every harness in crates/ui/src/kani_proofs.rs
#   scripts/kani.sh <harness>  one of them
# Installs Kani (pinned) if `cargo kani` is missing. Harnesses run one at a time: CBMC can need
# several GB for a harness that walks a Vec.
set -euo pipefail
cd "$(dirname "$0")/.."

VERSION=0.68.0
have=$(cargo kani --version 2>/dev/null || true)
if [[ "$have" != *"$VERSION"* ]]; then
  cargo install --locked kani-verifier --version "$VERSION"
  cargo kani setup
fi

harnesses=("$@")
if [ ${#harnesses[@]} -eq 0 ]; then
  mapfile -t harnesses < <(grep -A3 '#\[kani::proof\]' crates/ui/src/kani_proofs.rs \
    | sed -n 's/^fn \([a-z0-9_]*\)().*/\1/p')
fi
[ ${#harnesses[@]} -gt 0 ] || { echo "no harnesses found" >&2; exit 1; }

failed=0
for h in "${harnesses[@]}"; do
  echo "== $h"
  start=$SECONDS
  if out=$(cargo kani -p atoll-ui --harness "kani_proofs::$h" --exact 2>&1) \
     && grep -q "VERIFICATION:- SUCCESSFUL" <<<"$out"; then
    echo "   verified in $((SECONDS - start)) s"
  else
    grep -E "Failed Checks|Description|Location|VERIFICATION|error" <<<"$out" | head -40
    echo "FAIL: $h" >&2
    failed=1
  fi
done
exit $failed
