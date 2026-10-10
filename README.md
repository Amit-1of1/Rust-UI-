# Rust UI kit

A personal, shadcn-inspired component kit for [Dioxus](https://dioxuslabs.com), with Motion-style animation planned. It borrows instead of inventing: behaviour and accessibility come from [`dioxus-primitives`](https://github.com/DioxusLabs/components), and the styled layer started as a copy of DioxusLabs' components that this repo now owns.

**Read [docs/GUARDRAILS.md](docs/GUARDRAILS.md) before changing anything.**

## Layout

| Path | What |
|---|---|
| `crates/ui` | The kit (`use ui::prelude::*;`). One folder per component: `component.rs`, `style.css`, `docs.md`. |
| `crates/ui/assets/theme.css` | Color tokens for light and dark, plus the reduced-motion rule. |
| `crates/motion` | Motion layer: spring presets, `Pop` enter/exit, `use_spring`, `use_reduced_motion`. The only place that touches the animation engine. |
| `apps/gallery` | Live gallery of every component. A component not shown here isn't done. |

## Run the gallery

```sh
cargo install dioxus-cli --version 0.7.10 --locked   # once
cd apps/gallery
dx serve                     # web, http://localhost:8080
dx serve --platform desktop --no-default-features --features desktop
```

## Checks (run before every push; CI runs the same)

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build -p gallery --target wasm32-unknown-unknown
```

## Use it in an app

```toml
[dependencies]
dioxus = { version = "0.7.10", features = ["web"] }
ui = { git = "https://github.com/Amit-1of1/Rust-UI-", package = "rust-ui-kit", features = ["web"] }
```

```rust
use dioxus::prelude::*;
use ui::prelude::*;

fn App() -> Element {
    rsx! {
        ThemeStyles {}
        Button { variant: ButtonVariant::Outline, "Hello" }
    }
}
```

## Components

Behaviour comes from `dioxus-primitives`; each styled component lives in `crates/ui/src/components/<name>/` with a `<name>_demo.rs` shown in the gallery.

accordion, alert dialog, aspect ratio, avatar, badge, button, calendar, card, checkbox, collapsible, color picker, combobox, context menu, date picker, dialog, drag-and-drop list, dropdown menu, hover card, input, item, label, menubar, navbar, pagination, popover, progress, radio group, scroll area, select, separator, sheet, sidebar, skeleton, slider, switch, tabs, tag group, textarea, toast, toggle, toggle group, toolbar, tooltip, virtual list.

Original to this repo (not copied, accessible by construction): alert, breadcrumb, chart (SVG bar/line with a text summary), command palette, field (labels, hints, errors, `validate`), kbd, spinner, sortable table.

## Registry and CLI

`registry.json` lists every component with its files and the other components it needs. It is generated from the folders (`cargo run -p rust-ui-cli -- registry`), and a test fails if it goes stale.

```sh
cargo install --path crates/cli          # installs `rust-ui`
rust-ui --source /path/to/Rust-UI- list
rust-ui --source /path/to/Rust-UI- init  --dest assets/theme.css
rust-ui --source /path/to/Rust-UI- add command --dest src/components   # also copies dialog, button, kbd
rust-ui --source /path/to/Rust-UI- diff command --dest src/components  # exit 1 if your copy differs
```

`add` never overwrites a file you changed unless you pass `--force`. It prints the `pub mod` lines to paste into your `components/mod.rs`. The source is a local checkout (or `RUST_UI_SOURCE`); fetching over HTTP is not built.

## Accessibility check

`tests/e2e/axe.mjs` runs axe-core (WCAG 2 A/AA plus best practices) on the built gallery in light and dark. See the header of that file for how to run it. It is not in CI yet because CI cannot currently run (see below).

## Motion

```rust
use ui::motion::{AnimatePresence, Pop, SpringPreset, use_spring};

// Springs: retargeting mid-flight keeps momentum; reduced motion makes it instant.
let mut x = use_spring(0.0, SpringPreset::Snappy);
x.to(120.0);

// Presence: children finish their exit before unmounting. Give each a unique key.
rsx! {
    AnimatePresence {
        for id in items() {
            Pop { key: "{id}", "Item {id}" }
        }
    }
}
```

Baseline open/close animation of the overlay components stays in CSS; springs are for things CSS can't do (interruptible motion, exits driven by state).
