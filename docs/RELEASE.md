# Releasing Atoll

The plan for the first tag, `v0.1.0`, and the routine for every release after it. Version rules are in [VERSIONING.md](VERSIONING.md), the crates.io steps in [PUBLISHING.md](PUBLISHING.md).

## v0.1.0: the first release

**What it is.** The first version anyone can depend on by number. Everything listed as stable in [STABILITY.md](STABILITY.md) is covered by the semver promise from this tag on: a breaking change to it needs `0.2.0`, a CHANGELOG entry and a MIGRATION.md section. Experimental items ship too and stay free to change in any release, as their docs say.

**What ships.** Six crates at `0.1.0`, published in this order by `scripts/publish.sh --publish`: `atoll-attributes`, `atoll-primitives` (the fork), `atoll-form-derive`, `atoll-motion`, `atoll-ui`, `atoll-cli`. The tag `v0.1.0` on `main`, a GitHub release whose notes are the folded "0.1.0" CHANGELOG section, and the docs site and gallery on GitHub Pages.

**Before the release PR** (each is a PR or a setting, in this order):

1. The open PRs are merged: #62, #63, #64 and the rename PR. `main` is green: `scripts/check.sh --a11y`, the e2e workflow on the three engines, release readiness (consumer, getting-started, semver).
2. The owner does the account steps in [the list below](#what-needs-the-owners-accounts): fork branch fast-forwards, repository public, Pages on, crates.io token.

**The release PR** (one commit, nothing else in it):

1. `cargo run -p atoll-cli -- changelog`: folds `changes/` into CHANGELOG.md and MIGRATION.md. Rename the folded "Unreleased" heading to `## 0.1.0 - <date>` and add an empty `## Unreleased` above it.
2. Remove `publish = false` from `crates/ui/Cargo.toml` (it only guards against an early publish).
3. Change the "Nothing has been released" sentences in CHANGELOG.md, VERSIONING.md and INSTALL.md to point at `v0.1.0`.
4. `scripts/publish.sh` (dry run) and `scripts/semver.sh --release` pass.

**After it merges** (owner, about 15 minutes):

```sh
git checkout main && git pull
git tag -a v0.1.0 -m "Atoll 0.1.0" && git push origin v0.1.0
CARGO_REGISTRY_TOKEN=... scripts/publish.sh --publish
```

Then create the GitHub release from the tag with the 0.1.0 CHANGELOG section as its notes, and run the Pages workflow.

**The follow-up PR** (the day after): drop `git` and `rev` from the `dioxus-primitives` line in `Cargo.toml` and rerun `atoll registry`, so copied components ask for `atoll-primitives = "0.1"` from crates.io ([PUBLISHING.md](PUBLISHING.md#after-the-fork-crates-are-published)). Change the "until the first release" notes in GETTING_STARTED.md and the README quick start to the crates.io commands, and make `atoll create` without `--git` the documented default (it already is in the code).

**Check it worked** from a machine without this repository:

```sh
cargo install atoll-cli --locked
atoll create hello && cd hello && dx serve
```

## After v0.1.0

- **Patch releases** (`0.1.1`, ...) whenever fixes or new experimental API pile up, at most weekly. Same steps, without step 2 of the release PR.
- **`0.2.0`** collects breaking changes to stable items, at most one every six weeks; the previous minor gets security and accessibility fixes for three months from a `release/0.1` branch ([VERSIONING.md](VERSIONING.md#support-while-0x)).
- **`1.0`** when the stable API has gone three minor releases without a break, the kit is on the Dioxus line it will live on (0.8 is coming), and `atoll-primitives` is either upstream or settled as ours.
- **The fork crates** have their own version. Bump and publish `atoll-primitives` before the kit release that needs a new fork fix.

## What needs the owner's accounts

Everything else is done in pull requests. These need a person with the accounts:

1. **Merge the PRs** in order: #62, #63, #64, then the rename PR (it contains the other three, so its diff shrinks as they land).
2. **Fork branches** (GitHub, Amit-1of1 forks): fast-forward `rust-ui/primitives-9a75825` in `dioxus-components` to `cda2513` (branch `claude/project-thread-aqqle7`) and `rust-ui/blitz-dom-0.2.4` in `blitz` to `a095949`. The pinned commits stay reachable either way; this only keeps the canonical branches honest.
3. **Make the repository public** (Settings > General > Danger zone), and the `dioxus-components` fork with it. crates.io sources and `cargo install --git` need both readable. Optionally rename it to `atoll` (GitHub redirects old links; then change `--base-path Rust-UI-` in `.github/workflows/pages.yml` and `REPO` in `crates/cli/src/create.rs`).
4. **Actions and Pages**: allow Actions (and check minutes under Billing), set Settings > Pages > Source to GitHub Actions, then run "Docs and gallery on GitHub Pages".
5. **crates.io**: log in with GitHub at crates.io, create an API token with the "publish-new" and "publish-update" scopes, and run `scripts/publish.sh --publish` with it after tagging. Publishing claims the six `atoll-*` names; until then anyone can take them.
6. **Domain** (optional, about 12 USD a year): register `atollui.dev` (backup `atollui.com`) and point it at Pages (Settings > Pages > Custom domain).
7. **Pro sales** (when Pro launches): the Polar account and the private `atoll-pro` repository ([docs/pro/README.md](pro/README.md)).
