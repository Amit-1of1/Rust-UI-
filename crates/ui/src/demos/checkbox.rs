use crate::components::checkbox::*;
use crate::components::label::Label;
use dioxus::prelude::*;

const TOPICS: [&str; 3] = ["Invoices", "Payments", "Team changes"];

#[component]
pub fn Demo() -> Element {
    let mut picked = use_signal(|| vec![true, false, false]);
    let all = use_memo(move || {
        let p = picked();
        if p.iter().all(|v| *v) {
            CheckboxState::Checked
        } else if p.iter().any(|v| *v) {
            CheckboxState::Indeterminate
        } else {
            CheckboxState::Unchecked
        }
    });
    let row = "display: flex; align-items: center; gap: 0.5rem;";
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; align-items: flex-start; gap: 2rem;",
            div { style: row,
                Checkbox { id: "terms", name: "terms" }
                Label { html_for: "terms", "Accept terms" }
            }
            CheckboxGroup { label: "Email me about",
                div { style: row,
                    Checkbox { id: "topic-all",
                        checked: Some(all()),
                        on_checked_change: move |state| {
                            let on = state != CheckboxState::Unchecked;
                            picked.set(vec![on; TOPICS.len()]);
                        },
                    }
                    Label { html_for: "topic-all", "Everything" }
                }
                for (i, topic) in TOPICS.iter().enumerate() {
                    div { key: "{topic}", style: "{row} padding-inline-start: 1.5rem;",
                        Checkbox { id: "topic-{i}",
                            checked: Some(if picked()[i] { CheckboxState::Checked } else { CheckboxState::Unchecked }),
                            on_checked_change: move |state| picked.write()[i] = state == CheckboxState::Checked,
                        }
                        Label { html_for: "topic-{i}", "{topic}" }
                    }
                }
            }
        }
    }
}
