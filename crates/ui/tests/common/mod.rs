//! Shared by the integration tests that walk every demo.

#![allow(dead_code)]

use dioxus::prelude::*;

/// Renders a component on the server and returns its HTML.
pub fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

pub type Demo = (&'static str, fn() -> Element);

pub const DEMOS: &[Demo] = &[
    ("alert", ui::demos::alert::Demo),
    ("alert_dialog", ui::demos::alert_dialog::Demo),
    ("aspect_ratio", ui::demos::aspect_ratio::Demo),
    ("avatar", ui::demos::avatar::Demo),
    ("bottom_nav", ui::demos::bottom_nav::Demo),
    ("breadcrumb", ui::demos::breadcrumb::Demo),
    ("button_group", ui::demos::button_group::Demo),
    ("calendar", ui::demos::calendar::Demo),
    ("callout", ui::demos::callout::Demo),
    ("carousel", ui::demos::carousel::Demo),
    ("chart", ui::demos::chart::Demo),
    ("collapsible", ui::demos::collapsible::Demo),
    ("color_picker", ui::demos::color_picker::Demo),
    ("combobox", ui::demos::combobox::Demo),
    ("command", ui::demos::command::Demo),
    ("context_menu", ui::demos::context_menu::Demo),
    ("data_table", ui::demos::data_table::Demo),
    ("date_picker", ui::demos::date_picker::Demo),
    ("drag_and_drop_list", ui::demos::drag_and_drop_list::Demo),
    ("drawer", ui::demos::drawer::Demo),
    ("dropzone", ui::demos::dropzone::Demo),
    ("empty", ui::demos::empty::Demo),
    ("field", ui::demos::field::Demo),
    ("form", ui::demos::form::Demo),
    ("hover_card", ui::demos::hover_card::Demo),
    ("input_group", ui::demos::input_group::Demo),
    ("input_otp", ui::demos::input_otp::Demo),
    ("item", ui::demos::item::Demo),
    ("kbd", ui::demos::kbd::Demo),
    ("label", ui::demos::label::Demo),
    ("marquee", ui::demos::marquee::Demo),
    ("menubar", ui::demos::menubar::Demo),
    ("multi_select", ui::demos::multi_select::Demo),
    ("pagination", ui::demos::pagination::Demo),
    ("progress", ui::demos::progress::Demo),
    ("radio_group", ui::demos::radio_group::Demo),
    ("scroll_area", ui::demos::scroll_area::Demo),
    ("select", ui::demos::select::Demo),
    ("shimmer", ui::demos::shimmer::Demo),
    ("sidebar", ui::demos::sidebar::Demo),
    ("skeleton", ui::demos::skeleton::Demo),
    ("slider", ui::demos::slider::Demo),
    ("spinner", ui::demos::spinner::Demo),
    ("status", ui::demos::status::Demo),
    ("stepper", ui::demos::stepper::Demo),
    ("table", ui::demos::table::Demo),
    ("tag_group", ui::demos::tag_group::Demo),
    ("textarea", ui::demos::textarea::Demo),
    ("theme_customizer", ui::demos::theme_customizer::Demo),
    ("toggle", ui::demos::toggle::Demo),
    ("toggle_group", ui::demos::toggle_group::Demo),
    ("toolbar", ui::demos::toolbar::Demo),
    ("ui_strings", ui::demos::ui_strings::Demo),
    ("virtual_list", ui::demos::virtual_list::Demo),
    ("button", ui::demos::button::Demo),
    ("badge", ui::demos::badge::Demo),
    ("input", ui::demos::input::Demo),
    ("checkbox", ui::demos::checkbox::Demo),
    ("switch", ui::demos::switch::Demo),
    ("card", ui::demos::card::Demo),
    ("tabs", ui::demos::tabs::Demo),
    ("accordion", ui::demos::accordion::Demo),
    ("dialog", ui::demos::dialog::Demo),
    ("sheet", ui::demos::sheet::Demo),
    ("popover", ui::demos::popover::Demo),
    ("dropdown_menu", ui::demos::dropdown_menu::Demo),
    ("tooltip", ui::demos::tooltip::Demo),
    ("separator", ui::demos::separator::Demo),
    ("toast", ui::demos::toast::Demo),
    ("icons", ui::demos::icons::Demo),
    ("hooks", ui::demos::hooks::Demo),
    ("chips", ui::demos::chips::Demo),
    ("link", ui::demos::link::Demo),
    ("skip_link", ui::demos::skip_link::Demo),
    ("number_field", ui::demos::number_field::Demo),
    ("time_picker", ui::demos::time_picker::Demo),
    ("date_time_picker", ui::demos::date_time_picker::Demo),
];

/// Blocks (`ui::blocks`), checked like demos. Names start with `block_`.
pub const BLOCKS: &[Demo] = &[
    ("block_dashboard_01", ui::blocks::dashboard_01::Dashboard01),
    (
        "block_login_01",
        || rsx! { ui::blocks::login_01::Login01 {} },
    ),
    (
        "block_settings_01",
        || rsx! { ui::blocks::settings_01::Settings01 {} },
    ),
    (
        "block_signup_01",
        || rsx! { ui::blocks::signup_01::Signup01 {} },
    ),
];

/// Fails if a component folder has a `<name>_demo.rs` that [`DEMOS`] does not list.
pub fn assert_demos_complete() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src/components");
    for entry in std::fs::read_dir(dir).expect("components dir") {
        let path = entry.expect("entry").path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if path.join(format!("{name}_demo.rs")).exists() {
            assert!(
                DEMOS.iter().any(|(n, _)| *n == name),
                "{name}_demo.rs is not in DEMOS in tests/common/mod.rs"
            );
        }
    }
}
