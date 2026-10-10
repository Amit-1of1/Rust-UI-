use super::component::*;
use crate::components::button::Button;
use crate::components::callout::{Callout, CalloutVariant};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let form = use_form();
    let email = form.field(vec![required(), email()]);
    let password = form.field(vec![required(), min_len(8)]);
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
                Button { r#type: "submit", "Create account" }
            }
            if done() {
                Callout { variant: CalloutVariant::Success, "Account created." }
            }
        }
    }
}
