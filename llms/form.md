# Form

- Copy into your app: `atoll add form` or `dx components add form --git https://github.com/Amit-1of1/Rust-UI-` (also copies `checkbox`, `combobox`, `date_picker`, `date_time_picker`, `field`, `input`, `select`, `textarea`, `time_picker`, `ui_strings`).
- Import: `use crate::components::form::*;` after copying, or `use ui::prelude::*;` with the `atoll-ui` crate.

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

`TimeField` also takes `min` and `max`; it then adds a range rule itself ("Choose a time from 9:00 to 17:00."). `DateRangeField` binds a `DateRangePicker`: the field holds `2026-10-15/2026-10-20` (an ISO 8601 interval, read back with `parse_iso_date_range`), empty until both dates are picked, and `days_between(min, max, message)` limits the length of the stay:

```rust
let stay = form.field(vec![required(), days_between(Some(2), Some(14), "Stays are 2 to 14 nights.")]);
rsx! {
    DateRangeField { field: stay, label: "Stay", min_date: today, disabled_ranges: booked() }
}
```

A failed submit focuses the range picker's group, which is read with its label and error.

`TextField` and `TextareaField` pass any other attribute to their control (`inputmode: "numeric"`, `maxlength`, `pattern`, `rows`); the field's own `id`, `value` and ARIA wiring win over them.

Rules: `required()`, `email()`, `min_len(n)`, `max_len(n)`, `time_between(min, max, message)`, `days_between(min, max, message)`, or `custom(|v| ...)` returning `Err(message)`. The first failing rule shows. A field validates when it loses focus, then on every change until it is valid.

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

The generated code names the kit as `::ui`. If you copied the form component into your app (`atoll add form`), add the `atoll-form-derive` crate and point it at your copy: `#[form(path = "crate::components::form")]` on the struct.

## Accessibility

- Role and name: each control has a `<label>`; invalid controls get `aria-invalid="true"` and `aria-describedby` pointing at the description and the error. The error is `role="alert"` so it is announced.
- Keyboard: on a failed submit focus moves to the first invalid field. Enter submits as usual.
- Reduced motion: no animation.

Use `FormField` directly (value, error, ids, set, load, blur, mounted) to wire your own controls. Pass the control's `onmounted` to `field.mounted(e)` so a failed submit can focus it on every renderer, native included. `Select` and `Combobox` move `id`, `aria-describedby`, `aria-invalid` and `aria-required` to their focusable control (the trigger button or the text input), so a `<label for>` and focus-on-error reach it.

## Source

Original to this repo. The idea (a form-state hook plus bound fields) follows the pattern in Rust/UI's `use_form`, but nothing is copied; it is not Leptos- or serde-based.

## Stability

Experimental, `#[derive(Form)]` included. The form API was reshaped twice in October 2026 (`mounted`, then `field_with`, `load` and the select and combobox fields), and binding a raw `Select` is still open. It may change in any release; see `docs/STABILITY.md` in the kit repository.

## API

```rust
/// Checks one value and returns the message to show when it is not acceptable.
pub type Validator;

/// The value must not be empty (whitespace only counts as empty).
pub fn required() -> Validator

/// A deliberately loose `x@y.z` check.
pub fn email() -> Validator

pub fn min_len(min: usize) -> Validator

pub fn max_len(max: usize) -> Validator

/// A time (`"14:05"`) from `min` to `max`, both inclusive; a `max` earlier than `min` wraps
/// past midnight (22:00 to 02:00). Empty and unreadable values pass, so pair it with
/// `required()` when the time must be given. [`TimeField`] adds this rule itself when given
/// `min` or `max`.
pub fn time_between(
    min: Option<Time>,
    max: Option<Time>,
    message: impl Into<String>,
) -> Validator

/// A date range (`"2026-10-15/2026-10-20"`) that spans at least `min` and at most `max`
/// days (nights, for a stay). Empty and unreadable values pass, like [`time_between`].
pub fn days_between(min: Option<u32>, max: Option<u32>, message: impl Into<String>) -> Validator

/// Your own rule: return `Err(message)` to reject.
pub fn custom(rule: impl Fn(&str) -> Result<(), String> + 'static) -> Validator

/// The first failing message, checking rules in order. Rules other than `required()` do not
/// special-case empty input, so list `required()` first (or leave it out for an optional field
/// and make your own rules accept empty).
pub fn first_error(validators: &[Validator], value: &str) -> Option<String>

/// One field's state: value, error and touched flag, plus the ids that tie its label,
/// description and error message together. Create with [`FormState::field`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct FormField {
    value: Signal<String>,
    initial: Signal<String>,
    error: Signal<Option<String>>,
    touched: Signal<bool>,
    ids: Signal<FieldIds>,
    validators: Signal<Vec<ValidatorBox>>,
    /// The control, once mounted (see [`FormField::mounted`]).
    element: Signal<Option<Rc<MountedData>>>,
}

/// All the fields of one form. Create with [`use_form`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct FormState {
    fields: Signal<Vec<FormField>>,
}

/// Create the state for one form.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub fn use_form() -> FormState

/// A `<form>` wired to a [`FormState`]: native validation is off (we show our own messages),
/// and submit validates, focuses the first error, or calls `on_valid`.
#[component]
pub fn Form(
    /// The form's state: a [`FormState`], or a `#[derive(Form)]` form (it converts).
    #[props(into)]
    state: FormState,
    #[props(default)] on_valid: Callback<()>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Label, input, optional description and error, all wired to a [`FormField`] with the right
/// `aria-invalid` and `aria-describedby`.
#[component]
pub fn TextField(
    field: FormField,
    #[props(into)] label: String,
    #[props(into, default)] description: Option<String>,
    #[props(into, default = "text".to_string())] r#type: String,
    #[props(into, default)] placeholder: Option<String>,
    #[props(into, default)] autocomplete: Option<String>,
    /// Anything else for the `<input>`: `inputmode`, `maxlength`, `pattern`, `class`, ...
    /// The field's own wiring (`id`, `value`, `aria-invalid`, `aria-describedby`) wins.
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
) -> Element

/// One choice of a [`SelectField`] or [`ComboboxField`]: the value stored in the field and
/// the text shown. `"UTC".into()` uses the same text for both.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
pub struct FieldOption {
    pub value: String,
    pub label: String,
}

/// A [`Select`] bound to a [`FormField`], with the same wiring as [`TextField`]: a label, the
/// error and description in `aria-describedby`, `aria-invalid`, and focus on a failed submit.
/// The field holds the chosen option's `value` (empty while nothing is chosen, so
/// `required()` works). It validates when the list closes, as a text field does on blur.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn SelectField(
    field: FormField,
    #[props(into)] label: String,
    options: Vec<FieldOption>,
    #[props(into, default)] description: Option<String>,
    #[props(into, default)] placeholder: Option<String>,
    /// Mark the control `aria-required` (pair it with a `required()` rule).
    #[props(default)]
    required: bool,
) -> Element

/// A searchable [`Combobox`] bound to a [`FormField`], wired like [`SelectField`]. `empty` is
/// shown when the search matches nothing.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn ComboboxField(
    field: FormField,
    #[props(into)] label: String,
    options: Vec<FieldOption>,
    #[props(into, default)] description: Option<String>,
    #[props(into, default)] placeholder: String,
    #[props(into, default = "No results.".to_string())] empty: String,
    /// Mark the control `aria-required` (pair it with a `required()` rule).
    #[props(default)]
    required: bool,
) -> Element

/// A multi-line [`Textarea`] bound to a [`FormField`], wired like [`TextField`]. Extra
/// attributes (`rows`, `maxlength`, ...) go to the `<textarea>`.
#[component]
pub fn TextareaField(
    field: FormField,
    #[props(into)] label: String,
    #[props(into, default)] description: Option<String>,
    #[props(into, default)] placeholder: Option<String>,
    /// Grow with the text (see [`Textarea`]).
    #[props(default)]
    autosize: bool,
    #[props(extends=GlobalAttributes)]
    #[props(extends=textarea)]
    attributes: Vec<Attribute>,
) -> Element

/// A [`Checkbox`] with its label beside it, bound to a [`FormField`] ("I accept the terms").
/// The field holds `"true"` while checked and `""` while not, so `required()` (or `custom` with
/// your own message) makes it mandatory. It validates on every change, and a failed submit
/// focuses it like any other field.
#[component]
pub fn CheckboxField(
    field: FormField,
    #[props(into)] label: String,
    #[props(into, default)] description: Option<String>,
    /// Mark the control `aria-required` (pair it with a `required()` rule).
    #[props(default)]
    required: bool,
) -> Element

/// A [`TimePicker`](crate::components::time_picker::TimePicker) bound to a [`FormField`], wired
/// like [`TextField`]. The field holds `"14:05"` (`"14:05:09"` with `seconds`), the format of
/// `<input type="time">`, and stays empty until every segment is filled, so `required()`
/// works. It validates when focus leaves the picker.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn TimeField(
    field: FormField,
    #[props(into)] label: String,
    #[props(into, default)] description: Option<String>,
    #[props(default)] hour_cycle: HourCycle,
    #[props(default)] seconds: bool,
    #[props(default = 1)] minute_step: u8,
    /// Earliest allowed time. With `min` or `max` the field also gets a range rule
    /// ([`time_between`]), with the message from `UiStrings::time_out_of_range`.
    #[props(default)]
    min: Option<Time>,
    /// Latest allowed time (inclusive); earlier than `min` wraps past midnight.
    #[props(default)]
    max: Option<Time>,
) -> Element

/// A [`DateTimePicker`] bound to a [`FormField`], wired like [`TextField`]. The field holds
/// `"2026-10-15T14:05"`, the format of `<input type="datetime-local">`, and stays empty until
/// both the date and the time are entered. It validates when focus leaves the picker; a
/// failed submit focuses the time's first segment.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DateTimeField(
    field: FormField,
    #[props(into)] label: String,
    #[props(into, default)] description: Option<String>,
    #[props(default)] hour_cycle: HourCycle,
    #[props(default)] seconds: bool,
    #[props(default = 1)] minute_step: u8,
    /// Filled in when a date is picked while the time is still empty.
    #[props(default)]
    default_time: Option<Time>,
) -> Element

/// A [`DateRangePicker`] bound to a [`FormField`]. The field holds `"2026-10-15/2026-10-20"`
/// (an ISO 8601 interval; [`parse_iso_date_range`] reads it back) and stays empty until both
/// dates are entered, so `required()` works; [`days_between`] limits the length. The picker
/// group is named by the label and described by the description and error. It validates when
/// focus leaves the picker, its calendar included; a failed submit focuses the group.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[component]
pub fn DateRangeField(
    field: FormField,
    #[props(into)] label: String,
    #[props(into, default)] description: Option<String>,
    #[props(default = Date::from_calendar_date(1925, Month::January, 1).expect("valid date"))]
    min_date: Date,
    #[props(default = Date::from_calendar_date(2050, Month::December, 31).expect("valid date"))]
    max_date: Date,
    /// Dates that can't be picked (already booked, closed).
    #[props(default)]
    disabled_ranges: ReadSignal<Vec<DateRange>>,
    /// How many months the calendar shows at once.
    #[props(default = 1)]
    month_count: u8,
) -> Element
```

## Example

```rust
use crate::components::button::Button;
use crate::components::callout::{Callout, CalloutVariant};
use crate::components::form::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let form = use_form();
    let email = form.field(vec![required(), email()]);
    let password = form.field(vec![required(), min_len(8)]);
    let role = form.field(vec![custom(|v| {
        if v.is_empty() {
            Err("Pick a role.".to_string())
        } else {
            Ok(())
        }
    })]);
    let zone = form.field_with("UTC", vec![required()]);
    let mut done = use_signal(|| false);
    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 22rem;",
            Form {
                state: form,
                on_valid: move |_| {
                    done.set(true);
                    form.reset();
                },
                TextField {
                    field: email,
                    label: "Email",
                    r#type: "email",
                    placeholder: "you@example.com",
                    autocomplete: "email",
                    description: "We never share it.",
                }
                TextField {
                    field: password,
                    label: "Password",
                    r#type: "password",
                    autocomplete: "new-password",
                }
                SelectField {
                    field: role,
                    label: "Role",
                    placeholder: "Choose a role",
                    options: vec![
                        FieldOption::new("dev", "Developer"),
                        FieldOption::new("design", "Designer"),
                        FieldOption::new("pm", "Product manager"),
                    ],
                }
                ComboboxField {
                    field: zone,
                    label: "Time zone",
                    description: "Resets to UTC.",
                    placeholder: "Search time zones",
                    options: vec!["UTC".into(), "Europe/London".into(), "Asia/Tokyo".into()],
                }
                Button { r#type: "submit", "Create account" }
            }
            if done() {
                Callout { variant: CalloutVariant::Success, "Account created." }
            }
        }
    }
}
```
