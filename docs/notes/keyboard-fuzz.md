# Keyboard fuzzing (native)

`apps/native/tests/keyboard_fuzz.rs` presses random keys on every demo on headless Blitz, the demo placed between two sentinel buttons (`#fuzz-before`, `#fuzz-after`). Invariants after each key: no panic, focus not lost (exists, attached, not body, not inside `display: none`), focus stays inside an open modal; at the end, three Escapes close any modal and Tab reaches `#fuzz-after`.

- Deterministic: the seed is `FUZZ_SEED` (default 0x5eed) xor a hash of the demo name. `FUZZ=<demo>` prints each sequence; `FUZZ_CASES`/`FUZZ_LEN` lengthen the run (default 3 x 20 keys per demo, to keep CI time sane).
- Failures are shrunk (drop chunks, then single keys) to the shortest sequence that breaks the same invariant.
- Known native gaps live in `KNOWN` with their minimal sequence; `known_gaps_still_reproduce` fails when one gets fixed, so remove it then.
- No proptest: Blitz runs are too slow for 256 cases per demo, and a 20-line xorshift + shrinker covers it (GUARDRAILS: no crate for under 50 lines).
