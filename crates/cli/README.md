# rust-ui-cli

Copy-paste CLI for the [Rust UI kit](https://github.com/Amit-1of1/Rust-UI-): it copies components (and the components they depend on) into your Dioxus project, shadcn style.

```sh
cargo install --git https://github.com/Amit-1of1/Rust-UI- rust-ui-cli   # installs `rust-ui`
rust-ui --source https://github.com/Amit-1of1/Rust-UI- list
rust-ui --source https://github.com/Amit-1of1/Rust-UI- init --dest assets/theme.css
rust-ui --source https://github.com/Amit-1of1/Rust-UI- add command --dest src/components
rust-ui --source https://github.com/Amit-1of1/Rust-UI- diff command --dest src/components
```

`rust-ui new <snake_case_name>` (inside this repo only) scaffolds a component: folder, mod/prelude lines, demo, tests, gallery section and `registry.json`.

`--source` is a local checkout or a git URL (shallow-cloned into `~/.cache/rust-ui` and fast-forwarded on later runs; needs `git` on PATH). Set `RUST_UI_SOURCE` to avoid repeating it. `add` keeps files you have edited unless you pass `--force`, and prints the `pub mod` lines for your `components/mod.rs`.

The components themselves are not on crates.io: they depend on a git-pinned `dioxus-primitives`, which crates.io cannot host. That is why this tool copies source instead.
