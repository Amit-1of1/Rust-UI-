# Foundations

Atoll's foundations follow [Reshaped](https://reshaped.so) (MIT): a small set of named tokens for colour, layout, type, elevation and motion, all CSS custom properties in `crates/ui/assets/theme.css`. Components use the tokens, never literal values, and `cargo test -p atoll-ui --test foundations` keeps it that way. Atoll's own accent, mark and looks sit on top ([Brand and looks](BRAND.md)).

The checklist this page answers is the Foundations list from [designsystemchecklist.com](https://www.designsystemchecklist.com/category/foundations): colour, layout, typography, elevation, motion and iconography.

## Colour

**Semantic names.** Write CSS against what a colour is for, not what it looks like. The names follow Reshaped: `--color-` + `background`, `foreground` or `border` + a role + an optional variant.

| Token | Use |
| --- | --- |
| `--color-background-page`, `-page-faded` | The page; a sunken strip of it |
| `--color-background-elevation-base`, `-raised`, `-overlay` | Cards and panels; things that lift; menus, popovers, dialogs (see Elevation) |
| `--color-background-neutral`, `-neutral-faded`, `-neutral-highlighted` | Quiet fills: secondary buttons, hovered rows, selected rows |
| `--color-background-primary`, `-primary-faded` | The accent: primary buttons, checked controls; a tint of it |
| `--color-background-critical`, `-critical-faded`, `-positive-faded`, `-warning-faded` | Status fills |
| `--color-background-disabled` | Disabled controls |
| `--color-backdrop` | The scrim behind modals |
| `--color-foreground-neutral`, `-neutral-faded` | Body text; secondary text (hints, captions) |
| `--color-foreground-primary`, `-critical`, `-positive`, `-warning` | Text in a role colour |
| `--color-foreground-disabled` | Disabled text |
| `--color-on-background-primary` | Text and icons on the accent |
| `--color-border-neutral`, `-neutral-faded`, `-neutral-highlighted` | Dividers and outlines; faint ones; input edges |
| `--color-border-primary`, `-critical`, `-focus` | Selected, invalid and focused outlines |

The names point at the older numbered tokens (`--primary-color-*` for surfaces, `--secondary-color-*` for text), which stay the source each look sets and keep working.

**Neutrals.** Each look has 15 grey steps on Reshaped's lightness ladder (page at 100 % / 16 % lightness, body text at 24 % / 95 %, secondary text at 51 % / 74 %). Lagoon's are pure grey; Reef and Driftwood warm them slightly, Abyss cools them.

**Accessibility.** Every text colour is checked against every background it appears on, in every look and accent, light and dark (`cargo test -p atoll-cli --test contrast`, at least 4.5:1 for text and 3:1 for focus rings and chart series). axe scans the gallery in each look in light and dark. Colour never carries meaning alone: errors also have text, status badges have words.

**Dark mode.** Every colour is a `light-dark(light, dark)` pair: the page follows the OS, `data-theme="light|dark"` forces it on `<html>` or on any element (a dark panel in a light page). Dark surfaces get lighter as they rise, because shadows show less on dark.

**Guidelines.**

- One accent per screen. Use `primary` for the single most important action and for selected states, not for decoration.
- Status colours (critical, positive, warning) are for status only.
- Body text in `foreground-neutral`, everything secondary in `foreground-neutral-faded`. Don't invent a third grey for text.
- A new colour belongs in a look (`[data-preset]`) or an accent (`[data-accent]`), as a `light-dark()` pair, and goes through the contrast proof (`atoll contrast --theme yours.css`).

## Layout

**Units.** Everything sits on a 4 px grid. `--unit-x1` is 4 px, `--unit-x2` 8 px, up to `--unit-x10` (40 px), then `--unit-x12`, `-x16`, `-x20`, `-x24` (48 to 96 px) for page-sized gaps. They are in rem, so spacing grows with the reader's font size.

**Spacing.** Use the units for padding, gaps and margins: `x1`-`x2` inside small controls, `x3`-`x4` inside cards and between fields, `x6`-`x8` between sections. `Stack { gap: 4 }` spaces children 16 px apart; prefer a stack's gap to margins on children.

**Breakpoints.** Four, named by size. Media queries cannot read custom properties, so they are fixed values:

| Name | From | Page columns | Gutter | Page margin |
| --- | --- | --- | --- | --- |
| s | 0 | 4 | 16 px | 16 px |
| m | 768 px | 8 | 20 px | 24 px |
| l | 1024 px | 12 | 24 px | 32 px |
| xl | 1280 px | 12 | 24 px | 32 px |

`use_media_query(MOBILE_QUERY)` in Rust is "below m".

**Grid.** `--layout-columns`, `--layout-gutter` and `--layout-margin` follow the table above; `--layout-max-width` is 80 rem (1280 px). `Container` centres a page with the margin, `Grid` lays out the page columns, `GridItem { span: 4, span_m: 4, span_l: 6 }` spans them per breakpoint, and `Grid { min_item: "16rem" }` fits as many columns as there is room for (card lists).

## Typography

**Type scale.** Nine steps, each with a line height on the 4 px grid:

| Step | Size / line (s) | From m | Use |
| --- | --- | --- | --- |
| `display` | 36 / 44 | 48 / 56 | Landing pages, empty-state headlines |
| `title-1` | 30 / 36 | 36 / 44 | Page title |
| `title-2` | 24 / 32 | 28 / 36 | Section title |
| `title-3` | 20 / 28 | | Card and dialog titles |
| `title-4` | 18 / 28 | | Small headings, emphasised labels |
| `body-1` | 16 / 24 | | Long reading text |
| `body-2` | 14 / 20 | | The default: controls, tables, most UI text |
| `body-3` | 13 / 20 | | Dense UI: menus, shortcuts |
| `caption-1` | 12 / 16 | | Hints, metadata, badges |

Tokens: `--font-size-<step>` and `--line-height-<step>`; weights `--font-weight-regular|medium|semibold|bold` (400 to 700).

**Responsiveness.** `display`, `title-1` and `title-2` grow from the m breakpoint up; nothing else changes size, so controls stay put.

**Grid relation.** Every line height is a multiple of 4 px, so text lines up with the spacing units.

**Readability.** Body text is at least 14 px; long text uses `body-1`. Keep running text to `--text-measure` (70 characters). Headings track slightly tighter (`--heading-tracking`), body text never.

**Performance.** Fonts ship with the kit (`crates/ui/assets/fonts/`, Latin subsets of variable fonts, one file per family, 30 to 50 KB): nothing is fetched from a font service. A browser downloads a file only when the page uses it; text shows at once in the system font and swaps when the file arrives (`font-display: swap`), so fonts never hold up the first screen. The Classic look downloads nothing. Your own font, from a file or Google Fonts, goes in the same way: `atoll font` copies it into your app and `CustomFont` declares it ([Theming](THEMING.md)).

**Guidelines.** Pick the element by meaning and the look by step: `Text { variant: TextVariant::Title2, as_: TextAs::H2 }`. One `title-1` per page. Don't skip heading levels for size.

## Elevation

Three heights, each a surface colour and a shadow:

| Height | Surface | Shadow | Examples |
| --- | --- | --- | --- |
| Base | `--color-background-elevation-base` | none (a border) | Cards, panels, tables |
| Raised | `--color-background-elevation-raised` | `--shadow-raised` | Cards that lift, active tabs, a floating sidebar |
| Overlay | `--color-background-elevation-overlay` | `--shadow-overlay` | Menus, popovers, selects, dialogs, sheets, toasts |

The shadows are Reshaped's, with deeper colours in dark mode (`--color-shadow-*`), where surfaces also get lighter as they rise. Focus rings are not elevation: they use `--color-border-focus`.

**Z-index.** Layers, lowest to highest. Use the token, never a number:

| Token | Value | For |
| --- | --- | --- |
| `--z-index-raised` | 1 | A control lifted over its neighbours (a focused button in a group) |
| `--z-index-sticky` | 10 | Sticky headers, the sidebar |
| `--z-index-fixed` | 20 | Fixed bars, the sidebar's rail |
| `--z-index-dropdown` | 50 | Inline lists (combobox, multi-select) |
| `--z-index-overlay` | 1000 | Menus, popovers, tooltips, modal scrims |
| `--z-index-modal` | 1001 | Dialog, alert dialog, sheet and drawer panels |
| `--z-index-toast` | 9999 | Toasts |
| `--z-index-top` | 10000 | The skip link |

## Motion

**Easing.** `--easing-standard` (moves within the screen), `--easing-decelerate` (things entering), `--easing-accelerate` (things leaving). Entrances that should feel physical use the kit's springs (`--spring-snappy`, `--spring-gentle`, `--spring-bouncy`), traced from the Rust springs so CSS and Rust motion match.

**Duration.** `--duration-rapid` (100 ms: hover and press colours), `--duration-fast` (150 ms: small things, exits), `--duration-medium` (250 ms: panels, accordions), `--duration-slow` (400 ms: large surfaces). Things leave faster than they arrive.

**Hover and press.** Filled and outlined buttons lift by `--hover-lift` (1 px) onto `--shadow-hover` when a pointer hovers them; primary and destructive buttons add a soft glow in their own colour. In dark mode, where shadows barely show, `--shadow-hover` adds a lit top edge (`--color-highlight`). Pressing settles the button back at `--duration-rapid`. Focus keeps its ring instead of the lift, and touch screens never get the hover state (`@media (hover: hover)`).

**Accessibility.** Under the OS "reduce motion" setting the duration tokens drop to 0.01 ms, `--hover-lift` drops to 0 (the hover shadow stays) and every CSS animation and transition is cut to the same (theme.css), Rust springs jump to their end, and nothing moves on its own. Loops (spinners) slow down instead of stopping, so loading is still visible. No motion carries meaning alone.

## Iconography

**Style.** Lucide, every icon generated into the kit (`ui::icons::*`): 24 px grid, round caps, one stroke weight per look (`--icon-stroke-width`: 2 in Lagoon, Reef and Classic, 1.5 in Abyss and Driftwood).

**Naming.** Lucide's names in kebab case, as Rust components in PascalCase: `circle-check` is `ui::icons::CircleCheck`. Old names stay as aliases.

**Relation with the grid.** Sizes `IconSize::Xs` to `Xl` read `--icon-size-xs` ... `-xl` (12, 16, 20, 24, 32 px), multiples of 4. Next to text, use the size whose box matches the line height (16 px with `body-2`, 20 px with `body-1`).

**Keywords.** Every icon carries Lucide's search words in its rustdoc, and `atoll icons <words>` searches them: `atoll icons delete` finds `trash-2`, `eraser`, `delete` and the rest.

**Reserved icons.** The kit uses some icons for one meaning; use them only for that, and pick another icon for anything else. `atoll icons --reserved` prints the list, and search marks them:

| Icon | Means |
| --- | --- |
| `x` | Close or dismiss |
| `check` | Selected, done |
| `chevron-down`, `chevrons-up-down` | Opens a list or menu |
| `chevron-right`, `chevron-left` | Go into or next; back or previous |
| `search` | Search |
| `ellipsis` | More actions |
| `panel-left` | Show or hide the sidebar |
| `grip-vertical` | Drag to reorder |
| `loader-circle` | Loading |
| `circle-check`, `circle-alert`, `circle-x` | Success, warning, error |
| `external-link` | Opens outside the app |
| `plus`, `minus` | Add or create; remove one or decrease |
| `clock` | Pick a time |
| `arrow-up`, `arrow-down` | Sort ascending or descending |

**Accessibility.** Icons are decorative (`aria-hidden`) unless given a `label`; an icon-only button is named on the button. Icons draw in `currentColor`, so they meet the same contrast as the text beside them.

**Guidelines.** One icon per meaning across an app; an icon never replaces a word where the meaning is not obvious; directional icons mirror in right-to-left pages ([Icons](ICONS.md)).

## Checked

- `cargo test -p atoll-ui --test foundations`: no literal z-index, durations, drop-shadow colours or font sizes in component CSS; every token a component names exists; the type scale and units sit on the 4 px grid; reduced motion zeroes the durations.
- `cargo test -p atoll-ui --test theme` and `cargo test -p atoll-cli --test contrast`: colours, every look and accent, light and dark.
- `scripts/e2e.sh`: axe on the gallery in every look, light and dark, on three browser engines.
