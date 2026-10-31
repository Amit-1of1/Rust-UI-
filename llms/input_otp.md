# InputOtp

- Copy into your app: `atoll add input_otp` or `dx components add input_otp --git https://github.com/Amit-1of1/Rust-UI-` (also copies `ui_strings`).
- Import: `use crate::components::input_otp::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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

## API

```rust
/// Something the user did to one box of an [`InputOtp`].
pub enum OtpEvent {
    /// The box at `slot` received `text`: one character when typing, several for a paste
    /// or SMS autofill, empty when its character was deleted.
    Input {
        slot: usize,
        text: String,
    },
    /// Backspace in the box at `slot`.
    Backspace(usize),
    Left(usize),
    Right(usize),
    Home,
    End,
}

/// What [`OtpState::apply`] did.
pub enum OtpOutcome {
    /// Not ours; let the browser handle the key.
    Ignored,
    /// Only focus moved.
    Moved,
    /// The code changed (and focus may have moved).
    Changed,
}

/// The code inside an [`InputOtp`] and which box has focus. Pure, so the keyboard
/// behaviour is tested without a browser (`crates/ui/tests/keyboard_state.rs`).
pub struct OtpState {
    pub chars: Vec<Option<char>>,
    pub focus: usize,
    /// Accept only digits (otherwise letters and digits).
    pub numeric: bool,
}

/// One-time-code input: one box per character with auto-advance, Backspace to go back, arrow
/// keys, and paste or SMS autofill spread across the boxes. Each box is a real input, so it
/// works with screen readers and password managers.
#[component]
pub fn InputOtp(
    /// Number of characters.
    #[props(default = 6)]
    length: usize,
    /// Accessible name of the whole code, for example "Verification code".
    #[props(into)]
    label: Option<String>,
    /// Only accept digits (the default). Off accepts letters and digits.
    #[props(default = true)]
    numeric: bool,
    /// Draw a dash after every `group` boxes (0 for none), for example 3 for `123-456`.
    #[props(default = 0)]
    group: usize,
    #[props(default)] invalid: bool,
    #[props(default)] disabled: bool,
    /// Called on every change with the code so far.
    #[props(default)]
    on_change: Callback<String>,
    /// Called once all boxes are filled.
    #[props(default)]
    on_complete: Callback<String>,
) -> Element
```

## Example

```rust
use crate::components::input_otp::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut code = use_signal(String::new);
    let mut done = use_signal(|| false);
    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            InputOtp {
                label: "Verification code",
                length: 6,
                group: 3,
                on_change: move |c: String| {
                    code.set(c);
                    done.set(false);
                },
                on_complete: move |_| done.set(true),
            }
            p { style: "margin: 0; font-size: 0.875rem;", role: "status",
                if done() { "Code {code()} entered." } else { "Enter the 6-digit code." }
            }
        }
    }
}
```
