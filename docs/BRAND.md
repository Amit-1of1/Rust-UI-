# Brand and looks

Atoll's look has two layers. Underneath are the foundations, taken from Reshaped, a well-tried design system (MIT): neutral greys, Inter, 4 px spacing, a type scale, soft layered shadows, calm motion ([Foundations](FOUNDATIONS.md)). On top is what makes it Atoll: the lagoon-teal accent, the mark, and five looks that switch with one attribute. This page is the top layer.

## The idea

An atoll is a ring of reef around a calm lagoon, with channels out to the open sea. The kit borrows three things from that picture:

- **The lagoon:** a clear teal as the one colour that means "act here" (buttons, checked boxes, focus rings), on calm neutral greys.
- **The reef:** a soft, sturdy edge: 10 px corners on cards and dialogs, 6 px on controls, 4 px on small parts, and icons drawn a little lighter.
- **The island:** one warm coral spot, used sparingly: in the mark, and as a highlight in marketing pages, never for controls.

## The mark

[`docs/brand/atoll-mark.svg`](brand/atoll-mark.svg) (the ring alone) and `docs/brand/atoll-logo.svg` (ring and wordmark). The ring is three arcs of reef with channels between them, the lagoon is the pale disc inside, the island is the coral dot. In the docs and gallery the mark is drawn in `--accent-color`, so it takes the colour of whichever look is on.

Wordmark: "atoll" in lower case, Inter Semibold (650), tracking -0.015em.

## Colour

| Role | Light | Dark |
| --- | --- | --- |
| Lagoon (buttons, selected) | `#0b7f7f` | `#1b7e7e` |
| Label on the lagoon | `#ffffff` | `#ffffff` |
| Lagoon as text (links) | `#0b7f7f` | `#83c5c5` |
| Focus ring | `#0b7f7f` | `#6cb3b2` |
| Danger | `#c8433c` | `#c14b43` |
| Page | `#ffffff` | `#0c0b09` |
| Text | `#211f1b` | `#f0eeeb` |
| Muted text | `#66635e` | `#a09e9b` |
| Island (logo, highlights only) | `#f97362` | `#f97362` |

How the colours are chosen (the same rules for every look and accent):

- **Built in OKLCH**, a colour space where equal lightness numbers look equally bright. The accent, the danger red and the status colours sit at one lightness, so none of them shouts over the others.
- **One accent, one job:** the main action and "selected". Red means danger, green success, amber warning, and nothing else.
- **Greys lean a hair towards the look's temperature** (warm sand for Lagoon, rosy for Reef, cool slate for Abyss), at very low chroma, so the page and the accent feel like one family. They sit on Reshaped's lightness steps.
- **Dark mode is not inverted.** Coloured fills keep their colour and their white label; only the page and greys flip. Where the accent is text on a dark page (links, selected labels) a lighter tone of it is used, `--accent-text`.
- **Readable everywhere:** body text about 16:1, muted text at least 5.9:1, labels on coloured fills at least 4.8:1. The contrast proof checks every pairing for every look and accent, in light and dark.
- **Chart colours** are spaced around the colour wheel at one lightness, starting from the accent, and also differ by line style for colour-blind readers.

## Type

- **Headings and text:** Inter, the typeface Reshaped and much of the web's best UI is set in: very readable at small sizes, with a full range of weights. Headings use weight 650 and slightly tighter tracking. `--font-sans`, `--font-heading`.
- **Code:** JetBrains Mono. `--font-mono`.
- Sizes come from the type scale ([Foundations](FOUNDATIONS.md)).

The font files ship inside the kit (`crates/ui/assets/fonts/`, SIL Open Font License 1.1, Latin subsets of the variable fonts, 30 to 50 KB each). Nothing is fetched from a font service. `ThemeStyles` declares them, and a browser downloads a file only when the page uses that family. Text shows at once in the system font and switches when the file arrives (`font-display: swap`), so fonts never hold up the first screen. `ThemeStyles { fonts: false }` turns them off.

## Shape, density, motion, icons

- Corner radius `--radius: 0.625rem` (10 px). `--radius-large` is it, `--radius-medium` (6 px, controls) and `--radius-small` (4 px, small parts) follow from it, so one token rounds the whole kit.
- Density stays at 1; presets move it a little.
- Motion uses the duration and easing tokens, keeps the kit's springs for entrances, and switches off under reduced motion.
- Icons are Lucide and the Atoll set at a 2 px stroke (`--icon-stroke-width`).

## Five looks

A look (theme preset) changes colours, fonts, corner radius, density and icon stroke together, always on the same foundations. Lagoon is the default; the others are one attribute away.

| Look | Feel | Fonts | Corners |
| --- | --- | --- | --- |
| `lagoon` | Deep lagoon teal on warm greys | Inter | 10 px |
| `reef` | Coral pink on rosy greys, round and a little roomier | Geist | 16 px |
| `abyss` | Indigo on cool slate, square and compact | IBM Plex Sans | 4 px |
| `driftwood` | Weathered bronze on warm sand, serif headings | Source Serif 4 + Inter | 6 px |
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
