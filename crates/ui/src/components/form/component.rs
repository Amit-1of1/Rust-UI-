use crate::components::checkbox::{Checkbox, CheckboxState};
use crate::components::combobox::{Combobox, ComboboxEmpty, ComboboxOption};
use crate::components::date_time_picker::{
    format_iso_date_time, parse_iso_date_time, DateTimePicker, PrimitiveDateTime,
};
use crate::components::field::*;
use crate::components::input::Input;
use crate::components::select::{Select, SelectOption};
use crate::components::textarea::Textarea;
use crate::components::date_picker::{
    format_iso_date_range, parse_iso_date_range, Date, DateRange, DateRangePicker, Month,
};
use crate::components::time_picker::{
    format_iso_time, format_time, parse_iso_time, use_focus_exit, HourCycle, Time, TimeState,
};
use crate::components::ui_strings::{fill, use_ui_strings};
use crate::components::sound::{use_sound, SoundEvent};
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

/// A time (`"14:05"`) from `min` to `max`, both inclusive; a `max` earlier than `min` wraps
/// past midnight (22:00 to 02:00). Empty and unreadable values pass, so pair it with
/// `required()` when the time must be given. [`TimeField`] adds this rule itself when given
/// `min` or `max`.
pub fn time_between(
    min: Option<Time>,
    max: Option<Time>,
    message: impl Into<String>,
) -> Validator {
    let message = message.into();
    let state = TimeState::new(HourCycle::H24, true, 1).with_range(min, max);
    Rc::new(move |v| match parse_iso_time(v) {
        Some(time) if !state.in_range(time) => Err(message.clone()),
        _ => Ok(()),
    })
}

/// A date range (`"2026-10-15/2026-10-20"`) that spans at least `min` and at most `max`
/// days (nights, for a stay). Empty and unreadable values pass, like [`time_between`].
pub fn days_between(min: Option<u32>, max: Option<u32>, message: impl Into<String>) -> Validator {
    let message = message.into();
    Rc::new(move |v| {
        let Some(range) = parse_iso_date_range(v) else {
            return Ok(());
        };
        let days = (range.end() - range.start()).whole_days().max(0) as u32;
        if min.is_some_and(|m| days < m) || max.is_some_and(|m| days > m) {
            Err(message.clone())
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

    /// Add a rule after the ones the field was declared with. Used by fields whose props imply
    /// a rule (`TimeField`'s `min`/`max`); call it once, from a hook.
    pub(crate) fn add_validator(&self, rule: Validator) {
        let mut validators = self.validators;
        validators.write().push(ValidatorBox(rule));
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
    pub(crate) fn described_by(&self, has_description: bool) -> Option<String> {
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

    /// Add a field while the form is open: a new line item, another email address. Unlike
    /// [`field`](Self::field) this is not a hook, so call it from an event handler (or once,
    /// inside `use_hook`, for the rows a form starts with) and keep the returned fields in
    /// your own signal. The form validates it with the others and focuses it on a failed
    /// submit. Take it out again with [`remove_field`](Self::remove_field).
    ///
    /// ```rust,ignore
    /// let mut lines = use_signal(Vec::<FormField>::new);
    /// Button { onclick: move |_| lines.push(form.add_field("", vec![required()])), "Add line" }
    /// for (i, line) in lines().into_iter().enumerate() {
    ///     TextField { key: "{line.ids().control}", field: line, label: format!("Line {}", i + 1) }
    /// }
    /// ```
    pub fn add_field(&self, initial: impl Into<String>, validators: Vec<Validator>) -> FormField {
        let base = crate::components::hooks::next_unique_id("dx-field");
        let ids = FieldIds {
            control: base.clone(),
            description: format!("{base}-description"),
            error: format!("{base}-error"),
        };
        let initial = initial.into();
        let field = FormField {
            value: Signal::new(initial.clone()),
            initial: Signal::new(initial),
            error: Signal::new(None),
            touched: Signal::new(false),
            ids: Signal::new(ids),
            validators: Signal::new(validators.into_iter().map(ValidatorBox).collect()),
            element: Signal::new(None),
        };
        let mut fields = self.fields;
        fields.write().push(field);
        field
    }

    /// Stop validating a field added with [`add_field`](Self::add_field) (the line item was
    /// deleted). Fields declared with [`field`](Self::field) can be removed too.
    pub fn remove_field(&self, field: FormField) {
        let mut fields = self.fields;
        fields.write().retain(|f| *f != field);
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
    /// The form's state: a [`FormState`], or a `#[derive(Form)]` form (it converts).
    #[props(into)]
    state: FormState,
    #[props(default)] on_valid: Callback<()>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let sound = use_sound();
    rsx! {
        form {
            novalidate: true,
            onsubmit: move |event| {
                let mut valid = false;
                state.submit(event, || {
                    valid = true;
                    on_valid.call(());
                });
                if !valid {
                    sound.play(SoundEvent::Error);
                }
            },
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
    /// Anything else for the `<input>`: `inputmode`, `maxlength`, `pattern`, `class`, ...
    /// The field's own wiring (`id`, `value`, `aria-invalid`, `aria-describedby`) wins.
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let described = field.described_by(description.is_some());
    let wiring = attributes!(input {
        id: ids.control.clone(),
        r#type: r#type,
        placeholder: placeholder,
        autocomplete: autocomplete,
        value: field.value(),
        aria_invalid: invalid.then_some("true"),
        aria_describedby: described,
    });
    rsx! {
        Field { invalid,
            FieldLabel { html_for: ids.control.clone(), "{label}" }
            Input {
                attributes: merge_attributes(vec![attributes, wiring]),
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
    /// Mark the control `aria-required` (pair it with a `required()` rule).
    #[props(default)]
    required: bool,
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
                aria_required: required.then_some("true"),
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
    /// Mark the control `aria-required` (pair it with a `required()` rule).
    #[props(default)]
    required: bool,
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
                aria_required: required.then_some("true"),
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
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let wiring = attributes!(textarea {
        id: ids.control.clone(),
        placeholder: placeholder,
        value: field.value(),
        aria_invalid: invalid.then_some("true"),
        aria_describedby: field.described_by(description.is_some()),
    });
    rsx! {
        Field { invalid,
            FieldLabel { html_for: ids.control.clone(), "{label}" }
            Textarea {
                autosize,
                attributes: merge_attributes(vec![attributes, wiring]),
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
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let checked = field.value() == "true";
    rsx! {
        Field { invalid,
            div { class: Styles::dx_form_check,
                Checkbox {
                    id: ids.control.clone(),
                    required,
                    checked: Some(if checked { CheckboxState::Checked } else { CheckboxState::Unchecked }),
                    on_checked_change: move |state: CheckboxState| {
                        field.set(if state == CheckboxState::Checked { "true" } else { "" });
                        field.blur();
                    },
                    aria_invalid: invalid.then_some("true"),
                    aria_describedby: field.described_by(description.is_some()),
                }
                FieldLabel { html_for: ids.control.clone(), "{label}" }
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}

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
) -> Element {
    let strings = use_ui_strings();
    use_hook(|| {
        if min.is_some() || max.is_some() {
            let show = |t: Option<Time>, fallback: Time| {
                format_time(t.unwrap_or(fallback), hour_cycle, seconds, &strings)
            };
            let message = fill(
                &strings.time_out_of_range,
                &[
                    ("min", &show(min, Time::MIDNIGHT)),
                    ("max", &show(max, Time::from_hms(23, 59, 59).unwrap_or(Time::MIDNIGHT))),
                ],
            );
            field.add_validator(time_between(min, max, message));
        }
    });
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let label_id = format!("{}-label", ids.control);
    let value = use_memo(move || parse_iso_time(&field.value()));
    rsx! {
        Field { invalid,
            FieldLabel { id: label_id.clone(), html_for: ids.control.clone(), "{label}" }
            crate::components::time_picker::TimePicker {
                value: value(),
                on_value_change: move |t: Option<Time>| {
                    field.set(t.map(|t| format_iso_time(t, seconds)).unwrap_or_default())
                },
                hour_cycle,
                seconds,
                minute_step,
                min,
                max,
                labelled_by: label_id,
                first_segment_id: ids.control.clone(),
                invalid,
                described_by: field.described_by(description.is_some()),
                on_blur: move |_| field.blur(),
                on_mounted: move |e| field.mounted(e),
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}

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
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let label_id = format!("{}-label", ids.control);
    let value = use_memo(move || parse_iso_date_time(&field.value()));
    rsx! {
        Field { invalid,
            FieldLabel { id: label_id.clone(), html_for: ids.control.clone(), "{label}" }
            DateTimePicker {
                value: value(),
                on_value_change: move |v: Option<PrimitiveDateTime>| {
                    field.set(v.map(|v| format_iso_date_time(v, seconds)).unwrap_or_default())
                },
                hour_cycle,
                seconds,
                minute_step,
                default_time,
                labelled_by: label_id,
                time_segment_id: ids.control.clone(),
                invalid,
                described_by: field.described_by(description.is_some()),
                on_blur: move |_| field.blur(),
                on_mounted: move |e| field.mounted(e),
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}

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
) -> Element {
    let ids = field.ids();
    let error = field.error();
    let invalid = error.is_some();
    let label_id = format!("{}-label", ids.control);
    let value = use_memo(move || parse_iso_date_range(&field.value()));
    let mut within = use_focus_exit(Callback::new(move |_| field.blur()));
    rsx! {
        Field { invalid,
            // `for` points at the group, which a label can't focus; the group's
            // `aria-labelledby` is what names it.
            FieldLabel { id: label_id.clone(), html_for: ids.control.clone(), "{label}" }
            div {
                onfocusin: move |_| within.set(true),
                onfocusout: move |_| within.set(false),
                DateRangePicker {
                    // The group itself: a failed submit focuses it by id, and it is read with
                    // its label, description and error.
                    id: ids.control.clone(),
                    tabindex: "-1",
                    aria_labelledby: label_id,
                    aria_describedby: field.described_by(description.is_some()),
                    "data-invalid": invalid.then_some("true"),
                    selected_range: value(),
                    on_range_change: move |range: Option<DateRange>| {
                        field.set(range.map(format_iso_date_range).unwrap_or_default())
                    },
                    min_date,
                    max_date,
                    disabled_ranges,
                    month_count,
                }
            }
            if let Some(description) = description {
                FieldDescription { id: ids.description.clone(), "{description}" }
            }
            FieldError { id: ids.error.clone(), message: error }
        }
    }
}

/// Helpers for the code `#[derive(Form)]` generates. Not public API: they may change with the
/// derive at any time.
#[doc(hidden)]
pub mod __derive {
    use super::Validator;
    use std::fmt::Display;
    use std::rc::Rc;
    use std::str::FromStr;

    /// Not empty (whitespace only counts as empty), with a message that names the field.
    pub fn required(message: &'static str) -> Validator {
        Rc::new(move |v| {
            if v.trim().is_empty() {
                Err(message.into())
            } else {
                Ok(())
            }
        })
    }

    /// A checkbox that must be ticked.
    pub fn checked(message: &'static str) -> Validator {
        Rc::new(move |v| if v == "true" { Ok(()) } else { Err(message.into()) })
    }

    /// Run `rule` only when something was entered (optional fields).
    pub fn when_filled(rule: Validator) -> Validator {
        Rc::new(move |v| {
            if v.trim().is_empty() {
                Ok(())
            } else {
                rule(v)
            }
        })
    }

    /// `rule`, with its message replaced.
    pub fn with_message(rule: Validator, message: &'static str) -> Validator {
        Rc::new(move |v| rule(v).map_err(|_| message.into()))
    }

    /// The trimmed value parses as `T`.
    pub fn parses<T: FromStr>(message: &'static str) -> Validator {
        Rc::new(move |v| {
            v.trim()
                .parse::<T>()
                .map(|_| ())
                .map_err(|_| message.into())
        })
    }

    /// At least `min`. A value that does not parse passes here; [`parses`] reports it.
    pub fn at_least<T: FromStr + PartialOrd + Display + 'static>(min: T) -> Validator {
        Rc::new(move |v| match v.trim().parse::<T>() {
            Ok(n) if n < min => Err(format!("Must be at least {min}.")),
            _ => Ok(()),
        })
    }

    /// At most `max`. A value that does not parse passes here; [`parses`] reports it.
    pub fn at_most<T: FromStr + PartialOrd + Display + 'static>(max: T) -> Validator {
        Rc::new(move |v| match v.trim().parse::<T>() {
            Ok(n) if n > max => Err(format!("Must be at most {max}.")),
            _ => Ok(()),
        })
    }
}
