# Best practices

How to design and build product screens with Atoll so they stay consistent, accessible and easy to upgrade. The reasons behind these rules are in [PRINCIPLES.md](PRINCIPLES.md); worked examples are in [COMPOSITION.md](COMPOSITION.md).

## Designing with Atoll

Atoll has no Figma or Sketch library (yet). The design source of truth is the running kit: the gallery shows every component in every state, the docs site's [Theme builder](#theme-builder) previews your brand in light and dark, and `atoll theme --from-image logo.png` derives an accent from a logo. Design in those, or mock up in your tool with screenshots of the gallery, and name things the way the kit does.

- **Start from a block or an example app, not a blank page.** `login_01`, `signup_01`, `dashboard_01` and `settings_01` are whole screens built only from kit parts; `apps/dashboard`, `apps/booking` and `apps/site` show full flows. Changing one is faster and more consistent than drawing from scratch.
- **Pick variants, don't invent them.** Buttons have six variants and four sizes, badges four variants, and so on (the props table on each docs page lists them). A seventh button style is a design-system change: propose it ([CONTRIBUTING.md](../CONTRIBUTING.md)) instead of styling one button by hand.
- **Design in tokens.** Spacing, radius, colour and motion come from `theme.css` (`--radius`, `--density`, `--accent-color`, `--spring-snappy`, ...). If a design needs a value no token holds, that's a token proposal, not a hex code in a component.
- **Design both themes at once.** Every colour token is a light and dark pair. Check each screen in dark mode and with at least one accent preset before calling it done; text must reach 4.5:1 (WCAG AA) in both.
- **One primary action per view.** `ButtonVariant::Primary` is the one thing the screen is for; everything else is `Secondary`, `Outline` or `Ghost`. Destructive actions are `Destructive` and confirm in an `AlertDialog`.
- **Every control needs a visible label or an accessible one.** Icon-only buttons are `IconButton` with a `label`; a dialog has a `title` (hide it with `hide_title` if the content says it); a chart has a `label` and a text summary. Placeholder text is not a label.
- **Design the empty, loading and error states.** Use `Empty` for no data, `Skeleton` or `Spinner` while loading (with `DataTable { loading }` for tables), `FieldError` and `Callout` for errors. A screen with only the happy path isn't designed.
- **Mind density and touch.** `data-density` scales control heights; keep tap targets at least 24 by 24 CSS pixels (WCAG 2.2 AA) on touch layouts, and test the phone layout `apps/showcase` uses.
- **Motion explains, it doesn't decorate.** Use the overlays' built-in transitions and the spring presets. Anything new must have a reduced-motion version (usually: no motion) and must never delay input.
- **Right-to-left and long text.** Components use logical CSS properties and the kit's strings are translatable ([RTL.md](RTL.md)). Leave room for 30% longer labels, and check a screen with `dir="rtl"` if you ship to RTL locales.

Common pitfalls:

- Building a custom select, menu or dialog because the kit's looks slightly different. Restyle with tokens or `class` instead; the kit's version already has keyboard support and tests.
- Colour as the only signal (a red border alone for an error). Pair it with text or an icon.
- Hover-only actions. Anything revealed on hover must also appear on keyboard focus and stay visible on touch.

## Developing with Atoll

### Set-up

- **Pin what you depend on.** The `atoll-ui` git dependency takes a `tag` (once releases exist) or a `rev`, never a `branch` ([INSTALL.md](INSTALL.md)). Dioxus stays on the kit's minor (`0.7.x`).
- **Choose copy or crate per app, not per component.** Copying (`atoll add`) gives you the source to change; the crate gives you upgrades with `cargo update`. Mixing both for the same component gives you two `Button`s.
- **Load the theme once.** `ThemeStyles {}` near the root, or `atoll init` for copied components. Embedding in an existing app: use the scoped theme (`ThemeStyles { scoped: true }` and `atoll init --scoped`, [EXISTING_APP.md](EXISTING_APP.md)) so the kit's CSS can't leak into your pages.
- **Use the tooling.** `atoll doctor` finds hard-coded colours and undefined CSS variables, `atoll explain` translates kit-related compiler errors, `atoll docs <name>` and the MCP server give coding agents the real API.

### Writing screens

- **Import from the prelude** (`use ui::prelude::*;`) and icons by path (`use ui::icons::Search;`). Don't glob-import icons: their names clash with components.
- **Pass `class` and `style` as props; don't wrap to restyle.** A component merges your `class` with its own. Never write a `class:` next to `..attributes` in your own components either: two `class` attributes render and the browser drops one; merge with `merge_attributes`.
- **Keep state in your signals.** Controlled components (`open`, `value`, `pressed`, ...) take a value and a change callback; keep "which item to act on" in its own signal, not in the open flag (`AlertDialogAction` closes the dialog before it calls `on_click`).
- **Forms:** `Field` plus `use_field_ids` wires labels, hints and errors; `#[derive(Form)]` builds a whole validated form from a struct. Validate on submit and show errors next to the field, not only in a toast.
- **Theme state has one source:** `use_theme()`. Don't write `data-theme` on `<html>` yourself.
- **Lists that animate need stable keys:** `AnimatePresence` and `FlipList` children take `key: "{id}"` from your data, never an index.
- **Routing:** the kit's links are plain `<a href>`. Under the Dioxus router, use the router's `Link` with `button_look(...)` for a button look.
- **Never touch the network in demos or tests,** and keep pointer interactions inside the viewport in Playwright tests.

### Shipping

- **Release builds** use `strip = "debuginfo"`, never `strip = true` (on Rust 1.98+ it breaks every wasm build). `scripts/release-web.sh` shows the full release pipeline (CSS bundling, optional wasm splitting).
- **Prove your screens with the kit's checks.** The axe, keyboard and ARIA checks that run on the gallery can run on your app too ([EXISTING_APP.md, section 6](EXISTING_APP.md)).
- **Read the CHANGELOG before bumping,** and run `atoll diff` on copied components; `atoll update` merges upstream changes three ways and keeps your edits.

### Recommended dependencies

| Need | Use |
|---|---|
| UI framework | `dioxus` 0.7.x (the kit's minor) |
| Routing | `dioxus` with the `router` feature |
| Icons | the kit's generated Lucide set (`ui::icons`) |
| Motion | `ui::motion` (wraps the engine; don't depend on it directly) |
| Forms | `Field`, `use_form`, `#[derive(Form)]` from the kit |
| Dates and times | the kit's `DatePicker`, `DateRangePicker`, `TimePicker` |
| Strings and locales | `UiStringsProvider` ([RTL.md](RTL.md)) |

Anything with one maintainer goes behind a module of your own, the way the kit wraps its motion engine, so a replacement touches one file.
