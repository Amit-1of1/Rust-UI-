use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

#[test]
fn chat_log_is_a_named_log_that_is_not_itself_live() {
    let html = render(|| rsx! { ChatLog { label: "Conversation with Ada", "x" } });
    assert!(html.contains("role=\"log\""), "{html}");
    assert!(html.contains("aria-label=\"Conversation with Ada\""), "{html}");
    assert!(html.contains("aria-live=\"off\""), "{html}");
    assert!(html.contains("tabindex=\"0\""), "{html}");
    // Two polite regions take the announcements instead.
    assert_eq!(html.matches("aria-live=\"polite\"").count(), 2, "{html}");
    // Nothing to jump to yet.
    assert!(!html.contains("Jump to latest"), "{html}");
}

#[test]
fn chat_message_is_an_article_named_by_its_author() {
    let html = render(|| {
        rsx! { ChatMessage { author: "Ada", time: "10:42", "Hello" } }
    });
    assert!(html.starts_with("<article"), "{html}");
    let id = html
        .split("aria-labelledby=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("aria-labelledby");
    assert!(html.contains(&format!("id=\"{id}\"")), "{html}");
    assert!(html.contains(">Ada</span>"), "{html}");
    assert!(html.contains("10:42"), "{html}");
    assert!(html.contains("data-from=\"assistant\""), "{html}");
}

#[test]
fn hidden_author_is_still_read() {
    let html = render(|| {
        rsx! { ChatMessage { author: "You", from: ChatRole::User, hide_author: true, "Hi" } }
    });
    assert!(html.contains("class=\"sr-only\""), "{html}");
    assert!(html.contains(">You</span>"), "{html}");
    assert!(html.contains("data-from=\"user\""), "{html}");
}

#[test]
fn streaming_text_is_busy_with_a_decorative_caret_until_done() {
    let html = render(|| rsx! { StreamingText { text: "Hel", streaming: true } });
    assert!(html.contains("aria-busy=\"true\""), "{html}");
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
    let html = render(|| rsx! { StreamingText { text: "Hello" } });
    assert!(!html.contains("aria-busy"), "{html}");
    assert!(!html.contains("aria-hidden"), "{html}");
    assert!(html.contains("Hello"), "{html}");
}

#[test]
fn typing_indicator_says_who_in_words() {
    let html = render(|| rsx! { TypingIndicator { author: "Ada" } });
    assert!(html.contains("Ada is typing"), "{html}");
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 3, "{html}");
}

// Announcements run in effects, which SSR never runs: apps/native/tests/chat.rs checks them.
