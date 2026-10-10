# Guardrails: what can derail this project

These are standing rules for anyone working in this repo, human or AI. Read this file before starting any week's work. Re-check the list at the end of each week. If a rule has to be broken, record why in [Decision log](#decision-log) first.

The goal is a personal shadcn-inspired component library with Motion-style animation, built on Dioxus by **borrowing**, not by inventing. Plan: `research/rust-ui-borrow-plan.md` in the project files.

## Scope traps

- [ ] **Don't modify rustc, fork the compiler, or create a Rust dialect.** Macros and plain Rust are enough.
- [ ] **Don't build a renderer, layout engine, text engine or windowing layer.** The browser and the desktop webview already do this. Blitz or GPUI Kit come later, and only as a deliberate, separate decision.
- [ ] **Don't fork a framework** (uzor, ansiq, Dioxus itself, and so on). We depend on Dioxus and copy component source, nothing more.
- [ ] **Don't build a second framework target "while we're at it".** One framework (Dioxus 0.7). No Leptos, Yew or GPUI ports until the Dioxus kit is finished.
- [ ] **Don't chase "competing with shadcn/Motion".** The bar is "pleasant for my own apps". Stop polishing a component once it's good enough for that.
- [ ] **No new component until the current week's exit criteria pass.** Unfinished components pile up and never get finished.
- [ ] **Don't start on the CLI or registry before week 5.** Until then, copying folders by hand is fine.

## Dependency traps

- [ ] **Every git dependency is pinned to a commit `rev`, never a branch.** dioxus-primitives is git-only and changes without warning.
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

- [ ] **Never change a primitive's structure when restyling.** Change classes and CSS only. The ARIA roles, focus handling and keyboard behavior come from dioxus-primitives, and editing the markup silently breaks them.
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
- [ ] **Update this file and the plan when reality differs.** Don't silently drift.
- [ ] **End every week with a demo:** the gallery running, plus a one-paragraph note of what changed.
- [ ] **No timeline promises shorter than the plan.** If a week slips, cut scope, don't stack weeks.

## Platform notes (learned, keep current)

- dioxus-primitives uses `document::eval` (JavaScript) for focus traps, checkboxes, avatars and context menus. So the kit currently runs only in the **browser and the desktop webview**, not in Blitz/native. That's acceptable for now. Don't add *more* JavaScript of our own.
- DioxusLabs' styled components use plain CSS via `#[css_module]`, not Tailwind. We keep that model. See the decision log.
- Tabs use manual activation: the arrow keys move focus, and Enter or Space selects. That's valid WAI-ARIA, but it differs from Radix's default.
- Dropdown menu items expose `role="option"`, not `menuitem` the way Radix does. Check screen-reader output before relying on it (week 3).
- `dx` in locked-down environments: set `NO_DOWNLOADS=1` and put `esbuild` 0.27.3 and `wasm-bindgen-cli` (matching Cargo.lock) on PATH.

- `AnimatePresence` children need unique, formatted keys (`key: "{id}"`); a static string key is a compile error.
- Layout animation is `FlipList`/`FlipItem` (own FLIP over `get_client_rect`, web and desktop webview). The measured element is the outer wrapper and only the inner one is transformed, otherwise the transform pollutes the measurement. `AnimatePresence` also adds its own `data-dioxus-motion-layout-slot` wrappers; don't style or select them.
- `SwipeDismiss` captures the pointer on web (web-sys `set_pointer_capture` on the event *target*; Dioxus delegates listeners to the root, so `current_target` is the wrong element), so a drag keeps tracking outside the element. Off web it falls back to cancelling on leave. Release speed is passed to the spring (`Spring::set_velocity`), so a flick keeps its momentum. Keyboard alternative: Delete, Backspace or Escape. A pointer held still for 100 ms before release is not a flick.

- Select's trigger is a `<button aria-haspopup="listbox">`, not `role="combobox"` as in Radix. Verify with a screen reader before relying on it.
- Demos must never touch the network (no remote images, no httpbin). The avatar error-state demo requests a missing local path on purpose and shows one 404 in the console.
- `navbar` has no demo: its upstream demo needs a router.
- The command palette has no global Ctrl+K hotkey: it needs a document-level key listener, which means more JS. Open it from a button for now.
- Charts are hand-written SVG. Accessibility comes from `role="img"`, `<title>` and a `<desc>` text summary, not per-point focus.
- Text colours must pass WCAG AA in both themes. Use `--error-text-color` (not `--secondary-error-color`, which is a fill) for error text. `--secondary-color-5` in light was darkened to #6b6b6b for contrast; upstream's #848484 fails.
- The registry (`registry.json`) is generated; never hand-edit it. The CLI copies from a local checkout or a git URL (cloned with the `git` command, cached in `~/.cache/rust-ui`).
- The gallery has its own `apps/gallery/index.html` (lang, no Google Fonts import). Keep `{app_title}` and `{style_include}`; dx injects the script itself.
- Native (Blitz): `cargo check -p gallery --no-default-features --features native` compiles against `dioxus-native` 0.7.10. It needed `cargo update -p image --precise 0.25.6` (Blitz pins `image =0.25.6`; the lockfile now carries that). It has **not been run**: this sandbox has no GPU and lacks libxkbcommon-x11. Primitives use `document::eval`, so expect dialogs, focus traps and menus not to work there. Treat native as unproven.
- Release web bundles need `wasm-opt` (dx downloads it; offline, `npm i -g binaryen` provides one). `.github/workflows/pages.yml` publishes the gallery with `--base-path Rust-UI-`; it is manual and unrun on GitHub (the release bundle was served and loaded under that base path locally).
- `apps/site` must stay kit-only: if it needs something the kit lacks, add that to the kit (or log the gap here) instead of hand-rolling it in the site. Its only own code is page layout CSS and two `web-sys` calls (theme attribute, `location.hash`), both web-only.

- Spring easings (`--spring-gentle|snappy|bouncy` and `-duration`) in theme.css are generated from `motion::SpringPreset` (`motion::css`); `tests/motion_css.rs` fails on drift, `UPDATE_MOTION_CSS=1` regenerates. Use them for CSS animations instead of ad-hoc `150ms ease-out`, with a plain fallback: `var(--spring-snappy, ease-out)`.
- `crates/ui/tests/aria.rs` checks every demo's SSR HTML for broken idrefs, duplicate ids, unknown `aria-*`/roles, missing names and missing required states. It complements axe; it does not replace it. Known issues sit in its `KNOWN` list with a reason.

## Decision log

| Date | Decision | Why |
|---|---|---|
| 2026-10-10 | Use Dioxus 0.7 and dioxus-primitives as the base; no custom renderer. | Research showed that's 6–12+ months saved. |
| 2026-10-10 | Week 1 keeps DioxusLabs' plain-CSS modules and CSS-variable theme instead of rewriting to Tailwind. | The upstream components are already shadcn-styled in CSS. Rewriting 15 components into Tailwind would be churn with no visible gain. Tailwind can be added for app layouts later. |
| 2026-10-10 | Icons come from `dioxus-icons` (Lucide). | The same crate the upstream gallery uses. |
| 2026-10-10 | Motion engine is `dioxus-motion` pinned `=0.4.0`, wrapped by `crates/motion`. Overlays keep CSS animations. | One maintainer and an unstable API: the wrapper limits blast radius. CSS also keeps the native (Blitz) door open. |
| 2026-10-10 | Registry is generated from component folders; the CLI is a small `clap` tool, not `dx components`. | Could not confirm `dx components` accepts a third-party registry; ~200 lines of our own is cheaper than depending on that. |
| 2026-10-10 | Light `--secondary-color-5` changed from #848484 to #6b6b6b; added `--error-text-color`. | axe found contrast failures (3.7:1) in muted and error text. |
