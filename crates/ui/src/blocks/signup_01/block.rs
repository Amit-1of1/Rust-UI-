use crate::components::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::form::{
    custom, email, max_len, min_len, required, use_form, CheckboxField, Form, TextField,
};
use crate::components::input_otp::InputOtp;
use crate::components::link::TextLink;
use dioxus::prelude::*;

#[css_module("/src/blocks/signup_01/style.css")]
struct Styles;

/// What the person typed into [`Signup01`].
#[derive(Clone, PartialEq, Debug)]
pub struct SignupDetails {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// A two-step sign-up card. Step one asks for name, email, a password (typed twice) and
/// agreement to the terms. Step two asks for the 6-digit code you emailed; leave out
/// `on_verify` to skip it.
#[component]
pub fn Signup01(
    /// Called with valid details. Send the code here; the card then shows the code step.
    #[props(default)]
    on_submit: Callback<SignupDetails>,
    /// Called with the 6-digit code. Without it the card stops after step one.
    on_verify: Option<Callback<String>>,
    /// Called from "Send a new code".
    #[props(default)]
    on_resend: Callback<()>,
    /// A message from your server for the current step, such as "That code has expired."
    #[props(into, default)]
    error: Option<String>,
    #[props(into, default = "#terms".to_string())] terms_href: String,
    #[props(into, default = "#sign-in".to_string())] sign_in_href: String,
) -> Element {
    let form = use_form();
    let name = form.field(vec![required(), max_len(80)]);
    let email_field = form.field(vec![required(), email()]);
    let password = form.field(vec![required(), min_len(8)]);
    let confirm = form.field(vec![
        required(),
        custom(move |v| {
            if v == password.value() {
                Ok(())
            } else {
                Err("The passwords don't match.".into())
            }
        }),
    ]);
    let terms = form.field(vec![custom(|v| {
        if v == "true" {
            Ok(())
        } else {
            Err("Agree to the terms to continue.".into())
        }
    })]);
    let mut verifying = use_signal(|| false);
    let mut code = use_signal(String::new);
    let mut code_error = use_signal(|| None::<String>);
    let mut verify = move || {
        let Some(on_verify) = on_verify else { return };
        if code().len() == 6 {
            code_error.set(None);
            on_verify.call(code());
        } else {
            code_error.set(Some("Enter all 6 digits.".into()));
        }
    };

    rsx! {
        main { class: Styles::dx_block_signup, "data-block": "signup_01",
            Card { class: Styles::dx_block_signup_card,
                if verifying() {
                    CardHeader {
                        CardTitle {
                            h1 {
                                class: Styles::dx_block_signup_title,
                                tabindex: "-1",
                                // The step changed under the person's focus: say where they are now.
                                onmounted: move |e| async move {
                                    _ = e.set_focus(true).await;
                                },
                                "Check your email"
                            }
                        }
                        CardDescription { "We sent a 6-digit code to {email_field.value()}." }
                    }
                    CardContent { class: Styles::dx_block_signup_stack,
                        if let Some(message) = error.clone() {
                            ServerError { message }
                        }
                        form {
                            class: Styles::dx_block_signup_stack,
                            aria_label: "Verify your email",
                            onsubmit: move |e| {
                                e.prevent_default();
                                verify();
                            },
                            InputOtp {
                                label: "Verification code",
                                group: 3,
                                invalid: code_error().is_some(),
                                on_change: move |c: String| code.set(c),
                                on_complete: move |c: String| {
                                    code.set(c);
                                    verify();
                                },
                            }
                            p { class: Styles::dx_block_signup_error, role: "alert",
                                if let Some(message) = code_error() {
                                    "{message}"
                                }
                            }
                            Button { r#type: "submit", class: Styles::dx_block_signup_submit, "Verify" }
                        }
                    }
                    CardFooter { class: Styles::dx_block_signup_actions,
                        Button {
                            variant: ButtonVariant::Ghost,
                            onclick: move |_| verifying.set(false),
                            "Use a different email"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| on_resend.call(()),
                            "Send a new code"
                        }
                    }
                } else {
                    CardHeader {
                        CardTitle {
                            h1 { class: Styles::dx_block_signup_title, "Create an account" }
                        }
                        CardDescription { "It takes a minute. No card needed." }
                    }
                    CardContent {
                        if let Some(message) = error {
                            ServerError { message }
                        }
                        Form {
                            state: form,
                            aria_label: "Create an account",
                            on_valid: move |_| {
                                on_submit
                                    .call(SignupDetails {
                                        name: name.value().trim().to_string(),
                                        email: email_field.value().trim().to_string(),
                                        password: password.value(),
                                    });
                                if on_verify.is_some() {
                                    code.set(String::new());
                                    code_error.set(None);
                                    verifying.set(true);
                                }
                            },
                            TextField { field: name, label: "Name", autocomplete: "name" }
                            TextField {
                                field: email_field,
                                label: "Email",
                                r#type: "email",
                                autocomplete: "email",
                                placeholder: "you@example.com",
                            }
                            TextField {
                                field: password,
                                label: "Password",
                                r#type: "password",
                                autocomplete: "new-password",
                                description: "At least 8 characters.",
                            }
                            TextField {
                                field: confirm,
                                label: "Confirm password",
                                r#type: "password",
                                autocomplete: "new-password",
                            }
                            CheckboxField { field: terms, label: "I agree to the terms of service" }
                            p { class: Styles::dx_block_signup_muted,
                                TextLink { href: terms_href, "Read the terms" }
                            }
                            Button { r#type: "submit", class: Styles::dx_block_signup_submit, "Create account" }
                        }
                    }
                    CardFooter {
                        p { class: Styles::dx_block_signup_footer,
                            "Already have an account? "
                            TextLink { href: sign_in_href, "Sign in" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ServerError(message: String) -> Element {
    rsx! {
        Alert { variant: AlertVariant::Destructive, class: Styles::dx_block_signup_alert,
            AlertTitle { "Something went wrong" }
            AlertDescription { "{message}" }
        }
    }
}
