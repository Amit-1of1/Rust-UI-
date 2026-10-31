# Chat

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
