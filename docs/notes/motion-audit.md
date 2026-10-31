# Motion audit: what each animation costs the browser

Two checks, one static and one in the browser, prove where the kit's animations run.

**Static: `atoll motion`** (`crates/cli/src/motion.rs`, test `crates/cli/tests/motion.rs`). Every `@keyframes` property and every `transition`/`transition-property` in the kit's 89 stylesheets is classed:

- composite (`transform`, `translate`, `scale`, `rotate`, `opacity`, `filter`): the compositor thread runs it, so a busy main thread doesn't drop its frames;
- paint (colours, shadows, `visibility`, `clip-path`, ...): repainted each frame, no layout;
- layout (sizes, positions, spacing, `all`, anything unknown): the page is laid out again every frame.

A layout animation fails unless its rule (or `@keyframes` block) carries `/* atoll-motion: layout <props> (why) */`. Today: 173 composite, 74 paint, 24 layout, all 24 explained (sidebar collapse, accordion height, toast stack spreading, chip check mark). Every keyframe animation outside the accordion is composite-only. `atoll motion --css app.css` audits an app's own CSS.

The first run found 28 unexplained layout animations. Fixed: button (`transition-property: all`), sidebar rail (`all`), toggle group (`border` shorthand, which includes `border-width`), progress bar (`width` replaced by a full-width bar slid with `translateX`, as shadcn does). The other 24 genuinely need layout and are now explained in place.

**Browser: `tests/e2e/frames.mjs`** (in `scripts/e2e.sh`, all three engines). It runs every row of every component contract (96 rows, 24 components) on the docs site and records each CSS animation and transition that starts, from `animationstart`/`transitionrun` plus `getAnimations()` and `getKeyframes()`:

- every property that actually moved is composite or paint, or explained layout (the comment is read from the same CSS);
- under `prefers-reduced-motion: reduce` nothing runs longer than 1 ms;
- Chromium only: a performance trace (`blink.animations`) gives Chromium's own verdict per animation. Every composite-only CSS animation must run on the compositor (no `compositeFailed`). Frame times are measured with the CPU slowed 4x.

Measured 2026-10-31 (release docs build, headless Chromium 153, 4-core cloud VM; `bench/results/frames-2026-10-31-chromium.json`): 456 animations and transitions in 96 rows; 124 composite, 373 paint and 6 explained layout property runs (accordion); **70 of 70 composite-only CSS animations ran on the compositor**; frames while animating at 4x CPU slowdown: median 16.7 ms, p95 16.8 to 33.3 ms between runs, 4 to 6% over 25 ms (the slow ones are the first frame of a page's first overlay); reduced motion: 921 started, 0 longer than 1 ms. Frame times are only asserted with `FRAME_BUDGET=1` (local runs); CI machines are too noisy.

Gotchas: Chromium's trace reuses an animation's id after it ends, so a new `b` event starts a new record; the keyframes name is in `args.data.displayName`. Engines report shorthands as longhands (`border-color` arrives as four `border-*-color`); the browser list in `frames.mjs` carries the longhands. `css_module` hashes class names (`dx-sidebar-gap-1a2b3c4d`) but not keyframes names.
