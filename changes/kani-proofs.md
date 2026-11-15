### Added

- **ui:** Kani proofs of the real code (`crates/ui/src/kani_proofs.rs`, `scripts/kani.sh`, a "Kani" CI workflow): the chart cursor and the command palette's keys are proved for every size, not just up to the model checker's bound.

### Fixed

- **ui:** `page_items` (pagination) overflowed for a huge `siblings`; its output is now also proved well-formed for every case up to 60 pages.
