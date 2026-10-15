use dioxus::prelude::*;
use ui::prelude::*;

const PLANS: [&str; 3] = ["Hobby", "Team", "Enterprise"];

#[component]
pub fn Forms() -> Element {
    let toasts = use_toast();
    let name_ids = use_field_ids();
    let email_ids = use_field_ids();
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut name_error = use_signal(|| None::<String>);
    let mut email_error = use_signal(|| None::<String>);
    let mut plan = use_signal(|| Some(PLANS[1].to_string()));
    let mut seats = use_signal(|| 5.0_f64);
    let mut updates = use_signal(|| true);
    let mut saved = use_signal(|| None::<String>);

    let submit = move |event: FormEvent| {
        event.prevent_default();
        name_error.set(validate::required(&name.read()).err());
        email_error.set(
            validate::required(&email.read())
                .and_then(|_| validate::email(&email.read()))
                .err(),
        );
        if name_error.read().is_none() && email_error.read().is_none() {
            let summary = format!(
                "{} on {}, {} seats",
                name.read().trim(),
                plan().unwrap_or_default(),
                seats() as u32
            );
            toasts.notify_success(("Saved", summary.clone()));
            saved.set(Some(summary));
        } else {
            saved.set(None);
        }
    };

    rsx! {
        Card { class: "sc-form-card",
            CardHeader {
                CardTitle { "Workspace" }
                CardDescription { "Validation, selects, sliders and toggles, with no JavaScript." }
            }
            CardContent {
                form { id: "workspace-form", class: "sc-form", onsubmit: submit, novalidate: true,
                    Field { invalid: name_error().is_some(),
                        FieldLabel { html_for: name_ids.control.clone(), "Workspace name" }
                        Input {
                            id: name_ids.control.clone(),
                            placeholder: "Acme",
                            value: name,
                            aria_invalid: name_error().is_some().then_some("true"),
                            aria_describedby: name_ids.described_by(name_error().is_some()),
                            oninput: move |e: FormEvent| name.set(e.value()),
                        }
                        FieldDescription { id: name_ids.description.clone(), "Shown in the header." }
                        FieldError { id: name_ids.error.clone(), message: name_error() }
                    }
                    Field { invalid: email_error().is_some(),
                        FieldLabel { html_for: email_ids.control.clone(), "Billing email" }
                        Input {
                            id: email_ids.control.clone(),
                            r#type: "email",
                            placeholder: "you@example.com",
                            value: email,
                            aria_invalid: email_error().is_some().then_some("true"),
                            aria_describedby: email_ids.described_by(email_error().is_some()),
                            oninput: move |e: FormEvent| email.set(e.value()),
                        }
                        FieldDescription { id: email_ids.description.clone(), "Invoices go here." }
                        FieldError { id: email_ids.error.clone(), message: email_error() }
                    }
                    div { class: "sc-field",
                        span { class: "sc-label", id: "plan-label", "Plan" }
                        Select::<String> {
                            width: "100%",
                            aria_labelledby: "plan-label",
                            selected: plan(),
                            on_value_change: move |v: Option<String>| plan.set(v),
                            for (i , p) in PLANS.into_iter().enumerate() {
                                SelectOption::<String> { key: "{p}", index: i, value: p.to_string(), text_value: p, {p} }
                            }
                        }
                    }
                    div { class: "sc-field",
                        span { class: "sc-label", id: "billing-label", "Billing" }
                        RadioGroup { aria_labelledby: "billing-label", default_value: "monthly".to_string(),
                            RadioItem { value: "monthly".to_string(), index: 0usize, "Monthly" }
                            RadioItem { value: "yearly".to_string(), index: 1usize, "Yearly (2 months free)" }
                        }
                    }
                    div { class: "sc-field",
                        span { class: "sc-label", "Seats: {seats() as u32}" }
                        Slider {
                            label: "Seats",
                            horizontal: true,
                            min: 1.0,
                            max: 50.0,
                            step: 1.0,
                            default_value: 5.0,
                            on_value_change: move |v: f64| seats.set(v),
                        }
                    }
                    div { class: "sc-row",
                        Switch {
                            id: "updates",
                            checked: updates(),
                            aria_label: "Product updates by email",
                            on_checked_change: move |v| updates.set(v),
                        }
                        span { "Product updates by email" }
                    }
                    div { class: "sc-row",
                        Checkbox { id: "terms", name: "terms" }
                        Label { html_for: "terms", "I accept the terms" }
                    }
                    div { class: "sc-row",
                        Button { id: "save", r#type: "submit", "Save workspace" }
                    }
                    if let Some(s) = saved() {
                        p { role: "status", id: "saved", class: "sc-muted", "Saved: {s}" }
                    }
                }
            }
        }
    }
}
