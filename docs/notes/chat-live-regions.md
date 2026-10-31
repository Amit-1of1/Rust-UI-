# Chat logs, streaming text and screen readers

- **Don't make a streaming log live.** `role="log"` is polite-live by default, and a reply that
  grows token by token makes NVDA and VoiceOver read fragments, restart, or fall behind.
  `aria-busy` on the growing element is meant to defer that, but support is uneven. `ChatLog`
  sets `aria-live="off"` and announces finished messages through two hidden polite regions it
  owns (used in turn, so the same sentence twice is still read). Keep that split if you build
  a variation.
- **Effects don't run in SSR tests.** Announcements happen in `use_effect`; the kit's SSR tests
  (`crates/ui/tests`) never see them. Test them on headless Blitz (`apps/native/tests/chat.rs`),
  whose harness polls effects, or in the browser (`tests/e2e/chat.mjs`).
- **Blitz has no scroll events and no `ResizeObserver`.** `onscroll` and `onresize` are never
  dispatched there (dioxus-native-dom's converters are `unimplemented!`, so don't send them
  synthetically either). `ChatLog` follows by measuring after each render instead; a scroll
  offset lower than the one it last set means the reader scrolled.
- **Browsers deliver scroll events a frame late.** While a reply streams, a follow can run
  between the reader's scroll and its event and undo it, and an event can report a gap that
  only the newly streamed text made. So `ChatLog` decides from the offset it reads before each
  follow, and a scroll event only lets go when the offset moved up (CI caught both, on release
  builds in all three engines).
- **Blitz's absolute position of a scrolled element subtracts its own scroll offset.** Measure a
  scroll container's box on a wrapper, not on itself (`log_rect` in `apps/native/tests/chat.rs`).
- **Timers in native tests need a Tokio runtime** (`dioxus-sdk-time` sleeps through Tokio
  there). `apps/native/tests/contracts.rs` now enters one, since the chat demo streams.
- **IME:** check `KeyboardEvent::is_composing()` before treating Enter as "send". Safari can
  still send one Enter after `compositionend` with `isComposing` false; Dioxus's keyboard data
  has no `keyCode` (229) to catch it, so that case is not handled.
