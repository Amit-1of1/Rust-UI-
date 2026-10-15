//! Every demo renders on the server without panicking. This is a smoke test, not a
//! behaviour test; behaviour is checked in the browser (docs/GUARDRAILS.md).
//!
//! The list comes from `ui::demos::ALL`, generated from `src/demos`, so a new demo is covered
//! without adding a test here.

mod common;

use common::{render, DEMOS};

#[test]
fn every_demo_renders() {
    assert!(DEMOS.len() > 50, "only {} demos found", DEMOS.len());
    for (name, demo) in DEMOS {
        let html = render(*demo);
        assert!(!html.is_empty(), "{name} rendered nothing");
    }
}

#[test]
fn toast_demo_renders_a_button() {
    let html = render(ui::demos::toast::Demo);
    assert!(html.contains("<button"));
}

#[test]
fn ui_strings_demo_renders_translated_text() {
    let html = render(ui::demos::ui_strings::Demo);
    assert!(html.contains("Previous"), "{html}");
}
