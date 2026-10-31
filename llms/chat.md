# Chat

- Copy into your app: `rust-ui add chat` or `dx components add chat --git https://github.com/Amit-1of1/Rust-UI-` (also copies `icons`, `ui_strings`).
- Import: `use crate::components::chat::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

A conversation view for chat and agent apps: `ChatLog` (the scrolling log), `ChatMessage`
(one message, named by its author), `StreamingText` (a reply that arrives a piece at a time)
and `TypingIndicator`. Pair it with [`ChatComposer`](chat_composer) for input and
[`ToolCall`](tool_call) for an agent's tool calls.

```rust
ChatLog { label: "Conversation with Ada", style: "height: 24rem",
    ChatMessage { author: "You", from: ChatRole::User, hide_author: true, "Hi!" }
    ChatMessage { author: "Ada",
        StreamingText { text: reply(), streaming: streaming() }
    }
    if typing() { TypingIndicator { author: "Ada" } }
}
```

## Scrolling

The log follows new content while the reader is at the end, and stops the moment they
scroll up; a "Jump to latest" button then appears (it says "New messages below" once
something arrives) and takes them back, keeping focus in the log. Following is measured, not
event-driven: it re-checks after each render of the log and whenever its content changes
size (a `ResizeObserver` on the web), so streamed text that grows inside a child component is
followed too. On the native renderer, which sends no scroll events, a scroll offset that moved
up since the last follow counts as the reader scrolling away.

## Accessibility

- Role and name: the log is `role="log"` with a required `label` (a compile error without one).
  Each message is an `article` named by its required `author`, so screen reader users can
  move between messages and hear who wrote each. `hide_author` hides the name visually only.
- Live regions: the log is deliberately `aria-live="off"`. A streamed reply changes many times
  a second and would be read in fragments. Instead the log owns a polite announcer:
  `StreamingText` announces "Author: whole reply" once when `streaming` turns off, and
  `TypingIndicator` announces "Author is typing" when it appears. Call
  `use_chat_announcer()` inside the log to announce anything else. Turn `announce` off on
  text the reader sent themselves.
- `StreamingText` sets `aria-busy` while streaming; its caret is decorative.
- The log is focusable (`tabindex="0"`) so it can be scrolled from the keyboard.
- Reduced motion: the caret stops blinking, the typing dots stop bouncing (and stay visible),
  and messages and the jump button appear without rising.

## Source

Original to this repo. The "announce whole messages, not the stream" approach follows
WAI-ARIA's `aria-busy` guidance and common screen reader behaviour with chat logs.

## Stability

Experimental: new on 2026-10-30, and the props may change once a real agent app uses them (docs/STABILITY.md).

## Keyboard and accessibility

Pattern: Log. Roles: `log`. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | In the message box | Sends the message; it appears at the end of the conversation and focus stays in the box. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// Says things to screen readers on behalf of a [`ChatLog`].
///
/// The log itself is not a live region: a streamed reply changes dozens of times a second, and
/// screen readers would read every fragment. The log announces whole messages through this
/// instead: [`StreamingText`] says the finished reply, [`TypingIndicator`] says who is typing.
/// Call [`announce`](Self::announce) for anything else worth hearing (a failed tool call, a new
/// message from someone else).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct ChatAnnouncer {
    /// Two polite regions, used in turn, so saying the same text twice is still heard.
    slots: Signal<[String; 2]>,
}

/// The announcer of the enclosing [`ChatLog`], if there is one.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_chat_announcer() -> Option<ChatAnnouncer>

/// A scrolling conversation. It follows new messages while the reader is at the end, stops
/// when they scroll up, and then shows a "Jump to latest" button (which says "New messages
/// below" once something arrives). Size it with `style` or a class; it fills its parent's
/// height by default.
///
/// `label` is required: the log is a named region (`role="log"`), and a page with an unnamed
/// one gives screen reader users no way to tell conversations apart.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! {
///     ChatLog { label: "Conversation with Ada",
///         ChatMessage { author: "Ada", from: ChatRole::Assistant, "Hello!" }
///     }
/// }
/// # }
/// ```
///
/// Leaving `label` out is a compile error:
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { ChatLog { ChatMessage { author: "Ada", "Hello!" } } }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ChatLog(
    /// The conversation's accessible name ("Conversation with Ada").
    #[props(into)]
    label: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Who wrote a [`ChatMessage`]. Sets its alignment and look.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub enum ChatRole {
    /// The person using the app: right-aligned bubble.
    User,
    /// The assistant or another participant: full-width text.
    #[default]
    Assistant,
    /// Notices from the app ("Conversation started"): centred and muted.
    System,
}

/// One message in a [`ChatLog`]: an `article` named by its author, so screen reader users can
/// move from message to message and hear who wrote each.
///
/// `author` is required for the same reason; `hide_author` hides it visually (for the
/// user's own messages, say) while it is still read.
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { ChatMessage { from: ChatRole::User, "Hi" } }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ChatMessage(
    /// Who wrote it ("You", "Assistant", "Ada").
    #[props(into)]
    author: String,
    #[props(default)] from: ChatRole,
    /// Hide the author's name visually; screen readers still read it.
    #[props(default)]
    hide_author: bool,
    /// When it was sent, as the app wants it shown ("10:42").
    #[props(into)]
    time: Option<String>,
    /// Shown beside the message, decorative (the author's name is the label).
    avatar: Option<Element>,
    /// Buttons under the message (copy, retry). Give each a name.
    actions: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Text that arrives a piece at a time (a model's reply). Shows a caret while `streaming`,
/// keeps line breaks, and when streaming ends announces the whole text once, with its
/// author, through the [`ChatLog`]'s announcer. Screen readers never hear the fragments.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn StreamingText(
    /// Everything received so far.
    #[props(into)]
    text: String,
    /// More text is still coming.
    #[props(default)]
    streaming: bool,
    /// Announce the finished text (default). Turn off for text the reader has just sent.
    #[props(default = true)]
    announce: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element

/// Three pulsing dots for "someone is writing". The dots are decorative; the text
/// ("Assistant is typing") is read, and announced once when it appears inside a [`ChatLog`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn TypingIndicator(
    /// Who is typing. Fills [`UiStrings::typing`](crate::components::ui_strings::UiStrings).
    #[props(into)]
    author: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
```

## Example

```rust
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
```
