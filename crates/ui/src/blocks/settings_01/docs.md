# Settings 01

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
- `embedded: bool`: set inside a layout that already has a `main`.

## Accessibility

- The page title is an `h1` and each card title an `h2`; the tab list is named "Settings sections".
- Tabs use manual activation: arrow keys move, Enter or Space opens (docs/GUARDRAILS.md).
- "Saved." is a `role="status"` line, so it is announced without moving focus.
- Each switch is labelled by its row text; deleting asks first in an alert dialog, which traps focus and returns it on close.
