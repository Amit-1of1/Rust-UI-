# Icons

**Decision (2026-10-14): every Lucide icon, generated into the kit by `scripts/gen-lucide-icons.mjs`.** This replaces the 2026-10-13 decision below (Lucide through `dioxus-icons` 0.1.0). Apps import `ui::icons::Check`; components import `crate::components::icons::Check`. Nothing depends on an icon crate.

## How it works

- `scripts/gen-lucide-icons.mjs` downloads the pinned `lucide-static` release with `npm pack` (which checks the registry's integrity hash), reads `icon-nodes.json` (each icon's shapes), `tags.json` (search words, put in the rustdoc) and the alias exports (old names), and writes `crates/ui/src/components/icons/lucide.rs` through `rustfmt`. Run it again with `--version` to upgrade; `--from` takes an unpacked or packed package for offline use.
- `lucide.rs` has one constant per icon (`lucide::CHECK: IconData`, the shapes as `IconNode` values with Lucide's own numbers) and one component per icon (`components::Check`, re-exported as `ui::icons::Check`). Old names are `pub use` aliases.
- `Icon` (in `component.rs`) draws any `IconData`; every generated component calls the same function, so the per-icon code is a few bytes.
- **Binary size:** each constant and component is a separate item, so the linker keeps only the icons an app names (checked on the gallery's wasm build: icons it doesn't use are not in it). `lucide::ALL` names every icon and keeps all of them, so only tests use it.
- The generator fails on any SVG element or attribute it doesn't know, so a Lucide release that needs a new shape type can't be converted silently wrong.

## Rules

- **Decorative by default.** An icon without a `label` is `aria-hidden="true"`; with one it is `role="img"` with that name. An icon-only control is named on the control (`aria-label` or `sr-only` text), never on the SVG.
- **Theme sizes.** `IconSize::Xs` … `Xl` read `--icon-size-*` (default `Lg`, 24px). Use a number or CSS length (`"1em"`) for a fixed box or to follow the text. The size rules use `:where()`, so a component's own icon CSS wins.
- **Stroke.** `--icon-stroke-width` (set by the look: 1.75 in Lagoon) unless the caller passes `stroke_width`.
- **Finding one.** `atoll icons <words>` searches names and Lucide's search words (`atoll icons delete`).
- **Reserved icons.** The kit uses some icons for one meaning (close, select, open a list, loading, status...); `atoll icons --reserved` and docs/FOUNDATIONS.md list them. Use them only for that meaning.
- **Colour.** Icons draw with `currentColor`; colour them through the text colour or a token, never a literal.
- **Right-to-left.** Icons that point along the reading direction mirror inside `dir="rtl"` (`scale: -1 1`, so a component's own `transform` still applies): left/right arrows, chevrons, corners and moves, panel toggles, undo/redo, reply/forward, log in/out, indent, text wrap. Vertical arrows, media controls and clocks don't. The list is `mirrorsInRtl` in the generator; `mirror_rtl` overrides it per icon.
- **No sprite.** Icons stay inline SVG: that works on Blitz and in SSR with no extra file, and sprites only pay off with hundreds of repeated icons on one page.

## History: the 2026-10-13 comparison

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

### Why a wrapper item

- **One place to swap.** GUARDRAILS asks us to wrap outside dependencies. If `dioxus-icons` stalls, changing `components/icons/component.rs` is the whole migration.
- **The registry knows about it.** `atoll add select` now brings the `icons` item along (it is a registry dependency, detected from the import), so copied components compile without editing imports.
- **No glob in the prelude.** Lucide has icons named `Link`, `Badge`, `Calendar`, `Component` and more, so `ui::prelude::*` does not include icons. Import them by path.

### Rules then

- Decorative icons get `"aria-hidden": "true"` (every icon inside the kit's components now does).
- An icon-only control is named on the control (`aria-label` or visually hidden text), never on the SVG.
- Icons draw with `currentColor`; colour them through the text colour or a token (`stroke: "var(--...)"`), never a literal.
- Size with `size` (`16`, `"1rem"`, `"0.875em"`), so icons follow the text size where that matters.
