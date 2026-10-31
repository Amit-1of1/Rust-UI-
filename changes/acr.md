### Added

- **cli:** `rust-ui acr [names] [--product <name>] [--out <file>]` writes a draft Accessibility Conformance Report in the VPAT 2.5 layout (WCAG 2.2, Levels A and AA) for the kit components an app uses: each row says what the components do and which test shows it, the keyboard and focus rows list the used components' contracts and their recorded gaps, and what only the app can decide is marked *review*. The criteria live in `docs/acr/wcag22.toml`; a test fails if a cited check disappears. Experimental. (#58)
