# Sign-up 01

- Copy into your app: `rust-ui add signup_01` (also copies `alert`, `button`, `card`, `form`, `input_otp`, `link`).
- Import: `use crate::blocks::signup_01::*;` after copying, or `use ui::blocks::signup_01::*;` with the `rust-ui-kit` crate's `blocks` feature (internal).

A two-step sign-up card: name, email, password typed twice and a terms checkbox, then a 6-digit email code. Built from `card`, `form`, `input_otp`, `alert` and `link`.

```sh
rust-ui add signup_01
```

```rust
rsx! {
    Signup01 {
        on_submit: move |d: SignupDetails| send_code(&d.email),
        on_verify: move |code: String| check_code(&code),
        on_resend: move |_| resend(),
    }
}
```

## Props

- `on_submit: Callback<SignupDetails>`: called only with valid fields (`name`, `email`, `password`); the card then shows the code step.
- `on_verify: Option<Callback<String>>`: called with all 6 digits. Leave it out to stop after step one.
- `on_resend: Callback<()>`: the "Send a new code" button.
- `error: Option<String>`: a server message for the current step, shown in a destructive `Alert`.
- `terms_href`, `sign_in_href`: where the two links go.

## Accessibility

- The block is the page's `main` landmark.
- Each step has its own `h1`. Moving to the code step puts focus on its heading, so screen readers announce the new step.
- "Confirm password" checks that both passwords match; errors are tied to their inputs and a failed submit focuses the first one.
- Password inputs use `autocomplete="new-password"`, so password managers offer to generate one.
- The code boxes are one labelled group ("Verification code"); a short code is reported in a `role="alert"` line.

## API

```rust
/// What the person typed into [`Signup01`].
pub struct SignupDetails {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// A two-step sign-up card. Step one asks for name, email, a password (typed twice) and
/// agreement to the terms. Step two asks for the 6-digit code you emailed; leave out
/// `on_verify` to skip it.
#[component]
pub fn Signup01(
    /// Called with valid details. Send the code here; the card then shows the code step.
    #[props(default)]
    on_submit: Callback<SignupDetails>,
    /// Called with the 6-digit code. Without it the card stops after step one.
    on_verify: Option<Callback<String>>,
    /// Called from "Send a new code".
    #[props(default)]
    on_resend: Callback<()>,
    /// A message from your server for the current step, such as "That code has expired."
    #[props(into, default)]
    error: Option<String>,
    #[props(into, default = "#terms".to_string())] terms_href: String,
    #[props(into, default = "#sign-in".to_string())] sign_in_href: String,
) -> Element
```
