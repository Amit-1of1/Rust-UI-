use crate::components::combobox::{Combobox, ComboboxEmpty, ComboboxOption};
use crate::components::field::*;
use crate::components::input::Input;
use crate::components::select::{Select, SelectOption};
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
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
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, PartialEq)]
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

    /// Call from the control's `onmounted`, so a failed submit can focus it on every renderer
    /// (without it, focus moves by element id, which needs scripts).
    pub fn mounted(&self, event: MountedEvent) {
        let mut element = self.element;
        element.set(Some(event.data()));
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

    /// Load a saved value (an edited record, stored settings): it becomes both the value and
    /// what [`reset`](Self::reset) returns to, and the field is pristine again (no error, not
    /// touched). Non-reactive like [`set`](Self::set), so it is safe in an effect.
    pub fn load(&self, value: impl Into<String>) {
        let value = value.into();
        let (mut v, mut i) = (self.value, self.initial);
        i.set(value.clone());
        v.set(value);
        self.clear_state();
    }

    /// Back to the initial value (from [`FormState::field_with`] or the last
    /// [`load`](Self::load); empty otherwise), with no error and not touched.
    pub fn reset(&self) {
        let mut v = self.value;
        v.set(self.initial.peek().clone());
        self.clear_state();
    }

    fn clear_state(&self) {
        let (mut e, mut t) = (self.error, self.touched);
        e.set(None);
        t.set(false);
    }

    /// `aria-describedby` for the control: the description when there is one, and the error
    /// while there is one.
    fn described_by(&self, has_description: bool) -> Option<String> {
        let ids = self.ids.peek();
        let mut described = Vec::new();
        if has_description {
            described.push(ids.description.clone());
        }
        if self.error.read().is_some() {
            described.push(ids.error.clone());
        }
        (!described.is_empty()).then(|| described.join(" "))
    }
}

/// All the fields of one form. Create with [`use_form`].
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, Copy, PartialEq)]
pub struct FormState {
    fields: Signal<Vec<FormField>>,
}

impl FormState {
    /// Declare a field. Call it at the top of your component, like a hook, in a fixed order.
    pub fn field(&self, validators: Vec<Validator>) -> FormField {
        self.field_with("", validators)
    }

    /// Declare a field that starts with a value; [`FormField::reset`] returns to it.
    pub fn field_with(&self, initial: impl Into<String>, validators: Vec<Validator>) -> FormField {
        let mut fields = self.fields;
        let ids = use_field_ids();
        let initial = initial.into();
        use_hook(|| {
            let field = FormField {
                value: Signal::new(initial.clone()),
                initial: Signal::new(initial),
                error: Signal::new(None),
                touched: Signal::new(false),
                ids: Signal::new(ids),
                validators: Signal::new(
                    validators.into_iter().map(ValidatorBox).collect::<Vec<_>>(),
                ),
                element: Signal::new(None),
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
            if let Some(element) = first.element.peek().clone() {
                spawn(async move {
                    _ = element.set_focus(true).await;
                });
            } else {
                // The id is generated by `use_field_ids` (`dx-field-<n>`), never user text. It is
                // the focusable control itself: the input, or a select's trigger button.
                let id = first.ids.peek().control.clone();
                let _ = document::eval(&format!("document.getElementById('{id}')?.focus()"));
            }
        }
    }
}

/// Create the state for one form.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
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
            novalidate: true,
            onsubmit: move |event| state.submit(event, || on_valid.call(())),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_form }), attributes]),
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
    let described = field.described_by(description.is_some());
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
                aria_describedby: described,
                oninput: move |e: FormEvent| field.set(e.value()),
                onblur: move |_| field.blur(),
                onmounted: move |e| field.mounted(e),
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}

/// One choice of a [`SelectField`] or [`ComboboxField`]: the value stored in the field and
/// the text shown. `"UTC".into()` uses the same text for both.
///
/// # Stability
///
/// Experimental: may change in any release (docs/STABILITY.md).
#[derive(Clone, PartialEq, Debug)]
pub struct FieldOption {
    pub value: String,
    pub label: String,
}

impl FieldOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

impl From<&str> for FieldOption {
    fn from(text: &str) -> Self {
        Self::new(text, text)
    }
}

/// The field's value as a selection: empty means nothing chosen.
fn use_selection(field: FormField) -> Memo<Option<String>> {
    use_memo(move || Some(field.value()).filter(|v| !v.is_empty()))
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
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let label_id = format!("{}-label", ids.control);
    let selection = use_selection(field);
    rsx! {
        Field { invalid,
            FieldLabel { id: label_id.clone(), html_for: ids.control.clone(), "{label}" }
            Select::<String> {
                value: Some(selection.into()),
                on_value_change: move |v: Option<String>| field.set(v.unwrap_or_default()),
                on_open_change: move |open: bool| {
                    if !open {
                        field.blur();
                    }
                },
                id: ids.control.clone(),
                aria_labelledby: label_id,
                aria_invalid: invalid.then_some("true"),
                aria_describedby: field.described_by(description.is_some()),
                "placeholder": placeholder,
                for (i , option) in options.iter().enumerate() {
                    SelectOption::<String> {
                        key: "{option.value}",
                        index: i,
                        value: option.value.clone(),
                        text_value: option.label.clone(),
                        "{option.label}"
                    }
                }
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}

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
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let label_id = format!("{}-label", ids.control);
    let selection = use_selection(field);
    rsx! {
        Field { invalid,
            FieldLabel { id: label_id.clone(), html_for: ids.control.clone(), "{label}" }
            Combobox::<String> {
                value: Some(selection.into()),
                on_value_change: move |v: Option<String>| field.set(v.unwrap_or_default()),
                on_open_change: move |open: bool| {
                    if !open {
                        field.blur();
                    }
                },
                placeholder,
                list_aria_label: label.clone(),
                id: ids.control.clone(),
                aria_labelledby: label_id,
                aria_invalid: invalid.then_some("true"),
                aria_describedby: field.described_by(description.is_some()),
                ComboboxEmpty { "{empty}" }
                for (i , option) in options.iter().enumerate() {
                    ComboboxOption::<String> {
                        key: "{option.value}",
                        index: i,
                        value: option.value.clone(),
                        text_value: option.label.clone(),
                        "{option.label}"
                    }
                }
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}
