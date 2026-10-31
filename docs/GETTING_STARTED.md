# Getting started

From nothing to a running Atoll app in about five minutes: three commands, then edit `src/main.rs`. You need Rust 1.88 or newer (`rustup update`) and the WebAssembly target.

## 1. Install the tools (once)

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.10 --locked     # the `dx` command
cargo install atoll-cli --locked                       # the `atoll` command
```

Until the first release is on crates.io, install the CLI from git instead:

```sh
cargo install --git https://github.com/Amit-1of1/Rust-UI- --branch main atoll-cli --locked
```

## 2. Create an app

```sh
atoll create my-app
cd my-app
dx serve            # http://localhost:8080
```

`atoll create` writes a normal Dioxus 0.7 app: `Cargo.toml`, `Dioxus.toml`, `.gitignore` and a `src/main.rs` with a card, a form field, a dialog and a dark-mode switch, all from the kit. Before the first release, add `--git` so the kit comes from this repository (pin it with `--tag v0.1.0` or `--rev <commit>`):

```sh
atoll create my-app --git --tag v0.1.0
```

The first `dx serve` compiles Dioxus and the kit, which takes a few minutes on a cold machine; after that a save reloads the page in a second or two.

## 3. Build something

`src/main.rs` starts like this:

```rust
use dioxus::prelude::*;
use ui::prelude::*;

#[component]
fn App() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        ThemeStyles { reset: true }   // theme, component styles and base page styles; once, at the root
        Button { onclick: move |_| open.set(true), "Say hello" }
        Dialog {
            open: open(),
            on_open_change: move |v| open.set(v),
            title: "Hello",
            description: "Dialogs trap focus, close on Escape and return focus to the button.",
        }
    }
}
```

Everything in `ui::prelude` is ready to use: buttons, inputs, selects, date pickers, menus, dialogs, sheets, toasts, tables, charts, a command palette and more. Each component's page in these docs has a live demo, its props and its keyboard behaviour.

## Next steps

- **Own the code.** `atoll add dialog` copies a component's source into `src/components/` and wires it in, so you can restyle or change it ([Installing components](#install)). The prelude version stays available for everything you haven't copied.
- **Theme it.** Pick an accent, density and radius, or generate a preset from your logo with `atoll theme --from-image logo.png` ([Theming](#theming)).
- **Desktop.** `dx serve --platform desktop --no-default-features --features desktop` runs the same app in a native window.
- **Already have an app?** `atoll init` and `atoll add` work in an existing project, and the theme can be confined to one part of the page so it never touches your CSS ([EXISTING_APP.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/EXISTING_APP.md)).
- **Using a coding agent?** `atoll skill` installs the kit's agent skill, and `atoll mcp` serves the component docs over MCP ([MCP.md](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/MCP.md)).
- **Whole screens.** `atoll add login_01` (or `signup_01`, `dashboard_01`, `settings_01`) copies a finished screen built from kit components.

## Examples

Real apps built only from the kit, in the repository's `apps/` folder (run any with `dx serve --platform web` inside it):

- `apps/dashboard`: an admin dashboard with a sidebar, charts, data tables, sheets and a command palette.
- `apps/booking`: a routed booking site with search filters, a date-range booking box and a three-step checkout.
- `apps/site`: a landing page with pricing tabs, an FAQ and a validated signup form.
- `apps/showcase`: one app on the web, in a desktop window and in a phone layout.
- `apps/gallery`: every component and block on one page.

## If something goes wrong

| You see | Do this |
|---|---|
| `can't find crate for core` / wasm target missing | `rustup target add wasm32-unknown-unknown` |
| `dx` says the Dioxus versions differ | Install `dioxus-cli` 0.7.10 (the version in step 1); the app is on Dioxus 0.7. |
| `no matching package named atoll-ui found` | The release isn't on crates.io yet: recreate with `atoll create my-app --git`, or change the `ui` line in `Cargo.toml` to `ui = { package = "atoll-ui", git = "https://github.com/Amit-1of1/Rust-UI-", branch = "main" }`. |
| Components render unstyled | `ThemeStyles { reset: true }` must be rendered once at the root, and the app built with `dx`, not plain `cargo run` (the stylesheets are `dx` assets). |
| `atoll create` refuses the folder | It already has a `Cargo.toml`; use `atoll init` and `atoll add` there instead. |

`scripts/getting-started-check.sh` in the repository runs this guide against the current checkout (create, then build for the web and check the desktop build) and fails if a step breaks.
