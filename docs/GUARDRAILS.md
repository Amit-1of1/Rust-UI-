# Guardrails: what can derail this project

These are standing rules for anyone working in this repo, human or AI. Read this file before starting any week's work. Re-check the list at the end of each week. If a rule has to be broken, record why in [Decision log](#decision-log) first.

The goal is a personal shadcn-inspired component library with Motion-style animation, built on Dioxus by **borrowing**, not by inventing. Plan: `research/rust-ui-borrow-plan.md` in the project files.

## Scope traps

- [ ] **Don't modify rustc, fork the compiler, or create a Rust dialect.** Macros and plain Rust are enough.
- [ ] **Don't build a renderer, layout engine, text engine or windowing layer.** The browser and the desktop webview already do this, and Blitz does it for native. Fixing bugs *in* Blitz (see the next rule) is allowed; writing our own replacement is not.
- [ ] **Forks and patches of dependencies are for fixes, not features.** Allowed since 2026-10-14 (decision log) for the crates we depend on, dioxus-primitives included: Blitz (`blitz-dom`, `blitz-paint`, ...), Dioxus' `dioxus-native`/`dioxus-native-dom`, DioxusLabs/components. Each patch is small, marked `PATCH(rust-ui)`, sits on the exact upstream release or commit we already pin, is written so it could go upstream as is, and is listed in `docs/FORKS.md` with a test. Forks live under `Amit-1of1` (`blitz`, `dioxus`, `dioxus-components`), one branch per crate and release, named `rust-ui/<crate>-<version>`; don't vendor copies into this repo. A patch makes an existing upstream API behave the way it does on the web (or fills an upstream `todo!`/`unimplemented!`); it doesn't add new API. Don't open PRs or issues upstream without the owner's word.
- [ ] **Don't build a second framework target "while we're at it".** One framework (Dioxus 0.7). No Leptos, Yew or GPUI ports until the Dioxus kit is finished.
- [ ] **Don't chase "competing with shadcn/Motion".** The bar is "pleasant for my own apps". Stop polishing a component once it's good enough for that.
- [ ] **No new component until the current week's exit criteria pass.** Unfinished components pile up and never get finished.
- [ ] **Don't start on the CLI or registry before week 5.** Until then, copying folders by hand is fine.

## Dependency traps

- [ ] **Every git dependency is pinned to a commit `rev`, never a branch.** dioxus-primitives is git-only and changes without warning. This includes our own forks: `[patch]` entries point at a fork commit, never at a moving branch.
- [ ] **Pin Dioxus to a minor version (`0.7.x`).** Don't move to 0.8 alphas. Upgrade on purpose, on its own branch, with the gallery as the test.
- [ ] **Wrap every one-maintainer dependency behind our own module** (dioxus-motion, Animato, floating-ui-dioxus). App code imports `rust_ui::motion`, never the crate directly.
- [ ] **Avoid these:**
  - ansiq: no license.
  - spanda and rxy_ui: archived.
  - leptos-shadcn-ui, radix-leptos and leptos-motion (cloud-shuttle): abandoned, and look AI-generated.
  - dxui / dioxus-shadcn: unvetted.
  - Zed's `ui` crate: GPL, so read it but never copy from it.
- [ ] **Check the license before copying any file.** MIT, Apache-2.0, ISC and similar are fine. GPL or "no license" means don't copy. Record every copied source in `THIRD_PARTY.md`.
- [ ] **Don't add a crate for something that takes under 50 lines.** Copy the function and credit the source instead.

## Quality traps

- [ ] **Never change a primitive's structure when restyling.** Change classes and CSS only. The ARIA roles, focus handling and keyboard behavior come from dioxus-primitives, and editing the markup silently breaks them. A behaviour fix that has to live in the primitive goes into the fork as a marked patch (rule above), never into the kit's copy of the markup.
- [ ] **Every component must be usable by keyboard alone.** Check this in the gallery before marking it done: Tab, Shift+Tab, Enter or Space, Escape, and arrow keys where they apply.
- [ ] **Every animation must respect `prefers-reduced-motion`.** Add it in the same change as the animation, not later.
- [ ] **Don't hard-code colors in component CSS.** Use theme tokens (`var(--...)`) so light and dark both work.
- [ ] **`cargo fmt`, `cargo clippy -D warnings` and `cargo test` must pass before every push.** CI runs these, and so must you, before pushing.
- [ ] **The gallery must build for the web (wasm32) on every change.** A component missing from the gallery isn't done.

## AI-generated code traps

- [ ] **Don't line-by-line port TypeScript (Radix or Motion) unless no Rust source exists.** Copy existing Rust first, port algorithms second, and generate from scratch last.
- [ ] **No `unsafe`, no `Arc<Mutex<_>>` in UI code, and no `.unwrap()` on user input.** Reject AI output that uses these to get past the compiler.
- [ ] **Read every generated diff before committing it.** Code that compiles but has wrong focus or event behavior is the most common AI failure here.
- [ ] **When the docs disagree with an AI's claims, believe the docs.** Several "facts" in the original planning chat turned out to be wrong.

## Process traps

- [ ] **Small PRs, one theme each** (one week's slice, or one component family).
- [ ] **Every public API change gets a changelog entry; a breaking one also gets a migration note.** Both go in the branch's own `changes/<topic>.md` and `changes/<topic>.migration.md`, never straight into `CHANGELOG.md` or `docs/MIGRATION.md` ([changes/README.md](../changes/README.md)). New public items start experimental and are marked as `docs/STABILITY.md` says. Don't bump crate versions in feature PRs ([docs/VERSIONING.md](VERSIONING.md)).
- [ ] **Update this file and the plan when reality differs.** Don't silently drift.
- [ ] **End every week with a demo:** the gallery running, plus a one-paragraph note of what changed.
- [ ] **No timeline promises shorter than the plan.** If a week slips, cut scope, don't stack weeks.

## Parallel sessions

Several sessions (and PRs) run at once. Every conflict in PRs #17 to #40 came from two branches adding lines at the same place in a shared file: the top of `CHANGELOG.md` and `docs/MIGRATION.md` (all four PRs open on 2026-10-15), the end of this file (12 merges), the module, prelude, demo and test lists (`lib.rs`, `mod.rs`, `tests/common/mod.rs`, `tests/demos.rs`, `tests/render.rs`), and generated files (`registry.json`, `component.json`, `apps/docs/src/generated.rs`, `Cargo.lock`, snapshots). Those places are now either generated or one file per change, so the steps below are mostly mechanical. Who does what, and the sync bot that merges main into every open PR after each merge, are in [PROTOCOL.md](PROTOCOL.md).

**Starting**

- [ ] **Branch from the latest `main`:** `git fetch origin main && git checkout -B <branch> origin/main`. Never from another open PR's branch, and never from an old checkout.
- [ ] **One PR per session.** If the work needs a change that another open PR is making (a fork bump, a shared helper), say so and wait for that PR to merge instead of copying it.

**While working: add files, don't append to shared ones**

- [ ] **Changelog and migration notes:** `changes/<topic>.md` and `changes/<topic>.migration.md`. `scripts/check.sh` fails if a branch edits `CHANGELOG.md` or `docs/MIGRATION.md`.
- [ ] **Components, demos and their tests:** add the folder, `src/demos/<name>.rs` and `tests/components/<name>.rs` (`rust-ui new` does all three). `crates/ui/build.rs` generates `components::*`, the prelude, `ui::demos::ALL` and the test list, so nothing else needs a line. New tests for an existing component also go in `tests/components/<name>.rs`, not at the end of `render.rs`.
- [ ] **Platform notes and decisions:** a new file, `docs/notes/<topic>.md` or `docs/decisions/<date>-<topic>.md`. The two lists at the end of this file are closed to new lines; edit an existing line there only to correct it.
- [ ] **Generated files** (`registry.json`, every `component.json`, `apps/docs/src/generated.rs`, `llms.txt` and `llms/`, `Cargo.lock`, snapshots) are never edited or merged by hand; regenerate them (`rust-ui registry`, `cargo metadata`, `UPDATE_SNAPSHOTS=1`).
- [ ] **Lists that are still shared** (the gallery's sections, `scripts/check.sh`, `scripts/e2e.sh`, `tests/class.rs`, `apps/*/GAPS.md`, `docs/STABILITY.md`): change only your own lines, and add new lines next to related ones rather than at the end.

**Syncing**

- [ ] **Run `scripts/sync-main.sh` before every push, and whenever GitHub, a notice or the sync bot says the PR conflicts.** It merges `origin/main` (a merge commit, never a rebase or force-push), takes main's copy of each generated file and regenerates it, and lists anything left to resolve by hand; after resolving, `scripts/sync-main.sh --continue`. `scripts/check.sh` fails while the branch is behind `origin/main`.
- [ ] **A conflict in `CHANGELOG.md` or `MIGRATION.md`** means the branch predates fragments: keep main's side and move the branch's entries into `changes/`.

**Merging (the owner merges; sessions never merge)**

- [ ] **One PR at a time, and only when it is green on a branch that contains the latest `main`.** After a merge, wait for the "Sync open PRs with main" workflow to finish; a PR it could not sync gets a comment, and that PR's session syncs (sync-main, check, push) before the next merge.
- [ ] **Order:** fork pins and dependency bumps first (they rewrite every `component.json` and `Cargo.lock`), then shared infrastructure (scripts, CI, build), then features, smallest first. When two PRs touch the same component, merge the one that was ready first and let the other sync.
- [ ] **Before a release,** fold the fragments on `main` in a PR of its own: `cargo run -p rust-ui-cli -- changelog` (with `FOLDING_CHANGELOG=1 scripts/check.sh`).

## Platform notes (learned, keep current)

Closed to new lines (see [Parallel sessions](#parallel-sessions)): new notes go in [docs/notes/](notes/README.md), one file per topic.

- dioxus-primitives uses `document::eval` (JavaScript) for Escape-to-close, outside-click dismissal, focus traps, pointer tracking while dragging, avatars and context menus. Native (Blitz) has no JavaScript, so those stay inert there until the primitives get a Rust path (fork). Don't add *more* JavaScript of our own.
- DioxusLabs' styled components use plain CSS via `#[css_module]`, not Tailwind. We keep that model. See the decision log.
- Tabs use manual activation: the arrow keys move focus, and Enter or Space selects. That's valid WAI-ARIA, but it differs from Radix's default.
- Dropdown menu items expose `role="option"`, not `menuitem` the way Radix does. Check screen-reader output before relying on it (week 3).
- `dx` in locked-down environments: set `NO_DOWNLOADS=1` and put `esbuild` 0.27.3 and `wasm-bindgen-cli` (matching Cargo.lock) on PATH.

- `AnimatePresence` children need unique, formatted keys (`key: "{id}"`); a static string key is a compile error.
- Layout animation is `FlipList`/`FlipItem` (own FLIP over `get_client_rect`, web and desktop webview). The measured element is the outer wrapper and only the inner one is transformed, otherwise the transform pollutes the measurement. `AnimatePresence` also adds its own `data-dioxus-motion-layout-slot` wrappers; don't style or select them.
- `SwipeDismiss` captures the pointer on web (web-sys `set_pointer_capture` on the event *target*; Dioxus delegates listeners to the root, so `current_target` is the wrong element), so a drag keeps tracking outside the element. Off web it falls back to cancelling on leave. Release speed is passed to the spring (`Spring::set_velocity`), so a flick keeps its momentum. Keyboard alternative: Delete, Backspace or Escape. A pointer held still for 100 ms before release is not a flick.

- Select's trigger is a `<button aria-haspopup="listbox">`, not `role="combobox"` as in Radix. Verify with a screen reader before relying on it. `aria_label`/`aria_labelledby` on `Select` are moved to the trigger, which lists itself after the label so the value is still read.
- Primitive ARIA bugs worked around in the styled layer (upstream issue drafts in the project files, `rust-ui/upstream/`): tab panels get `aria-labelledby` pointing at their tab (by index); the tooltip trigger drops `aria-describedby` while closed. The date segment's dangling `aria-labelledby` can't be overridden (the primitive writes it before our attributes, so ours would be a duplicate) and stays in `KNOWN` in `tests/aria.rs`. Upstream ids are private, so components that need ids keep their own counter (`rui-<component>-N`).
- Demos must never touch the network (no remote images, no httpbin). The avatar error-state demo requests a missing local path on purpose and shows one 404 in the console.
- `navbar` has no demo: its upstream demo needs a router.
- Ctrl+K opens the command palette through `CommandDialog { hotkey: "k" }` (which calls `ui::hooks::use_hotkey`): a web-sys `keydown` listener on `window`, written in Rust (no `eval`, no JS of ours). Web only; keep the visible button, which is the only way in on desktop and native.
- Charts are hand-written SVG. Accessibility comes from `role="img"`, `<title>` and a `<desc>` text summary. Per-point keyboard access is opt-in (`keyboard: true`): one tab stop (a `role="group"`), arrow keys move a cursor, and a `role="status"` line names the point; the marks themselves are never focusable, so a page of charts doesn't flood the tab order. Charts v2 adds a `table: true` "Show data" table (native `<details>`) as the keyboard-reachable alternative. Series colours come from `--chart-1` (falls back to the accent) to `--chart-5`; keep new ones at 3:1 against the page in both modes (`tests/theme.rs`), and give line series a dash pattern too so colour isn't the only cue.
- Text colours must pass WCAG AA in both themes. Use `--error-text-color` (not `--secondary-error-color`, which is a fill) for error text. `--secondary-color-5` in light was darkened to #6b6b6b for contrast; upstream's #848484 fails.
- `llms.txt`, `llms-full.txt` and `llms/*.md` are generated by `rust-ui registry` from each component's `docs.md`, public signatures and demo; never hand-edit them. Change the component, its docs or `docs/llms-intro.md` (the hand-written rules at the top) and regenerate.
- The registry (`registry.json`) is generated; never hand-edit it. The CLI copies from a local checkout or a git URL (cloned with the `git` command, cached in `~/.cache/rust-ui`).
- The gallery has its own `apps/gallery/index.html` (lang, no Google Fonts import). Keep `{app_title}` and `{style_include}`; dx injects the script itself.
- Native (Blitz): `cargo check -p gallery --no-default-features --features native` compiles against `dioxus-native` 0.7.10. It needed `cargo update -p image --precise 0.25.6` (Blitz pins `image =0.25.6`; the lockfile now carries that). `apps/native/tests/interactions.rs` drives the demos headless on Blitz (clicks, layout, pixels) inside `cargo test`. Modals, click popups, click controls, hover UI and keyboard focus work. Pointer, hover, focus and `onmounted` events come from our fixes to `blitz-dom`, `blitz-paint` and `dioxus-native-dom` on the `Amit-1of1/blitz` and `Amit-1of1/dioxus` forks (wired in with `[patch.crates-io]`, pinned to commits; `docs/FORKS.md` lists every fix and `apps/native/tests/engine.rs` tests each). `document::eval` still does nothing there; the primitives fork (`Amit-1of1/dioxus-components`) falls back to element events for Escape, outside clicks, focus traps and drags. Results and limits are in `docs/NATIVE.md`; treat native as a preview.
- Native CSS rules that keep the web identical: no grid row around a scrolling list, nothing that only becomes visible through an animation's fill. Full-window overlays use `position: fixed` directly, which needs the patched renderer (docs/FORKS.md); percentage transforms, `pointer-events` and cross-ancestor `z-index` also work there. Behaviour that used `document::eval` goes through `MountedData` (focus, scroll) or element events, with the script kept only for web-only extras.
- Browser checks run on Chromium, Firefox and WebKit (`scripts/e2e.sh --browsers ...`, `.github/workflows/e2e.yml`). Engine gaps are checked as fallbacks, not skipped: without CSS anchor positioning overlays keep their absolute layout (no flip), without `field-sizing` the textarea keeps a fixed height, and WebKit cannot emulate forced colors. Controls revealed on hover must also show on their own `:focus-visible` with a ring, and stay visible where there is no hover. CI tests release builds on Rust 1.97.0: on newer stable (LLVM 23) `strip = true` drops the section wasm-bindgen needs, which will break `scripts/release-web.sh` and `pages.yml` too once they run on it. In tests, keep pointer drags inside the viewport (Firefox's automation drops a release outside it). `tests/e2e/snapshots/aria/` holds the accessibility-tree snapshots; a changed role or name there is an announced change. What still needs a real screen reader is in `docs/SCREEN_READERS.md`.
- Release web bundles need `wasm-opt` (dx downloads it; offline, `npm i -g binaryen` provides one). `.github/workflows/pages.yml` publishes the docs site at `--base-path Rust-UI-` and the gallery at `Rust-UI-/gallery`; it is manual and unrun on GitHub (setup steps in `docs/PUBLISHING.md`).
- `rust-ui-kit` is `publish = false` until our primitives fork is on crates.io as `rust-ui-primitives` (decisions/2026-10-30-publish-primitives-fork.md; release steps in `docs/PUBLISHING.md`, dry run `scripts/publish.sh`). Never run `cargo publish` or change repo settings without the owner asking.
- Release web builds go through `scripts/release-web.sh <app>`: `dx build --release`, then `rust-ui postbuild` (drops wasm debug names, bundles the CSS into one file). The bundle's `<link data-rust-ui-bundle>` lists the files it replaces; `ThemeStyles` reads it and wraps the app's `Document` so components skip those links (Rust and web-sys, no JS). Without the bundle nothing changes.
- Split release builds are the default for the gallery, the dashboard and the trial (each has a `wasm-split` feature that `scripts/release-web.sh` turns on). They need `dx` from `scripts/install-dx.sh`: stock 0.7.10's splitter leaves some lazy parts in the main file and panics on others (docs/FORKS.md); `release-web.sh` fails if a lazy file comes out nearly empty. Make lazy parts few and large: code shared by several lazy parts and not by main is copied into each of them (68 per-demo parts made the gallery 2.4x its unsplit size; one part for everything below the first screen costs +12%). Check a split build with `scripts/check.sh --release`; CI runs the same browser checks on split builds in Chromium, Firefox and WebKit.
- `dx build --wasm-split` needs the linker at `-O0` (`--rustc-args=-C link-arg=-O0`): at -O2 wasm-ld tail-merges identical constants, and dx 0.7.10's splitter zeroes bytes the main module still reads (rsx node paths), which scrambles the first render. A lazy component that holds a `Dialog` must mount closed and open later: mounted already open, it runs the focus-trap `eval` before `focus-trap.js` has loaded, and focus never moves into the dialog.
- `crates/ui/src/components/icons/lucide.rs` is generated; never hand-edit it. Re-run `node scripts/gen-lucide-icons.mjs` (needs `npm`, `tar`, `rustfmt`). Don't name `lucide::ALL` outside tests: it keeps every icon in the binary.
- `apps/site` must stay kit-only: if it needs something the kit lacks, add that to the kit (or log the gap here) instead of hand-rolling it in the site. Its only own code is page layout CSS and two `web-sys` calls (theme attribute, `location.hash`), both web-only.

- Spring easings (`--spring-gentle|snappy|bouncy` and `-duration`) in theme.css are generated from `motion::SpringPreset` (`motion::css`); `tests/motion_css.rs` fails on drift, `UPDATE_MOTION_CSS=1` regenerates. Use them for CSS animations instead of ad-hoc `150ms ease-out`, with a plain fallback: `var(--spring-snappy, ease-out)`.
- Floating content (tooltip, popover, hover card, dropdown, select, combobox, menubar) uses CSS anchor positioning to flip at screen edges, inside `@supports`; older engines keep the absolute layout. `css_module` only renames classes inside `@media`, `@layer` and `@container` (manganis-core `css_module_parser.rs`), so put `@supports` *inside* a rule (CSS nesting), never around it. `tests/e2e/anchor.mjs` checks the flip.
- Playwright is pinned to 1.63.0 (`tests/e2e/package.json`): 1.56.1's WebKit 26.0 traps in our wasm at random (docs/SCREEN_READERS.md). Before moving the pin, run `tests/e2e/start_stress.mjs` on WebKit (150 loads, no trap). A cloud container's preinstalled Chromium is older than the pin; `engine.mjs` falls back to `/opt/pw-browsers/chromium` (or `CHROMIUM_PATH`) when the pinned one is missing.
- In Playwright, open things with a DOM click (`el.click()`) when position matters: its mouse click scrolls the page to "stabilise" the target first.
- Markup snapshots live in `crates/ui/tests/snapshots/`; after an intended markup change run `UPDATE_SNAPSHOTS=1 cargo test -p rust-ui-kit --test snapshots` and review the diff. Calendar and date picker are skipped (they render today's date).
- `rust-ui doctor` fails on `var(--x)` without a fallback when nothing defines `--x` (variables set inline by dioxus-primitives are listed in `PRIMITIVE_VARS`).
- `TimePicker` segments are our own `<input role="spinbutton">`s, not dioxus-primitives' date segments (private and `contenteditable`). Keyboard logic lives in the pure `TimeState::apply` (proptest in `tests/time_picker.rs`). Focus leaving a whole picker is detected with `focusin`/`focusout` plus an effect (`use_focus_exit`), no JavaScript. `tests/e2e/time_picker.mjs` waits by polling for state, never with fixed pauses; prefer that in new e2e scripts.
- `crates/ui/tests/aria.rs` checks every demo's SSR HTML for broken idrefs, duplicate ids, unknown `aria-*`/roles, missing names and missing required states. It complements axe; it does not replace it. Known issues sit in its `KNOWN` list with a reason.

- A component's own `class` goes through `merge_attributes` (dioxus-primitives) with the caller's attributes, never as a separate `class:` next to `..attributes` (that renders two `class` attributes and the browser drops one). Primitive props with a named `class: Option<String>` (radio item, navbar item, popover and dialog content) are joined by hand instead. `crates/ui/tests/class.rs` renders each component with a class; for a new component, put that test in its `tests/components/<name>.rs` instead.
- `Select` and `Combobox` move `id`, `aria-describedby`, `aria-invalid` and `aria-required` (and `aria-label`/`aria-labelledby` for `Select`) from their wrapper to the focusable control, so they can be form fields (`SelectField`, `ComboboxField`).
- Theme mode, accent, density and radius have one source of truth, `use_theme()`; app code should not write `data-theme` itself.
- `ui::demos` is behind the kit's `demos` feature (gallery, docs, apps/native and the kit's tests turn it on), so apps don't ship demo code or demo CSS.
- `AlertDialogAction` closes the dialog before it calls `on_click`; keep "what to act on" in its own signal, not in the open flag.
- Calling `FormField::set` from an effect is fine (it reads with `peek`); keep new `FormField` methods non-reactive for the same reason.
- `apps/booking` is the first app on the Dioxus router (`dioxus` feature `router`). Kit links are plain `<a href>` (a full page load under the router): use the router's `Link` (with `button_look` for a button look) or `onclick` + `navigator()`. See `apps/booking/GAPS.md`.
- In e2e scripts, don't click a "Dark mode" switch to get the dark theme: under `colorScheme: "dark"` it is already on, and the click turns it off.

- Blocks (`crates/ui/src/blocks/<name>/`, `ui::blocks`) are whole screens built only from kit components; if a block needs something the kit lacks, add it to the kit or log the gap, don't hand-roll it in the block. Each block is the page's `main` (or takes `embedded` when a shell already provides one), has exactly one `h1`, and is listed in `BLOCKS` in `crates/ui/tests/common/mod.rs` (aria check, snapshot, `tests/blocks.rs`) and in the gallery's `BLOCKS` (its own page at `#block/<name>`, scanned by `tests/e2e/blocks.mjs`). The registry marks them `"kind": "block"`.

## Decision log

Closed to new rows: new decisions go in [docs/decisions/](decisions/README.md), one file each. Older ones stay here.

| Date | Decision | Why |
|---|---|---|
| 2026-10-10 | Use Dioxus 0.7 and dioxus-primitives as the base; no custom renderer. | Research showed that's 6–12+ months saved. |
| 2026-10-10 | Week 1 keeps DioxusLabs' plain-CSS modules and CSS-variable theme instead of rewriting to Tailwind. | The upstream components are already shadcn-styled in CSS. Rewriting 15 components into Tailwind would be churn with no visible gain. Tailwind can be added for app layouts later. |
| 2026-10-10 | Icons come from `dioxus-icons` (Lucide). | The same crate the upstream gallery uses. |
| 2026-10-10 | Motion engine is `dioxus-motion` pinned `=0.4.0`, wrapped by `crates/motion`. Overlays keep CSS animations. | One maintainer and an unstable API: the wrapper limits blast radius. CSS also keeps the native (Blitz) door open. |
| 2026-10-10 | Registry is generated from component folders; the CLI is a small `clap` tool, not `dx components`. | Could not confirm `dx components` accepts a third-party registry; ~200 lines of our own is cheaper than depending on that. |
| 2026-10-10 | Light `--secondary-color-5` changed from #848484 to #6b6b6b; added `--error-text-color`. | axe found contrast failures (3.7:1) in muted and error text. |
| 2026-10-10 | Allow Rust event listeners on `window` via web-sys (first use: Ctrl+K). Still no JavaScript of our own. | The user approved it ("Allow" on the decision card, 2026-10-10). The guardrail is about JS; web-sys is Rust and the site already used it. |
| 2026-10-11 | Fix primitive keyboard gaps in the styled layer, not the primitive: Slider Home/End (own value copy), dropdown Escape returns focus to the trigger (web-sys `focus()`, same rule as the Ctrl+K listener). | tests/e2e/keyboard.mjs found both; primitive markup stays untouched. Worth reporting upstream. |
| 2026-10-11 | Kit strings come from `UiStringsProvider` (English defaults); `--density` scales control heights. Both are registry items/tokens, so copied components keep working. | Tier 2 ideas 12 and 14. |
| 2026-10-13 | (Replaced on 2026-10-30 by decisions/2026-10-30-publish-primitives-fork.md.) Keep `dioxus-primitives` as a pinned git dependency; `rust-ui-kit` is `publish = false` and installs from git or by copying. `rust-ui-motion` and `rust-ui-cli` are publish-ready but unpublished. | crates.io only has an empty `0.0.0` placeholder for primitives, and upstream also needs `dioxus-attributes` released. Forking ~22,600 lines of a11y code costs more than waiting. Details in docs/PUBLISHING.md. |
| 2026-10-13 | Work around primitive ARIA bugs in the styled layer only when the fix is attribute-level and merges cleanly (tabs, tooltip, select naming); otherwise keep the finding in `tests/aria.rs` `KNOWN` and draft an upstream issue. Never patch or fork the primitive. | User asked to fix the recorded accessibility bugs; GUARDRAILS forbids forks. |
| 2026-10-13 | Icons stay Lucide via `dioxus-icons`, now behind the `icons` registry item; shared hooks are the `hooks` registry item. Neither is in the prelude as a glob (Lucide names clash with components). | Compared with Rust/UI's icon crate in `docs/ICONS.md`. Registry items let `rust-ui add` bring them along with the components that import them. |
| 2026-10-14 | Dependencies may be forked or patched to fix them (Blitz, dioxus-native, dioxus-primitives included), replacing "don't fork a framework" for these crates. The fixes live on branches of forks under `Amit-1of1` (blitz, dioxus, dioxus-components), wired in with `[patch.crates-io]` pinned to commits. | The owner asked for native to be fixed properly and for each dependency to be forked rather than waiting on upstream (project thread, 2026-10-14). The renderer bugs were small and local; waiting would have kept native a preview indefinitely. |
| 2026-10-14 | Removed the `overlay` component (`ModalPortal`/`OverlayLayer`); the kit now relies on the patched renderer for full-window overlays on native. The kit's `native` feature stays, empty. | Coordinator relay of the owner's "start everything" (project thread, 2026-10-14). With the forks, `position: fixed` covers the window, and the native modal tests pass without the layer. |
| 2026-10-14 | Icons are generated into the kit from Lucide (`lucide-static` on npm, ISC) by `scripts/gen-lucide-icons.mjs`; `dioxus-icons` is dropped. | Owner asked for the whole Lucide set translated into a Rust module (project thread, 2026-10-14). Removes a dependency, gets Lucide 1.57 (1,890 icons), and lets the kit own the icon rules (aria-hidden by default, theme sizes, RTL mirroring). Unused icons still drop out of the binary. |
