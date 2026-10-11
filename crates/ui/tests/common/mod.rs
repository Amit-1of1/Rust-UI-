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
    ("alert", ui::components::alert::alert_demo::Demo),
    (
        "alert_dialog",
        ui::components::alert_dialog::alert_dialog_demo::Demo,
    ),
    (
        "aspect_ratio",
        ui::components::aspect_ratio::aspect_ratio_demo::Demo,
    ),
    ("avatar", ui::components::avatar::avatar_demo::Demo),
    (
        "bottom_nav",
        ui::components::bottom_nav::bottom_nav_demo::Demo,
    ),
    (
        "breadcrumb",
        ui::components::breadcrumb::breadcrumb_demo::Demo,
    ),
    (
        "button_group",
        ui::components::button_group::button_group_demo::Demo,
    ),
    ("calendar", ui::components::calendar::calendar_demo::Demo),
    ("callout", ui::components::callout::callout_demo::Demo),
    ("carousel", ui::components::carousel::carousel_demo::Demo),
    ("chart", ui::components::chart::chart_demo::Demo),
    (
        "collapsible",
        ui::components::collapsible::collapsible_demo::Demo,
    ),
    (
        "color_picker",
        ui::components::color_picker::color_picker_demo::Demo,
    ),
    ("combobox", ui::components::combobox::combobox_demo::Demo),
    ("command", ui::components::command::command_demo::Demo),
    (
        "context_menu",
        ui::components::context_menu::context_menu_demo::Demo,
    ),
    (
        "data_table",
        ui::components::data_table::data_table_demo::Demo,
    ),
    (
        "date_picker",
        ui::components::date_picker::date_picker_demo::Demo,
    ),
    (
        "drag_and_drop_list",
        ui::components::drag_and_drop_list::drag_and_drop_list_demo::Demo,
    ),
    ("drawer", ui::components::drawer::drawer_demo::Demo),
    ("dropzone", ui::components::dropzone::dropzone_demo::Demo),
    ("empty", ui::components::empty::empty_demo::Demo),
    ("field", ui::components::field::field_demo::Demo),
    ("form", ui::components::form::form_demo::Demo),
    (
        "hover_card",
        ui::components::hover_card::hover_card_demo::Demo,
    ),
    (
        "input_group",
        ui::components::input_group::input_group_demo::Demo,
    ),
    ("input_otp", ui::components::input_otp::input_otp_demo::Demo),
    ("item", ui::components::item::item_demo::Demo),
    ("kbd", ui::components::kbd::kbd_demo::Demo),
    ("marquee", ui::components::marquee::marquee_demo::Demo),
    ("menubar", ui::components::menubar::menubar_demo::Demo),
    (
        "multi_select",
        ui::components::multi_select::multi_select_demo::Demo,
    ),
    (
        "pagination",
        ui::components::pagination::pagination_demo::Demo,
    ),
    ("progress", ui::components::progress::progress_demo::Demo),
    (
        "radio_group",
        ui::components::radio_group::radio_group_demo::Demo,
    ),
    (
        "scroll_area",
        ui::components::scroll_area::scroll_area_demo::Demo,
    ),
    ("select", ui::components::select::select_demo::Demo),
    ("shimmer", ui::components::shimmer::shimmer_demo::Demo),
    ("sidebar", ui::components::sidebar::sidebar_demo::Demo),
    ("skeleton", ui::components::skeleton::skeleton_demo::Demo),
    ("slider", ui::components::slider::slider_demo::Demo),
    ("spinner", ui::components::spinner::spinner_demo::Demo),
    ("status", ui::components::status::status_demo::Demo),
    ("stepper", ui::components::stepper::stepper_demo::Demo),
    ("table", ui::components::table::table_demo::Demo),
    ("tag_group", ui::components::tag_group::tag_group_demo::Demo),
    ("textarea", ui::components::textarea::textarea_demo::Demo),
    (
        "theme_customizer",
        ui::components::theme_customizer::theme_customizer_demo::Demo,
    ),
    ("toggle", ui::components::toggle::toggle_demo::Demo),
    (
        "toggle_group",
        ui::components::toggle_group::toggle_group_demo::Demo,
    ),
    ("toolbar", ui::components::toolbar::toolbar_demo::Demo),
    (
        "ui_strings",
        ui::components::ui_strings::ui_strings_demo::Demo,
    ),
    (
        "virtual_list",
        ui::components::virtual_list::virtual_list_demo::Demo,
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
