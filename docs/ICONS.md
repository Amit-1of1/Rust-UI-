# Icons

**Decision (2026-10-13): Lucide, through `dioxus-icons` 0.1.0, wrapped by the `icons` registry item.** Apps import `ui::icons::Check`; components import `crate::components::icons::Check`. Nothing outside `crates/ui/src/components/icons` names `dioxus_icons`.

## Options compared

| | `dioxus-icons` (chosen) | Rust/UI `crates/icons` | Copy Lucide SVGs into the repo |
|---|---|---|---|
| Set | Lucide 1.14.0, 1,705 icons | Lucide-based, plus animated variants | Whatever we copy |
| Maintainer | DioxusLabs org | Rust/UI (one company) | Us |
| License | MIT, icon data ISC | MIT | ISC (Lucide) |
| Bundle | One component per icon; unused icons are dropped by the linker | One `DioxusIcon` that looks icons up in a shared table | Only what we copy |
| Extra deps | none (just `dioxus`) | `strum` 0.28 (we use 0.27), `tw_merge`, edition 2024 | none |
| Markup | Bare `<svg>`, `currentColor`, any SVG attribute passes through | Every icon carries `<title>Rust/UI Icons - Name</title>`, which screen readers read on decorative icons | Ours |
| Already used | Yes, by 14 components and by DioxusLabs' own gallery | No | No |

Rust/UI's set loses on the accessibility point alone (a title on every decorative icon), and it brings Tailwind merging we don't use. Copying SVGs by hand would mean maintaining a generator; `dioxus-icons` already is one.

## Why a wrapper item

- **One place to swap.** GUARDRAILS asks us to wrap outside dependencies. If `dioxus-icons` stalls, changing `components/icons/component.rs` is the whole migration.
- **The registry knows about it.** `rust-ui add select` now brings the `icons` item along (it is a registry dependency, detected from the import), so copied components compile without editing imports.
- **No glob in the prelude.** Lucide has icons named `Link`, `Badge`, `Calendar`, `Component` and more, so `ui::prelude::*` does not include icons. Import them by path.

## Rules

- Decorative icons get `"aria-hidden": "true"` (every icon inside the kit's components now does).
- An icon-only control is named on the control (`aria-label` or visually hidden text), never on the SVG.
- Icons draw with `currentColor`; colour them through the text colour or a token (`stroke: "var(--...)"`), never a literal.
- Size with `size` (`16`, `"1rem"`, `"0.875em"`), so icons follow the text size where that matters.
