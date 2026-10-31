use crate::components::chat::*;
use crate::components::chat_composer::*;
use crate::components::tool_call::*;
use dioxus::prelude::*;
use dioxus_sdk_time::sleep;
use std::time::Duration;

#[derive(Clone, PartialEq)]
enum Item {
    System(String),
    User(String),
    Tool {
        name: &'static str,
        summary: String,
        status: ToolStatus,
    },
    Reply {
        text: String,
        streaming: bool,
    },
}

const ANSWER: &str = "Keep the log pinned to the end while the reader is there, and let go as soon as they scroll up. ChatLog measures instead of listening for scroll events, so this works on the native renderer too. Screen readers hear each reply once, when it is complete, not every fragment as it streams in.";

fn reply_to(question: &str) -> String {
    format!("You asked: “{question}”. {ANSWER}")
}

/// A pretend agent: the reply streams in word by word after a tool call. Nothing leaves the
/// page.
#[component]
pub fn Demo() -> Element {
    let mut items = use_signal(|| {
        vec![
            Item::System("Conversation started".into()),
            Item::User("How do I keep a chat scrolled to the newest message?".into()),
            Item::Tool {
                name: "search_docs",
                summary: "“scroll to latest”, 3 results".into(),
                status: ToolStatus::Succeeded,
            },
            Item::Reply {
                text: ANSWER.into(),
                streaming: false,
            },
        ]
    });
    let mut busy = use_signal(|| false);
    let mut typing = use_signal(|| false);
    let mut stop = use_signal(|| false);

    let ask = move |question: String| {
        items.write().push(Item::User(question.clone()));
        busy.set(true);
        stop.set(false);
        typing.set(true);
        spawn(async move {
            sleep(Duration::from_millis(500)).await;
            items.write().push(Item::Tool {
                name: "search_docs",
                summary: format!("“{question}”"),
                status: ToolStatus::Running,
            });
            sleep(Duration::from_millis(700)).await;
            if let Some(Item::Tool { status, summary, .. }) = items.write().last_mut() {
                *status = ToolStatus::Succeeded;
                summary.push_str(", 3 results");
            }
            typing.set(false);
            items.write().push(Item::Reply {
                text: String::new(),
                streaming: true,
            });
            for word in reply_to(&question).split_inclusive(' ') {
                if stop() {
                    break;
                }
                sleep(Duration::from_millis(45)).await;
                if let Some(Item::Reply { text, .. }) = items.write().last_mut() {
                    text.push_str(word);
                }
            }
            if let Some(Item::Reply { streaming, .. }) = items.write().last_mut() {
                *streaming = false;
            }
            busy.set(false);
        });
    };

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "0.75rem",
            height: "28rem",
            max_width: "40rem",
            ChatLog { label: "Conversation with the docs assistant",
                for (i, item) in items().into_iter().enumerate() {
                    match item {
                        Item::System(text) => rsx! {
                            ChatMessage { key: "{i}", author: "System", from: ChatRole::System, hide_author: true, "{text}" }
                        },
                        Item::User(text) => rsx! {
                            ChatMessage { key: "{i}", author: "You", from: ChatRole::User, hide_author: true, "{text}" }
                        },
                        Item::Tool { name, summary, status } => rsx! {
                            ToolCall { key: "{i}", name, summary, status,
                                pre { "{{ \"query\": \"scroll to latest\", \"limit\": 3 }}" }
                            }
                        },
                        Item::Reply { text, streaming } => rsx! {
                            ChatMessage { key: "{i}", author: "Assistant",
                                StreamingText { text, streaming }
                            }
                        },
                    }
                }
                if typing() {
                    TypingIndicator { author: "Assistant" }
                }
            }
            ChatComposer {
                label: "Message the docs assistant",
                placeholder: "Ask about the kit…",
                busy: busy(),
                onsend: ask,
                onstop: move |_| stop.set(true),
            }
        }
    }
}
