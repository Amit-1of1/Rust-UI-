//! Settings: a longer form mixing text fields with non-text controls.

use dioxus::prelude::*;
use ui::prelude::*;

const TIMEZONES: [&str; 8] = [
    "UTC",
    "Europe/London",
    "Europe/Berlin",
    "America/New_York",
    "America/Los_Angeles",
    "Asia/Kolkata",
    "Asia/Tokyo",
    "Australia/Sydney",
];

#[component(lazy)]
pub fn Settings() -> Element {
    let toasts = use_toast();
    let form = use_form();
    let company = form.field(vec![required(), max_len(60)]);
    let billing = form.field(vec![required(), email()]);
    let mut tz = use_signal(|| Some("UTC".to_string()));
    let mut tz_query = use_signal(String::new);
    let mut weekly = use_signal(|| true);
    let mut alerts = use_signal(|| false);
    let mut confirm_reset = use_signal(|| false);

    use_hook(move || {
        company.set("Acme Inc");
        billing.set("billing@acme.example");
    });

    let tz_error = tz().is_none().then_some("Choose a time zone");

    rsx! {
        div { class: "db-grid-2 db-settings",
            Card {
                CardHeader {
                    CardTitle { "Workspace" }
                    CardDescription { "Shown on invoices and emails." }
                }
                CardContent {
                    Form {
                        id: "settings-form",
                        state: form,
                        on_valid: move |_| {
                            if tz().is_some() {
                                toasts.success("Settings saved".to_string(), ToastOptions::new());
                            }
                        },
                        TextField { field: company, label: "Company name", autocomplete: "organization" }
                        TextField {
                            field: billing,
                            label: "Billing email",
                            r#type: "email",
                            autocomplete: "email",
                            description: "Invoices go here.",
                        }
                        div { class: "db-field",
                            span { id: "tz-label", class: "db-label", "Time zone" }
                            Combobox::<String> {
                                value: Some(tz.into()),
                                on_value_change: move |v: Option<String>| tz.set(v),
                                query: Some(tz_query()),
                                on_query_change: move |q| tz_query.set(q),
                                placeholder: "Search time zones",
                                aria_label: "Time zone",
                                list_aria_label: "Time zones",
                                ComboboxEmpty { "No time zone found." }
                                for (i , z) in TIMEZONES.iter().enumerate() {
                                    ComboboxOption::<String> {
                                        index: i,
                                        value: z.to_string(),
                                        text_value: z.to_string(),
                                        {*z}
                                    }
                                }
                            }
                            if let Some(e) = tz_error {
                                p { class: "db-error", role: "alert", "{e}" }
                            }
                        }
                        div { class: "db-actions",
                            Button { r#type: "submit", "Save" }
                        }
                    }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Notifications" }
                    CardDescription { "Email me when…" }
                }
                CardContent {
                    div { class: "db-switch-row",
                        Label { html_for: "weekly", "Weekly summary" }
                        Switch {
                            id: "weekly",
                            checked: weekly(),
                            on_checked_change: move |v| weekly.set(v),
                        }
                    }
                    div { class: "db-switch-row",
                        Label { html_for: "alerts", "Payment failures" }
                        Switch {
                            id: "alerts",
                            checked: alerts(),
                            on_checked_change: move |v| alerts.set(v),
                        }
                    }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Appearance" }
                }
                CardContent { ThemeCustomizer {} }
            }
            Card {
                CardHeader {
                    CardTitle { "Danger zone" }
                    CardDescription { "Remove all customers and orders from this workspace." }
                }
                CardFooter {
                    Button {
                        variant: ButtonVariant::Destructive,
                        onclick: move |_| confirm_reset.set(true),
                        "Reset workspace"
                    }
                }
            }
        }
        AlertDialog {
            open: confirm_reset(),
            on_open_change: move |v| confirm_reset.set(v),
            AlertDialogTitle { "Reset the workspace?" }
            AlertDialogDescription { "This demo only pretends; nothing is deleted." }
            AlertDialogActions {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction {
                    on_click: move |_| toasts.warning("Reset is disabled in the demo".to_string(), ToastOptions::new()),
                    "Reset"
                }
            }
        }
    }
}
