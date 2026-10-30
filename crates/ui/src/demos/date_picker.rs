use crate::components::button::Button;
use crate::components::date_picker::*;
use crate::components::form::*;
use dioxus::prelude::*;

use time::Date;

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);
    let mut selected_range = use_signal(|| None::<DateRange>);
    let form = use_form();
    let stay = form.field(vec![
        custom(|v| {
            if v.is_empty() {
                Err("Choose your dates.".to_string())
            } else {
                Ok(())
            }
        }),
        days_between(Some(2), Some(14), "Stays are 2 to 14 nights."),
    ]);
    let mut booked = use_signal(|| None::<String>);

    rsx! {
        div { style: "display: grid; gap: 1rem; justify-items: start;",
            DatePicker {
                selected_date: selected_date(),
                on_value_change: move |v| {
                    tracing::info!("Selected date changed: {:?}", v);
                    selected_date.set(v);
                },
            }
            DateRangePicker {
                selected_range: selected_range(),
                on_range_change: move |r| selected_range.set(r),
            }
            Form {
                state: form,
                on_valid: move |_| booked.set(Some(stay.value())),
                DateRangeField {
                    field: stay,
                    label: "Stay",
                    description: "Check-in and check-out, 2 to 14 nights.",
                }
                Button { r#type: "submit", "Check availability" }
            }
            if let Some(range) = booked() {
                p { role: "status", style: "margin: 0;", "Looking for {range}" }
            }
        }
    }
}
