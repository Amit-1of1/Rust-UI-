# ThemeCustomizer

- Copy into your app: `rust-ui add theme_customizer` or `dx components add theme_customizer --git https://github.com/Amit-1of1/Rust-UI-` (also copies `hooks`).
- Import: `use crate::components::theme_customizer::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

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

## API

```rust
/// Accent presets defined in `theme.css` (`[data-accent=...]`). `neutral` is the default.
pub const ACCENTS: [&str; 6];

/// Whether [`use_theme`]'s setters also write `<html>` (on by default).
/// `ThemeStyles { scoped: true }` turns it off, so an existing app's own `data-theme` on
/// `<html>` is left alone; an app that copied the components and loads `theme.scoped.css`
/// itself calls `set_writes_html(false)` once at startup instead.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn set_writes_html(on: bool)

/// Light or dark, or whatever the operating system prefers.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

/// The page's theme: mode, accent, density and corner radius. One shared state, so a
/// header switch, a settings screen and [`ThemeCustomizer`] always agree. Get it with
/// [`use_theme`]; the setters write `<html>` and every [`ThemeScope`], like the customizer.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct Theme {
    choices: Choices,
    prefers_dark: ReadSignal<bool>,
}

/// The shared [`Theme`]. Reading it subscribes the component, so it re-renders on changes
/// made anywhere else.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_theme() -> Theme

/// Applies [`ThemeCustomizer`]'s choices to its children, and marks where the kit's scoped
/// theme applies (`data-rui-scope`).
///
/// On the web and in desktop webviews the customizer sets them on `<html>` and this wrapper
/// is not needed. Renderers without JavaScript (dioxus-native) can't reach `<html>`, so wrap
/// the app in `ThemeScope` there. It is a `display: contents` box: it doesn't change layout.
///
/// With `ThemeStyles { scoped: true }` (an existing app with its own CSS) the kit's tokens
/// and rules apply only inside a `ThemeScope`; wrap each part of the app that uses the kit,
/// or the whole app. Several scopes share one [`use_theme`] state.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ThemeScope(children: Element) -> Element

/// Density presets defined in `theme.css` (`[data-density=...]`). `default` sets nothing.
pub const DENSITIES: [&str; 3];

/// Controls for the accent colour, corner radius, density and light/dark mode of the whole page.
///
/// It only sets `data-accent` / `data-density` / `data-theme` and `--radius` on `<html>`. Its
/// state is the shared [`use_theme`], so app controls (a header dark-mode switch) stay in step.
#[component]
pub fn ThemeCustomizer() -> Element
```

## Example

```rust
use crate::components::theme_customizer::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        // ThemeScope carries the choices where `<html>` can't be reached (native).
        ThemeScope { ThemeCustomizer {} }
    }
}
```
