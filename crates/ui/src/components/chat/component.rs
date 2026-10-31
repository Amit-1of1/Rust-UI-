use crate::components::icons::ArrowDown;
use crate::components::ui_strings::{fill, use_ui_strings};
use dioxus::html::geometry::PixelsVector2D;
use dioxus::html::ScrollBehavior;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/chat/style.css")]
struct Styles;

/// How close to the end (in pixels) still counts as "reading the latest".
const SLACK: f64 = 24.0;

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
#[derive(Clone, Copy, PartialEq)]
pub struct ChatAnnouncer {
    /// Two polite regions, used in turn, so saying the same text twice is still heard.
    slots: Signal<[String; 2]>,
}

impl ChatAnnouncer {
    pub fn announce(&self, text: impl Into<String>) {
        let mut slots = self.slots;
        let next = usize::from(slots.peek()[1].is_empty());
        let mut fresh = [String::new(), String::new()];
        fresh[next] = text.into();
        slots.set(fresh);
    }
}

/// The announcer of the enclosing [`ChatLog`], if there is one.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_chat_announcer() -> Option<ChatAnnouncer> {
    try_use_context::<ChatAnnouncer>()
}

#[component]
fn Announcer() -> Element {
    let Some(announcer) = use_chat_announcer() else {
        return rsx! {};
    };
    let slots = announcer.slots.read();
    rsx! {
        div { class: "sr-only", "data-slot": "chat-announcer", aria_live: "polite", aria_atomic: "true", "{slots[0]}" }
        div { class: "sr-only", "data-slot": "chat-announcer", aria_live: "polite", aria_atomic: "true", "{slots[1]}" }
    }
}

/// Keeps the log at the newest message while the reader is there, and lets go when they
/// scroll up. Before each follow it compares the scroll offset with the one it last set: one
/// that moved up means the reader scrolled. That works without scroll events (Blitz has none)
/// and catches a scroll whose event has not arrived yet (browsers send them a frame late,
/// and a follow in between would otherwise undo it).
#[derive(Clone, Copy)]
struct Follow {
    el: Signal<Option<Rc<MountedData>>>,
    pinned: Signal<bool>,
    unseen: Signal<bool>,
    height: Signal<f64>,
    top: Signal<Option<f64>>,
    /// The scroll offset the last scroll event reported (web).
    seen_top: Signal<Option<f64>>,
}

impl Follow {
    async fn run(mut self) {
        let Some(el) = self.el.peek().clone() else {
            return;
        };
        let Ok(size) = el.get_scroll_size().await else {
            return;
        };
        let before = *self.height.peek();
        let grew = size.height > before + 0.5;
        let shrank = size.height + 0.5 < before;
        self.height.set(size.height);
        if !*self.pinned.peek() {
            if grew && !*self.unseen.peek() {
                self.unseen.set(true);
            }
            return;
        }
        if !shrank {
            if let (Some(top), Ok(now)) = (*self.top.peek(), el.get_scroll_offset().await) {
                if now.y + SLACK < top {
                    self.pinned.set(false);
                    if grew {
                        self.unseen.set(true);
                    }
                    return;
                }
            }
        }
        _ = el
            .scroll(
                PixelsVector2D::new(0.0, size.height),
                ScrollBehavior::Instant,
            )
            .await;
        if let Ok(now) = el.get_scroll_offset().await {
            self.top.set(Some(now.y));
        }
    }

    async fn jump(mut self) {
        self.pinned.set(true);
        self.unseen.set(false);
        self.top.set(None);
        self.run().await;
        // The button goes away; keep focus on the conversation instead of the page.
        if let Some(el) = self.el.peek().clone() {
            _ = el.set_focus(true).await;
        }
    }
}

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
) -> Element {
    let strings = use_ui_strings();
    use_context_provider(|| ChatAnnouncer {
        slots: Signal::new([String::new(), String::new()]),
    });
    let follow = Follow {
        el: use_signal(|| None),
        pinned: use_signal(|| true),
        unseen: use_signal(|| false),
        height: use_signal(|| 0.0),
        top: use_signal(|| None),
        seen_top: use_signal(|| None),
    };
    // Parents re-render the log when they add a message; follow after every render.
    dioxus::core::use_after_render(move || {
        spawn(follow.run());
    });
    let pinned = (follow.pinned)();
    let unseen = (follow.unseen)();

    rsx! {
        div {
            "data-slot": "chat",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_chat }), attributes]),
            div {
                class: Styles::dx_chat_log,
                role: "log",
                aria_label: label,
                // Not live: see `ChatAnnouncer`. Whole messages are announced instead.
                aria_live: "off",
                // Scrollable regions must be reachable by keyboard to be scrolled.
                tabindex: "0",
                onmounted: move |e: MountedEvent| {
                    let mut el = follow.el;
                    el.set(Some(e.data()));
                    spawn(follow.run());
                },
                onscroll: move |e: ScrollEvent| {
                    let mut f = follow;
                    let top = e.scroll_top();
                    let gap = f64::from(e.scroll_height()) - top - f64::from(e.client_height());
                    let at_end = gap <= SLACK;
                    // Only the reader moving up lets go. Scroll events arrive a frame late, and
                    // by then a streaming reply may have grown the content below a scroll
                    // that did reach the end (WebKit shows this often).
                    let moved_up = f.seen_top.peek().or(*f.top.peek()).is_some_and(|before| top + 1.0 < before);
                    f.seen_top.set(Some(top));
                    if at_end && !*f.pinned.peek() {
                        f.pinned.set(true);
                    } else if !at_end && moved_up && *f.pinned.peek() {
                        f.pinned.set(false);
                    }
                    if at_end && *f.unseen.peek() {
                        f.unseen.set(false);
                    }
                },
                div {
                    class: Styles::dx_chat_log_content,
                    // Streaming text grows the content without re-rendering the log.
                    onresize: move |_| {
                        spawn(follow.run());
                    },
                    {children}
                }
            }
            if !pinned {
                div { class: Styles::dx_chat_jump_row,
                    button {
                        class: Styles::dx_chat_jump,
                        r#type: "button",
                        "data-slot": "chat-jump",
                        "data-unseen": unseen.then_some("true"),
                        onclick: move |_| {
                            spawn(follow.jump());
                        },
                        ArrowDown { size: "0.875rem" }
                        if unseen {
                            "{strings.new_messages}"
                        } else {
                            "{strings.jump_to_latest}"
                        }
                    }
                }
            }
            Announcer {}
        }
    }
}

/// Who wrote a [`ChatMessage`]. Sets its alignment and look.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub enum ChatRole {
    /// The person using the app: right-aligned bubble.
    User,
    /// The assistant or another participant: full-width text.
    #[default]
    Assistant,
    /// Notices from the app ("Conversation started"): centred and muted.
    System,
}

impl ChatRole {
    fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::System => "system",
        }
    }
}

/// The author of the enclosing [`ChatMessage`], for announcements.
#[derive(Clone, PartialEq)]
struct MessageAuthor(String);

fn next_id() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
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
) -> Element {
    let author_id = use_hook(|| format!("dx-chat-author-{}", next_id()));
    let mut ctx = use_context_provider(|| Signal::new(MessageAuthor(author.clone())));
    if ctx.peek().0 != author {
        ctx.set(MessageAuthor(author.clone()));
    }
    let hide_meta = hide_author && time.is_none();

    rsx! {
        article {
            "data-slot": "chat-message",
            "data-from": from.as_str(),
            aria_labelledby: author_id.clone(),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_chat_message }), attributes]),
            if let Some(avatar) = avatar {
                div { class: Styles::dx_chat_message_avatar, aria_hidden: "true", {avatar} }
            }
            div { class: Styles::dx_chat_message_body,
                div {
                    class: Styles::dx_chat_message_meta,
                    "data-hidden": hide_meta.then_some("true"),
                    span {
                        id: author_id,
                        class: if hide_author { "sr-only".to_string() } else { Styles::dx_chat_message_author.to_string() },
                        "{author}"
                    }
                    if let Some(time) = time {
                        span { class: Styles::dx_chat_message_time, "{time}" }
                    }
                }
                div { class: Styles::dx_chat_message_content, {children} }
                if let Some(actions) = actions {
                    div { class: Styles::dx_chat_message_actions, {actions} }
                }
            }
        }
    }
}

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
) -> Element {
    let announcer = use_chat_announcer();
    let author = try_use_context::<Signal<MessageAuthor>>();
    let template = use_ui_strings().message_from;
    let was_streaming = use_hook(|| Rc::new(std::cell::Cell::new(streaming)));
    let said = text.clone();
    use_effect(use_reactive!(|(streaming, said)| {
        let ended = was_streaming.replace(streaming) && !streaming;
        if ended && announce && !said.trim().is_empty() {
            if let Some(announcer) = announcer {
                let line = match author {
                    Some(author) => fill(&template, &[("author", &author.peek().0), ("text", &said)]),
                    None => said.clone(),
                };
                announcer.announce(line);
            }
        }
    }));

    rsx! {
        div {
            "data-slot": "streaming-text",
            "data-streaming": streaming.then_some("true"),
            aria_busy: streaming.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_streaming_text }), attributes]),
            "{text}"
            if streaming {
                span { class: Styles::dx_streaming_caret, aria_hidden: "true" }
            }
        }
    }
}

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
) -> Element {
    let text = fill(&use_ui_strings().typing, &[("author", &author)]);
    let announcer = use_chat_announcer();
    let said = text.clone();
    use_effect(move || {
        if let Some(announcer) = announcer {
            announcer.announce(said.clone());
        }
    });
    rsx! {
        div {
            "data-slot": "typing-indicator",
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_typing }), attributes]),
            span { class: Styles::dx_typing_dot, aria_hidden: "true" }
            span { class: Styles::dx_typing_dot, aria_hidden: "true" }
            span { class: Styles::dx_typing_dot, aria_hidden: "true" }
            span { class: "sr-only", "{text}" }
        }
    }
}
