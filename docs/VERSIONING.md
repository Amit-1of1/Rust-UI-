# Versioning and breaking changes

How `rust-ui-kit`, `rust-ui-motion` and `rust-ui-cli` change between versions, so apps built on them know what to expect. What each part of the API promises is in [STABILITY.md](STABILITY.md); what changed is in [CHANGELOG.md](../CHANGELOG.md), with upgrade steps in [MIGRATION.md](MIGRATION.md).

## Version numbers

The crates follow [Semantic Versioning](https://semver.org) in its `0.x` form, the same way Cargo reads it:

| Change | Version bump while `0.x` | After `1.0` |
|---|---|---|
| Breaking change to a **stable** item (see below) | minor: `0.1.x` to `0.2.0` | major |
| New stable API, behaviour fix, or any change to an **experimental** item | patch: `0.1.0` to `0.1.1` | minor or patch |
| Docs, tests, internal code | none needed | none needed |

Cargo treats `0.1.x` and `0.2.x` as incompatible, so an app on `0.1` never picks up a breaking release by itself.

- The three crates share one version number and move together, because the kit depends on the motion crate and the CLI copies the kit's source. Bump `version` in each crate's `Cargo.toml` and in the root `[workspace.dependencies]` (`ui`, `motion`) in the same commit.
- A release is a git tag `v<version>` on `main`. Nothing has been tagged or published yet: every crate is `0.1.0`, and the [CHANGELOG](../CHANGELOG.md) collects everything under "Unreleased". The first tag (`v0.1.0`) closes that section. Publishing to crates.io is a separate decision ([PUBLISHING.md](PUBLISHING.md)).
- `1.0` waits until the kit itself is on crates.io (ready once `rust-ui-primitives` is published, see PUBLISHING.md), the stable API has gone three minor releases without a breaking change, and the kit has moved to the Dioxus release it will live on. A Dioxus minor (0.7 to 0.8) is breaking for every app, so it always gets a kit minor of its own and nothing else breaking rides with it.

### Support while `0.x`

What an app that adopts the kit can count on before 1.0:

- **Breaking releases are batched and rare.** Breaks collect on `main` behind deprecations and ship together in one minor release, not more than one every six weeks (a security fix is the exception). Everything between is patch releases, which Cargo picks up by itself and which never break a stable item.
- **The previous minor gets fixes.** For three months after `0.N+1.0`, security fixes and fixes for crashes or broken keyboard and screen-reader behaviour are backported as `0.N.x` patch releases, from a `release/0.N` branch cut at the `v0.N.0` tag.
- **Every break comes with steps.** A MIGRATION.md section with before and after. Where a change can be scripted (a rename), `rust-ui update` applies it to copied components.
- **Rust version (MSRV).** `rust-version` in the workspace (now 1.88) only goes up in a minor release, and never to a Rust release less than six months old.
- **Dioxus.** Each kit minor supports exactly one Dioxus minor (`0.7.x` now). The kit follows Dioxus patch releases within it.

### Apps installing from git

The kit is installed from git or copied, not from crates.io, so a version number alone doesn't protect you. Pin what you depend on:

- **Crate dependency:** pin a tag (`tag = "v0.1.0"`) once tags exist, or a `rev`, never a branch ([INSTALL.md](INSTALL.md)). Read the CHANGELOG between your old and new revision before bumping.
- **Copied components** (`rust-ui add`, `dx components add`): the copy is yours and never changes by itself. `rust-ui diff` shows what changed upstream; the CHANGELOG says which changes are breaking.

## What counts as breaking

For **stable** items, each of these is breaking and needs a minor bump (while `0.x`), a CHANGELOG entry under "Breaking" and a section in MIGRATION.md:

**Rust API**
- Removing or renaming a public item, module, prop, method, enum variant, feature flag or prelude export.
- Changing a prop's type, making an optional prop required, or changing its default value.
- Changing a function signature or a trait's required items.
- Adding a field to a public struct that apps build with a struct literal, unless the struct is `#[non_exhaustive]` or documented to be built with `..Default::default()` (such as `UiStrings`).
- Adding a variant to an enum that is not `#[non_exhaustive]`. The variant enums (`ButtonVariant`, `BadgeVariant` and so on) are `#[non_exhaustive]`, so new variants there are not breaking.
- Moving an item out of `ui::prelude`.

**Rendered output and behaviour**
- Changing which element carries an `id`, a role or an `aria-*` attribute that apps can pass in or target (for example moving `id` from a wrapper to the input).
- Changing a landmark or role the component renders (apps may already provide one, and two landmarks nest).
- Changing what a prop or method *does* in a way that keeps compiling but gives a different result (for example `class` adding instead of replacing, or `reset()` returning to a different value).
- Removing or renaming a `data-*` attribute or a public CSS class name that is documented for styling.

**Theme and CSS**
- Removing or renaming a theme token (`--primary-color`, `--radius`, `--spring-snappy`, ...), a `data-theme` / `data-accent` / `data-density` value, or the global `.sr-only` class.
- Changing how a token must be written by apps that override it (as the move to `light-dark()` pairs did; that one kept the old form working).

**CLI**
- Removing or renaming a `rust-ui` command or flag, or changing its exit code meaning.
- Changing the `registry.json` or `component.json` format in a way older CLIs or `dx components` can't read.
- Renaming a registry item (a component folder name).

Not breaking, even for stable items:
- New components, props with defaults, methods, hooks, tokens or commands.
- Visual changes made only through CSS (spacing, colours within the token system, animation timing) that keep roles, ids and the documented classes.
- Bug fixes that make behaviour match the docs, WAI-ARIA or the primitive. Call them out in the CHANGELOG under "Fixed" when apps may notice.
- Bumping a pinned dependency (`dioxus-primitives` rev, a fork rev, `dioxus` within `0.7.x`) when no kit API changes. A bump that changes a re-exported type (`use_toast`, `ToastOptions`, `ContentSide`, the `dioxus-motion` items) is breaking for those items.
- Anything marked **experimental** or **internal** in [STABILITY.md](STABILITY.md). Experimental changes still get a CHANGELOG line, and a MIGRATION section when the fix isn't obvious.

## Checked by CI

Rules only help if a machine enforces them. `scripts/semver.sh` runs [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) on every PR (the `semver` job in `.github/workflows/release-readiness.yml`). It compares the public API of `rust-ui-kit` and `rust-ui-motion` (with the `web` feature) against the PR's base and fails when:

- the API breaks the way a patch release must not (a removed or renamed item, a changed signature, a new required field, a removed enum variant, and so on), **and**
- the branch doesn't say so: no `changes/<topic>.migration.md` and no `### Breaking` entry in its fragment.

It can't tell stable from experimental items, so an experimental break needs the one-line entry too. That's cheap, and apps using experimental items still learn about it. It doesn't see behaviour, markup or CSS changes. Those stay with review and the browser checks. Before a release, `scripts/semver.sh --release` compares against the last published version (or the last tag, until one exists) and says whether the version bump is big enough.

## Deprecation

When a stable item is replaced, keep the old one working for at least one minor release when that is practical:

1. Mark it `#[deprecated(since = "0.x.0", note = "use `NewThing`; see docs/MIGRATION.md")]` and keep it compiling, delegating to the new code where possible. For a prop, keep the old prop and document it as deprecated in its doc comment (Dioxus props can't carry `#[deprecated]` usefully).
2. Add a "Deprecated" entry in the CHANGELOG and the upgrade steps in MIGRATION.md.
3. Remove it in the next minor release at the earliest, with a "Breaking" entry pointing to the same MIGRATION section.

Skip the deprecation step only when keeping the old form is impossible or would keep a bug alive (a broken ARIA wiring, two `class` attributes). Say so in the CHANGELOG entry.

Behaviour and markup changes can't be deprecated by the compiler. Announce them in the CHANGELOG under "Breaking" with before and after.

## Every change that touches the API

- Add a line to `CHANGELOG.md` under "Unreleased", in the right group (Breaking, Added, Changed, Deprecated, Removed, Fixed), naming the crate and the PR.
- For a breaking change, add a section to `MIGRATION.md` with before/after code.
- New public items start as **experimental** unless they only mirror an existing stable pattern (a new variant, a new prop with a default). Mark them as [STABILITY.md](STABILITY.md#marking-experimental-items) says, and move them to stable in a later release once an app has used them.
- Don't change a crate's `version` in a feature PR. The version is bumped once, in the release commit that also tags it.
