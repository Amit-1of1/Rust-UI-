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

## Remember the theme between visits

`PersistTheme {}` saves `use_theme()`'s choices in `localStorage` (key `"theme"`, or `storage_key`) and restores them on the next visit. Render it once next to `ThemeStyles {}`. It restores after the first render, so it is safe in fullstack apps. To avoid a light flash for a returning dark-mode user while the app loads, also put `THEME_INIT_SCRIPT` in the page `<head>`: in a fullstack app `document::Script { {THEME_INIT_SCRIPT} }` (the server renders it into the HTML), in a client-only app paste it into a `<script>` in your `index.html`. It is your app's script, opt in, and only sets known values.

```rust
rsx! {
    ThemeStyles {}
    PersistTheme {}
    document::Script { {THEME_INIT_SCRIPT} }
    Router::<Route> {}
}
```

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

/// Applies [`ThemeCustomizer`]'s choices to its children.
///
/// On the web and in desktop webviews the customizer sets them on `<html>` and this wrapper
/// is not needed. Renderers without JavaScript (dioxus-native) can't reach `<html>`, so wrap
/// the app in `ThemeScope` there. It is a `display: contents` box: it doesn't change layout.
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

/// Saves [`use_theme`]'s choices (mode, accent, density, radius) in the browser's
/// `localStorage` under `storage_key` (default `"theme"`), and restores them on the next visit. Render it once, next to
/// `ThemeStyles {}`:
///
/// ```rust,ignore
/// rsx! { ThemeStyles {} PersistTheme {} Router::<Route> {} }
/// ```
///
/// The saved value is restored right after the first render, so it is safe in a fullstack
/// app (the server has no `localStorage`). Until then the page shows the default theme; to
/// avoid that flash, also put [`THEME_INIT_SCRIPT`] in the page's `<head>`.
///
/// Web only (a Rust `web-sys` call, no JavaScript of the kit's); elsewhere it does nothing.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn PersistTheme(
    /// The `localStorage` key. Change it when two apps share an origin.
    #[props(into, default = "theme".to_string())]
    storage_key: String,
) -> Element

/// `mode=dark;accent=blue;density=compact;radius=12` (radius only when it isn't the default).
pub fn encode_theme(theme: &Theme) -> String

/// Applies a value written by [`encode_theme`]. The stored text is user-controlled, so every part
/// is checked against the known values and anything else is ignored.
pub fn apply_saved_theme(theme: &Theme, saved: &str)

/// A small script for the page's `<head>` that applies the saved theme before the first
/// paint, so a returning dark-mode user never sees a white page while the app loads. It
/// reads the `theme` key that [`PersistTheme`] writes and only sets known values.
///
/// This is the app's own script, opt in: put it in your `index.html`, or render it in a
/// fullstack app with `document::Script { {THEME_INIT_SCRIPT} }`.
pub const THEME_INIT_SCRIPT: &str;
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
