### Added

- **motion:** `VelocityTracker::len` and `is_empty`.
- Mutation testing in CI (`.github/workflows/mutants.yml`): cargo-mutants on the motion physics and drag code; every mutant must be caught (docs/notes/mutation-testing.md).

### Fixed

- **motion:** `VelocityTracker::velocity` returns zero once the pointer has been still for 100 ms, as documented (it returned a speed at exactly 100 ms).
