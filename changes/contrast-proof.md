### Added

- **cli:** Contrast proof: `crates/cli/tests/contrast.rs` (in `cargo test`) checks every text colour, icon colour and focus ring each component stylesheet draws, on the background it actually sits on (same element, selector ancestors, or where the SSR snapshots render it), in light and dark and under every accent preset: 4.5:1 for text, 3:1 for large text, icons and focus indicators. Hover, selected, checked and invalid states count; disabled ones are exempt. `rust-ui contrast [--theme my-theme.css]` runs the same proof on your own theme or `rust-ui theme --brand` preset (experimental).
- **ui:** `--success-text-color` and `--warning-text-color` tokens: success and warning as readable text on their tinted fills.
- **native:** Keyboard fuzzing on headless Blitz (`apps/native/tests/keyboard_fuzz.rs`): random key sequences (base keys plus each component's contract keys) on every demo; no panic, focus never lost, focus stays in an open modal, and no keyboard trap. Failing sequences are shrunk and printed with their seed.

### Fixed

- **ui:** Contrast failures the proof found: toast success (2.4:1) and warning (2.1:1) text in light mode, toast info text in dark mode (1.4:1); white text on the hovered error action in alert dialog and popover (3.8:1); primary (green, orange accents) and destructive button hover fills that faded under AA; the focus ring of an invalid button (a 20% red halo was the only cue), switch, colour picker button and sidebar items (1.9 to 2.9:1); selected tag text on the focus colour; a selected outside-month day in the calendar; drag handle and remove icons in drag-and-drop list and tag group (1.5:1); the sheet close button (2.2:1).
- **ui:** On the web, focus goes back to the trigger when a dropdown menu, select or menubar menu closes from the keyboard (Escape, or Enter/Space on an item); select used to drop it to the page. Contracts check it; on native it is still a gap (listed in the contracts and the fuzz test).
- **ui:** `Pager` keeps focus on the page button you pressed; its items were keyed by position too, so the pressed button was replaced and focus fell to the page.
