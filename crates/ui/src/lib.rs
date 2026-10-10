//! A personal shadcn-inspired component kit for Dioxus.
//!
//! Behaviour and accessibility come from `dioxus-primitives`; the styled layer in
//! [`components`] is copied from DioxusLabs/components and owned here from now on.

pub mod components;

use dioxus::prelude::*;

/// Theme tokens (colors, light/dark) shared by every component.
pub const THEME_CSS: Asset = asset!("/assets/theme.css");

/// Loads the theme stylesheet. Render once near the root of the app.
#[component]
pub fn ThemeStyles() -> Element {
    rsx! {
        document::Stylesheet { href: THEME_CSS }
    }
}

/// Everything an app usually needs: `use ui::prelude::*;`
pub mod prelude {
    pub use crate::components::accordion::*;
    pub use crate::components::badge::*;
    pub use crate::components::button::*;
    pub use crate::components::card::*;
    pub use crate::components::checkbox::*;
    pub use crate::components::dialog::*;
    pub use crate::components::dropdown_menu::*;
    pub use crate::components::input::*;
    pub use crate::components::label::*;
    pub use crate::components::popover::*;
    pub use crate::components::separator::*;
    pub use crate::components::sheet::*;
    pub use crate::components::switch::*;
    pub use crate::components::tabs::*;
    pub use crate::components::toast::*;
    pub use crate::components::tooltip::*;
    pub use crate::ThemeStyles;
    pub use dioxus_primitives::toast::{use_toast, ToastOptions};
    pub use dioxus_primitives::ContentSide;
}
