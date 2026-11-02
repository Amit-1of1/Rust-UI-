### Added

- **motion:** a TLA+ model of `MotionValue` (`proofs/tla/MotionValue.tla`), checked in CI by
  `scripts/tla.sh`: one loop per value, no frozen animation, `is_animating()` tells the truth,
  every animation settles at the last target, nothing written after unmount. Two deliberate
  mutants must fail, so the checks are shown to catch real mistakes.
