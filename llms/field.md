# Field

- Copy into your app: `rust-ui add field` or `dx components add field --git https://github.com/Amit-1of1/Rust-UI-`.
- Import: `use crate::components::field::*;` after copying, or `use ui::prelude::*;` with the `rust-ui-kit` crate.

Wires a label, description and error to a control with matching ids. `use_field_ids()` returns the ids; for most forms use `TextField` from `form`.

## Accessibility

- Role and name: the control gets `aria-describedby` pointing at the description and error; `FieldError` renders `role="alert"` only when there is a message
- Keyboard: none of its own
- Reduced motion: none

## API

```rust
/// DOM ids that tie a control to its description and error message, so assistive
/// technology reads them together. Create with [`use_field_ids`].
pub struct FieldIds {
    pub control: String,
    pub description: String,
    pub error: String,
}

/// Stable, unique ids for one field.
pub fn use_field_ids() -> FieldIds

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
) -> Element

#[component]
pub fn FieldLabel(
    #[props(into)] html_for: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

#[component]
pub fn FieldDescription(
    #[props(into)] id: String,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element

/// Renders nothing until there is a message, then announces it (`role="alert"`).
#[component]
pub fn FieldError(#[props(into)] id: String, #[props(default)] message: Option<String>) -> Element {
    match message
```

## Example

```rust
use crate::components::button::Button;
use crate::components::field::*;
use crate::components::input::Input;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let email_ids = use_field_ids();
    let password_ids = use_field_ids();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut email_error = use_signal(|| None::<String>);
    let mut password_error = use_signal(|| None::<String>);
    let mut done = use_signal(|| false);

    let submit = move |event: FormEvent| {
        event.prevent_default();
        email_error.set(
            validate::required(&email.read())
                .and_then(|_| validate::email(&email.read()))
                .err(),
        );
        password_error.set(validate::min_len(&password.read(), 8).err());
        done.set(email_error.read().is_none() && password_error.read().is_none());
    };

    rsx! {
        form { style: "display: grid; gap: 1rem; max-width: 20rem;", onsubmit: submit, novalidate: true,
            Field { invalid: email_error().is_some(),
                FieldLabel { html_for: email_ids.control.clone(), "Email" }
                Input {
                    id: email_ids.control.clone(),
                    r#type: "email",
                    placeholder: "you@example.com",
                    value: email,
                    aria_invalid: email_error().is_some().then_some("true"),
                    aria_describedby: email_ids.described_by(email_error().is_some()),
                    oninput: move |e: FormEvent| email.set(e.value()),
                }
                FieldDescription { id: email_ids.description.clone(), "We never share it." }
                FieldError { id: email_ids.error.clone(), message: email_error() }
            }
            Field { invalid: password_error().is_some(),
                FieldLabel { html_for: password_ids.control.clone(), "Password" }
                Input {
                    id: password_ids.control.clone(),
                    r#type: "password",
                    value: password,
                    aria_invalid: password_error().is_some().then_some("true"),
                    aria_describedby: password_ids.described_by(password_error().is_some()),
                    oninput: move |e: FormEvent| password.set(e.value()),
                }
                FieldDescription { id: password_ids.description.clone(), "At least 8 characters." }
                FieldError { id: password_ids.error.clone(), message: password_error() }
            }
            Button { r#type: "submit", "Create account" }
            if done() {
                p { role: "status", style: "margin: 0; font-size: 0.875rem;", "Looks good. Account created." }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::field::validate::*;

    #[test]
    fn required_rejects_blank() {
        assert!(required("").is_err());
        assert!(required("   ").is_err());
        assert!(required("x").is_ok());
    }

    #[test]
    fn min_len_counts_characters_not_bytes() {
        assert!(min_len("ééééééé", 8).is_err());
        assert!(min_len("éééééééé", 8).is_ok());
    }

    #[test]
    fn email_accepts_and_rejects() {
        for ok in ["a@b.co", "first.last@example.com"] {
            assert!(email(ok).is_ok(), "{ok}");
        }
        for bad in [
            "", "a", "a@", "@b.co", "a@b", "a@b.", "a b@c.de", "a@@b.co", "a@b@c.de",
        ] {
            assert!(email(bad).is_err(), "{bad}");
        }
    }
}
```
