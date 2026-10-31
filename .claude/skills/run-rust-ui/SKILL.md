---
name: run-rust-ui
description: Build, run, drive and screenshot the Rust UI kit's apps (gallery, docs, site, dashboard, booking, showcase) in headless Chromium, or render components headlessly on native Blitz. Use when asked to run, start, open, click through, screenshot or visually check a component, block or app in this repo, or to confirm a UI change works in the real app and not just in tests.
---

# Run the Rust UI kit

The kit is a Dioxus 0.7 component library; you see it through its apps (`apps/gallery` shows
every component). The agent path is `driver.mjs` in this folder: it serves an app's
`dx build` output, opens it in headless Chromium (Playwright from `tests/e2e`) and runs steps
such as `click:`, `fill:`, `theme:dark`, `aria:` and `ss:`. Screenshots land in
`target/run-rust-ui/`. Paths below are relative to the repo root.

## Prerequisites (fresh cloud container, about 12 minutes once)

```bash
rustup target add wasm32-unknown-unknown
scripts/install-dx.sh                                   # dx 0.7.10 from the fork, ~8.5 min
cargo install wasm-bindgen-cli --version 0.2.129 --locked   # must match Cargo.lock, ~1.5 min
npm i -g esbuild@0.27.3 binaryen
(cd tests/e2e && npm ci)                                # Playwright 1.63.0
export NO_DOWNLOADS=1                                   # every shell that runs dx
```

`dx --version` should print `dioxus 0.7.10 (73a027c)`. Chromium is the container's
`/opt/pw-browsers/chromium`; the driver falls back to it on its own.

## Run (agent path): the driver

```bash
# Steps as arguments (builds the gallery first if target/dx/gallery/debug is missing)
node .claude/skills/run-rust-ui/driver.mjs 'goto:#dialog' 'click:role=button[name="Open dialog"]' \
  'ss:dialog-open' 'press:Escape' 'theme:dark' 'accent:violet' 'ss:gallery-dark'

# Or one step per line on stdin
node .claude/skills/run-rust-ui/driver.mjs <<'EOF'
goto:#dialog
click:role=button[name="Open dialog"]
aria:role=dialog
press:Escape
count:role=dialog
eval:document.querySelectorAll('.gallery-section').length
viewport:390x844
ss:gallery-phone
EOF

# A block page, typing into a field
node .claude/skills/run-rust-ui/driver.mjs 'goto:#block/login_01' "wait:[data-block='login_01'] h1" \
  'fill:role=textbox[name="Email"]=me@example.com' 'ss:block-login'

# Another app; --build rebuilds after you change code (dx build, ~1-2 min incremental)
NO_DOWNLOADS=1 node .claude/skills/run-rust-ui/driver.mjs --app booking --build 'ss:booking-home' 'aria:nav'
node .claude/skills/run-rust-ui/driver.mjs --app booking 'goto:/trips' 'ss:booking-trips' \
  'click:role=link[name="Explore"]' 'eval:location.pathname'
```

Then look at the PNG with the Read tool. `node .claude/skills/run-rust-ui/driver.mjs --help`
lists every step. The ones you will use most:

| Step | Does |
|---|---|
| `goto:#<section>` / `goto:#block/<name>` / `goto:/<route>` | open a gallery section, a block page, or a router path, fresh |
| `click:` `hover:` `fill:<sel>=<text>` `press:<key>` | Playwright selectors: `role=button[name="Save"]`, `text=Open`, `#id`, CSS |
| `theme:dark\|light\|os`, `accent:blue\|green\|violet\|rose\|orange\|none` | sets `data-theme` / `data-accent` on `<html>` |
| `viewport:390x844` | phone size |
| `aria:<sel>` | prints the accessibility tree: the fastest way to check roles, names and states |
| `text:` `count:` `eval:` | print text, match count, or any JS expression |
| `ss:<name>`, `ss:<name>:full`, `ss:<name>@<sel>` | screenshot viewport / whole page / one element, after animations finish |

Every step prints `ok` or `FAIL <reason>`; the exit status is 1 if any failed. Page errors (a
wasm panic) and console errors print as they happen. Flags: `--app`, `--build`, `--release`,
`--dark`, `--reduced-motion`, `--port`.

Apps: `gallery` (default), `site`, `docs`, `dashboard`, `booking`, `showcase`, `trial`. Each
waits for its own ready selector (listed at the top of `driver.mjs`) before the first step.

## Direct invocation (no browser): most component PRs

```bash
cargo test -q -p rust-ui-kit --test render            # SSR render of every component, ~2 min cold
NATIVE_SHOTS=1 cargo test -q -p native --test showcase   # headless Blitz, ~4 min cold
```

The second writes CPU-painted PNGs to `target/native-headless/showcase-*.png` (desktop and phone,
dialog, sheet, menu, dark). `apps/native/tests/interactions.rs` does the same per demo with
`NATIVE_SHOTS=1`. Native is a different renderer from the browser: check both when a change
touches layout or overlays.

## Run (human path)

`cd apps/gallery && dx serve` serves on http://localhost:8080 with hot reload. It needs a
browser on the same machine, so it is no use here; use the driver.

## Full gate

`scripts/check.sh` (fmt, clippy, tests, wasm builds, registry) and `scripts/check.sh --a11y`
(also builds all six web apps and runs every `tests/e2e/*.mjs` on Chromium) are what CLAUDE.md
asks for before pushing. The driver is for looking; the gate is for proving.

## Gotchas

- **dx cannot download anything here.** It fetches wasm-bindgen from GitHub releases and esbuild
  from npm itself, ignoring the container's proxy CA. Without `NO_DOWNLOADS=1` the build dies with
  `Failed to download esbuild from https://registry.npmjs.org/...` (a long backtrace). With it, dx
  uses `esbuild` and `wasm-bindgen` from PATH, which is why those two are installed by hand.
- **The gallery reads `#block/<name>` only on load.** A hash change is a same-document
  navigation, so the driver reloads on every `goto:` that has a `#`.
- **Overlays animate.** A screenshot right after `click:` caught the dialog half faded in, and one
  right after `press:Escape` still showed it fading out. `ss:` now waits for finite animations
  and transitions (spinner, marquee and shimmer loop forever and are skipped). Use `settle` before
  `eval:` if you read geometry mid-animation, or pass `--reduced-motion`.
- **`count:role=dialog` is 0 the moment Escape is pressed**, while the dialog is still painted on
  its way out: the role goes before the pixels.
- **Debug builds try a hot-reload WebSocket** (`/_dioxus?build_id=0`) that only `dx serve`
  answers. The driver hides that one console error; anything else printed is real.
- **The driver serves `target/dx/<app>/debug/web/public` as it is.** After editing Rust or CSS,
  pass `--build` or you are looking at the old build.
- Unknown paths fall back to `index.html`, so router apps (booking) open on deep links like
  `goto:/trips`.

## Troubleshooting

| Symptom | Fix |
|---|---|
| `ERROR Failed to install wasm-bindgen-cli@0.2.129: error sending request` then `Failed to download esbuild` | `export NO_DOWNLOADS=1`, and install both tools as in Prerequisites |
| `wait:` times out on a block page right after `goto:#block/...` | run the current driver (it reloads on hash); quote the selector: `"wait:[data-block='login_01'] h1"` |
| Screenshot shows a dimmed or half-transparent overlay | it was mid-animation; take it with `ss:` (settles) rather than an `eval` screenshot |
