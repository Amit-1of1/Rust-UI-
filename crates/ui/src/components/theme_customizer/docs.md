# ThemeCustomizer

Pick an accent colour, corner radius and light/dark mode for the whole page. It only writes `data-accent`, `data-theme` and `--radius` on `<html>`, so you can drive the same attributes from your own settings.

## Theming without the component

- `data-accent="blue|green|violet|rose|orange"` on `<html>` recolors primary buttons, checked checkboxes/switches/radios, slider and progress fills and the focus ring. Remove it for the neutral default.
- For a brand colour set `--accent-color` and `--accent-contrast` yourself (keep text on the fill at 4.5:1 or better).
- `--radius` (default `0.5rem`) scales the corners of controls.

## Accessibility

- Role and name: native radio inputs grouped in `fieldset`/`legend`; the radius slider is a native range input with a label.
- Keyboard: arrow keys move within a group, Tab between groups (native behaviour).
- Reduced motion: no animation.

## Source

Original to this repo.
