### Added

- **ui:** `ui::icons::atoll`, 101 original Atoll icons drawn for the stack (crate, wasm module, desktop/mobile/web app, native window, terminal, build, hot reload, agent, tool call, prompt, permission, charts, component, block, spring motion, focus) and for everyday system actions, as components (`atoll::Crate`) and shape data (`atoll::CRATE`) (experimental).
- **ui:** `atoll::INDEX` and `atoll::meta(name)`: every Atoll icon's default accessible name, keywords, category and reserved meaning, as `IconMeta` (strings only, so naming it keeps no icon shapes in the binary) (experimental).
- **ui:** `IconNode::Fill`, a solid path filled with `currentColor` (experimental).
- **docs:** searchable Atoll icon browser on the Icons page of the gallery and docs site; icon drawing and usage rules (grid, keylines, stroke, naming, keywords, reserved icons) in `docs/ICONS.md`.
- **tools:** `scripts/gen-atoll-icons.mjs` turns the SVGs in `crates/ui/icons/atoll/` into `atoll.rs` and rejects shapes outside the safe area, missing names or keywords, groups and transforms.
