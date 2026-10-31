# Publishing

Nothing here has been published. This page is the plan and the checklist. Running `cargo publish` or turning on GitHub Pages is the repo owner's decision.

## Where each crate stands

`scripts/publish.sh` (a dry run unless given `--publish`) packages every crate below, builds an app outside this workspace from the packages, and lists what a real run would publish. It passed on 2026-10-30. The release workflow's `consumer` job runs the same build on every PR (`scripts/consumer-check.sh`).

| Crate | Where it comes from | crates.io |
|---|---|---|
| `rust-ui-attributes` | Our fork of DioxusLabs/components (`dioxus-attributes`, renamed) | Ready: packages and verifies |
| `rust-ui-primitives` | Our fork of DioxusLabs/components (`dioxus-primitives`, renamed; library still `dioxus_primitives`) | Ready: packages and verifies |
| `rust-ui-form-derive` | This repo | Ready |
| `rust-ui-motion` | This repo | Ready |
| `rust-ui-kit` | This repo | Ready once the two fork crates are published; `publish = false` until then, so nobody publishes it by accident |
| `rust-ui-cli` | This repo | Ready |
| apps (`gallery`, `docs`, `site`, `trial`, `native`, ...) | This repo | Never (`publish = false`) |

Three things to settle before the first `cargo publish`, because a published name and version can never be taken back:

- **Names.** `rust-ui` on crates.io belongs to an unrelated project, and Rust/UI (rust-ui.com) now has a Dioxus edition. Every `rust-ui-*` name above was still free on 2026-10-30, but a rename should happen before publishing, not after. Renaming is mechanical: the package names in each `Cargo.toml`, `package =` in the root `[workspace.dependencies]`, the fork's two manifests, and `CRATES` in `scripts/semver.sh` and `scripts/consumer-check.sh`.
- **The repo is private.** Publishing puts the source on crates.io for anyone, and every crate's `repository` link points here and at the fork.
- **Who maintains the fork crates.** Publishing `rust-ui-primitives` means we release it when we change the fork. See [the decision](decisions/2026-10-30-publish-primitives-fork.md).

## dioxus-primitives

Checked again on 2026-10-30: crates.io still has `dioxus-primitives` only as an empty `0.0.0` placeholder held by DioxusLabs. Upstream (`DioxusLabs/components`) declares `0.0.1`, has no releases, and its `main` is still `9a75825`, the commit our fork is based on. It depends on `dioxus-attributes` by path, and that crate isn't published either.

On 2026-10-13 we chose to wait. Since 2026-10-14 we carry a fork anyway (docs/FORKS.md), so the main cost of publishing it, owning the code, is already paid. On 2026-10-30 the fork was made publishable under our own names ([decision](decisions/2026-10-30-publish-primitives-fork.md)):

- Fork branch `claude/project-thread-6c30v9` (commit `fd533b7`, to be fast-forwarded into `rust-ui/primitives-9a75825`) renames the packages to `rust-ui-primitives` and `rust-ui-attributes`. It keeps the library names, so every `use dioxus_primitives::...` compiles unchanged. It changes no code.
- The kit depends on it as `dioxus-primitives = { package = "rust-ui-primitives", version = "0.1.0", git = ..., rev = ... }`. Cargo builds from `git`. A packaged kit keeps only `version`, so the published kit depends on `rust-ui-primitives` from crates.io.
- `component.json` names `rust-ui-primitives`, so `dx components add` and `rust-ui add` install the same crate as the kit. Copied components and the kit no longer ship different types.

**Asking upstream first is still the best outcome.** A draft issue asking DioxusLabs to publish both crates is with the owner (not filed). If they publish, switch back: depend on `dioxus-primitives = "<version>"`, and keep the fork only as a `[patch]` for the fixes that native needs.

### After the fork crates are published

1. In the root `Cargo.toml`, delete `git` and `rev` from the `dioxus-primitives` line, keeping `package` and `version`.
2. Run `cargo run -p rust-ui-cli -- registry`. `crates/cli/src/manifest.rs` sees a registry source and writes `{"name": "rust-ui-primitives", "version": ...}` (and `rust-ui-motion` by version) into every `component.json`, so copying a component needs no git at all.
3. `scripts/consumer-check.sh` now fails if any git source is left in an outside app's dependency tree.

## Release checklist (for the owner)

```sh
scripts/check.sh --a11y
scripts/semver.sh --release   # against the last release: is the version bump big enough?
scripts/publish.sh            # dry run: packages everything, builds an outside app from the packages
```

Then, once names and visibility are settled:

1. In the release PR: bump `version` in each crate's `Cargo.toml` and in the root `[workspace.dependencies]` (`ui`, `motion`, `form-derive`) together, as [VERSIONING.md](VERSIONING.md) describes. Fold the changelog (`cargo run -p rust-ui-cli -- changelog`). For the first release only, remove `publish = false` from `crates/ui/Cargo.toml`.
2. Merge it, tag the merge commit `v<version>`, and push the tag.
3. `CARGO_REGISTRY_TOKEN=... scripts/publish.sh --publish`. It publishes in dependency order: `rust-ui-attributes`, `rust-ui-primitives` (from the fork commit pinned in `Cargo.toml`), `rust-ui-form-derive`, `rust-ui-motion`, `rust-ui-kit`, `rust-ui-cli`. It skips any crate already published at that version, so a run that stops half way can be restarted.
4. Do the three steps under "After the fork crates are published" in a follow-up PR.

The fork crates have their own version (`0.1.0`). Bump it in the fork when a fork fix changes them, and publish it before the kit release that needs it.

## Apps outside this repo: what they get

What `scripts/consumer-check.sh` proves on every PR, for an app that depends on the kit:

- **Stock Dioxus from crates.io.** The kit asks for `dioxus = "0.7.10"`, and nothing replaces it. Our `[patch.crates-io]` section (Blitz and `dioxus-native-dom`) only applies inside this workspace, so it never reaches an outside app.
- **No git dependency except the primitives fork**, and none at all once `rust-ui-primitives` is published.
- **Native (Blitz) is the exception.** An app that uses the `native` feature and wants the renderer fixes copies the `[patch.crates-io]` section from this repo's `Cargo.toml` ([FORKS.md](FORKS.md#using-the-fixes-in-an-app-outside-this-repo)). Without it, native works with upstream's known gaps.
- **Split wasm builds** (`dx build --wasm-split`) need our `dx` (`scripts/install-dx.sh`) until the splitter fix is upstream. `dx serve` and ordinary builds work with stock `dx` 0.7.10.

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
scripts/install-dx.sh     # dx 0.7.10 with the wasm splitter fixes (docs/FORKS.md)
scripts/release-web.sh docs --base-path Rust-UI-
scripts/release-web.sh gallery --base-path Rust-UI-/gallery   # split: the first screen loads first
# output: target/dx/docs/release/web/public and target/dx/gallery/release/web/public
```

A release bundle needs `wasm-opt`; `dx` downloads it from GitHub. Where GitHub downloads are blocked, `npm i -g binaryen` provides one.
