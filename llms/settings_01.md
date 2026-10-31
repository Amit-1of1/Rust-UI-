# Settings 01

- Copy into your app: `atoll add settings_01` (also copies `alert_dialog`, `button`, `card`, `form`, `label`, `switch`, `tabs`, `theme_customizer`).
- Import: `use crate::blocks::settings_01::*;` after copying, or `use ui::blocks::settings_01::*;` with the `atoll-ui` crate's `blocks` feature (internal).

A settings page in four tabs: Profile (name, email, bio), Notifications (email switches), Appearance (the theme customizer) and Account (delete, behind a confirmation). Built from `tabs`, `card`, `form`, `switch`, `label`, `theme_customizer` and `alert_dialog`.

```sh
atoll add settings_01
```

It stands alone (its root is the page's `main`), or fits inside `DashboardShell` from `dashboard_01` with `embedded: true`, which makes the root a plain `div` because the shell already has a `main`:

```rust
rsx! {
    Settings01 {
        profile: Profile { name: "Ada".into(), email: "ada@example.com".into(), bio: String::new() },
        on_save_profile: move |p: Profile| save(p),
        on_email_prefs: move |e: EmailPrefs| save_prefs(e),
        on_delete_account: move |_| delete_account(),
    }
}
```

## Props

- `profile: Profile`: the values the form starts with.
- `on_save_profile: Callback<Profile>`: called only with valid fields.
- `email_prefs: EmailPrefs`, `on_email_prefs: Callback<EmailPrefs>`: called on every switch change.
- `on_delete_account: Callback<()>`: called after the person confirms.
- `embedded: bool`: set inside a layout that already has a `main` and an `h1` (the block then drops its own "Settings" `h1`).

## Accessibility

- The page title is an `h1` and each card title an `h2`; the tab list is named "Settings sections".
- Tabs use manual activation: arrow keys move, Enter or Space opens (docs/GUARDRAILS.md).
- "Saved." is a `role="status"` line, so it is announced without moving focus.
- Each switch is labelled by its row text; deleting asks first in an alert dialog, which traps focus and returns it on close.

## API

```rust
/// The profile fields of [`Settings01`].
pub struct Profile {
    pub name: String,
    pub email: String,
    pub bio: String,
}

/// Which emails the person wants, from the Notifications tab of [`Settings01`].
pub struct EmailPrefs {
    pub product: bool,
    pub security: bool,
    pub weekly: bool,
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
```
