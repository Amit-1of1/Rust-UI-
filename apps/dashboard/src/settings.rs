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

#[component]
pub fn Settings() -> Element {
    let toasts = use_toast();
    let form = use_form();
    let company = form.field_with("Acme Inc", vec![required(), max_len(60)]);
    let billing = form.field_with("billing@acme.example", vec![required(), email()]);
    let tz = form.field_with("UTC", vec![required()]);
    let mut weekly = use_signal(|| true);
    let mut alerts = use_signal(|| false);
    let mut confirm_reset = use_signal(|| false);

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
                        on_valid: move |_| toasts.notify_success("Settings saved"),
                        TextField { field: company, label: "Company name", autocomplete: "organization" }
                        TextField {
                            field: billing,
                            label: "Billing email",
                            r#type: "email",
                            autocomplete: "email",
                            description: "Invoices go here.",
                        }
                        ComboboxField {
                            field: tz,
                            label: "Time zone",
                            placeholder: "Search time zones",
                            empty: "No time zone found.",
                            options: TIMEZONES.iter().map(|z| FieldOption::from(*z)).collect::<Vec<_>>(),
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
                    on_click: move |_| toasts.notify_warning("Reset is disabled in the demo"),
                    "Reset"
                }
            }
        }
    }
}
