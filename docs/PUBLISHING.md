# Publishing

Nothing here has been published. This page is the plan and the checklist; running `cargo publish` or turning on GitHub Pages is a decision for the repo owner.

## Where each crate stands

| Crate | crates.io | Why |
|---|---|---|
| `rust-ui-motion` | Ready (`cargo package` builds it) | Depends only on crates.io crates (`dioxus`, `dioxus-motion =0.4.0`). |
| `rust-ui-cli` | Ready (`cargo package` builds it) | Plain Rust; it copies components from a checkout or git URL at run time. |
| `rust-ui-kit` | Blocked, `publish = false` | Depends on `dioxus-primitives` from git, and crates.io refuses git dependencies. |
| apps (`gallery`, `docs`, `site`, `trial`, `native`) | Never | `publish = false`. |

Two things to settle before the first `cargo publish`, because a published name and version can never be taken back:

- **Names.** `rust-ui` on crates.io belongs to an unrelated project, and `ui-cli` is the CLI of [rust-ui/ui](https://github.com/rust-ui/ui), which some of our components are adapted from. `rust-ui-kit`, `rust-ui-motion` and `rust-ui-cli` are free (checked 2026-10-13), but readers may take them for that project's crates. If that matters, rename before publishing.
- **The repo is private.** Publishing puts the crate's source on crates.io for anyone, and every crate's `repository` link points here. The CLI is also only useful to people who can clone this repo, since it copies components from it. Publish once the repo is public, or accept that the links 404 for others.

## dioxus-primitives: the decision

Checked 2026-10-13:

- crates.io has `dioxus-primitives` only as an empty `0.0.0` placeholder held by DioxusLabs (April 2025).
- Upstream (`DioxusLabs/components`) declares `0.0.1`, has no tags or releases, and depends on `dioxus-attributes`, a path crate that is not on crates.io either. So upstream needs two releases before we can depend on a version.
- Upstream's latest commit is still the one we pin (`9a75825`, 2026-09-07); four commits landed since June.

| Option | Cost | Verdict |
|---|---|---|
| Wait for an upstream release | Nothing now; the kit stays git-installed until DioxusLabs publishes. No date announced. | **Chosen**, together with the next row. |
| Keep git install and document it | Users add one git line, or copy components with `dx components add` / `rust-ui add`, which is how shadcn-style kits are meant to be used anyway. | **Chosen.** Done in [INSTALL.md](INSTALL.md). |
| Fork as `rust-ui-primitives` + `rust-ui-attributes` and publish those | Allowed by the MIT/Apache licence with credit, but we would own about 22,600 lines of focus, keyboard and ARIA code and have to port upstream fixes by hand. `dx components add` users would still get upstream's `dioxus-primitives`, so the two routes would ship different types. It also breaks the GUARDRAILS premise that behaviour comes from dioxus-primitives. | Not now. Revisit only if crates.io becomes a hard requirement and upstream has still not released. |

**Switch-over when upstream publishes** (both `dioxus-primitives` and `dioxus-attributes` on crates.io):

1. In the root `Cargo.toml`, replace the git line with `dioxus-primitives = "=<version>"` matching the commit we test against.
2. In `crates/cli/src/manifest.rs`, emit `{"name": "dioxus-primitives", "version": ...}` instead of the `git`/`rev` pair, so `component.json` files ask for the version, then run `cargo run -p rust-ui-cli -- registry`.
3. Remove `publish = false` from `crates/ui/Cargo.toml`, run the checks below, and publish `rust-ui-kit` after `rust-ui-motion`.

## Release checklist (for the owner)

```sh
scripts/check.sh --a11y
cargo package -p rust-ui-motion -p rust-ui-cli        # builds each crate from its packaged files
cargo publish --dry-run -p rust-ui-motion
cargo publish --dry-run -p rust-ui-cli
# then, once names and visibility are settled:
cargo publish -p rust-ui-motion
cargo publish -p rust-ui-cli
```

Publish `rust-ui-motion` before `rust-ui-kit`, because the kit depends on it. Bump `version` in each crate's `Cargo.toml` and in the root `[workspace.dependencies]` (`ui`, `motion`) together.

## Docs site on GitHub Pages

`.github/workflows/pages.yml` builds `apps/docs` as the landing page and `apps/gallery` under `/gallery/`, then deploys both. Once it runs, the site is at `https://amit-1of1.github.io/Rust-UI-/` and the gallery at `https://amit-1of1.github.io/Rust-UI-/gallery/`.

It has never run: GitHub Actions has not started a runner on this repo. Before the first run, the owner needs to:

1. **Make Pages possible.** GitHub Pages on a private repo needs a paid plan (Pro, Team or Enterprise). On the free plan, make the repo public first.
2. **Turn on Actions.** Settings > Actions > General > allow actions. For a private repo, check that the account has Actions minutes left (Settings > Billing); a runner that never starts usually means Actions is disabled or out of minutes.
3. **Choose the Pages source.** Settings > Pages > Build and deployment > Source: **GitHub Actions**.
4. **Run it.** Actions tab > "Docs and gallery on GitHub Pages" > Run workflow. It is manual on purpose: it builds `dx` from source, which is slow.

The base paths are tied to the repo name. If the repo is renamed, change `--base-path Rust-UI-` in the workflow to the new name.

To reproduce the build locally:

```sh
cargo install dioxus-cli --version 0.7.10 --locked
(cd apps/docs && dx bundle --platform web --release --base-path Rust-UI-)
(cd apps/gallery && dx bundle --platform web --release --base-path Rust-UI-/gallery)
# output: target/dx/docs/release/web/public and target/dx/gallery/release/web/public
```

A release bundle needs `wasm-opt`; `dx` downloads it from GitHub. Where GitHub downloads are blocked, `npm i -g binaryen` provides one.
