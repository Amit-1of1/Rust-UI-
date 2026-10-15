use crate::components::button::Button;
use crate::components::date_time_picker::*;
use crate::components::form::*;
use crate::components::time_picker::{HourCycle, Time};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let form = use_form();
    let departure = form.field(vec![custom(|v| {
        if v.is_empty() {
            Err("Pick a date and a time.".to_string())
        } else {
            Ok(())
        }
    })]);
    let mut saved = use_signal(|| None::<String>);
    rsx! {
        div { style: "display: grid; gap: 1rem; justify-items: start;",
            Form {
                state: form,
                on_valid: move |_| saved.set(Some(departure.value())),
                DateTimeField {
                    field: departure,
                    label: "Departure",
                    description: "Picking a date fills in 09:00.",
                    default_time: Time::from_hms(9, 0, 0).ok(),
                    minute_step: 5,
                }
                Button { r#type: "submit", "Book" }
            }
            if let Some(value) = saved() {
                p { role: "status", style: "margin: 0;", "Departure: {value}" }
            }
            DateTimePicker { label: "Return", hour_cycle: HourCycle::H12 }
        }
    }
}
