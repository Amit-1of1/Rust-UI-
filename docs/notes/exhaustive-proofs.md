# Exhaustive proofs

When a function's input is a finite set small enough to walk (all sRGB colours, all `f32`s, all
dates), checking every input is a proof, not a test: there is no case left to miss. These run as
`#[ignore]`d tests in release, from `scripts/proofs.sh`, in the "Proofs" workflow
(`.github/workflows/proofs.yml`), because a debug build would take far too long.

| Proof | Domain | Release time (4 cores) |
|---|---|---|
| `atoll theme --brand` gives text on fill of at least 4.5:1 in light and dark mode, judged by the contrast proof's `ratio`, not the generator's own `contrast` | all 16,777,216 brand colours | about 4 min (the generator steps lightness in 0.5% steps) |
| the generator's `theme::contrast` equals `contrast::ratio` against white and `#0a0a0a` | all 16,777,216 colours | 3 s |
| `Rgb::from_hsl(c.hsl()) == c` | all 16,777,216 colours | 2 s |

What makes them airtight:

- **An outside judge.** The first version of the generator's test checked its output with its own
  `contrast()`, so a wrong WCAG formula agreed with itself: 14 of 153 mutants of `theme.rs`
  survived (`0.2126 * c` → `0.2126 + c`, `/ 12.92` → `* 12.92`, a missing hue sextant in
  `from_hsl`). `theme_proof.rs` now pins the formula to published WCAG values (21:1, #777777 on
  white 4.48:1, #767676 4.54:1, the primaries) and checks a colour in every hue sextant, which runs
  in the normal `cargo test`.
- **A test that matches nothing fails.** `scripts/proofs.sh` passes `--exact` and requires
  "1 passed", so renaming a proof can't silently stop it running.

Adding one: write the test with `#[ignore]` and a doc comment saying what it proves, add a line
to `proofs` in `scripts/proofs.sh`, and a row here.
