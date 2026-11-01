//! A personal shadcn-inspired component kit for Dioxus.
//!
//! Behaviour and accessibility come from `dioxus-primitives`; the styled layer in
//! [`components`] is copied from DioxusLabs/components and owned here from now on.
//!
//! # Stability
//!
//! Import from [`prelude`], [`hooks`], [`icons`] and [`motion`]. Which items are stable,
//! experimental or internal is listed in docs/STABILITY.md; experimental and internal items
//! also say so in their docs. Breaking changes follow docs/VERSIONING.md and are listed in
//! CHANGELOG.md.
#![forbid(unsafe_code)]

/// Whole screens to copy (`atoll add login_01`): sign-in, sign-up, dashboard, settings
/// (feature `blocks`).
///
/// Internal: not part of the public API (docs/STABILITY.md).
#[cfg(feature = "blocks")]
pub mod blocks;
/// Component modules. Internal paths: import from [`prelude`] instead (docs/STABILITY.md).
pub mod components;
pub mod css_bundle;
/// Demos shown in the gallery and the docs site (feature `demos`).
///
/// Internal: not part of the public API (docs/STABILITY.md).
#[cfg(feature = "demos")]
pub mod demos;
/// The fonts the theme presets use (OFL-1.1, shipped with the kit).
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub mod fonts;

/// Shared hooks (`use_hotkey`, `Selection`, `use_media_query`, ...): `ui::hooks::*`.
pub use components::hooks;
/// The kit's icon set (Lucide): `ui::icons::Check`. Import by path, not with a glob.
pub use components::icons;

/// Spring presets, presence animations and reduced-motion handling.
pub use motion;

/// `#[derive(Form)]`: a struct's fields become a validated, accessible form (see the `form`
/// component's docs). Also in [`prelude`].
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub use form_derive::Form;

use dioxus::prelude::*;

/// Theme tokens (colors, light/dark) shared by every component.
pub const THEME_CSS: Asset = asset!("/assets/theme.css");

/// [`THEME_CSS`] confined to [`ThemeScope`](components::theme_customizer::ThemeScope): the same tokens and rules, applied only inside
/// an element with `data-rui-scope`, for apps that already have their own styles. Generated
/// from `theme.css` by `atoll registry`.
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub const THEME_SCOPED_CSS: Asset = asset!("/assets/theme.scoped.css");

/// Optional base styles (box sizing, page background, text colour and font), the page the
/// contrast proof assumes. [`ThemeStyles`]`{ reset: true }` loads it.
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub const RESET_CSS: Asset = asset!("/assets/reset.css");

/// [`RESET_CSS`] confined to [`ThemeScope`](components::theme_customizer::ThemeScope), like [`THEME_SCOPED_CSS`].
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub const RESET_SCOPED_CSS: Asset = asset!("/assets/reset.scoped.css");

/// Loads the theme stylesheet. Render once near the root of the app.
///
/// It also switches to the single bundled stylesheet when the page has one (see [`css_bundle`]).
///
/// In an app that already has its own CSS, pass `scoped: true` and wrap the part of the app
/// that uses the kit in [`ThemeScope`](components::theme_customizer::ThemeScope): the kit's tokens (`--primary-color`, `--radius`, ...)
/// and page-wide rules (reduced motion, forced colours, `.sr-only`) then apply only inside
/// it, and [`use_theme`](components::theme_customizer::use_theme) stops writing `data-theme` and friends on `<html>`. `reset: true`
/// adds the small base stylesheet [`RESET_CSS`] (scoped too when `scoped` is set).
/// docs/EXISTING_APP.md walks through it.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     ThemeStyles { scoped: true }
///     ThemeScope { Button { "Only this part uses the kit" } }
/// }
/// # }
/// ```
#[component]
pub fn ThemeStyles(
    /// Confine the theme to [`ThemeScope`](components::theme_customizer::ThemeScope) (experimental).
    #[props(default)]
    scoped: bool,
    /// Also load the base styles, [`RESET_CSS`] (experimental).
    #[props(default)]
    reset: bool,
    /// The look to start with, one of
    /// [`PRESETS`](components::theme_customizer::PRESETS) (`"reef"`, `"abyss"`, ...): the
    /// app's build-time choice. Without it the page shows Atoll's default, `lagoon`, or what
    /// [`use_theme`](components::theme_customizer::use_theme)`().set_preset(..)` picks later
    /// (experimental).
    #[props(default)]
    preset: Option<&'static str>,
    /// Declare the presets' fonts (`@font-face`, files shipped with the kit; on by default).
    /// Off, every preset uses the system fonts at the end of its stack (experimental).
    #[props(default = true)]
    fonts: bool,
) -> Element {
    use_hook(css_bundle::install);
    use_hook(|| components::theme_customizer::set_writes_html(!scoped));
    let theme = components::theme_customizer::use_theme();
    use_hook(move || {
        if let Some(name) = preset {
            theme.set_preset(name);
        }
    });
    let theme_css = if scoped { THEME_SCOPED_CSS } else { THEME_CSS };
    let reset_css = if scoped { RESET_SCOPED_CSS } else { RESET_CSS };
    rsx! {
        document::Stylesheet { href: theme_css }
        if reset {
            document::Stylesheet { href: reset_css }
        }
        if fonts {
            document::Style { {fonts::font_face_css()} }
        }
    }
}

/// Everything an app usually needs: `use ui::prelude::*;`
pub mod prelude {
    // Every component's items, generated by build.rs (crates/ui/not_in_prelude.txt says what is left out).
    include!(concat!(env!("OUT_DIR"), "/prelude.rs"));
    pub use crate::ThemeStyles;
    pub use dioxus_primitives::toast::{use_toast, ToastOptions};
    pub use dioxus_primitives::ContentSide;
    pub use form_derive::Form;
}
