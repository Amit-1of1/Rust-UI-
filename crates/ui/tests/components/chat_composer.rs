use super::render;
use dioxus::prelude::*;
use ui::components::chat_composer::{composer_key, ComposerKey};
use ui::prelude::*;

#[test]
fn chat_composer_names_its_box_and_says_how_to_send() {
    let html = render(|| rsx! { ChatComposer { label: "Message", onsend: move |_: String| {} } });
    assert!(html.contains("<textarea"), "{html}");
    assert!(html.contains("aria-label=\"Message\""), "{html}");
    assert!(html.contains("aria-keyshortcuts=\"Enter\""), "{html}");
    let hint = html
        .split("aria-describedby=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("aria-describedby");
    assert!(html.contains(&format!("id=\"{hint}\"")), "{html}");
    assert!(html.contains("Enter sends"), "{html}");
    // Empty: Send is named but marked unavailable, not removed from the tab order.
    assert!(html.contains("aria-label=\"Send\""), "{html}");
    assert!(html.contains("aria-disabled=\"true\""), "{html}");
    assert!(!html.contains(" disabled"), "{html}");
}

#[test]
fn busy_composer_offers_stop() {
    let html = render(|| {
        rsx! { ChatComposer { label: "Message", busy: true, onsend: move |_: String| {}, onstop: move |_| {} } }
    });
    assert!(html.contains(">Stop"), "{html}");
    assert!(!html.contains("aria-label=\"Send\""), "{html}");
}

#[test]
fn enter_sends_shift_enter_adds_a_line_and_ime_never_sends() {
    use dioxus::prelude::Key;
    assert_eq!(composer_key(&Key::Enter, false, false, false), ComposerKey::Send);
    assert_eq!(composer_key(&Key::Enter, true, false, false), ComposerKey::Default);
    // Enter that confirms an IME composition picks the characters.
    assert_eq!(composer_key(&Key::Enter, false, true, false), ComposerKey::Default);
    assert_eq!(composer_key(&Key::Escape, false, false, true), ComposerKey::Stop);
    assert_eq!(composer_key(&Key::Escape, false, false, false), ComposerKey::Default);
    assert_eq!(
        composer_key(&Key::Character("a".into()), false, false, false),
        ComposerKey::Default
    );
}
