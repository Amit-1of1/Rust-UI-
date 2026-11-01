# Icons

**Decision (2026-10-14): every Lucide icon, generated into the kit by `scripts/gen-lucide-icons.mjs`.** This replaces the 2026-10-13 decision below (Lucide through `dioxus-icons` 0.1.0). Apps import `ui::icons::Check`; components import `crate::components::icons::Check`. Nothing depends on an icon crate.

## Atoll icons (2026-11-01)

**Decision: the kit draws its own icons for what Lucide doesn't cover well (the Rust and app stack: crates, wasm, native windows, agents, tool calls, motion, focus) plus the everyday system actions, in one style that matches the kit's design system (Reshaped's foundations: Feather-style outline icons).** They live next to Lucide in `ui::icons::atoll` and use the same `IconData`, renderer, sizes, stroke token and RTL rule, so the two sets mix in one row. Style: soft outline, chosen from a three-way sample sheet (soft line / two-tone / crisp) and then matched to Reshaped, which the owner picked as the design system: Reshaped's own components use Feather icons (24px box, 2px stroke, round ends, 2px corners) and size icons in 4px units.

### Drawing rules

- **Grid:** 24 x 24 viewBox, the same box as Lucide, so icons pair with text and controls at `--icon-size-*` (12, 16, 20, 24, 32px).
- **Safe area and keylines:** shapes stay inside 1..23 (the generator rejects circles, rects, lines and points outside it), and the live area is 2..22. A full square is 18 x 18 at (3, 3), a full circle `r = 9` at the centre, portrait shapes 12-16 wide x 20 tall, landscape ones 20 wide x 13-16 tall. Leave visual weight even: a circle reaches 1px further than a square.
- **Stroke:** drawn at 2 with round ends and joins; the rendered width is the theme's `--icon-stroke-width`, so a preset can thin or thicken every icon at once. No shape relies on a hairline: gaps between strokes are at least 2px, so icons survive at 16px.
- **Corners:** outer corners of boxes round by 2px (3px for tall devices, 1px for small tiles); interior details stay sharp.
- **Sizes:** on the 4px unit scale (12, 16, 20, 24, 32px, the `--icon-size-*` tokens). Use `size: "1em"` to follow the text size, as Reshaped's icons do by default.
- **Fills:** outline style. Solid fill (`fill="currentColor" stroke="none"`) only for dots (status dots, the "more" ellipsis, eyes) and for icons whose meaning is half-filled (`theme`). Colour always comes from `currentColor`.
- **No text, no logos.** No letters except where the meaning is a glyph (`typography`); no third-party marks (the WebAssembly logo, Rust's gear, Git's logo). `wasm-module` and `branch` are our own metaphors.
- **Flat SVG:** no groups, transforms, styles, titles or masks; the generator rejects them so the file stays a list of shapes.

### Naming, keywords and accessibility (from the design-system checklist)

- **Name by purpose, not shape:** `delete` not `trash`, `next` not `arrow-right`, `notification` not `bell`. A test rejects shape words (arrow, chevron, circle, triangle...) in names. The shape words go in the keywords.
- **Keywords:** at least two per icon in `icons.json`; the docs browser and rustdoc search them.
- **Default accessible name:** every icon has a `label` in `icons.json` (`atoll::meta(name).label`). Icons stay decorative by default; when an icon stands alone and means something, pass that label (or a better one in context).
- **Reserved icons:** system actions (`add`, `remove`, `close`, `delete`, `edit`, `copy`, `search`, `settings`, `menu`, `more`, `filter`, `sort`, `refresh`, `download`, `upload`, `share`, `show`, `hide`, `back`, `next`, `expand`, `collapse`, `home`, `sign-in`, `sign-out`, the five status icons, `light-mode`, `dark-mode`, `sparkles`, `permission`, `atoll`) carry a `reserved` meaning, and no two icons share one. Don't use a reserved icon for anything else, and don't draw a second icon for a reserved action.
- **Categories:** system, navigation, status, content, people, dev, app, agent, data, design, brand (`atoll::CATEGORIES`).

### Adding an icon

1. Draw `crates/ui/icons/atoll/<name>.svg` on the grid above (copy the root `<svg>` line from any existing file).
2. Add its entry to `crates/ui/icons/atoll/icons.json`: `label`, `category`, `keywords`, and `reserved` / `mirror_rtl` when they apply.
3. Run `node scripts/gen-atoll-icons.mjs` (needs `rustfmt`), then `cargo test -p atoll-ui --test atoll_icons`. The test fails when an SVG and `atoll.rs` disagree.
4. Check it in the gallery's Icons section at 16 and 24px, in light and dark.

### Don't

- Don't hand-edit `atoll.rs`.
- Don't use an icon as the only cue for state or meaning; pair it with text or give it a name.
- Don't mix sizes in one row, and don't scale icons to sizes that aren't on the token scale.
- Don't recolour parts of an icon; tint the whole icon through `color`.

## How it works

- `scripts/gen-lucide-icons.mjs` downloads the pinned `lucide-static` release with `npm pack` (which checks the registry's integrity hash), reads `icon-nodes.json` (each icon's shapes), `tags.json` (search words, put in the rustdoc) and the alias exports (old names), and writes `crates/ui/src/components/icons/lucide.rs` through `rustfmt`. Run it again with `--version` to upgrade; `--from` takes an unpacked or packed package for offline use.
- `lucide.rs` has one constant per icon (`lucide::CHECK: IconData`, the shapes as `IconNode` values with Lucide's own numbers) and one component per icon (`components::Check`, re-exported as `ui::icons::Check`). Old names are `pub use` aliases.
- `Icon` (in `component.rs`) draws any `IconData`; every generated component calls the same function, so the per-icon code is a few bytes.
- **Binary size:** each constant and component is a separate item, so the linker keeps only the icons an app names (checked on the gallery's wasm build: icons it doesn't use are not in it). `lucide::ALL` names every icon and keeps all of them, so only tests use it.
- The generator fails on any SVG element or attribute it doesn't know, so a Lucide release that needs a new shape type can't be converted silently wrong.

## Rules

- **Decorative by default.** An icon without a `label` is `aria-hidden="true"`; with one it is `role="img"` with that name. An icon-only control is named on the control (`aria-label` or `sr-only` text), never on the SVG.
- **Theme sizes.** `IconSize::Xs` … `Xl` read `--icon-size-*` (default `Lg`, 24px). Use a number or CSS length (`"1em"`) for a fixed box or to follow the text. The size rules use `:where()`, so a component's own icon CSS wins.
- **Stroke.** `--icon-stroke-width` (2) unless the caller passes `stroke_width`.
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
