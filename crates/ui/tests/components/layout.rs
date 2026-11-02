use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

#[test]
fn stack_spaces_children_from_the_unit_scale() {
    let html = render(|| rsx! { Stack { gap: 3, direction: StackDirection::Row, "a" "b" } });
    assert!(html.contains(r#"data-gap="3""#), "{html}");
    assert!(html.contains(r#"data-direction="row""#), "{html}");
    // Off-scale gaps round down to a step that has a token.
    let html = render(|| rsx! { Stack { gap: 11, "a" } });
    assert!(html.contains(r#"data-gap="10""#), "{html}");
}

#[test]
fn grid_follows_the_page_grid_unless_told_otherwise() {
    let html = render(|| rsx! { Grid { GridItem { span: 4, span_l: 6, "x" } } });
    assert!(!html.contains("data-columns"), "{html}");
    assert!(html.contains(r#"data-span="4""#) && html.contains(r#"data-span-l="6""#), "{html}");
    let html = render(|| rsx! { Grid { columns: 20, min_item: "12rem", "x" } });
    assert!(html.contains(r#"data-columns="12""#), "clamped to 12: {html}");
    assert!(html.contains("--dx-grid-min-item: 12rem"), "{html}");
}

#[test]
fn text_renders_the_element_it_is_asked_for() {
    let html = render(|| rsx! { Text { variant: TextVariant::Title2, as_: TextAs::H2, color: TextColor::Faded, "Billing" } });
    assert!(html.starts_with("<h2") || html.contains("<h2 "), "{html}");
    assert!(html.contains(r#"data-variant="title-2""#), "{html}");
    assert!(html.contains(r#"data-color="faded""#), "{html}");
    let html = render(|| rsx! { Text { "plain" } });
    assert!(html.contains("<span") && html.contains(r#"data-variant="body-2""#), "{html}");
}

#[test]
fn container_keeps_the_callers_class() {
    let html = render(|| rsx! { Container { class: "mine", "x" } });
    assert!(html.contains("mine"), "{html}");
}
