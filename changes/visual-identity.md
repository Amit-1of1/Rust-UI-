### Changed

- **ui:** Atoll has its own default look, "Lagoon": a lagoon-teal accent (`data-accent` unset is now teal, not black), neutrals tinted toward sea water at the same brightness as the old greys, Figtree for text and Bricolage Grotesque for headings (`--font-sans`, `--font-heading`, `--font-mono`), 12 px corners (`--radius: 0.75rem`) and a 1.75 icon stroke. Dialogs, sheets, the calendar, popovers, menus, items and the sidebar now round with `--radius` instead of fixed pixels, and component titles use the heading font. The old look is the `classic` preset. docs/BRAND.md describes it. (#PR)

### Added

- **ui:** Theme presets ("looks"): `data-preset="reef|abyss|driftwood|classic"` (and `lagoon`, the default) switches colours, fonts, corner radius, density and icon stroke in one attribute, on `<html>` or any element. `ThemeStyles { preset: "reef" }` picks one at build time; `use_theme().set_preset(..)`, `preset()`, `preset_info()` and `radius_override()` switch at runtime; `PRESETS`, `PRESET_INFO` and the `PresetPicker` component; `ThemeCustomizer` shows the picker first; `PersistTheme` and `THEME_INIT_SCRIPT` remember the look (experimental). (#PR)
- **ui:** Six OFL-1.1 fonts ship with the kit (`crates/ui/assets/fonts/`, Latin subsets of Figtree, Bricolage Grotesque, Nunito, Space Grotesk, Fraunces and JetBrains Mono). `ThemeStyles` declares them with `font-display: swap`; a browser fetches a file only when the page uses it. `ThemeStyles { fonts: false }` and the `classic` preset use system fonts only. `ui::fonts` (experimental). (#PR)
- **cli:** `atoll theme --preset <name>` prints a theme preset as the page default (`:root`), `--preset list` lists them. (#PR)
- **docs:** Brand and looks guide (docs/BRAND.md, the docs site's `#brand`), the Atoll mark (`docs/brand/atoll-mark.svg`, `atoll-logo.svg`), a look switcher in the docs and gallery headers, a "Five looks" section on the home page and a look picker on the theme builder. (#PR)
