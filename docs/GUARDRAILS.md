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

## Decision log

| Date | Decision | Why |
|---|---|---|
| 2026-10-10 | Use Dioxus 0.7 and dioxus-primitives as the base; no custom renderer. | Research showed that's 6–12+ months saved. |
| 2026-10-10 | Week 1 keeps DioxusLabs' plain-CSS modules and CSS-variable theme instead of rewriting to Tailwind. | The upstream components are already shadcn-styled in CSS. Rewriting 15 components into Tailwind would be churn with no visible gain. Tailwind can be added for app layouts later. |
| 2026-10-10 | Icons come from `dioxus-icons` (Lucide). | The same crate the upstream gallery uses. |
