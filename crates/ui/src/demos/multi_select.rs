use crate::components::multi_select::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut picked = use_signal(|| vec!["design".to_string()]);
    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            MultiSelect {
                label: "Teams",
                placeholder: "Choose teams",
                default_selected: vec!["design".to_string()],
                options: vec![
                    MultiOption::new("design", "Design"),
                    MultiOption::new("eng", "Engineering"),
                    MultiOption::new("ops", "Operations"),
                    MultiOption::new("sales", "Sales"),
                    MultiOption::new("support", "Support"),
                ],
                on_change: move |v| picked.set(v),
            }
            p { style: "margin: 0; font-size: 0.875rem;", "Selected: {picked().join(\", \")}" }
        }
    }
}
