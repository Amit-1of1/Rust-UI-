# Theming

All colour, type, radius and mode live in `theme.css` as CSS custom properties. Components only read tokens, so changing a token restyles every component. `atoll doctor` warns about components that hard-code a colour instead.

## Looks (theme presets)

Atoll's default look is Lagoon: teal on neutral greys, Inter, 10 px corners, on foundations taken from Reshaped ([Foundations](#foundations) lists the tokens, [Brand and looks](#brand) explains the look). Four more looks change colours, fonts, corner radius, density and icon stroke together: `reef`, `abyss`, `driftwood` and `classic` (the old black-and-white look, system fonts).

- At build time: `ThemeStyles { preset: "reef" }`, or `data-preset="reef"` on `<html>` in `index.html`.
- While the app runs: `use_theme().set_preset("abyss")`, or render `PresetPicker {}` (also the first part of `ThemeCustomizer`). `PersistTheme` remembers it.
- On part of a page: `data-preset` on any element, like `data-theme`.
- Baked into your own CSS: `atoll theme --preset driftwood` prints the look as a `:root` block; `--preset list` lists them.

An explicit `data-accent` or `data-density` wins over the look's own. Fonts ship with the kit (OFL-1.1) and download only when used; `ThemeStyles { fonts: false }` keeps system fonts. Every look passes the contrast proof and axe, in light and dark.

## Fonts

`--font-sans` (text), `--font-heading` (titles, `h1`-`h4` with the reset), `--font-mono` (code, keys), plus `--heading-weight` and `--heading-tracking`. Set them on `:root` to use your own fonts with any look.

## Mode

Set `data-theme="light"` or `"dark"` on `<html>`. Without it the OS preference applies. The attribute only sets `color-scheme`, and every colour token is a `light-dark(light, dark)` pair, so the scheme picks the value:

```css
--primary-color: light-dark(#fff, #000);
```

`color-scheme` also gives native scrollbars, form controls and the page canvas the matching scheme.

`light-dark()` needs Chrome 123, Safari 17.5 or Firefox 120. For older engines `theme.css` carries a generated `@supports not (color: light-dark(...))` block that restates the kit's tokens as space toggles (`var(--dark, X) var(--light, Y)`); `cargo test -p atoll-ui --test theme` fails when it drifts and `UPDATE_THEME_CSS=1` regenerates it. The `--dark`/`--light` toggles stay defined for values `light-dark()` cannot hold, such as shadows.

## Accent colour

Set `data-accent` on `<html>` to `blue`, `green`, `violet`, `rose` or `orange`. Without it the look's own accent is used (lagoon teal by default; `neutral` in `ACCENTS` means that). A preset sets three tokens: `--accent-color` (the fill), `--accent-contrast` (the label on the fill) and `--focused-border-color`.

## Your own brand colour

Try it in the browser first: the [Theme builder](#theme-builder) page runs the same generator live, shows the kit in your colour in light and dark mode with the contrast it reaches, and copies the CSS.

```sh
cargo run -p atoll-cli -- theme --brand '#7c3aed' --name brand >> assets/theme.css
```

The block uses `light-dark()`. Inside this repo, rerun `UPDATE_THEME_CSS=1 cargo test -p atoll-ui --test theme` after appending, so the fallback block covers it too. Then set `data-accent="brand"`. The generator keeps your hue and saturation and moves only the lightness, until the label text on the fill reaches 4.5:1 (WCAG AA) in both dark and light mode. A colour that is already readable is kept as is. Check the result against a pasted preset with `cargo test -p atoll-ui --test theme`.

### From your logo

```sh
cargo run -p atoll-cli -- theme --from-image logo.svg --name brand >> assets/theme.css
```

`--from-image` takes a PNG, JPEG or SVG and finds the brand colour in it, then fits it exactly like `--brand`. It counts the logo's colours (for SVG, every `fill`, `stroke` and `stop-color`), leaves out a white, black or grey backdrop (the colour of the image's border, or a full-size first `rect`), and picks the most visible saturated colour: how much of the logo it covers times how colourful it is, so a small orange mark wins over a large grey wordmark. The output starts with a comment listing the colours it found and which one it chose. When the logo has a second, clearly different hue (at least 40 degrees round the colour wheel and 3% of the logo), a second preset named `<name>-2` follows as an alternative; delete the one you don't want. A black-and-white logo gives a neutral accent from its main colour.

## Radius

`--radius` (default `0.75rem`; each look sets its own) rounds every component; smaller parts use fractions of it. The `ThemeCustomizer` component exposes accent, radius and mode as controls.

## Overlay

`--overlay-color` is the scrim behind modal overlays (dialog, alert dialog, sheet, drawer): black at 40% in light mode and 60% in dark mode.

## Not supported

Colours are not defined in Rust. See the decision in `docs/GUARDRAILS.md`: the web and desktop webview only understand CSS, and custom properties are the supported theming route.
