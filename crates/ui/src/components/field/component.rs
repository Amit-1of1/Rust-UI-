use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/components/field/style.css")]
struct Styles;

/// DOM ids that tie a control to its description and error message, so assistive
/// technology reads them together. Create with [`use_field_ids`].
#[derive(Clone, PartialEq, Debug)]
pub struct FieldIds {
    pub control: String,
    pub description: String,
    pub error: String,
}

impl FieldIds {
    /// Value for the control's `aria-describedby`: the description, plus the error when there is one.
    pub fn described_by(&self, has_error: bool) -> String {
        if has_error {
            format!("{} {}", self.description, self.error)
        } else {
            self.description.clone()
        }
    }
}

/// Stable, unique ids for one field.
pub fn use_field_ids() -> FieldIds {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    use_hook(|| {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        FieldIds {
            control: format!("dx-field-{n}"),
            description: format!("dx-field-{n}-description"),
            error: format!("dx-field-{n}-error"),
        }
    })
}

/// Groups a label, a control, a description and an error message.
///
/// ```ignore
/// let ids = use_field_ids();
/// Field { invalid: error().is_some(),
///     FieldLabel { html_for: ids.control.clone(), "Email" }
///     Input { id: ids.control.clone(), aria_invalid: error().is_some(),
///             aria_describedby: ids.described_by(error().is_some()) }
///     FieldDescription { id: ids.description.clone(), "We never share it." }
///     FieldError { id: ids.error.clone(), message: error() }
/// }
/// ```
#[component]
pub fn Field(
    #[props(default)] invalid: bool,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "field",
            "data-invalid": invalid.then_some("true"),
            ..merge_attributes(vec![attributes!(div { class: Styles::dx_field }), attributes]),
            {children}
        }
    }
}

#[component]
pub fn FieldLabel(
    #[props(into)] html_for: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        label { r#for: html_for, ..merge_attributes(vec![attributes!(div { class: Styles::dx_field_label }), attributes]), {children} }
    }
}

#[component]
pub fn FieldDescription(
    #[props(into)] id: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        p { id, ..merge_attributes(vec![attributes!(div { class: Styles::dx_field_description }), attributes]), {children} }
    }
}

/// Renders nothing until there is a message, then announces it (`role="alert"`).
#[component]
pub fn FieldError(#[props(into)] id: String, #[props(default)] message: Option<String>) -> Element {
    match message {
        Some(message) => rsx! {
            p { class: Styles::dx_field_error, id, role: "alert", "{message}" }
        },
        None => rsx! {},
    }
}

/// Small validators for common cases. Each returns the message to show on failure.
pub mod validate {
    pub fn required(value: &str) -> Result<(), String> {
        if value.trim().is_empty() {
            Err("This field is required.".into())
        } else {
            Ok(())
        }
    }

    pub fn min_len(value: &str, min: usize) -> Result<(), String> {
        if value.chars().count() < min {
            Err(format!("Must be at least {min} characters."))
        } else {
            Ok(())
        }
    }

    /// A deliberately loose check (`x@y.z`); real verification is sending an email.
    pub fn email(value: &str) -> Result<(), String> {
        let ok = value.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty()
                && !domain.contains('@')
                && domain
                    .split_once('.')
                    .is_some_and(|(host, tld)| !host.is_empty() && !tld.is_empty())
                && !value.contains(char::is_whitespace)
        });
        if ok {
            Ok(())
        } else {
            Err("Enter a valid email address.".into())
        }
    }
}
