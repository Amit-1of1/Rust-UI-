# Chat composer

- Copy into your app: `rust-ui add chat_composer` or `dx components add chat_composer --git https://github.com/Amit-1of1/Rust-UI-` (also copies `button`, `icons`, `textarea`, `ui_strings`).
- Import: `use crate::components::chat_composer::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

The message box of a chat. It grows with the text (CSS `field-sizing`, through
[`Textarea`](textarea)'s `autosize`), sends with Enter, adds a line with Shift+Enter, and
turns Send into Stop while a reply is `busy` (Escape stops too). Sending trims the text,
ignores an empty message, empties the box and keeps focus in it. Extra controls (attach, a
model picker) go in its children.

```rust
ChatComposer {
    label: "Message",
    busy: streaming(),
    onsend: move |text: String| ask(text),
    onstop: move |_| cancel(),
}
```

Pass `value: Signal<String>` to hold the draft yourself.

## Accessibility

- Role and name: a text box named by the required `label` (a compile error without one);
  how to send is read after it (`aria-describedby`), and `aria-keyshortcuts="Enter"`.
- IME: Enter that confirms a composition (Japanese, Chinese, Korean input) picks the
  characters and never sends. `composer_key` holds the rule and is unit-tested.
- The send button stays focusable while there is nothing to send (`aria-disabled`, not
  `disabled`), so its name is still found.
- Escape stops a running reply only while `busy`; it is left out of the contract because this
  demo is never busy, and is checked in the browser by `tests/e2e/chat.mjs`.
- Reduced motion: the focus ring appears without a transition.

## Source

Original to this repo.

## Stability

Experimental: new on 2026-10-30, and the props may change once a real agent app uses them (docs/STABILITY.md).

## Keyboard and accessibility

Pattern: Text box with a send button. Works on native (Blitz): yes.

| Key | When | What it does | Native |
|---|---|---|---|
| Enter | In the message box | Sends the message, empties the box and keeps focus in it. Does nothing while the box is empty. | yes |
| Shift + Enter | In the message box | Starts a new line instead of sending. | yes |
| Space | On the send button | Sends the message. | yes |

Every row is run as a keyboard test on the web, and on native where it says yes. Generated from the component's `contract.toml`.

## API

```rust
/// What a key press in the composer does. Kept apart from the component so the rules are
/// tested on their own (`tests/components/chat_composer.rs`).
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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
pub fn composer_key(key: &Key, shift: bool, composing: bool, busy: bool) -> ComposerKey

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
) -> Element
```

## Example

```rust
use crate::components::button::{ButtonVariant, IconButton, IconButtonSize};
use crate::components::chat_composer::*;
use crate::components::icons::Paperclip;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut sent = use_signal(Vec::<String>::new);
    rsx! {
        div { display: "flex", flex_direction: "column", gap: "0.75rem", max_width: "32rem",
            ChatComposer {
                label: "Message",
                placeholder: "Write a message…",
                onsend: move |text| sent.write().push(text),
                IconButton {
                    label: "Attach a file",
                    variant: ButtonVariant::Ghost,
                    size: IconButtonSize::Sm,
                    r#type: "button",
                    Paperclip { size: "1rem" }
                }
            }
            if sent().is_empty() {
                p { "data-slot": "sent", style: "margin: 0; color: var(--secondary-color-5); font-size: 0.875rem;",
                    "Nothing sent yet."
                }
            } else {
                ul { "data-slot": "sent", style: "margin: 0; padding-inline-start: 1.25rem; font-size: 0.875rem;",
                    for (i, text) in sent().into_iter().enumerate() {
                        li { key: "{i}", style: "white-space: pre-wrap;", "{text}" }
                    }
                }
            }
        }
    }
}
```
