# Sign-up 01

A two-step sign-up card: name, email, password typed twice and a terms checkbox, then a 6-digit email code. Built from `card`, `form`, `input_otp`, `alert` and `link`.

```sh
atoll add signup_01
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
