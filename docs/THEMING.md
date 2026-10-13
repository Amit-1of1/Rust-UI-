# Theming

All colour, radius and mode live in `theme.css` as CSS custom properties. Components only read tokens, so changing a token restyles every component. `rust-ui doctor` warns about components that hard-code a colour instead.

## Mode

Set `data-theme="light"` or `"dark"` on `<html>`. Without it the OS preference applies. The attribute only sets `color-scheme`, and every colour token is a `light-dark(light, dark)` pair, so the scheme picks the value:

```css
--primary-color: light-dark(#fff, #000);
```

`color-scheme` also gives native scrollbars, form controls and the page canvas the matching scheme.

`light-dark()` needs Chrome 123, Safari 17.5 or Firefox 120. For older engines `theme.css` carries a generated `@supports not (color: light-dark(...))` block that restates the kit's tokens as space toggles (`var(--dark, X) var(--light, Y)`); `cargo test -p rust-ui-kit --test theme` fails when it drifts and `UPDATE_THEME_CSS=1` regenerates it. The `--dark`/`--light` toggles stay defined for values `light-dark()` cannot hold, such as shadows.

## Accent colour

Set `data-accent` on `<html>` to `blue`, `green`, `violet`, `rose` or `orange`. Without it the neutral accent is used. A preset sets three tokens: `--accent-color` (the fill), `--accent-contrast` (the label on the fill) and `--focused-border-color`.

## Your own brand colour

```sh
cargo run -p rust-ui-cli -- theme --brand '#7c3aed' --name brand >> assets/theme.css
```

The block uses `light-dark()`. Inside this repo, rerun `UPDATE_THEME_CSS=1 cargo test -p rust-ui-kit --test theme` after appending, so the fallback block covers it too. Then set `data-accent="brand"`. The generator keeps your hue and saturation and moves only the lightness, until the label text on the fill reaches 4.5:1 (WCAG AA) in both dark and light mode. A colour that is already readable is kept as is. Check the result against a pasted preset with `cargo test -p rust-ui-kit --test theme`.

## Radius

`--radius` (default `0.5rem`) rounds every component. The `ThemeCustomizer` component exposes accent, radius and mode as controls.

## Overlay

`--overlay-color` is the scrim behind modal overlays (dialog, alert dialog, sheet, drawer): black at 40% in light mode and 60% in dark mode.

## Not supported

Colours are not defined in Rust. See the decision in `docs/GUARDRAILS.md`: the web and desktop webview only understand CSS, and custom properties are the supported theming route.
