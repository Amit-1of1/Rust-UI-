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

Start a field with a value with `form.field_with("Acme", rules)`; `reset()` returns to it. To load a record into the form (an edit sheet), call `field.load(value)`: it sets the value, makes it the one `reset()` returns to, and clears the error. Both `set` and `load` are safe to call from an effect.

`SelectField` and `ComboboxField` bind a `Select` or a searchable `Combobox` to a field the same way, so they can be required, show their error and get focus on a failed submit:

```rust
let plan = form.field(vec![required()]);
let zone = form.field_with("UTC", vec![required()]);
rsx! {
    SelectField { field: plan, label: "Plan", placeholder: "Choose a plan",
        options: vec![FieldOption::new("free", "Free"), FieldOption::new("pro", "Pro")] }
    ComboboxField { field: zone, label: "Time zone", options: vec!["UTC".into(), "Asia/Tokyo".into()] }
}
```

The field holds the option's `value` (empty while nothing is chosen). They validate when the list closes, as a text field does on blur.

Rules: `required()`, `email()`, `min_len(n)`, `max_len(n)`, or `custom(|v| ...)` returning `Err(message)`. The first failing rule shows. A field validates when it loses focus, then on every change until it is valid.

## Accessibility

- Role and name: each control has a `<label>`; invalid controls get `aria-invalid="true"` and `aria-describedby` pointing at the description and the error. The error is `role="alert"` so it is announced.
- Keyboard: on a failed submit focus moves to the first invalid field. Enter submits as usual.
- Reduced motion: no animation.

Use `FormField` directly (value, error, ids, set, load, blur) to wire your own controls such as a checkbox. `Select` and `Combobox` move `id`, `aria-describedby`, `aria-invalid` and `aria-required` to their focusable control (the trigger button or the text input), so a `<label for>` and focus-on-error reach it.

## Source

Original to this repo. The idea (a form-state hook plus bound fields) follows the pattern in Rust/UI's `use_form`, but nothing is copied; it is not Leptos- or serde-based.
