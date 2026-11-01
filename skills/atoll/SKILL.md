---
name: atoll
description: Build Dioxus 0.7 UI with the Atoll kit (shadcn-style, accessible, animated components copied into the app). Use when the app has atoll-lock.json, a src/components folder from `atoll add`, imports `ui::prelude`, or the user asks for Dioxus components, forms, dialogs, themes or motion.
---

# Atoll kit

The kit's components are Rust source that `atoll add` copies into the app (or the `ui` crate, package `atoll-ui`, from git). They are the user's code once copied, so edit them freely, but keep their accessibility and motion behaviour intact.

## 1. Read the project first

Run this before writing UI code, and again after adding or updating components:

```sh
atoll info --json
```

It reports the Dioxus requirement, the theme file and its accent presets, every installed component or block with the folder it lives in, which files the user edited, any unresolved update conflicts, and whether the MCP server is set up. Trust it over guessing paths. If `atoll` is not installed: `cargo install --git https://github.com/Amit-1of1/Rust-UI- atoll-cli`.

If `theme` is missing, run `atoll --source <kit> init` and load it once near the root (`document::Stylesheet { href: asset!("/assets/theme.css") }`), or `ui::ThemeStyles {}` with the crate. Without it components render unstyled.

## 2. Find the right component before writing your own

- With the MCP server (`atoll mcp`, see the kit's docs/MCP.md): `search_components`, then `get_component` for props and an example.
- Without it: `atoll --source <kit> list` and `atoll --source <kit> docs <name>`. `atoll docs` alone prints the index with the kit-wide rules.

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
atoll --source <kit> add dialog button
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
atoll update --dry-run   # shows the diff
atoll update             # three-way merge that keeps the user's edits
```

Then run `atoll info --json` and resolve every file under `conflicts` (it holds `<<<<<<< yours` markers) before building.

## 6. Check your work

`cargo build`, then `dx serve` to look at it. When the build fails, run `atoll --source <kit> explain` (MCP: `explain_errors`) before guessing: it turns errors from kit components into the missing or misspelt prop, its docs, the enum variants that exist, or the component to `add`. Tab through what you built: every control must be reachable and show a focus ring; `atoll contract <name>` (MCP: `get_contract`) lists what each key must do.

## Starting a new app or a brand theme

- A new app of a kind the kit already has (admin dashboard, booking or other routed app with long forms, marketing site, web + desktop + mobile): `atoll create my-app --git --template dashboard` (or `booking`, `marketing`, `showcase`; MCP: `list_templates`) instead of building the shell from scratch. Anything else: `atoll create my-app --git`.
- A brand colour: `atoll theme --brand '#7c3aed' --name brand` (MCP: `generate_theme`), or `--from-image logo.png`. Never hand-pick an accent: the generated one is proven readable in light and dark.
