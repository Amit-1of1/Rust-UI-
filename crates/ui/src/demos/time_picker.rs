use crate::components::button::Button;
use crate::components::form::*;
use crate::components::time_picker::*;
use crate::components::ui_strings::use_ui_strings;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let strings = use_ui_strings();
    let mut meeting = use_signal(|| Time::from_hms(9, 30, 0).ok());
    let form = use_form();
    let reminder = form.field(vec![custom(|v| {
        if v.is_empty() {
            Err("Enter a time.".to_string())
        } else {
            Ok(())
        }
    })]);
    // Its own form, so the reminder form above keeps one time field.
    let shift_form = use_form();
    let night_shift = shift_form.field(vec![]);
    let mut saved = use_signal(|| None::<String>);
    let meeting_text = meeting()
        .map(|t| format_time(t, HourCycle::H24, false, &strings))
        .unwrap_or_else(|| "not set".to_string());
    rsx! {
        div { style: "display: grid; gap: 1rem; justify-items: start;",
            TimePicker {
                label: "Meeting",
                value: meeting(),
                on_value_change: move |t| meeting.set(t),
                minute_step: 15,
            }
            p { style: "margin: 0;", "Meeting: {meeting_text}" }
            TimePicker { label: "Alarm", hour_cycle: HourCycle::H12, seconds: true }
            // Up and Down skip hours outside 9:00 to 17:00; a typed 18:00 is marked invalid.
            TimePicker {
                label: "Office hours",
                min: Time::from_hms(9, 0, 0).ok(),
                max: Time::from_hms(17, 0, 0).ok(),
                minute_step: 30,
            }
            Form {
                state: form,
                on_valid: move |_| {
                    let text = parse_iso_time(&reminder.value())
                        .map(|t| format_time(t, HourCycle::H12, false, &strings));
                    saved.set(text);
                },
                TimeField {
                    field: reminder,
                    label: "Reminder",
                    hour_cycle: HourCycle::H12,
                    description: "Type the hour, minutes, then A or P.",
                }
                Button { r#type: "submit", "Set reminder" }
            }
            Form { state: shift_form,
                TimeField {
                    field: night_shift,
                    label: "Night shift start",
                    min: Time::from_hms(22, 0, 0).ok(),
                    max: Time::from_hms(2, 0, 0).ok(),
                    description: "Between 22:00 and 02:00.",
                }
            }
            if let Some(text) = saved() {
                p { role: "status", style: "margin: 0;", "Reminder set for {text}" }
            }
        }
    }
}
