# Contributing to Atoll

Thanks for helping. Bug reports, docs fixes, examples, new props and new components are all welcome. This page explains how the project works, what gets accepted, and how to set up and test a change. How requests are handled and how fast is in [docs/MAINTAINING.md](docs/MAINTAINING.md).

## House rules

Atoll moves more slowly than an app does, on purpose: a component change lands in every app that uses it, so each one has to stay accessible, themeable and backward compatible. Expect that:

- **Every change runs the full gate.** Formatting, clippy with warnings as errors, 500+ Rust tests, the wasm build, and (for UI changes) axe, keyboard contracts and accessibility-tree snapshots on Chromium, Firefox and WebKit. A PR is ready when `scripts/check.sh --a11y` passes.
- **The rules in [docs/GUARDRAILS.md](docs/GUARDRAILS.md) are not negotiable without a decision record.** The ones contributors meet most: never change a primitive's markup or ARIA when restyling; theme tokens only, no hard-coded colours; every animation stops under reduced motion; every component works by keyboard alone; check the licence before copying any file and credit it in `THIRD_PARTY.md`.
- **Public API changes follow [docs/VERSIONING.md](docs/VERSIONING.md).** New items start experimental; a breaking change to a stable item needs a migration note and waits for the next minor release.
- **Big changes start as a proposal**, not a pull request: a new component, a new dependency, a new token family, or anything that changes how several components work. Use the component proposal template; it saves you building something that won't be merged.
- **One change per pull request.** A bug fix, a prop, a component. Small PRs get reviewed quickly ([service levels](docs/MAINTAINING.md#service-levels)).
- **The owner merges.** Pull requests are never merged by their authors, and agents working on the repo follow [docs/PROTOCOL.md](docs/PROTOCOL.md).
- **Be kind and specific.** Critique code, not people. Harassment of any kind gets you removed from the project.

## What gets accepted

| Usually accepted | Usually declined |
|---|---|
| Bug fixes with a test | Visual tweaks to one component that break consistency with the rest |
| Accessibility fixes | A component that only one app needs (build it locally: [docs/LOCAL_LIBRARIES.md](docs/LOCAL_LIBRARIES.md)) |
| A prop with a default that two real use cases need | A prop that duplicates `class`, `style` or an existing variant |
| A new variant (variant enums are `#[non_exhaustive]`) | New JavaScript (the kit has none of its own) |
| Docs, examples, typo fixes | A dependency for something under 50 lines |
| A generic component with an accepted proposal | Ports to another framework (see the roadmap's "Not planned") |

Accepted proposals go on the [roadmap](docs/ROADMAP.md). Declined ones get a reason, and the decision is recorded if the question is likely to come back.

## Setting up

You need Rust (stable, 1.88 or newer) with the wasm target, and `dx` 0.7.10:

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.10 --locked   # or scripts/install-dx.sh for split release builds and --hot-patch
git clone https://github.com/Amit-1of1/Rust-UI- atoll && cd atoll
```

Run the gallery (every component, every demo) while you work:

```sh
cd apps/gallery && dx serve        # http://localhost:8080
```

The docs site is `apps/docs` the same way. For the browser checks you also need Node and the Playwright engines:

```sh
cd tests/e2e && npm ci && npx playwright install chromium firefox webkit
```

On Linux, desktop builds need `libwebkit2gtk-4.1-dev` and `libxdo-dev`; the native renderer tests run headless with no extra setup (`cargo test -p native`).

## Making a change

1. Branch from the latest `main`.
2. Make the change. For a new component, run `cargo run -p atoll-cli -- new <name>`: it creates the component folder, a demo, a test file and a gallery section. Module lists, the prelude and the registry are generated; don't edit them by hand (`registry.json`, `component.json`, `llms*`, `apps/docs/src/generated.rs`). Regenerate with `cargo run -p atoll-cli -- registry`.
3. Describe it in a changelog fragment: a new file `changes/<topic>.md` ([changes/README.md](changes/README.md)). Never edit `CHANGELOG.md` directly. Credit yourself: `- **ui:** \`Slider\` gets a \`step\` prop. Thanks @you. (#123)`.
4. Run the gate:

   ```sh
   scripts/check.sh            # fmt, clippy, tests, wasm builds, registry, doctor
   scripts/check.sh --a11y     # plus the browser checks; needed for any UI change
   ```

5. Open the pull request and fill in its template. CI runs the same checks on all three engines.

### A new component's checklist

- [ ] An accepted proposal (issue) linked from the PR.
- [ ] Built on a `dioxus-primitives` component or native HTML elements; markup of primitives untouched.
- [ ] Theme tokens only; looks right in light, dark and every accent preset.
- [ ] Keyboard: Tab, Shift+Tab, Enter or Space, Escape and arrows where they apply; a `contract.toml` if it follows a WAI-ARIA pattern ([docs/CONTRACTS.md](docs/CONTRACTS.md)).
- [ ] Required accessible names are required props.
- [ ] Reduced motion handled in the same change as any animation.
- [ ] `docs.md` with a description, a **Component Structure** snippet, accessibility notes and a `## Stability` section marking it experimental.
- [ ] A demo, a test in `crates/ui/tests/components/<name>.rs`, a gallery section.
- [ ] A `THIRD_PARTY.md` row for anything copied.

## Reporting bugs and asking for features

Use the [issue chooser](https://github.com/Amit-1of1/Rust-UI-/issues/new/choose): bug report, feature request or component proposal. Each template asks for what triage needs (a reproduction, the platform, what you expected). Questions go to Discussions. Security issues never go in a public issue: see [SECURITY.md](SECURITY.md).

## Recognition

Every merged contribution is credited by handle in the changelog and the release notes, and the monthly "What's new" post thanks contributors and the people whose reports led to fixes ([docs/MAINTAINING.md](docs/MAINTAINING.md#recognition)).

## Licence

Atoll is dual-licensed under MIT or Apache-2.0. By contributing, you agree that your contribution is licensed the same way. Atoll Pro is a separate, paid product; contributions to this repository stay in the free kit.
