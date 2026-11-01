# Your own components on top of Atoll

Atoll covers the patterns that repeat across apps: buttons, fields, menus, dialogs, tables, charts. It will never hold every screen element your product needs, and it shouldn't. This page is for teams building their own components on the kit: when to build one, where it should live, what it must meet, and how to keep it in step with Atoll's releases.

## When to build your own

Build a local component when **any** of these is true:

- It encodes your product's domain: an invoice line, a flight card, a pipeline status. No other app would use it as is.
- It's a composition of kit parts that you repeat in your app (a "user picker" made of `Combobox` and `Avatar`). Wrap it once instead of copying the composition.
- You need it this week and the kit's [roadmap](ROADMAP.md) doesn't have it soon.

Ask the kit instead (a feature request or a component proposal, see [CONTRIBUTING.md](../CONTRIBUTING.md)) when:

- It's a generic pattern most apps need (a tree view, a rich-text toolbar, a timeline) or an interactive primitive with a WAI-ARIA pattern (anything with roving focus, typeahead or a popup).
- An existing component is almost right and a prop or variant would fix it. Ask for the prop; don't fork the component for one difference.
- You hit a bug or an accessibility gap. Report it; meanwhile a local workaround is fine, labelled with a link to the issue.

Don't build it at all when an existing component does the job with a different look: restyle it with tokens, `class` and data attributes ([ANATOMY.md](ANATOMY.md)).

## Horizontal and vertical libraries

| | Horizontal library | Vertical library |
|---|---|---|
| **Who uses it** | Several apps or teams | One app or team |
| **Example** | Atoll itself; a company crate `acme-ui` with your brand theme and shared domain components (account switcher, billing badge) | `src/components/` in one app: the invoice editor's parts |
| **Built from** | Atoll components and tokens only | Atoll and the horizontal library above it |
| **Release** | Versioned crate, semver, CHANGELOG | Ships with the app; no version of its own |
| **Bar** | Everything in "Library expectations" below | Accessibility and tokens; docs optional |

Code moves **up** when a second team copies it: a vertical component that two apps use belongs in the horizontal library, and a horizontal component that most Atoll users would want belongs in Atoll (propose it). Nothing moves down: Atoll never depends on your crates.

A horizontal library can be shaped like Atoll so the same tools work on it: one folder per component with `component.rs`, `style.css`, `docs.md`, and a registry (`atoll registry` run in your repo), so `atoll add --source <your repo>` copies your components the way it copies the kit's. That is how Atoll Pro is delivered.

## Library expectations

The minimum for a local library to ship, horizontal or vertical. Where the kit has a tool or a test to copy, it's named.

**Accessibility**
- Usable by keyboard alone (Tab, Shift+Tab, Enter or Space, Escape, arrows where they apply). Write the keys down as a `contract.toml` ([CONTRACTS.md](CONTRACTS.md)); the kit's runner, `tests/e2e/contracts.mjs`, is the model for a Playwright test of your own.
- Every control has an accessible name; make it a required prop, as the kit does, so leaving it out doesn't compile. `crates/ui/tests/aria.rs` (an SSR check for broken idrefs and missing names) is easy to copy.
- axe passes in light and dark (`tests/e2e/axe.mjs` shows how to run axe-core with Playwright on a built app), and text contrast holds under your theme: `atoll contrast --theme assets/brand.css`.
- Animations stop under `prefers-reduced-motion`. The kit's `theme.css` rule covers CSS animations (`atoll doctor` checks it's there); springs must read `use_reduced_motion`.

**Design**
- Theme tokens only, no hard-coded colours or timings. `atoll doctor` reports hard-coded colours and undefined CSS variables.
- Works in every accent preset and both modes, and with `dir="rtl"` if you ship to RTL locales.
- Composes kit parts instead of re-creating them; never edits a copied primitive's markup.

**Docs and tests** (horizontal libraries)
- A `docs.md` per component with a Component Structure snippet and accessibility notes, and a demo.
- At least one test per component (render, interaction or snapshot).
- A CHANGELOG, and the same breaking-change rules as Atoll ([VERSIONING.md](VERSIONING.md)).

**Ownership**
- A named owner who answers issues, and a stated response time (Atoll's is in [MAINTAINING.md](MAINTAINING.md#service-levels)).
- Copied third-party code is credited, with its licence notice kept.

## Keeping in step with Atoll's releases

Atoll's release rules ([VERSIONING.md](VERSIONING.md), [RELEASE.md](RELEASE.md)) are there so local libraries can plan:

- **Patch releases** (`0.N.x`) never break stable items. Take them freely; Cargo does by itself.
- **Minor releases** (`0.N+1.0` while 0.x) batch breaking changes, at most one every six weeks, each with a MIGRATION.md section. A Dioxus minor upgrade always gets a kit minor of its own.
- **The previous minor** gets security and accessibility fixes for three months, so a library has a quarter to move.

What a horizontal library should do:

1. Depend on Atoll by tag (`tag = "v0.N.x"`) or a version once on crates.io, never a branch.
2. Watch the repository's releases (GitHub "Watch > Custom > Releases") and read the CHANGELOG's "Breaking" section for every minor.
3. Support a new Atoll minor **within one month** of its release, so the apps on top of you aren't blocked from upgrading. Run your tests against Atoll `main` in CI weekly to see breaks early.
4. Keep your own minor in step: when you move to Atoll `0.N+1`, release your own minor too, and say in your CHANGELOG which Atoll minor each of your versions supports.
5. If you copied components (`atoll add`), run `atoll diff` after each Atoll release and `atoll update` to merge upstream fixes into your copies.

If a breaking change in Atoll would cost your library a lot, say so on the issue or discussion before the minor ships; breaking changes are announced on `main` behind deprecations first, which is the window for that.
