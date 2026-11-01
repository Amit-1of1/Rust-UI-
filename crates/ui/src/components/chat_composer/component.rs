use crate::components::button::{Button, ButtonSize, ButtonVariant, IconButton, IconButtonSize};
use crate::components::icons::{ArrowUp, Square};
use crate::components::textarea::{Textarea, TextareaVariant};
use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/chat_composer/style.css")]
struct Styles;

/// What a key press in the composer does. Kept apart from the component so the rules are
/// tested on their own (`tests/components/chat_composer.rs`).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ComposerKey {
    /// Send the message (and keep the newline out of the box).
    Send,
    /// Stop the reply that is streaming.
    Stop,
    /// Let the text box handle it (type, add a line, move the caret).
    Default,
}

/// Enter sends, Shift+Enter adds a line, Escape stops a running reply. Enter that confirms an
/// IME composition (Japanese, Chinese, Korean input) never sends: it picks the characters.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn composer_key(key: &Key, shift: bool, composing: bool, busy: bool) -> ComposerKey {
    match key {
        _ if composing => ComposerKey::Default,
        Key::Named(NamedKey::Enter) if !shift => ComposerKey::Send,
        Key::Named(NamedKey::Escape) if busy => ComposerKey::Stop,
        _ => ComposerKey::Default,
    }
}

fn next_id() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// The message box of a chat: grows with the text, sends on Enter (Shift+Enter for a new
/// line), and turns its send button into Stop while a reply is `busy`. Sending trims the text,
/// ignores an empty message, clears the box and keeps focus in it.
///
/// `label` is required: it is the text box's only accessible name (the placeholder is not
/// one). How to send is read after it.
///
/// ```
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { ChatComposer { label: "Message", onsend: move |text: String| println!("{text}") } }
/// # }
/// ```
///
/// Leaving `label` out is a compile error:
///
/// ```compile_fail,E0061
/// # use dioxus::prelude::*;
/// # use ui::prelude::*;
/// # fn app() -> Element {
/// rsx! { ChatComposer { onsend: move |_: String| {} } }
/// # }
/// ```
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ChatComposer(
    /// The text box's accessible name ("Message", "Ask the agent").
    #[props(into)]
    label: String,
    /// Called with the trimmed text when the reader sends.
    onsend: EventHandler<String>,
    #[props(into)] placeholder: Option<String>,
    /// A reply is streaming: sending waits, and Stop (with `onstop`) replaces Send.
    #[props(default)]
    busy: bool,
    /// Called by the Stop button and by Escape while `busy`.
    onstop: Option<EventHandler<()>>,
    #[props(default)] disabled: bool,
    /// Hold the draft yourself (to keep it across routes, or fill it from a suggestion).
    /// Without it the composer keeps its own.
    value: Option<Signal<String>>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    /// Extra controls left of the send button (attach, model picker). Name each one.
    children: Element,
) -> Element {
    let strings = use_ui_strings();
    let own = use_signal(String::new);
    let mut draft = value.unwrap_or(own);
    let hint_id = use_hook(|| format!("dx-chat-composer-{}-hint", next_id()));
    let mut field = use_signal(|| None::<Rc<MountedData>>);
    let can_stop = busy && onstop.is_some();
    let empty = draft.read().trim().is_empty();
    let send_disabled = disabled || busy || empty;

    let mut send = move || {
        let text = draft.peek().trim().to_string();
        if text.is_empty() || busy || disabled {
            return;
        }
        draft.set(String::new());
        onsend.call(text);
        if let Some(el) = field.peek().clone() {
            spawn(async move {
                _ = el.set_focus(true).await;
            });
        }
    };
    let stop = move || {
        if let Some(onstop) = onstop {
            onstop.call(());
        }
    };

    rsx! {
        div {
            "data-slot": "chat-composer",
            "data-busy": busy.then_some("true"),
            "data-disabled": disabled.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_chat_composer }), attributes]),
            Textarea {
                class: Styles::dx_chat_composer_input,
                variant: TextareaVariant::Ghost,
                autosize: true,
                rows: "1",
                aria_label: label,
                aria_describedby: hint_id.clone(),
                aria_keyshortcuts: "Enter",
                placeholder,
                disabled,
                value: "{draft}",
                oninput: move |e: FormEvent| draft.set(e.value()),
                onmounted: move |e: MountedEvent| field.set(Some(e.data())),
                onkeydown: move |e: KeyboardEvent| {
                    let composing = e.is_composing();
                    let shift = e.modifiers().shift();
                    match composer_key(&e.key(), shift, composing, busy) {
                        ComposerKey::Send => {
                            e.prevent_default();
                            send();
                        }
                        ComposerKey::Stop => {
                            e.prevent_default();
                            stop();
                        }
                        ComposerKey::Default => {}
                    }
                },
            }
            span { id: hint_id, class: "sr-only", "{strings.composer_hint}" }
            div { class: Styles::dx_chat_composer_bar,
                div { class: Styles::dx_chat_composer_tools, {children} }
                if can_stop {
                    Button {
                        r#type: "button",
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        "data-slot": "chat-composer-stop",
                        onclick: move |_| stop(),
                        Square { size: "0.75rem" }
                        "{strings.stop}"
                    }
                } else {
                    IconButton {
                        r#type: "button",
                        label: strings.send.clone(),
                        size: IconButtonSize::Sm,
                        "data-slot": "chat-composer-send",
                        // Not `disabled`: a disabled button drops out of the tab order and
                        // its name stops being found. aria-disabled keeps both.
                        aria_disabled: send_disabled.then_some("true"),
                        onclick: move |_| send(),
                        ArrowUp { size: "1rem" }
                    }
                }
            }
        }
    }
}
