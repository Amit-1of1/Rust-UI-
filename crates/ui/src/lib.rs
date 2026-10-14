//! A personal shadcn-inspired component kit for Dioxus.
//!
//! Behaviour and accessibility come from `dioxus-primitives`; the styled layer in
//! [`components`] is copied from DioxusLabs/components and owned here from now on.

pub mod components;
pub mod css_bundle;
pub mod demos;

/// Shared hooks (`use_hotkey`, `Selection`, `use_media_query`, ...): `ui::hooks::*`.
pub use components::hooks;
/// The kit's icon set (Lucide): `ui::icons::Check`. Import by path, not with a glob.
pub use components::icons;

/// Spring presets, presence animations and reduced-motion handling.
pub use motion;

use dioxus::prelude::*;

/// Theme tokens (colors, light/dark) shared by every component.
pub const THEME_CSS: Asset = asset!("/assets/theme.css");

/// Loads the theme stylesheet. Render once near the root of the app.
///
/// It also switches to the single bundled stylesheet when the page has one (see [`css_bundle`]).
#[component]
pub fn ThemeStyles() -> Element {
    use_hook(css_bundle::install);
    rsx! {
        document::Stylesheet { href: THEME_CSS }
    }
}

/// Everything an app usually needs: `use ui::prelude::*;`
pub mod prelude {
    pub use crate::components::accordion::*;
    pub use crate::components::alert::*;
    pub use crate::components::alert_dialog::*;
    pub use crate::components::aspect_ratio::*;
    pub use crate::components::avatar::*;
    pub use crate::components::badge::*;
    pub use crate::components::bottom_nav::*;
    pub use crate::components::breadcrumb::*;
    pub use crate::components::button::*;
    pub use crate::components::button_group::*;
    pub use crate::components::calendar::*;
    pub use crate::components::callout::*;
    pub use crate::components::card::*;
    pub use crate::components::carousel::*;
    pub use crate::components::chart::*;
    pub use crate::components::checkbox::*;
    pub use crate::components::chips::*;
    pub use crate::components::collapsible::*;
    pub use crate::components::color_picker::*;
    pub use crate::components::combobox::*;
    pub use crate::components::command::*;
    pub use crate::components::context_menu::*;
    pub use crate::components::data_table::*;
    pub use crate::components::date_picker::*;
    pub use crate::components::dialog::*;
    pub use crate::components::drawer::*;
    pub use crate::components::dropdown_menu::*;
    pub use crate::components::dropzone::*;
    pub use crate::components::empty::*;
    pub use crate::components::field::*;
    pub use crate::components::form::*;
    pub use crate::components::hooks::*;
    pub use crate::components::hover_card::*;
    pub use crate::components::input::*;
    pub use crate::components::input_group::*;
    pub use crate::components::input_otp::*;
    pub use crate::components::item::*;
    pub use crate::components::kbd::*;
    pub use crate::components::label::*;
    pub use crate::components::link::*;
    pub use crate::components::marquee::*;
    pub use crate::components::menubar::*;
    pub use crate::components::multi_select::*;
    pub use crate::components::navbar::*;
    pub use crate::components::pagination::*;
    pub use crate::components::popover::*;
    pub use crate::components::progress::*;
    pub use crate::components::radio_group::*;
    pub use crate::components::scroll_area::*;
    pub use crate::components::select::*;
    pub use crate::components::separator::*;
    pub use crate::components::sheet::*;
    pub use crate::components::shimmer::*;
    pub use crate::components::sidebar::*;
    pub use crate::components::skeleton::*;
    pub use crate::components::slider::*;
    pub use crate::components::spinner::*;
    pub use crate::components::status::*;
    pub use crate::components::stepper::*;
    pub use crate::components::switch::*;
    pub use crate::components::table::*;
    pub use crate::components::tabs::*;
    pub use crate::components::textarea::*;
    pub use crate::components::theme_customizer::*;
    pub use crate::components::toast::*;
    pub use crate::components::toggle::*;
    pub use crate::components::toggle_group::*;
    pub use crate::components::toolbar::*;
    pub use crate::components::tooltip::*;
    pub use crate::components::ui_strings::*;
    pub use crate::components::virtual_list::*;
    // drag_and_drop_list and tag_group both export `RemoveButton`; use their module paths.
    pub use crate::ThemeStyles;
    pub use dioxus_primitives::toast::{use_toast, ToastOptions};
    pub use dioxus_primitives::ContentSide;
}
