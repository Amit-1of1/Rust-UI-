# Login 01

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
