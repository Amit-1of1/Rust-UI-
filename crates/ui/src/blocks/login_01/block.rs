use crate::components::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};
use crate::components::button::Button;
use crate::components::card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::form::{email, required, use_form, CheckboxField, Form, TextField};
use crate::components::link::TextLink;
use dioxus::prelude::*;

#[css_module("/src/blocks/login_01/style.css")]
struct Styles;

/// What the person typed into [`Login01`].
#[derive(Clone, PartialEq, Debug)]
pub struct LoginDetails {
    pub email: String,
    pub password: String,
    pub remember: bool,
}

/// A centred sign-in card: email, password, "remember me", a forgot-password link and a link
/// to sign up. Fields are checked before `on_submit` runs; a failed submit moves focus to the
/// first field that needs fixing.
#[component]
pub fn Login01(
    /// Called with valid details. Check them on your server, then set `error` or move on.
    #[props(default)]
    on_submit: Callback<LoginDetails>,
    /// A message from your server, such as "Wrong email or password." Announced when it appears.
    #[props(into, default)]
    error: Option<String>,
    /// While true the button is disabled and says "Signing in…".
    #[props(default)]
    busy: bool,
    #[props(into, default = "#forgot-password".to_string())] forgot_href: String,
    #[props(into, default = "#sign-up".to_string())] sign_up_href: String,
) -> Element {
    let form = use_form();
    let email_field = form.field(vec![required(), email()]);
    let password = form.field(vec![required()]);
    let remember = form.field(vec![]);

    rsx! {
        main { class: Styles::dx_block_login, "data-block": "login_01",
            Card { class: Styles::dx_block_login_card,
                CardHeader {
                    CardTitle {
                        h1 { class: Styles::dx_block_login_title, "Sign in" }
                    }
                    CardDescription { "Enter your email and password to continue." }
                }
                CardContent {
                    if let Some(message) = error {
                        Alert { variant: AlertVariant::Destructive, class: Styles::dx_block_login_alert,
                            AlertTitle { "Couldn't sign you in" }
                            AlertDescription { "{message}" }
                        }
                    }
                    Form {
                        state: form,
                        aria_label: "Sign in",
                        on_valid: move |_| {
                            on_submit
                                .call(LoginDetails {
                                    email: email_field.value().trim().to_string(),
                                    password: password.value(),
                                    remember: remember.value() == "true",
                                })
                        },
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
                            autocomplete: "current-password",
                        }
                        div { class: Styles::dx_block_login_row,
                            CheckboxField { field: remember, label: "Remember me" }
                            TextLink { href: forgot_href, "Forgot password?" }
                        }
                        Button {
                            r#type: "submit",
                            class: Styles::dx_block_login_submit,
                            disabled: busy,
                            if busy {
                                "Signing in…"
                            } else {
                                "Sign in"
                            }
                        }
                    }
                }
                CardFooter {
                    p { class: Styles::dx_block_login_footer,
                        "No account yet? "
                        TextLink { href: sign_up_href, "Sign up" }
                    }
                }
            }
        }
    }
}
