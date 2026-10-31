# Installing components

Adding the kit to an app that already runs, with its own CSS? Start with [EXISTING_APP.md](EXISTING_APP.md): the scoped theme, what `atoll init` and `atoll add` change in your app, and how to adopt one screen at a time.

There are two ways to use the kit in a Dioxus 0.7 app. Neither goes through crates.io yet. The kit and its primitives (`atoll-primitives`, our fork of DioxusLabs' `dioxus-primitives`) are ready to publish, and nothing has been published so far ([plan](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/PUBLISHING.md)). Either way your app stays on stock Dioxus from crates.io: the kit patches no Dioxus crate for web or desktop.

- **Copy the source** (the shadcn way): the component files land in your own `src/components`, yours to restyle and change. Use `dx` 0.7.10 or the `atoll` CLI.
- **Depend on the crate**: `atoll-ui` as a normal git dependency, updated by bumping one commit. Best when you want the kit as it is.

Both need read access to `https://github.com/Amit-1of1/Rust-UI-` while it is private: `git` (or Cargo) must be able to clone it with your GitHub credentials.

## Path 1: copy the source

### With Dioxus' installer

```sh
dx components add button dialog --git https://github.com/Amit-1of1/Rust-UI-
dx components add --all --git https://github.com/Amit-1of1/Rust-UI-
```

`dx` resolves each component's dependencies (such as `icons`) from the repository's default branch whatever `--rev` you pass; that branch is `main` since 2026-10-31. `dx` also keeps its clone of a registry and never updates it, so if you ran this before that date and get old components (ones that import `dioxus_icons` or use `css_module`), run `dx components clean` and add them again.

`dx` reads the `component.json` next to each component (generated, never edit by hand) and copies the files into `src/components`, adds the crates they need to your `Cargo.toml` (`atoll-primitives` at the pinned revision, plus icons, time, palette and so on) and copies `theme.css` into `assets/`. Demos are not copied: they live in `crates/ui/src/demos`, outside the component folders. Add `mod components;` to `main.rs` and load the theme with `document::Stylesheet`.

Components name each other with this repository's URL, so `dialog` pulls in `button` from the same place.

`scripts/dx-compat.sh` proves this route: it builds a throwaway registry from the working tree, installs every component with `dx components add --all`, and compiles the result. It is slow, so it is not part of `scripts/check.sh`; run it after changing manifests or imports.

### With our own CLI

```sh
cargo install --git https://github.com/Amit-1of1/Rust-UI- --branch main atoll-cli   # installs `atoll`
export ATOLL_SOURCE=https://github.com/Amit-1of1/Rust-UI-
atoll init                 # theme.css into assets/
atoll add button dialog    # components into src/components
atoll add login_01         # a block (whole screen) into src/blocks, its components into src/components
atoll diff button dialog   # which copied files differ from the registry
atoll update --dry-run     # what updating to the registry's current version would change
atoll update               # update every installed component, keeping your edits
```

It copies the same files and resolves component dependencies from `registry.json`, then wires them in: the `pub mod` lines in `src/components/mod.rs`, `mod components;` in `src/main.rs` (or `lib.rs`), and the crates each component's `component.json` lists, added to your `Cargo.toml` (a crate you already have is never rewritten). `--no-wire` only copies and prints the lines instead. Blocks (`atoll list` marks them) are whole screens: add `mod blocks;` next to `mod components;` and use them like your own code. `dx components add` installs components only. `--source` (or `ATOLL_SOURCE`) is a local checkout or a git URL; without it the CLI looks in the current directory. Inside this repository, `cargo run -p atoll-cli -- <command>` does the same without installing.

#### Updating copied components

`atoll add` records what it copied in `atoll-lock.json` (the source, its commit, and a hash of each file) and keeps an untouched copy of each file in `.atoll/base/`. Commit both with your app. Run the CLI from the project root, where the lockfile is.

`atoll update [names]` (all installed items when no names are given) then brings each file up to the registry's current version with a three-way merge, the way `git merge` does: the recorded copy is the common ancestor, your file is one side and the registry's file is the other.

| You | The registry | `update` |
|---|---|---|
| didn't touch the file | changed it | `updated`: replaced with the new version |
| edited it | changed other lines | `merged`: both changes kept |
| edited it | changed the same lines | `CONFLICT`: written with `<<<<<<< yours` / `\|\|\|\|\|\|\| registry <old>` / `=======` / `>>>>>>> registry <new>` markers; edit them out |
| edited it | didn't change it | `kept` |
| deleted it | anything | `deleted`: not restored |
| had it | removed it | `removed`: your copy stays, no longer tracked |
| - | added a file or a new dependency | `added` (it prints the `pub mod` line for a new component) |

`--dry-run` prints the same list plus a unified diff of every file that would change, and writes nothing. The command exits with status 1 while a file has conflict markers, or when a file differs from the registry but was never recorded (copied by hand or edited before the lockfile existed); compare that one with `atoll diff` and take the registry's copy with `atoll add --force <name>`. Without `--source`, `update` uses the source the lockfile names. `--lock <path>` uses another lockfile.

## Path 2: depend on the crate from git

Pin a commit, as this repo does with its own git dependencies:

```toml
[dependencies]
dioxus = "0.7.10"
ui = { git = "https://github.com/Amit-1of1/Rust-UI-", rev = "<commit>", package = "atoll-ui" }

[features]
default = ["web"]
web = ["dioxus/web", "ui/web"]
desktop = ["dioxus/desktop", "ui/desktop"]
```

`package = "atoll-ui"` with the key `ui` keeps the import short. `atoll-primitives` and `atoll-motion` come along at the revisions this repo pins, so nothing else goes in your `Cargo.toml`. Stay on Dioxus `0.7.x`, the line the kit is built and tested against.

Then load the theme once and use the prelude:

```rust
use dioxus::prelude::*;
use ui::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ThemeStyles {}
        Button { "Hello" }
    }
}
```

Each component's stylesheet is an asset inside the crate; `dx serve` and `dx bundle` collect it, so build with `dx` rather than plain `cargo`. To update, change `rev` and run `cargo update -p atoll-ui`.

## Generated files

`registry.json` and every `component.json` come from `cargo run -p atoll-cli -- registry`. A test fails when they are stale.

## For coding agents

`llms.txt` at the repository root is an index for AI coding agents (the [llms.txt](https://llmstxt.org) shape): what the kit is, the rules for using it, and one line per component linking to `llms/<name>.md`. A component page holds its `docs.md`, the Rust signatures of its public items (props, enums, hooks) and its demo as a working example. `llms-full.txt` is all of it in one file. The same `atoll registry` run writes them, so they can't drift from the components. Without a checkout, the CLI prints the same text:

```sh
atoll docs            # the index
atoll docs dialog     # one component's page
```

Point your agent at `llms.txt` (or paste `atoll docs <name>` into it) before it writes UI code. The rules at the top come from `docs/llms-intro.md`.

Tools that speak MCP (Claude Code, Cursor, VS Code, Codex) can instead run `atoll mcp`, which lets the agent list, search, read and add components itself. Setup for each client is in [MCP.md](MCP.md).
