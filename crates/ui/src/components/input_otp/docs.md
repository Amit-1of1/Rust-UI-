# InputOtp

A one-time-code input: one box per character.

```rust
InputOtp { label: "Verification code", length: 6, group: 3, on_complete: move |code| verify(code) }
```

Typing advances to the next box; Backspace on an empty box clears and goes back; Left/Right/Home/End move; pasting (or an SMS autofill into the first box, which carries `autocomplete="one-time-code"`) spreads across the boxes. `numeric: false` accepts letters too. Set `invalid` to show an error state (pair it with a visible error message).

## Accessibility

- Role and name: a `role="group"` named by `label`; every box is a real `<input>` named "label, character n of N".
- Keyboard: as above; Tab leaves the group normally.
- Reduced motion: no animation.

## Source

Original to this repo. Rust/UI has an `input_otp` driven by a wasm-bindgen DOM controller; this one is plain Dioxus state and native inputs.
