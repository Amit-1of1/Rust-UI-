### Changed

- **motion:** `use_reduced_motion` reads and watches the OS setting once per app instead of once per component, and is now live: turning "reduce motion" on while the app is open applies to the next animation (`MotionValue`, `use_spring`) and re-renders `Pop`. (#66)
- **ui:** `DataTable` filters and sorts only when its rows, query, sort or page change (not on a selection click), skips the filter pass for an empty query, and sorts with precomputed keys instead of lower-casing or parsing cells in every comparison. Order is unchanged (property test against the old definition). (#66)

### Fixed

- **ui:** The command palette's input shows keyboard focus (its bottom rule takes the focus colour). (#66)
