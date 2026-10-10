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
    let html = render(ui::components::alert_dialog::alert_dialog_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn aspect_ratio_demo_renders() {
    let html = render(ui::components::aspect_ratio::aspect_ratio_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn avatar_demo_renders() {
    let html = render(ui::components::avatar::avatar_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn calendar_demo_renders() {
    let html = render(ui::components::calendar::calendar_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn collapsible_demo_renders() {
    let html = render(ui::components::collapsible::collapsible_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn color_picker_demo_renders() {
    let html = render(ui::components::color_picker::color_picker_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn combobox_demo_renders() {
    let html = render(ui::components::combobox::combobox_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn context_menu_demo_renders() {
    let html = render(ui::components::context_menu::context_menu_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn date_picker_demo_renders() {
    let html = render(ui::components::date_picker::date_picker_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn drag_and_drop_list_demo_renders() {
    let html = render(ui::components::drag_and_drop_list::drag_and_drop_list_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn hover_card_demo_renders() {
    let html = render(ui::components::hover_card::hover_card_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn item_demo_renders() {
    let html = render(ui::components::item::item_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn menubar_demo_renders() {
    let html = render(ui::components::menubar::menubar_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn pagination_demo_renders() {
    let html = render(ui::components::pagination::pagination_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn progress_demo_renders() {
    let html = render(ui::components::progress::progress_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn radio_group_demo_renders() {
    let html = render(ui::components::radio_group::radio_group_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn scroll_area_demo_renders() {
    let html = render(ui::components::scroll_area::scroll_area_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn select_demo_renders() {
    let html = render(ui::components::select::select_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn sidebar_demo_renders() {
    let html = render(ui::components::sidebar::sidebar_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn skeleton_demo_renders() {
    let html = render(ui::components::skeleton::skeleton_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn slider_demo_renders() {
    let html = render(ui::components::slider::slider_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn tag_group_demo_renders() {
    let html = render(ui::components::tag_group::tag_group_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn textarea_demo_renders() {
    let html = render(ui::components::textarea::textarea_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn toggle_demo_renders() {
    let html = render(ui::components::toggle::toggle_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn toggle_group_demo_renders() {
    let html = render(ui::components::toggle_group::toggle_group_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn toolbar_demo_renders() {
    let html = render(ui::components::toolbar::toolbar_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn virtual_list_demo_renders() {
    let html = render(ui::components::virtual_list::virtual_list_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn alert_demo_renders() {
    let html = render(ui::components::alert::alert_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn breadcrumb_demo_renders() {
    let html = render(ui::components::breadcrumb::breadcrumb_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn kbd_demo_renders() {
    let html = render(ui::components::kbd::kbd_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn spinner_demo_renders() {
    let html = render(ui::components::spinner::spinner_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn field_demo_renders() {
    let html = render(ui::components::field::field_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn table_demo_renders() {
    let html = render(ui::components::table::table_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn command_demo_renders() {
    let html = render(ui::components::command::command_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn chart_demo_renders() {
    let html = render(ui::components::chart::chart_demo::Demo);
    assert!(!html.is_empty());
}

#[test]
fn theme_customizer_demo_renders() {
    let html = render(ui::components::theme_customizer::theme_customizer_demo::Demo);
    assert!(!html.is_empty());
}
