# Principles

What Atoll values, in the order it gives them up when two collide. Every decision about a component, a prop or a release is checked against this list; when a principle loses, the reason goes in the [decision log](decisions/README.md).

## 1. Accessible by test, not by promise

A component is accessible when a test proves it, not when its docs say so. Every demo passes axe in light and dark, a Tab-only walkthrough and an accessibility-tree snapshot on Chromium, Firefox and WebKit; a component with a keyboard contract (`contract.toml`) has every row run as a test on the web and on native. Required accessible names are required props, so leaving one out is a compile error, not a runtime warning.

*In practice:* a new component isn't done until it passes `scripts/check.sh --a11y`. What tests can't prove (a real screen reader's output) is listed in [SCREEN_READERS.md](SCREEN_READERS.md) instead of being claimed.

## 2. Honest claims

Every number and promise in the README, on the docs site and in a release note is something the repository checks. [CLAIMS.md](CLAIMS.md) is the ledger, and a test fails when a number drifts from what it counts. [HARD_QUESTIONS.md](HARD_QUESTIONS.md) says where Atoll loses. If something isn't tested on a platform, the docs say "untested", not "supported".

## 3. Borrow before inventing

Behaviour comes from `dioxus-primitives` (through our fork), rendering from the browser, the desktop webview or Blitz, motion from a wrapped engine. We copy proven Rust first, port algorithms second and write from scratch last. No compiler fork, no renderer of our own ([GUARDRAILS.md](GUARDRAILS.md)). A fork of a dependency exists only to fix it, in a patch small enough to go upstream.

## 4. You own the code

A component is plain Rust and plain CSS that `atoll add` copies into your app. Nothing in it phones home, nothing changes under you, and `atoll diff` / `atoll update` show and merge upstream changes when you choose. Depending on the crate is equally supported, pinned to a tag or a commit, never a branch.

## 5. Motion that respects people

Motion explains a change (where a sheet came from, which row moved); it never blocks input and never plays for its own sake. Every animation turns off under `prefers-reduced-motion`, in the same change that adds it. Springs are for what CSS can't do (interruptible, state-driven motion); simple open and close stays in CSS.

## 6. Theme tokens, not hard-coded looks

Colours, radius, density, motion timing and fonts are CSS variables in `theme.css`. Components use tokens only, so light, dark, every accent preset and an app's own brand work without touching component code, and contrast is proved for every preset ([THEMING.md](THEMING.md)).

## 7. Small, stable surface

A new public item starts experimental and becomes stable only after an app has used it ([STABILITY.md](STABILITY.md)). Breaking changes to stable items are batched into rare minor releases with a migration section and, where possible, an automatic `atoll update` ([VERSIONING.md](VERSIONING.md)). A prop is added when two real screens need it, not when one might.

## 8. Built for real apps

Every feature is tried in a real-shaped app before it ships: the gallery, the docs site and the example apps in `apps/` (an admin dashboard, a booking site, a landing page, a cross-platform showcase). Each app's `GAPS.md` records what the kit lacked; those gaps drive the [roadmap](ROADMAP.md).

## When principles collide

Accessibility beats motion and looks. Honesty beats marketing. Stability beats a nicer API: a better name ships as an addition with the old one deprecated, not as a rename. Borrowing beats speed: waiting for an upstream fix is fine when a local workaround would change a primitive's markup.
