# Form

Validation state for a form, with the accessibility wiring done for you. `use_form()` creates the state, `form.field(vec![required(), email()])` declares each field (call it like a hook), and `Form` / `TextField` render a native `<form novalidate>` and fully labelled inputs.

```rust
let form = use_form();
let email = form.field(vec![required(), email()]);
rsx! {
    Form { state: form, on_valid: move |_| save(email.value()),
        TextField { field: email, label: "Email", r#type: "email" }
        Button { r#type: "submit", "Save" }
    }
}
```

Rules: `required()`, `email()`, `min_len(n)`, `max_len(n)`, or `custom(|v| ...)` returning `Err(message)`. The first failing rule shows. A field validates when it loses focus, then on every change until it is valid.

## Accessibility

- Role and name: each control has a `<label>`; invalid controls get `aria-invalid="true"` and `aria-describedby` pointing at the description and the error. The error is `role="alert"` so it is announced.
- Keyboard: on a failed submit focus moves to the first invalid field. Enter submits as usual.
- Reduced motion: no animation.

Use `FormField` directly (value, error, ids, set, blur) to wire your own controls such as a select or checkbox.

## Source

Original to this repo. The idea (a form-state hook plus bound fields) follows the pattern in Rust/UI's `use_form`, but nothing is copied; it is not Leptos- or serde-based.
