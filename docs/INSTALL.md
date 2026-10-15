# Installing components

There are two ways to use the kit in a Dioxus 0.7 app. Neither goes through crates.io yet: the kit depends on `dioxus-primitives`, which is only published as a git repository, and crates.io refuses crates with git dependencies ([why, and the plan](https://github.com/Amit-1of1/Rust-UI-/blob/main/docs/PUBLISHING.md)).

- **Copy the source** (the shadcn way): the component files land in your own `src/components`, yours to restyle and change. Use `dx` 0.7.10 or the `rust-ui` CLI.
- **Depend on the crate**: `rust-ui-kit` as a normal git dependency, updated by bumping one commit. Best when you want the kit as it is.

Both need read access to `https://github.com/Amit-1of1/Rust-UI-` while it is private: `git` (or Cargo) must be able to clone it with your GitHub credentials.

## Path 1: copy the source

### With Dioxus' installer

```sh
dx components add button dialog --git https://github.com/Amit-1of1/Rust-UI-
dx components add --all --git https://github.com/Amit-1of1/Rust-UI-
```

`dx` reads the `component.json` next to each component (generated, never edit by hand) and copies the files into `src/components`, adds the crates they need to your `Cargo.toml` (`dioxus-primitives` at the pinned revision, plus icons, time, palette and so on) and copies `theme.css` into `assets/`. Demos are not copied: they live in `crates/ui/src/demos`, outside the component folders. Add `mod components;` to `main.rs` and load the theme with `document::Stylesheet`.

Components name each other with this repository's URL, so `dialog` pulls in `button` from the same place.

`scripts/dx-compat.sh` proves this route: it builds a throwaway registry from the working tree, installs every component with `dx components add --all`, and compiles the result. It is slow, so it is not part of `scripts/check.sh`; run it after changing manifests or imports.

### With our own CLI

```sh
cargo install --git https://github.com/Amit-1of1/Rust-UI- rust-ui-cli   # installs `rust-ui`
export RUST_UI_SOURCE=https://github.com/Amit-1of1/Rust-UI-
rust-ui init                 # theme.css into assets/
rust-ui add button dialog    # components into src/components
rust-ui add login_01         # a block (whole screen) into src/blocks, its components into src/components
rust-ui diff button dialog   # which copied files differ from the registry
```

It copies the same files and resolves component dependencies from `registry.json`. Blocks (`rust-ui list` marks them) are whole screens: add `mod blocks;` next to `mod components;` and use them like your own code. `dx components add` installs components only. `--source` (or `RUST_UI_SOURCE`) is a local checkout or a git URL; without it the CLI looks in the current directory. Inside this repository, `cargo run -p rust-ui-cli -- <command>` does the same without installing.

## Path 2: depend on the crate from git

Pin a commit, as this repo does with its own git dependencies:

```toml
[dependencies]
dioxus = "0.7.10"
ui = { git = "https://github.com/Amit-1of1/Rust-UI-", rev = "<commit>", package = "rust-ui-kit" }

[features]
default = ["web"]
web = ["dioxus/web", "ui/web"]
desktop = ["dioxus/desktop", "ui/desktop"]
```

`package = "rust-ui-kit"` with the key `ui` keeps the import short. `dioxus-primitives` and `rust-ui-motion` come along at the revisions this repo pins, so nothing else goes in your `Cargo.toml`. Stay on Dioxus `0.7.x`, the line the kit is built and tested against.

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

Each component's stylesheet is an asset inside the crate; `dx serve` and `dx bundle` collect it, so build with `dx` rather than plain `cargo`. To update, change `rev` and run `cargo update -p rust-ui-kit`.

## Generated files

`registry.json` and every `component.json` come from `cargo run -p rust-ui-cli -- registry`. A test fails when they are stale.
