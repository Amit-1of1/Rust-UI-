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

`TextareaField` is the multi-line `TextField`. `CheckboxField` puts a checkbox and its label on one line; the field holds `"true"` while it is ticked, so a rule makes it mandatory ("accept the terms") and it is focused on a failed submit like the rest:

```rust
let notes = form.field(vec![max_len(300)]);
let terms = form.field(vec![custom(|v| if v == "true" { Ok(()) } else { Err("Accept the terms.".into()) })]);
rsx! {
    TextareaField { field: notes, label: "Notes", maxlength: "300" }
    CheckboxField { field: terms, label: "I accept the terms" }
}
```

`TimeField` and `DateTimeField` bind a `TimePicker` or `DateTimePicker`. The field holds `14:05` or `2026-10-15T14:05` (the formats of `<input type="time">` and `datetime-local`), empty until every part is entered; they validate when focus leaves the picker:

```rust
let start = form.field(vec![required()]);
let departure = form.field(vec![required()]);
rsx! {
    TimeField { field: start, label: "Start", hour_cycle: HourCycle::H12, minute_step: 5 }
    DateTimeField { field: departure, label: "Departure", default_time: Time::from_hms(9, 0, 0).ok() }
}
```

`TextField` and `TextareaField` pass any other attribute to their control (`inputmode: "numeric"`, `maxlength`, `pattern`, `rows`); the field's own `id`, `value` and ARIA wiring win over them.

Rules: `required()`, `email()`, `min_len(n)`, `max_len(n)`, or `custom(|v| ...)` returning `Err(message)`. The first failing rule shows. A field validates when it loses focus, then on every change until it is valid.

## Derive a form from a struct

`#[derive(Form)]` (in `ui::prelude`) turns a struct into the same fields, so the rules and messages live next to the types:

```rust
#[derive(Form, Clone, PartialEq)]
struct SignUp {
    #[form(required, min_len = 3, autocomplete = "username")]
    user_name: String,
    #[form(required, email, description = "We never share it.")]
    email: String,
    #[form(min = 18)]
    age: u32,
    #[form(select = plans, placeholder = "Choose a plan")]
    plan: Option<String>,
    #[form(required = "Accept the terms to continue.", label = "I accept the terms")]
    terms: bool,
}

let form = SignUp::use_form(); // or SignUp::use_form_with(&saved)
rsx! {
    Form { state: form, on_valid: move |_| if let Some(v) = form.value() { save(v) },
        {form.fields()}
        Button { r#type: "submit", "Sign up" }
    }
}
```

It generates `SignUpForm`, with one public `FormField` per struct field (`form.email`), plus `fields()` (every field, rendered in order), `value()` (the typed struct, `None` while a field does not parse), `load(&value)`, `reset()`, `validate()` and `state()`. Render a field yourself to lay it out differently: `TextField { field: form.email, label: "Email" }`.

The type picks the control and the base rules:

- `String`: a `TextField`. `Option<String>`: the same, optional (empty is `None`).
- `bool`: a `CheckboxField`; `required` means it must be ticked.
- Numbers and any other `FromStr + Display` type: a `TextField` (`inputmode` numeric or decimal for numbers) that must parse, so it is required. `Option<T>` makes it optional.

Attributes, all inside `#[form(...)]`:

- `label = "..."` (default: the field name, `user_name` becomes "User name"), `description`, `placeholder`, `autocomplete`, `input = "password"` (the input `type`; `email` sets `type="email"` itself).
- `required` or `required = "message"`. The default message names the field ("Email is required."), since an error is announced on its own.
- `email`, `min_len = n`, `max_len = n` for text; `min = n`, `max = n` and `invalid = "message"` (when it doesn't parse) for parsed types; `validate = my_rule` (any `fn(&str) -> Result<(), String>`, repeatable).
- `textarea`, `select = options_fn` or `combobox = options_fn` (a `fn() -> Vec<FieldOption>`) to change the control.

Rules run in that order and the first failure shows. On an optional field the rules only check what was typed. Required fields get `aria-required`, and the errors use the same `aria-invalid` / `aria-describedby` / `role="alert"` wiring as the hand-written fields. Unknown attributes, and ones that don't fit the type (`email` on a number, `required` on an `Option`), are compile errors.

The generated code names the kit as `::ui`. If you copied the form component into your app (`rust-ui add form`), add the `rust-ui-form-derive` crate and point it at your copy: `#[form(path = "crate::components::form")]` on the struct.

## Accessibility

- Role and name: each control has a `<label>`; invalid controls get `aria-invalid="true"` and `aria-describedby` pointing at the description and the error. The error is `role="alert"` so it is announced.
- Keyboard: on a failed submit focus moves to the first invalid field. Enter submits as usual.
- Reduced motion: no animation.

Use `FormField` directly (value, error, ids, set, load, blur, mounted) to wire your own controls. Pass the control's `onmounted` to `field.mounted(e)` so a failed submit can focus it on every renderer, native included. `Select` and `Combobox` move `id`, `aria-describedby`, `aria-invalid` and `aria-required` to their focusable control (the trigger button or the text input), so a `<label for>` and focus-on-error reach it.

## Source

Original to this repo. The idea (a form-state hook plus bound fields) follows the pattern in Rust/UI's `use_form`, but nothing is copied; it is not Leptos- or serde-based.

## Stability

Experimental, `#[derive(Form)]` included. The form API was reshaped twice in October 2026 (`mounted`, then `field_with`, `load` and the select and combobox fields), and binding a raw `Select` is still open. It may change in any release; see `docs/STABILITY.md` in the kit repository.
