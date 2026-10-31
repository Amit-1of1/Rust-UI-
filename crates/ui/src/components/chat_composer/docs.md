# Chat composer

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
