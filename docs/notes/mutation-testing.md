# Mutation testing

A test suite can pass and still check little: a test that asserts "the speed is positive"
passes whether the code divides or multiplies. [cargo-mutants](https://mutants.rs) (MIT/Apache)
measures this. It changes the code one small edit at a time (`<` to `<=`, `-` to `+`, a body
replaced by a constant), runs the tests, and reports each change the tests didn't notice: a
**missed** mutant is behaviour nothing pins down.

## What runs

`.github/workflows/mutants.yml` runs it on `crates/motion/src/physics.rs` and `drag.rs` (the
spring, glide, pointer-speed, rubber-band and drag maths) on every PR that touches them and
weekly. The job fails on any missed mutant (timeouts count as caught).

```bash
cargo install cargo-mutants --locked --version 27.1.0
cargo mutants -p atoll-motion -f crates/motion/src/physics.rs -f crates/motion/src/drag.rs --timeout 10 -j 2
```

About 510 mutants, 15 to 25 minutes (18 on a 4-core machine with -j 2). A mutant that makes a spring never settle hangs its test;
`--timeout 10` counts that as caught.

## Results

The first run (2026-11-15) missed 65 of 516. Most survived because the tests checked direction
or sign, or used values at the origin where `x - 0` and `x + 0` agree. The exact-value tests in
`crates/motion/tests/mutants.rs` kill all of them except the equivalent ones, which no caller can
observe. Those are listed with the reason in `.cargo/mutants.toml`.

It found one real mismatch: `VelocityTracker::velocity` was documented as "zero if the pointer
has been still for 100 ms" but returned a speed at exactly 100 ms. It now returns zero, as
documented.

## Not covered yet

- **The kit's components.** `crates/ui/build.rs` generates the `mod` lines for components, and
  cargo-mutants finds files by following `mod` declarations, so it sees none of them. The
  researcher ran the pure parts of pagination and the command palette on a copy: the command
  palette's model checker and property tests killed every mutant; pagination's gaps are closed
  by the exhaustive tests in `tests/components/pagination.rs`.
- **atoll-cli's contrast code** (663 mutants) is the next candidate.

**Gotcha.** Don't run cargo-mutants with `CARGO_TARGET_DIR` pointing at a target directory you
share with normal builds: a mutated build of the crate can be left there and a later
`cargo test` runs it (a velocity test returned NaN until the source was touched).
