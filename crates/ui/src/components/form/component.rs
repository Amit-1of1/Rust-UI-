use crate::components::field::*;
use crate::components::input::Input;
use dioxus::prelude::*;
use std::rc::Rc;

#[css_module("/src/components/form/style.css")]
struct Styles;

/// Checks one value and returns the message to show when it is not acceptable.
pub type Validator = Rc<dyn Fn(&str) -> Result<(), String>>;

/// The value must not be empty (whitespace only counts as empty).
pub fn required() -> Validator {
    Rc::new(validate::required)
}

/// A deliberately loose `x@y.z` check.
pub fn email() -> Validator {
    Rc::new(validate::email)
}

pub fn min_len(min: usize) -> Validator {
    Rc::new(move |v| validate::min_len(v, min))
}

pub fn max_len(max: usize) -> Validator {
    Rc::new(move |v| {
        if v.chars().count() > max {
            Err(format!("Must be at most {max} characters."))
        } else {
            Ok(())
        }
    })
}

/// Your own rule: return `Err(message)` to reject.
pub fn custom(rule: impl Fn(&str) -> Result<(), String> + 'static) -> Validator {
    Rc::new(rule)
}

/// The first failing message, checking rules in order. Rules other than `required()` do not
/// special-case empty input, so list `required()` first (or leave it out for an optional field
/// and make your own rules accept empty).
pub fn first_error(validators: &[Validator], value: &str) -> Option<String> {
    validators.iter().find_map(|check| check(value).err())
}

/// One field's state: value, error and touched flag, plus the ids that tie its label,
/// description and error message together. Create with [`FormState::field`].
#[derive(Clone, Copy, PartialEq)]
pub struct FormField {
    value: Signal<String>,
    error: Signal<Option<String>>,
    touched: Signal<bool>,
    ids: Signal<FieldIds>,
    validators: Signal<Vec<ValidatorBox>>,
}

/// `Rc<dyn Fn>` is not `PartialEq`; compare by pointer so the signal can sit in a `PartialEq` struct.
#[derive(Clone)]
struct ValidatorBox(Validator);

impl PartialEq for ValidatorBox {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl FormField {
    pub fn value(&self) -> String {
        (self.value)()
    }

    pub fn error(&self) -> Option<String> {
        (self.error)()
    }

    pub fn ids(&self) -> FieldIds {
        self.ids.read().clone()
    }

    /// Set the value. Once the field has been blurred or has an error, it re-validates on every
    /// change so the message clears as soon as the user fixes it.
    ///
    /// Reads nothing reactively, so calling it from an effect (to load a record into the form)
    /// doesn't make that effect re-run, and overwrite the user's edits, when the field is blurred.
    pub fn set(&self, value: impl Into<String>) {
        let mut v = self.value;
        v.set(value.into());
        if *self.touched.peek() || self.error.peek().is_some() {
            self.validate();
        }
    }

    /// Call when the control loses focus: marks it touched and validates.
    pub fn blur(&self) {
        let mut touched = self.touched;
        touched.set(true);
        self.validate();
    }

    /// Run the rules now. Returns `true` when the value is valid.
    pub fn validate(&self) -> bool {
        let rules: Vec<Validator> = self.validators.peek().iter().map(|b| b.0.clone()).collect();
        let message = first_error(&rules, &self.value.peek());
        let ok = message.is_none();
        let mut error = self.error;
        error.set(message);
        ok
    }

    pub fn reset(&self) {
        let (mut v, mut e, mut t) = (self.value, self.error, self.touched);
        v.set(String::new());
        e.set(None);
        t.set(false);
    }
}

/// All the fields of one form. Create with [`use_form`].
#[derive(Clone, Copy, PartialEq)]
pub struct FormState {
    fields: Signal<Vec<FormField>>,
}

impl FormState {
    /// Declare a field. Call it at the top of your component, like a hook, in a fixed order.
    pub fn field(&self, validators: Vec<Validator>) -> FormField {
        let mut fields = self.fields;
        let ids = use_field_ids();
        use_hook(|| {
            let field = FormField {
                value: Signal::new(String::new()),
                error: Signal::new(None),
                touched: Signal::new(false),
                ids: Signal::new(ids),
                validators: Signal::new(
                    validators.into_iter().map(ValidatorBox).collect::<Vec<_>>(),
                ),
            };
            fields.write().push(field);
            field
        })
    }

    /// Validate every field. Returns `true` when all are valid.
    pub fn validate_all(&self) -> bool {
        let fields = self.fields.peek().clone();
        // Validate all (no short-circuit) so every message shows at once.
        let results: Vec<bool> = fields.iter().map(FormField::validate).collect();
        results.into_iter().all(|ok| ok)
    }

    pub fn reset(&self) {
        for field in self.fields.peek().iter() {
            field.reset();
        }
    }

    /// Handle a submit event: prevent the page reload, validate everything, and either call
    /// `on_valid` or move focus to the first invalid field so it is announced.
    pub fn submit(&self, event: FormEvent, on_valid: impl FnOnce()) {
        event.prevent_default();
        if self.validate_all() {
            on_valid();
        } else if let Some(first) = self.fields.peek().iter().find(|f| f.error.peek().is_some()) {
            // The id is generated by this module (`dx-form-<n>`), never user text.
            let id = first.ids.peek().control.clone();
            let _ = document::eval(&format!("document.getElementById('{id}')?.focus()"));
        }
    }
}

/// Create the state for one form.
pub fn use_form() -> FormState {
    FormState {
        fields: use_hook(|| Signal::new(Vec::new())),
    }
}

/// A `<form>` wired to a [`FormState`]: native validation is off (we show our own messages),
/// and submit validates, focuses the first error, or calls `on_valid`.
#[component]
pub fn Form(
    state: FormState,
    #[props(default)] on_valid: Callback<()>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        form {
            class: Styles::dx_form,
            novalidate: true,
            onsubmit: move |event| state.submit(event, || on_valid.call(())),
            ..attributes,
            {children}
        }
    }
}

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
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let mut described = Vec::new();
    if description.is_some() {
        described.push(ids.description.clone());
    }
    if invalid {
        described.push(ids.error.clone());
    }
    let described = described.join(" ");
    rsx! {
        Field { invalid,
            FieldLabel { html_for: ids.control.clone(), "{label}" }
            Input {
                id: ids.control.clone(),
                r#type: "{r#type}",
                placeholder: placeholder,
                autocomplete: autocomplete,
                value: "{field.value()}",
                aria_invalid: invalid.then_some("true"),
                aria_describedby: (!described.is_empty()).then_some(described),
                oninput: move |e: FormEvent| field.set(e.value()),
                onblur: move |_| field.blur(),
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}
