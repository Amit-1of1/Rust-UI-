### Added

- **cli:** The contrast proof's every pair (5,817 lines over 72 components) is recorded in `crates/cli/tests/contrast_pairs/<component>.txt`, so a theme, stylesheet or markup change shows its effect on every colour pair as a diff, and a bug in the proof that drops pairs or shifts a ratio is caught. Update with `UPDATE_CONTRAST_PAIRS=1 cargo test -p atoll-cli --test contrast`; `scripts/sync-main.sh` regenerates them after a merge.
