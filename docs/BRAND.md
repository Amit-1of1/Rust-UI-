# Brand and looks

Atoll used to look like every other shadcn-style kit: black, white and grey with Inter. This page is the look it has now, why, and the four other looks that ship with it.

## The idea

An atoll is a ring of reef around a calm lagoon, with channels out to the open sea. The kit borrows three things from that picture:

- **The lagoon:** a clear teal as the one colour that means "act here" (buttons, checked boxes, focus rings), on neutrals with a hint of sea water in them instead of flat grey.
- **The reef:** a soft, sturdy edge. Corners are rounder than before (12 px), and icons are drawn a little lighter.
- **The island:** one warm coral spot, used sparingly: in the mark, and as a highlight in marketing pages, never for controls.

## The mark

[`docs/brand/atoll-mark.svg`](brand/atoll-mark.svg) (the ring alone) and `docs/brand/atoll-logo.svg` (ring and wordmark). The ring is three arcs of reef with channels between them, the lagoon is the pale disc inside, the island is the coral dot. In the docs and gallery the mark is drawn in `--accent-color`, so it takes the colour of whichever look is on.

Wordmark: "atoll" in lower case, Bricolage Grotesque Bold, tracking -0.02em.

## Colour

| Role | Light | Dark |
| --- | --- | --- |
| Lagoon (accent, focus) | `#0f766e` | `#2dd4bf` |
| Label on the lagoon | `#ffffff` | `#042f2e` |
| Page | `#faffff` | `#041111` |
| Text | `#041414` | `#c7d8d8` |
| Muted text | `#5c6f6f` | `#95a4a5` |
| Island (logo, highlights only) | `#f97362` | `#f97362` |

The neutrals were made from the old greys: each keeps the exact brightness (WCAG luminance) of the grey it replaces and only gains a little hue. Every contrast ratio the kit had, it still has; the contrast proof checks it again for every look, in light and dark.

## Type

- **Headings:** Bricolage Grotesque, a grotesque with a bit of character in the curves. `--font-heading`.
- **Text:** Figtree, friendly and very readable at small sizes. `--font-sans`.
- **Code:** JetBrains Mono. `--font-mono`.

The font files ship inside the kit (`crates/ui/assets/fonts/`, SIL Open Font License 1.1, Latin subsets, about 20 to 40 KB each). Nothing is fetched from a font service. `ThemeStyles` declares them, and a browser downloads a file only when the page uses that family. Text shows at once in the system font and switches when the file arrives (`font-display: swap`), so fonts never hold up the first screen. `ThemeStyles { fonts: false }` turns them off.

## Shape, density, motion, icons

- Corner radius `--radius: 0.75rem`. Smaller parts (menu rows, checkboxes, keys) use fractions of it, so one token rounds the whole kit.
- Density stays at 1; presets move it a little.
- Motion keeps the kit's springs (`--spring-snappy` for entrances, short plain exits) and switches off under reduced motion.
- Icons are Lucide at a 1.75 stroke (`--icon-stroke-width`).

## Five looks

A look (theme preset) changes colours, fonts, corner radius, density and icon stroke together. Lagoon is the default; the others are one attribute away.

| Look | Feel | Fonts | Corners |
| --- | --- | --- | --- |
| `lagoon` | Lagoon teal on sea-glass neutrals | Bricolage Grotesque + Figtree | 12 px |
| `reef` | Coral on warm sand, round and roomy | Nunito | 18 px |
| `abyss` | Deep-sea navy with an indigo glow, sharp and compact | Space Grotesk | 4 px |
| `driftwood` | Sun-bleached sand and terracotta, serif headings | Fraunces + Figtree | 6 px |
| `classic` | Atoll's old plain black and white | System fonts, no downloads | 8 px |

Pick one when you build the app:

```rust
rsx! { ThemeStyles { preset: "reef" } }
```

or let people switch while it runs, with the same shared state the customizer uses:

```rust
let theme = use_theme();
theme.set_preset("abyss");
```

`PresetPicker {}` is a ready-made switcher (each option drawn in its own look), and `ThemeCustomizer {}` shows it first. `PersistTheme {}` remembers the choice. Without the kit's Rust code, set `data-preset="driftwood"` on `<html>` or on any element: like `data-theme`, it switches just that part of the page.

An explicit `data-accent` or `data-density` still wins over a look's own, so "Abyss, but green" is `data-preset="abyss" data-accent="green"`.

To bake one look into your own stylesheet instead, `atoll theme --preset reef` prints its tokens as a `:root` block.

## Checked

- `cargo test -p atoll-ui --test theme`: each look's text, muted text, accent label and chart colours against its own page, in both modes; Lagoon's preset block equals the default.
- `cargo test -p atoll-cli --test contrast`: the contrast proof runs every component stylesheet under every look, as it does for every accent.
- `scripts/e2e.sh`: axe scans the gallery in every look, light and dark (Chromium).
