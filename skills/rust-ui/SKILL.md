---
name: rust-ui
description: Build Dioxus 0.7 UI with the Rust UI kit (shadcn-style, accessible, animated components copied into the app). Use when the app has rust-ui-lock.json, a src/components folder from `rust-ui add`, imports `ui::prelude`, or the user asks for Dioxus components, forms, dialogs, themes or motion.
---

# Rust UI kit

The kit's components are Rust source that `rust-ui add` copies into the app (or the `ui` crate, package `rust-ui-kit`, from git). They are the user's code once copied, so edit them freely, but keep their accessibility and motion behaviour intact.

## 1. Read the project first

Run this before writing UI code, and again after adding or updating components:

```sh
rust-ui info --json
```

It reports the Dioxus requirement, the theme file and its accent presets, every installed component or block with the folder it lives in, which files the user edited, any unresolved update conflicts, and whether the MCP server is set up. Trust it over guessing paths. If `rust-ui` is not installed: `cargo install --git https://github.com/Amit-1of1/Rust-UI- rust-ui-cli`.

If `theme` is missing, run `rust-ui --source <kit> init` and load it once near the root (`document::Stylesheet { href: asset!("/assets/theme.css") }`), or `ui::ThemeStyles {}` with the crate. Without it components render unstyled.

## 2. Find the right component before writing your own

- With the MCP server (`rust-ui mcp`, see the kit's docs/MCP.md): `search_components`, then `get_component` for props and an example.
- Without it: `rust-ui --source <kit> list` and `rust-ui --source <kit> docs <name>`. `rust-ui docs` alone prints the index with the kit-wide rules.

Read the component's page before using it: prop names come from its Rust signatures, and the example shows the intended composition. Common choices:

| Need | Use |
| --- | --- |
| Form with validation | `#[derive(Form)]` + `form` and `field`; `input`, `select`, `combobox`, `checkbox`, `number_field`, `date_picker`, `time_picker` |
| Modal / side panel / bottom panel | `dialog` / `sheet` / `drawer`; destructive confirm: `alert_dialog` |
| Menu on a button / right-click | `dropdown_menu` / `context_menu`; app menu bar: `menubar` |
| Command palette (Ctrl+K) | `command` with `ui::hooks::use_hotkey` |
| Choose one of a few / toggle options | `radio_group`, `toggle_group`, `tabs`; many: `multi_select`, `chips` |
| Feedback | `toast`, `alert`, `callout`, `progress`, `spinner`, `skeleton`, `empty` |
| Data | `table`, `data_table`, `virtual_list`, `chart`, `pagination` |
| Whole screens | blocks `login_01`, `signup_01`, `settings_01`, `dashboard_01` |

## 3. Add, then wire it up

```sh
rust-ui --source <kit> add dialog button
```

It copies the items and the components they depend on into `src/components` (blocks into `src/blocks`) and prints the `pub mod` lines to add to `src/components/mod.rs`. Import with `use crate::components::dialog::*;`. Through MCP, `add_component` does the same.

## 4. Rules that keep the kit's guarantees

- Restyle with `class` and the theme's custom properties (`--primary-color`, `--accent-color`, `--radius`, ...). No hard-coded colours. Light/dark: `data-theme` on `<html>`; accents: `data-accent` with a name `info` listed.
- Don't change a component's markup, roles, `tabindex` or key handling; ARIA and focus come from `dioxus-primitives`.
- Required accessibility props are enforced by the compiler: `Dialog`, `Sheet`, `Drawer`, `AlertDialog` need `title` (use `hide_title` to hide it visually), `IconButton` needs `label`, `AvatarImage` needs `alt` (`""` only for decorative images). When the compiler asks for one, give a real value; don't work around it.
- Every animation must respect `prefers-reduced-motion`. Use `ui::motion` (or the copied motion helpers) instead of raw CSS keyframes or JavaScript.
- No `document::eval` for behaviour the kit already has.
- Dioxus is pinned to 0.7: write 0.7 APIs (`#[component]`, `rsx!`, signals), not 0.6 or 0.8 ones.

## 5. Updating copied components

Never re-run `add --force` on files `info` lists as edited: it overwrites the user's changes. Instead:

```sh
rust-ui update --dry-run   # shows the diff
rust-ui update             # three-way merge that keeps the user's edits
```

Then run `rust-ui info --json` and resolve every file under `conflicts` (it holds `<<<<<<< yours` markers) before building.

## 6. Check your work

`cargo build`, then `dx serve` to look at it. Tab through what you built: every control must be reachable and show a focus ring.
