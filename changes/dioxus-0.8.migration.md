### Dioxus 0.8 (PR #PR)

**Who:** every app; Atoll on this branch needs Dioxus 0.8.

**Before:** `dioxus = "0.7"`, `Key::Escape`, `render_toast: Callback<toast::ToastPropsWithOwner, Element>`.

**After:** `dioxus = "=0.8.0-alpha.3"`, `Key::Named(NamedKey::Escape)`, `render_toast: Callback<toast::ToastProps, Element>`.

**What to do:** bump `dioxus` and `dioxus-cli` to `0.8.0-alpha.3`, rewrite key matches (`Key::X` to `Key::Named(NamedKey::X)` for every key but `Character`), and change the type of a custom `render_toast`. Follow Dioxus' own 0.8 notes for the rest of your app.
