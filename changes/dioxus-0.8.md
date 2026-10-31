### Breaking

- **all:** Atoll runs on Dioxus `=0.8.0-alpha.3` (was 0.7.10). Keys are `Key::Named(NamedKey::Escape)`, and `ToastProvider`'s `render_toast` takes `Callback<toast::ToastProps, Element>` (was `ToastPropsWithOwner`). `apps/native` stays on 0.7. Kept on its own branch until Dioxus 0.8 is stable; see docs/notes/dioxus-0.8.md. (#PR)
