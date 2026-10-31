# Adding the kit to an app you already have

This guide is for a Dioxus 0.7 app that already runs, has its own CSS and maybe its own
components, and wants to use the kit for some screens without a rewrite. For a new app,
[INSTALL.md](INSTALL.md) is shorter.

The plan is the one a careful team would follow with any UI dependency: adopt it one screen
at a time, keep its styles from leaking into yours, and let its checks prove each screen
before you move on.

## 1. Check the fit (two minutes)

| Question | What to check | If the answer is no |
|---|---|---|
| Is the app on Dioxus `0.7.x`? | `atoll init` prints `dioxus 0.7.x: supported`, or warns | Upgrade Dioxus first, on its own branch. The kit pins the 0.7 line on purpose (GUARDRAILS). |
| Can your build fetch git dependencies? | `cargo fetch` with a `git = ...` dependency | Copy the source (path 1 below): the copied files are yours, and only the primitives crate stays a dependency. Where it comes from and the plan for crates.io are in [PUBLISHING.md](PUBLISHING.md). |
| Web or desktop webview? | your `dioxus` features | Native (Blitz) works but is a preview, and needs the renderer patches in `[patch.crates-io]`: see [FORKS.md](FORKS.md) and [NATIVE.md](NATIVE.md). Web and desktop need none of them. |
| Can you live with 0.x? | [VERSIONING.md](VERSIONING.md) | Copy the source: an update is then a three-way merge you run when you choose (`atoll update`), never a surprise. |

## 2. Pick a path

- **Copy the source** (recommended for an existing app). The component files land in your
  tree, so you can restyle them to match your design, and nothing changes until you run
  `atoll update`. This is also the path with the fewest dependencies.
- **Depend on the crate.** One git dependency, updated by bumping `rev`. Best when you want
  the kit exactly as it is.

## 3. Path 1: copy the source

From the app's root (where its `Cargo.toml` is):

```sh
cargo install --git https://github.com/Amit-1of1/Rust-UI- --branch main atoll-cli
export ATOLL_SOURCE=https://github.com/Amit-1of1/Rust-UI-

atoll init --scoped          # the theme, confined to ThemeScope; checks your Dioxus version and CSS
atoll add dialog select      # the components and everything they need
```

`atoll init` copies `assets/theme.css` and then looks at the app:

- it checks the `dioxus` version in `Cargo.toml`;
- it lists every place your CSS defines something the kit's theme also defines
  (`--radius`, `--primary-color`, `[data-theme]`, `.sr-only`, ...), with file and line;
- it notices Tailwind and says what that means (below);
- it prints the two lines that load the theme.

`atoll add` copies the files and wires them in, so the app builds straight away:

- `pub mod dialog;` (and each dependency's line) in `src/components/mod.rs`, created if missing;
- `mod components;` in `src/main.rs` (or `lib.rs`), after the header comments, if it isn't there;
- the crates the components use, added to `[dependencies]` from each component's
  `component.json` (the same list `dx components add` reads), with your file's comments and
  layout kept. A crate you already depend on is never rewritten: missing features are added,
  and a different source or revision is reported (`kept your ...`), not changed.

Every step is idempotent: running `add` again changes nothing. `--no-wire` only copies the
files and prints the lines instead, for a layout the CLI doesn't know (a workspace with the
UI in its own crate, say). `atoll-lock.json` and `.atoll/base/` record what was copied;
commit them, so `atoll update` can merge the kit's later fixes with your edits.

Then load the theme once and wrap the screens that use the kit:

```rust
use components::theme_customizer::{set_writes_html, ThemeScope}; // `atoll add theme_customizer`

#[component]
fn App() -> Element {
    use_hook(|| set_writes_html(false)); // with --scoped: leave your <html data-theme> alone
    rsx! {
        document::Stylesheet { href: asset!("/assets/theme.css") }
        YourExistingLayout {
            ThemeScope { NewSettingsScreen {} }
        }
    }
}
```

## 4. Path 2: depend on the crate

```toml
[dependencies]
ui = { git = "https://github.com/Amit-1of1/Rust-UI-", rev = "<commit>", package = "atoll-ui" }
```

```rust
use ui::prelude::*;

rsx! {
    ThemeStyles { scoped: true }          // add `reset: true` for the base styles
    YourExistingLayout {
        ThemeScope { NewSettingsScreen {} }
    }
}
```

## 5. Keep the styles apart

The kit's component classes are CSS modules with hashed names, so they never collide with
yours. What can collide is the theme: custom properties on `:root` with common names
(`--radius`, `--primary-color`), the `data-theme`/`data-accent`/`data-density` switches, and
a few page-wide rules (reduced motion, forced colours, `.sr-only`).

| | Page-wide theme (default) | Scoped theme (`--scoped`, `ThemeStyles { scoped: true }`) |
|---|---|---|
| Tokens defined on | `:root` | each `ThemeScope` (`[data-rui-scope]`) |
| Your `:root { --radius }` | changes the kit's look | stays yours; the kit uses its own inside the scope |
| Reduced-motion and forced-colours rules | whole page | inside the scope only |
| `use_theme().set_mode(Dark)` | writes `<html data-theme>` | writes the scopes only, so your own `data-theme` keeps its meaning |
| Your `<html data-theme="dark">` | switches the kit | still switches the kit, so one dark mode drives both |

`theme.scoped.css` is generated from `theme.css` by `atoll registry` (a test fails when it
is stale), so the two never drift. `ThemeScope` is `display: contents`: it adds no box and
doesn't change your layout. Use as many scopes as you like; they share one theme state.

**Base styles are opt-in.** `reset.css` (`--reset`, `reset: true`) sets `box-sizing`, the
page colours and the font the kit is designed and contrast-checked against. An app with its
own reset (Tailwind's preflight, normalize.css) should leave it off.

**Tailwind** works next to the kit: the kit uses no utility classes, and Tailwind doesn't
define the kit's tokens. Use the scoped theme if your Tailwind config also defines
`--radius` or `--primary-color`, which shadcn-style setups often do.

**Release builds:** if you run `atoll postbuild` (one CSS file instead of one per
component), pass `--scoped` when the app uses the scoped theme, so the bundle holds that
one.

## 6. Let the kit's checks prove your screens

These are what make the kit worth adopting rather than writing the components yourself;
point them at your app, not only at the gallery.

- **Accessibility mistakes don't compile.** A `Dialog` without a `title`, an `IconButton`
  without a `label` or an avatar image without `alt` is a compile error, so a migrated screen
  can't lose its accessible names.
- **Your colours are proved readable.** `atoll contrast --theme assets/brand.css` checks
  every text colour, icon and focus ring the components draw against the background it sits
  on, in light and dark, under your overrides. Run it in CI after you restyle anything.
  `atoll theme --brand '#7c3aed'` makes an accent preset that passes from one colour.
- **Keyboard behaviour is a contract.** Each component's `contract.toml` lists its keys, and
  the kit's tests press every one on the web and on native ([CONTRACTS.md](CONTRACTS.md)).
  `atoll contract dialog` prints the table for your QA notes.
- **Coding agents get the real API.** `atoll mcp` lets Claude Code, Cursor or VS Code list,
  read and add components in your app ([MCP.md](MCP.md)); `atoll docs dialog` prints the
  page an agent should read before writing UI code. Agents then use the kit's components
  instead of inventing markup.
- **Motion respects the user.** Every animation honours `prefers-reduced-motion`, and the
  spring tokens (`--spring-snappy`, ...) let your own CSS animations match the kit's.
- **Focus comes back.** Closing a dialog, sheet, drawer or popover from the keyboard puts
  focus back on the control that opened it, on the web and on native, so a keyboard user
  never lands at the top of the page.

## 7. Migrate one screen at a time

1. Pick a leaf screen with few dependencies (settings, a form) and wrap it in `ThemeScope`.
2. Replace its controls with kit components; keep your layout CSS.
3. Run `atoll contrast` with your theme, your tests, and a keyboard pass (Tab, Shift+Tab,
   Enter, Space, Escape, arrows).
4. Ship it, then widen the scope (or move `ThemeScope` up) for the next screen.

Blocks (`atoll add login_01`, `settings_01`, `dashboard_01`) are whole screens to start
from; they go to `src/blocks/` and are your code from then on.

## Known limits

- The kit installs from git until the primitives are on crates.io ([PUBLISHING.md](PUBLISHING.md)).
- Native (Blitz) is a preview: the renderer fixes live on forks ([FORKS.md](FORKS.md)), and
  native Android is not wired up yet; the webview gallery starts on an Android emulator in
  CI (`apps/showcase/README.md`).
- 0.x: breaking changes are listed in `CHANGELOG.md` with a migration note for each.
