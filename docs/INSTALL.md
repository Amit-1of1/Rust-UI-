# Installing components

Two routes, same source files.

## With Dioxus' installer

```sh
dx components add button dialog --git https://github.com/Amit-1of1/Rust-UI-
dx components add --all --git https://github.com/Amit-1of1/Rust-UI-
```

`dx` reads the `component.json` next to each component (generated, never edit by hand) and copies the files into `src/components`, adds the crates they need to your `Cargo.toml` (`dioxus-primitives` at the pinned revision, plus icons, time, palette and so on) and copies `theme.css` into `assets/`. Demos are not copied: they live in `crates/ui/src/demos`, outside the component folders. Add `mod components;` to `main.rs` and load the theme with `document::Stylesheet`.

Components name each other with this repository's URL, so `dialog` pulls in `button` from the same place. The repo is private, so `git` needs access to it.

`scripts/dx-compat.sh` proves this route: it builds a throwaway registry from the working tree, installs every component with `dx components add --all`, and compiles the result. It is slow, so it is not part of `scripts/check.sh`; run it after changing manifests or imports.

## With our own CLI

```sh
cargo run -p rust-ui-cli -- --source <checkout-or-git-url> add button dialog
cargo run -p rust-ui-cli -- --source <checkout-or-git-url> init
```

It copies the same files and resolves component dependencies from `registry.json`. `rust-ui diff` shows which copied files differ from the registry.

## Generated files

`registry.json` and every `component.json` come from `cargo run -p rust-ui-cli -- registry`. A test fails when they are stale.
