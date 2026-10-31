use crate::components::label::Label;
use crate::components::native_select::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut plan = use_signal(|| "team".to_string());
    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 16rem;",
            Label { html_for: "native-select-plan", "Plan" }
            NativeSelect {
                id: "native-select-plan",
                value: plan(),
                onchange: move |e: FormEvent| plan.set(e.value()),
                NativeSelectOption { value: "starter", "Starter" }
                NativeSelectOption { value: "team", "Team" }
                NativeSelectOption { value: "business", "Business" }
                NativeSelectGroup { label: "Legacy",
                    NativeSelectOption { value: "pro-2024", disabled: true, "Pro (2024)" }
                }
            }
            p { style: "margin: 0; font-size: 0.875rem;", "Selected plan: {plan}" }
        }
    }
}
