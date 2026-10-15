use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

#[test]
fn skip_link_takes_a_custom_target_and_text() {
    let html = render(|| rsx! { SkipLink { target: "results", "Skip to results" } });
    assert!(html.contains("href=\"#results\""), "{html}");
    assert!(html.contains("Skip to results"), "{html}");
}
