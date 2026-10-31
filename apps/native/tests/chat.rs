//! The chat set on headless Blitz: the log follows new messages while the reader is at the
//! end, lets go when they scroll up (Blitz sends no scroll events, so ChatLog has to notice
//! from the offset alone), and the jump button brings them back. Announcements, which run in
//! effects that SSR tests never reach, are checked here too.
//!
//! Run with `cargo test -p native --test chat`.

mod common;
use common::*;

use dioxus::prelude::*;
use ui::prelude::*;

/// Adds ten messages per click to a short log.
fn growing() -> Element {
    let mut count = use_signal(|| 3usize);
    rsx! {
        ui::ThemeStyles {}
        main { style: "padding: 24px; width: 480px;",
            button { id: "more", onclick: move |_| count += 10, "More" }
            div { style: "height: 240px; display: flex; flex-direction: column;",
                ChatLog { label: "Chat",
                    for i in 0..count() {
                        ChatMessage { key: "{i}", author: "Ada", class: "msg", id: "m{i}", "Message {i}" }
                    }
                }
            }
        }
    }
}

/// The log's box. Measured on its wrapper: Blitz's absolute position of a scrolled element
/// takes off its own scroll offset too.
fn log_rect(h: &Harness) -> Rect {
    h.rect("[data-slot=chat]").expect("log")
}

/// The last message's bottom edge is inside the log's box.
fn last_visible(h: &Harness, last: usize) -> bool {
    let log = log_rect(h);
    let m = h.rect(&format!("#m{last}")).expect("last message");
    m.y + m.h <= log.y + log.h + 1.0 && m.y + m.h > log.y
}

fn log_id(h: &Harness) -> usize {
    h.find("[role=log]").expect("log")
}

#[test]
fn log_follows_new_messages_until_the_reader_scrolls_up() {
    let mut h = Harness::new(growing);
    h.click("#more");
    assert!(last_visible(&h, 12), "did not follow to message 12");
    assert!(
        h.find("[data-slot=chat-jump]").is_none(),
        "jump button while at the end"
    );

    // The reader scrolls up a long way, then more arrives: stay put, offer the way back.
    let id = log_id(&h);
    h.doc.scroll_node_by(id, 0.0, 400.0);
    h.settle();
    h.click("#more");
    assert!(
        !last_visible(&h, 22),
        "followed although the reader had scrolled up"
    );
    let jump = h
        .find("[data-slot=chat-jump]")
        .expect("no jump button after scrolling up");
    let text = h.doc.get_node(jump).expect("node").text_content();
    assert!(text.contains("New messages below"), "{text}");

    h.click("[data-slot=chat-jump]");
    assert!(
        last_visible(&h, 22),
        "jump did not reach the newest message"
    );
    assert_eq!(
        h.focused(),
        Some(log_id(&h)),
        "focus did not move to the log"
    );
    assert!(
        h.find("[data-slot=chat-jump]").is_none(),
        "jump button stayed"
    );

    // Back at the end, it follows again.
    h.click("#more");
    assert!(last_visible(&h, 32), "stopped following after the jump");
}

fn announcer_text(h: &Harness) -> String {
    let mut out = String::new();
    for id in h
        .doc
        .query_selector_all("[data-slot=chat-announcer]")
        .expect("selector")
    {
        out.push_str(&h.doc.get_node(id).expect("node").text_content());
        out.push('|');
    }
    out
}

fn streaming() -> Element {
    let mut done = use_signal(|| false);
    let mut typing = use_signal(|| true);
    rsx! {
        ui::ThemeStyles {}
        main { style: "padding: 24px; width: 480px;",
            button { id: "finish", onclick: move |_| { typing.set(false); done.set(true) }, "Finish" }
            div { style: "height: 300px; display: flex; flex-direction: column;",
                ChatLog { label: "Chat",
                    ChatMessage { author: "Ada",
                        StreamingText {
                            text: if done() { "All done, tests pass." } else { "All do" },
                            streaming: !done(),
                        }
                    }
                    if typing() {
                        TypingIndicator { author: "Ada" }
                    }
                }
            }
        }
    }
}

#[test]
fn whole_reply_is_announced_once_when_streaming_ends() {
    let mut h = Harness::new(streaming);
    let before = announcer_text(&h);
    assert!(before.contains("Ada is typing"), "{before}");
    assert!(!before.contains("All do"), "fragment announced: {before}");
    h.click("#finish");
    let after = announcer_text(&h);
    assert!(after.contains("Ada: All done, tests pass."), "{after}");
    assert!(!after.contains("typing"), "old announcement kept: {after}");
}

fn approval() -> Element {
    let mut status = use_signal(|| ToolStatus::Running);
    rsx! {
        ui::ThemeStyles {}
        main { style: "padding: 24px; width: 480px;",
            button { id: "ask", onclick: move |_| status.set(ToolStatus::NeedsApproval), "Ask" }
            div { style: "height: 300px; display: flex; flex-direction: column;",
                ChatLog { label: "Chat",
                    ToolCall { name: "deploy_preview", status: status(), "details" }
                }
            }
        }
    }
}

#[test]
fn tool_call_needing_approval_is_announced() {
    let mut h = Harness::new(approval);
    assert!(!announcer_text(&h).contains("deploy_preview"));
    h.click("#ask");
    let text = announcer_text(&h);
    assert!(text.contains("deploy_preview: Needs approval"), "{text}");
}

#[test]
fn composer_sends_on_enter_and_grows_the_log_on_native() {
    fn app() -> Element {
        rsx! {
            ui::ThemeStyles {}
            main { style: "padding: 24px; max-width: 760px;", ui::demos::chat::Demo {} }
        }
    }
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("runtime");
    let _rt = rt.enter();
    let mut h = Harness::new(app);
    h.click("textarea");
    h.type_text("Does it work natively?");
    h.key(Key::Enter, Code::Enter, Modifiers::empty());
    let log = h.doc.get_node(log_id(&h)).expect("log").text_content();
    assert!(log.contains("Does it work natively?"), "{log}");
    let field = h.find("textarea").expect("textarea");
    assert_eq!(h.focused(), Some(field), "focus left the message box");
    // The reply is streaming: Send became Stop.
    assert!(
        h.find("[data-slot=chat-composer-stop]").is_some(),
        "no Stop while busy"
    );
}
