use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

#[test]
fn list_renders_its_children() {
    let html = render(|| rsx! { List { ListItem { "hello" } } });
    assert!(html.contains("<ul"), "{html}");
    assert!(html.contains("<li"), "{html}");
    assert!(html.contains("hello"), "{html}");
}

#[test]
fn ordered_markers_render_an_ol_and_plain_lists_keep_the_list_role() {
    let html = render(|| rsx! { List { marker: ListMarker::Number, ListItem { "a" } } });
    assert!(html.starts_with("<ol"), "{html}");
    let html = render(|| rsx! { List { marker: ListMarker::None, ListItem { "a" } } });
    assert!(html.contains("role=\"list\""), "{html}");
    let html = render(|| rsx! { List { ListItem { "a" } } });
    assert!(!html.contains("role="), "a bulleted list needs no role: {html}");
}
