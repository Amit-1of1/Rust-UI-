use crate::components::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogActions, AlertDialogCancel,
};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{
    Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::form::{email, max_len, required, use_form, Form, TextField, TextareaField};
use crate::components::label::Label;
use crate::components::switch::Switch;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::theme_customizer::ThemeCustomizer;
use dioxus::prelude::*;

#[css_module("/src/blocks/settings_01/style.css")]
struct Styles;

/// The profile fields of [`Settings01`].
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub bio: String,
}

/// Which emails the person wants, from the Notifications tab of [`Settings01`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct EmailPrefs {
    pub product: bool,
    pub security: bool,
    pub weekly: bool,
}

impl Default for EmailPrefs {
    fn default() -> Self {
        Self {
            product: false,
            security: true,
            weekly: true,
        }
    }
}

/// A settings page in four tabs: Profile (a form), Notifications (switches), Appearance
/// (the theme customizer) and Account (a danger zone behind a confirmation).
#[component]
pub fn Settings01(
    /// The values the profile form starts with.
    #[props(default)]
    profile: Profile,
    /// Called with a valid profile from "Save".
    #[props(default)]
    on_save_profile: Callback<Profile>,
    #[props(default)] email_prefs: EmailPrefs,
    /// Called on every switch change.
    #[props(default)]
    on_email_prefs: Callback<EmailPrefs>,
    /// Called once the person confirms "Delete account".
    #[props(default)]
    on_delete_account: Callback<()>,
    /// Set when the page already has a `<main>` and an `h1`, as inside `DashboardShell`;
    /// otherwise this block is the page's `main` landmark with its own "Settings" `h1`.
    #[props(default)]
    embedded: bool,
) -> Element {
    let form = use_form();
    let name = form.field_with(profile.name.clone(), vec![required(), max_len(80)]);
    let email_field = form.field_with(profile.email.clone(), vec![required(), email()]);
    let bio = form.field_with(profile.bio.clone(), vec![max_len(280)]);
    let mut saved = use_signal(|| false);
    let mut prefs = use_signal(|| email_prefs);
    let mut confirm_delete = use_signal(|| false);
    let mut set_pref = move |change: fn(&mut EmailPrefs, bool), on: bool| {
        change(&mut prefs.write(), on);
        on_email_prefs.call(prefs());
    };

    let page = rsx! {
        header { class: Styles::dx_block_settings_header,
            // Embedded, the page around it (DashboardShell) already has the h1.
            if !embedded {
                h1 { class: Styles::dx_block_settings_title, "Settings" }
            }
            p { class: Styles::dx_block_settings_muted, "Manage your profile, emails and appearance." }
        }
        Tabs { default_value: "profile".to_string(), horizontal: true,
            TabList { class: Styles::dx_block_settings_tabs, aria_label: "Settings sections",
                TabTrigger { value: "profile".to_string(), index: 0usize, "Profile" }
                TabTrigger { value: "notifications".to_string(), index: 1usize, "Notifications" }
                TabTrigger { value: "appearance".to_string(), index: 2usize, "Appearance" }
                TabTrigger { value: "account".to_string(), index: 3usize, "Account" }
            }
            TabContent { index: 0usize, value: "profile".to_string(),
                Card {
                    CardHeader {
                        CardTitle {
                            h2 { class: Styles::dx_block_settings_card_title, "Profile" }
                        }
                        CardDescription { "How you appear to others." }
                    }
                    CardContent {
                        // Any edit clears "Saved." (input events bubble up from the fields).
                        div { oninput: move |_| saved.set(false),
                        Form {
                            state: form,
                            aria_label: "Profile",
                            on_valid: move |_| {
                                saved.set(true);
                                on_save_profile
                                    .call(Profile {
                                        name: name.value().trim().to_string(),
                                        email: email_field.value().trim().to_string(),
                                        bio: bio.value(),
                                    });
                            },
                            TextField { field: name, label: "Name", autocomplete: "name" }
                            TextField {
                                field: email_field,
                                label: "Email",
                                r#type: "email",
                                autocomplete: "email",
                            }
                            TextareaField {
                                field: bio,
                                label: "Bio",
                                description: "Up to 280 characters.",
                                autosize: true,
                            }
                            div { class: Styles::dx_block_settings_actions,
                                p { class: Styles::dx_block_settings_muted, role: "status",
                                    if saved() {
                                        "Saved."
                                    }
                                }
                                Button { r#type: "submit", "Save" }
                            }
                        }
                        }
                    }
                }
            }
            TabContent { index: 1usize, value: "notifications".to_string(),
                Card {
                    CardHeader {
                        CardTitle {
                            h2 { class: Styles::dx_block_settings_card_title, "Email me about" }
                        }
                        CardDescription { "Changes are saved as you go." }
                    }
                    CardContent {
                        PrefRow {
                            id: "settings-pref-product",
                            label: "Product news",
                            checked: prefs().product,
                            on_change: move |on| set_pref(|p, on| p.product = on, on),
                        }
                        PrefRow {
                            id: "settings-pref-security",
                            label: "Security alerts",
                            checked: prefs().security,
                            on_change: move |on| set_pref(|p, on| p.security = on, on),
                        }
                        PrefRow {
                            id: "settings-pref-weekly",
                            label: "Weekly summary",
                            checked: prefs().weekly,
                            on_change: move |on| set_pref(|p, on| p.weekly = on, on),
                        }
                    }
                }
            }
            TabContent { index: 2usize, value: "appearance".to_string(),
                Card {
                    CardHeader {
                        CardTitle {
                            h2 { class: Styles::dx_block_settings_card_title, "Appearance" }
                        }
                        CardDescription { "Accent colour, corners and light or dark mode." }
                    }
                    CardContent { ThemeCustomizer {} }
                }
            }
            TabContent { index: 3usize, value: "account".to_string(),
                Card { class: Styles::dx_block_settings_danger,
                    CardHeader {
                        CardTitle {
                            h2 { class: Styles::dx_block_settings_card_title, "Delete account" }
                        }
                        CardDescription { "Removes your profile and everything in it. This can't be undone." }
                    }
                    CardFooter {
                        Button {
                            variant: ButtonVariant::Destructive,
                            onclick: move |_| confirm_delete.set(true),
                            "Delete account"
                        }
                    }
                }
            }
        }
        AlertDialog {
            open: confirm_delete(),
            on_open_change: move |v| confirm_delete.set(v),
            title: "Delete your account?",
            description: "Your profile and everything in it will be removed. This can't be undone.",
            AlertDialogActions {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction { on_click: move |_| on_delete_account.call(()), "Delete account" }
            }
        }
    };
    // A page of its own is the `main` landmark; inside a shell that has one, a plain `div`.
    if embedded {
        rsx! {
            div { class: Styles::dx_block_settings, "data-block": "settings_01", {page} }
        }
    } else {
        rsx! {
            main { class: Styles::dx_block_settings, "data-block": "settings_01", {page} }
        }
    }
}

#[component]
fn PrefRow(
    id: &'static str,
    label: &'static str,
    checked: bool,
    on_change: Callback<bool>,
) -> Element {
    rsx! {
        div { class: Styles::dx_block_settings_row,
            Label { html_for: id, "{label}" }
            Switch {
                id,
                checked,
                on_checked_change: move |v| on_change.call(v),
            }
        }
    }
}
