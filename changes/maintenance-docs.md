### Added

- **docs:** every component page on the docs site has a generated **Properties** table per part (prop, type, default or required, description), read from the source by `atoll registry` (`crates/cli/src/props.rs`). Parts that take a primitive's props name that struct.
- **docs:** maintenance guides from the design-system maintenance checklist: `docs/PRINCIPLES.md`, `docs/BEST_PRACTICES.md`, `docs/ANATOMY.md`, `docs/COMPOSITION.md` (five compositions compiled as doctests), `docs/SUPPORT_MATRIX.md` (browser and OS tiers), `docs/LOCAL_LIBRARIES.md`, `docs/ROADMAP.md`, `docs/MAINTAINING.md` (stakeholders, service levels, support time, analytics without telemetry, monthly updates) and `docs/MAINTENANCE_CHECKLIST.md` (the scorecard). The docs site lists them as guides; `llms.txt` links the composition and best-practices guides.
- **repo:** `CONTRIBUTING.md`, `SECURITY.md`, issue forms for bug reports, feature requests and component proposals, a pull request template, and a decision-record template with an index of `docs/decisions/`.
- **cli:** `atoll new` writes a `## Component Structure` section into the new component's `docs.md`; `crates/cli/tests/maintenance.rs` checks the maintenance docs' links and that the list of components without one only shrinks.

### Fixed

- **docs:** links between guides on the docs site went nowhere (they pointed at `.md` files); they now open the guide's page, or the file on GitHub when it isn't a guide.
