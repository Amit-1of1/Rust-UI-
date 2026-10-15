//! Every demo renders on the server without panicking. This is a smoke test, not a
//! behaviour test; behaviour is checked in the browser (docs/GUARDRAILS.md).

use dioxus::prelude::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn alert_dialog_demo_renders() {
    let html = render(ui::demos::alert_dialog::Demo);
    assert!(!html.is_empty());
}

#[test]
fn aspect_ratio_demo_renders() {
    let html = render(ui::demos::aspect_ratio::Demo);
    assert!(!html.is_empty());
}

#[test]
fn avatar_demo_renders() {
    let html = render(ui::demos::avatar::Demo);
    assert!(!html.is_empty());
}

#[test]
fn calendar_demo_renders() {
    let html = render(ui::demos::calendar::Demo);
    assert!(!html.is_empty());
}

#[test]
fn collapsible_demo_renders() {
    let html = render(ui::demos::collapsible::Demo);
    assert!(!html.is_empty());
}

#[test]
fn color_picker_demo_renders() {
    let html = render(ui::demos::color_picker::Demo);
    assert!(!html.is_empty());
}

#[test]
fn combobox_demo_renders() {
    let html = render(ui::demos::combobox::Demo);
    assert!(!html.is_empty());
}

#[test]
fn context_menu_demo_renders() {
    let html = render(ui::demos::context_menu::Demo);
    assert!(!html.is_empty());
}

#[test]
fn date_picker_demo_renders() {
    let html = render(ui::demos::date_picker::Demo);
    assert!(!html.is_empty());
}

#[test]
fn drag_and_drop_list_demo_renders() {
    let html = render(ui::demos::drag_and_drop_list::Demo);
    assert!(!html.is_empty());
}

#[test]
fn hover_card_demo_renders() {
    let html = render(ui::demos::hover_card::Demo);
    assert!(!html.is_empty());
}

#[test]
fn item_demo_renders() {
    let html = render(ui::demos::item::Demo);
    assert!(!html.is_empty());
}

#[test]
fn menubar_demo_renders() {
    let html = render(ui::demos::menubar::Demo);
    assert!(!html.is_empty());
}

#[test]
fn pagination_demo_renders() {
    let html = render(ui::demos::pagination::Demo);
    assert!(!html.is_empty());
}

#[test]
fn progress_demo_renders() {
    let html = render(ui::demos::progress::Demo);
    assert!(!html.is_empty());
}

#[test]
fn radio_group_demo_renders() {
    let html = render(ui::demos::radio_group::Demo);
    assert!(!html.is_empty());
}

#[test]
fn scroll_area_demo_renders() {
    let html = render(ui::demos::scroll_area::Demo);
    assert!(!html.is_empty());
}

#[test]
fn select_demo_renders() {
    let html = render(ui::demos::select::Demo);
    assert!(!html.is_empty());
}

#[test]
fn sidebar_demo_renders() {
    let html = render(ui::demos::sidebar::Demo);
    assert!(!html.is_empty());
}

#[test]
fn skeleton_demo_renders() {
    let html = render(ui::demos::skeleton::Demo);
    assert!(!html.is_empty());
}

#[test]
fn slider_demo_renders() {
    let html = render(ui::demos::slider::Demo);
    assert!(!html.is_empty());
}

#[test]
fn tag_group_demo_renders() {
    let html = render(ui::demos::tag_group::Demo);
    assert!(!html.is_empty());
}

#[test]
fn textarea_demo_renders() {
    let html = render(ui::demos::textarea::Demo);
    assert!(!html.is_empty());
}

#[test]
fn toggle_demo_renders() {
    let html = render(ui::demos::toggle::Demo);
    assert!(!html.is_empty());
}

#[test]
fn toggle_group_demo_renders() {
    let html = render(ui::demos::toggle_group::Demo);
    assert!(!html.is_empty());
}

#[test]
fn toolbar_demo_renders() {
    let html = render(ui::demos::toolbar::Demo);
    assert!(!html.is_empty());
}

#[test]
fn virtual_list_demo_renders() {
    let html = render(ui::demos::virtual_list::Demo);
    assert!(!html.is_empty());
}

#[test]
fn alert_demo_renders() {
    let html = render(ui::demos::alert::Demo);
    assert!(!html.is_empty());
}

#[test]
fn breadcrumb_demo_renders() {
    let html = render(ui::demos::breadcrumb::Demo);
    assert!(!html.is_empty());
}

#[test]
fn kbd_demo_renders() {
    let html = render(ui::demos::kbd::Demo);
    assert!(!html.is_empty());
}

#[test]
fn label_demo_renders() {
    let html = render(ui::demos::label::Demo);
    assert!(!html.is_empty());
}

#[test]
fn spinner_demo_renders() {
    let html = render(ui::demos::spinner::Demo);
    assert!(!html.is_empty());
}

#[test]
fn field_demo_renders() {
    let html = render(ui::demos::field::Demo);
    assert!(!html.is_empty());
}

#[test]
fn table_demo_renders() {
    let html = render(ui::demos::table::Demo);
    assert!(!html.is_empty());
}

#[test]
fn command_demo_renders() {
    let html = render(ui::demos::command::Demo);
    assert!(!html.is_empty());
}

#[test]
fn chart_demo_renders() {
    let html = render(ui::demos::chart::Demo);
    assert!(!html.is_empty());
}

#[test]
fn theme_customizer_demo_renders() {
    let html = render(ui::demos::theme_customizer::Demo);
    assert!(!html.is_empty());
}

#[test]
fn callout_demo_renders() {
    let html = render(ui::demos::callout::Demo);
    assert!(!html.is_empty());
}

#[test]
fn empty_demo_renders() {
    let html = render(ui::demos::empty::Demo);
    assert!(!html.is_empty());
}

#[test]
fn status_demo_renders() {
    let html = render(ui::demos::status::Demo);
    assert!(!html.is_empty());
}

#[test]
fn shimmer_demo_renders() {
    let html = render(ui::demos::shimmer::Demo);
    assert!(!html.is_empty());
}

#[test]
fn marquee_demo_renders() {
    let html = render(ui::demos::marquee::Demo);
    assert!(!html.is_empty());
}

#[test]
fn button_group_demo_renders() {
    let html = render(ui::demos::button_group::Demo);
    assert!(!html.is_empty());
}

#[test]
fn stepper_demo_renders() {
    let html = render(ui::demos::stepper::Demo);
    assert!(!html.is_empty());
}

#[test]
fn bottom_nav_demo_renders() {
    let html = render(ui::demos::bottom_nav::Demo);
    assert!(!html.is_empty());
}

#[test]
fn form_demo_renders() {
    let html = render(ui::demos::form::Demo);
    assert!(!html.is_empty());
}

#[test]
fn data_table_demo_renders() {
    let html = render(ui::demos::data_table::Demo);
    assert!(!html.is_empty());
}

#[test]
fn input_otp_demo_renders() {
    let html = render(ui::demos::input_otp::Demo);
    assert!(!html.is_empty());
}

#[test]
fn input_group_demo_renders() {
    let html = render(ui::demos::input_group::Demo);
    assert!(!html.is_empty());
}

#[test]
fn carousel_demo_renders() {
    let html = render(ui::demos::carousel::Demo);
    assert!(!html.is_empty());
}

#[test]
fn dropzone_demo_renders() {
    let html = render(ui::demos::dropzone::Demo);
    assert!(!html.is_empty());
}

#[test]
fn multi_select_demo_renders() {
    let html = render(ui::demos::multi_select::Demo);
    assert!(!html.is_empty());
}

#[test]
fn drawer_demo_renders() {
    let html = render(ui::demos::drawer::Demo);
    assert!(!html.is_empty());
}

#[test]
fn button_demo_renders() {
    let html = render(ui::demos::button::Demo);
    assert!(!html.is_empty());
}

#[test]
fn badge_demo_renders() {
    let html = render(ui::demos::badge::Demo);
    assert!(!html.is_empty());
}

#[test]
fn input_demo_renders() {
    let html = render(ui::demos::input::Demo);
    assert!(!html.is_empty());
}

#[test]
fn checkbox_demo_renders() {
    let html = render(ui::demos::checkbox::Demo);
    assert!(!html.is_empty());
}

#[test]
fn switch_demo_renders() {
    let html = render(ui::demos::switch::Demo);
    assert!(!html.is_empty());
}

#[test]
fn card_demo_renders() {
    let html = render(ui::demos::card::Demo);
    assert!(!html.is_empty());
}

#[test]
fn tabs_demo_renders() {
    let html = render(ui::demos::tabs::Demo);
    assert!(!html.is_empty());
}

#[test]
fn accordion_demo_renders() {
    let html = render(ui::demos::accordion::Demo);
    assert!(!html.is_empty());
}

#[test]
fn dialog_demo_renders() {
    let html = render(ui::demos::dialog::Demo);
    assert!(!html.is_empty());
}

#[test]
fn sheet_demo_renders() {
    let html = render(ui::demos::sheet::Demo);
    assert!(!html.is_empty());
}

#[test]
fn popover_demo_renders() {
    let html = render(ui::demos::popover::Demo);
    assert!(!html.is_empty());
}

#[test]
fn dropdown_menu_demo_renders() {
    let html = render(ui::demos::dropdown_menu::Demo);
    assert!(!html.is_empty());
}

#[test]
fn tooltip_demo_renders() {
    let html = render(ui::demos::tooltip::Demo);
    assert!(!html.is_empty());
}

#[test]
fn separator_demo_renders() {
    let html = render(ui::demos::separator::Demo);
    assert!(!html.is_empty());
}

#[test]
fn toast_demo_renders() {
    let html = render(ui::demos::toast::Demo);
    assert!(html.contains("<button"));
}

#[test]
fn ui_strings_demo_renders() {
    let html = render(ui::demos::ui_strings::Demo);
    assert!(html.contains("Previous"), "{html}");
}

#[test]
fn icons_demo_renders() {
    let html = render(ui::demos::icons::Demo);
    assert!(!html.is_empty());
}

#[test]
fn hooks_demo_renders() {
    let html = render(ui::demos::hooks::Demo);
    assert!(!html.is_empty());
}

#[test]
fn chips_demo_renders() {
    let html = render(ui::demos::chips::Demo);
    assert!(!html.is_empty());
}

#[test]
fn link_demo_renders() {
    let html = render(ui::demos::link::Demo);
    assert!(!html.is_empty());
}

#[test]
fn time_picker_demo_renders() {
    let html = render(ui::demos::time_picker::Demo);
    assert!(!html.is_empty());
}

#[test]
fn date_time_picker_demo_renders() {
    let html = render(ui::demos::date_time_picker::Demo);
    assert!(!html.is_empty());
}
