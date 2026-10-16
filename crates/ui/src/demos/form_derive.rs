use crate::components::button::Button;
use crate::components::callout::{Callout, CalloutVariant};
use crate::components::form::*;
use dioxus::prelude::*;

/// One struct gives the fields, their rules and their messages.
#[derive(crate::Form, Clone, PartialEq, Debug)]
// Inside the kit; apps leave this out (the default is `::ui::components::form`).
#[form(path = "crate::components::form")]
pub struct Booking {
    #[form(required, autocomplete = "name")]
    pub name: String,
    #[form(required, email, autocomplete = "email", description = "For the confirmation only.")]
    pub email: String,
    #[form(label = "Guests", min = 1, max = 8)]
    pub guests: u8,
    #[form(select = rooms, placeholder = "Choose a room")]
    pub room: Option<String>,
    #[form(textarea, max_len = 200, label = "Requests")]
    pub notes: Option<String>,
    #[form(required = "Accept the house rules to book.", label = "I accept the house rules")]
    pub rules: bool,
}

fn rooms() -> Vec<FieldOption> {
    vec![
        FieldOption::new("single", "Single"),
        FieldOption::new("double", "Double"),
        FieldOption::new("suite", "Suite"),
    ]
}

#[component]
pub fn Demo() -> Element {
    let form = Booking::use_form();
    let mut booked = use_signal(|| None::<Booking>);
    rsx! {
        div { style: "display: grid; gap: 1rem; max-width: 22rem;",
            Form {
                state: form,
                on_valid: move |_| {
                    booked.set(form.value());
                    form.reset();
                },
                {form.fields()}
                Button { r#type: "submit", "Reserve a room" }
            }
            if let Some(b) = booked() {
                Callout { variant: CalloutVariant::Success,
                    "Booked for {b.name}, {b.guests} guest(s)."
                }
            }
        }
    }
}
