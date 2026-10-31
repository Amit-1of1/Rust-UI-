# Login 01

- Copy into your app: `atoll add login_01` (also copies `alert`, `button`, `card`, `form`, `link`).
- Import: `use crate::blocks::login_01::*;` after copying, or `use ui::blocks::login_01::*;` with the `atoll-ui` crate's `blocks` feature (internal).

A centred sign-in card: email, password, "remember me", a forgot-password link and a sign-up link. Built from `card`, `form`, `alert` and `link`.

```sh
atoll add login_01   # copies src/blocks/login_01 and the components it uses
```

```rust
let mut error = use_signal(|| None::<String>);
rsx! {
    Login01 {
        error: error(),
        on_submit: move |d: LoginDetails| {
            if d.password != "secret" {
                error.set(Some("Wrong email or password.".into()));
            }
        },
    }
}
```

## Props

- `on_submit: Callback<LoginDetails>`: called only with valid fields (`email`, `password`, `remember`).
- `error: Option<String>`: a server message shown in a destructive `Alert`.
- `busy: bool`: disables the button while you wait for the server.
- `forgot_href`, `sign_up_href`: where the two links go.

## Accessibility

- The block is the page's `main` landmark.
- The page heading is an `h1` ("Sign in"); the form is named "Sign in".
- Inputs carry `autocomplete="email"` and `"current-password"`, so password managers fill them.
- Errors are tied to their inputs with `aria-describedby`; a failed submit focuses the first one.
- The server error is a `role="alert"`, so it is announced when it appears.

## API

```rust
/// What the person typed into [`Login01`].
pub struct LoginDetails {
    pub email: String,
    pub password: String,
    pub remember: bool,
}

/// A centred sign-in card: email, password, "remember me", a forgot-password link and a link
/// to sign up. Fields are checked before `on_submit` runs; a failed submit moves focus to the
/// first field that needs fixing.
#[component]
pub fn Login01(
    /// Called with valid details. Check them on your server, then set `error` or move on.
    #[props(default)]
    on_submit: Callback<LoginDetails>,
    /// A message from your server, such as "Wrong email or password." Announced when it appears.
    #[props(into, default)]
    error: Option<String>,
    /// While true the button is disabled and says "Signing in…".
    #[props(default)]
    busy: bool,
    #[props(into, default = "#forgot-password".to_string())] forgot_href: String,
    #[props(into, default = "#sign-up".to_string())] sign_up_href: String,
) -> Element
```
