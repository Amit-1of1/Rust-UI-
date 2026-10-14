# ThemeCustomizer

Pick an accent colour, corner radius and light/dark mode for the whole page. It only writes `data-accent`, `data-theme` and `--radius` on `<html>`, so you can drive the same attributes from your own settings.

## One theme state: `use_theme()`

The customizer's choices live in one shared state. Read and change it from app code with `use_theme()`, so a header switch and the customizer never disagree:

```rust
let theme = use_theme();
rsx! {
    Switch {
        checked: theme.is_dark(),
        aria_label: "Dark mode",
        on_checked_change: move |dark| theme.set_mode(if dark { ThemeMode::Dark } else { ThemeMode::Light }),
    }
}
```

`mode()` / `set_mode(ThemeMode::System | Light | Dark)`, `is_dark()` (follows the OS in `System` mode, on the web), `accent()` / `set_accent(name)` (one of `ACCENTS`), `density()` / `set_density(name)` (one of `DENSITIES`) and `radius()` / `set_radius(px)`. Don't write `data-theme` yourself as well; that is the second source of truth this replaces.

## Native (no JavaScript)

dioxus-native can't run the script that writes to `<html>`. Wrap the app in `ThemeScope { ... }` there: it applies the same choices to everything inside it, and doesn't change layout (`display: contents`). On the web and desktop it is harmless.

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

## Stability

`ACCENTS`, `DENSITIES` and the attributes and tokens the customizer writes are stable. `use_theme`, `Theme`, `ThemeMode`, `ThemeScope` and the customizer's own markup are experimental and may change in any release; see `docs/STABILITY.md` in the kit repository.
